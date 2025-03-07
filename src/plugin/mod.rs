mod systems;

use bevy::prelude::*;
use systems::*;
use crate::shared::{CameraState, SimulationState};

pub struct SysPlugin;
impl Plugin for SysPlugin {
    fn build(&self, app: &mut App) {
        app
            // First insert the resources
            .insert_resource(CameraState::default())
            .insert_resource(SimulationState::default())

            // Add setup and post setup systems
            .add_systems(Startup, (setup_plugin, setup_text_visual))
            .add_systems(PostStartup, setup_camera)

            // System for update body position and the visual 
            .add_systems(Update, update_bodies)

            // System for updating ui
            .add_systems(Update, update_camera_position)
            .add_systems(Update, update_text_visual)
            .add_systems(Update,  (setup_body_visuals, update_body_visuals))

            // System for handling user input
            .add_systems(Update, 
                (handle_wheel_event, handle_keyboard_event, handle_mouse_motion_event));
    }
}
