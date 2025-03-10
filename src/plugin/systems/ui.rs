use bevy::prelude::*;

use crate::shared::{components::any::BottomText, Body, CameraState, SimulationState};

// This system updates the visuals for bodies when they move
pub fn update_body_visuals(
    mut query: Query<(&Body, &mut Transform)>,
) {
    for (body, mut transform) in query.iter_mut() {
        // Update the position to match the body's physics position
        transform.translation.x = body.get_position().x;
        transform.translation.y = body.get_position().y;
    }
}

pub fn setup_text_visual(mut commands: Commands, _asset_server: Res<AssetServer>) {
    // Load a font
    // let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    
    // Spawn the text bundle
    commands.spawn((
        Text::new(""),
        TextFont {
            font_size: 15.,
            ..default()
        } ,
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        },
        BottomText,
    ));
}

pub fn update_text_visual(
    game_state: Res<SimulationState>,
    camera_state: Res<CameraState>,
    mut query: Query<&mut Text, With<BottomText>>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        let time_factor = game_state.time_factor.to_string();
        let next_text = format!(
            "Center : ({}, {}) ; Scale : {} ; Time factor : {}",
            camera_state.position().x, camera_state.position().y,
            camera_state.scale(),
            if !game_state.paused { time_factor } else { format!("{}, (Paused)", time_factor) },
        );
        **text = next_text;
    }
}