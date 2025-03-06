use bevy::prelude::*;

use crate::components::body::Body;
use crate::utility::utility_funcs::*;
use crate::components::{SysCamera, CameraState, CameraFocusType};

// This system updates the visuals for bodies when they move
pub fn update_camera_position(
    query_window: Query<&Window>,
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), With<SysCamera>>,
    camera_state: Res<CameraState>,
    body_query: Query<(Entity, &Body)>
) {
    // Get camera and update fields
    let (mut transform, mut projection) = camera_query.single_mut();

    // Get new center / scale
    match &camera_state.focus_type {
        CameraFocusType::Global(margin) => {
            // Get only the body in this case
            let bodies = body_query.iter().map(|t| t.1).collect::<Vec<_>>();
            let (center, scale) = get_camera_global_settings(bodies, query_window, *margin);
            transform.translation = Vec3::new(center.x, center.y, transform.translation.z);
            projection.scale = scale;
        },
        CameraFocusType::BodyCentered(id, scale) => {
            if let Ok(body) = body_query.get_component::<Body>(Entity::from_raw(*id)) {
                let pos = *body.get_position();
                transform.translation = Vec3::new(pos.x, pos.y, transform.translation.z);
                projection.scale = *scale;
            }
            // If the body wasn't found, we just don't do anything ...
        },
        CameraFocusType::Fixed(pos, scale) => {
            transform.translation = Vec3::new(pos.x, pos.y, transform.translation.z);
            projection.scale = *scale;
        },
        CameraFocusType::FixedAutoScale(pos, margin) => {
            let bodies = body_query.iter().map(|t| t.1).collect::<Vec<_>>();
            let scale = get_camera_fixed_settings(*pos, bodies, query_window, *margin);
            transform.translation = Vec3::new(pos.x, pos.y, transform.translation.z);
            projection.scale = scale;
        }
    };
}

fn get_camera_global_settings(bodies: Vec<&Body>, query_window: Query<&Window>, margin: f32) -> (Vec2, f32){
    let window = query_window.single();
    let bounds = get_window_bounds(&bodies);
    get_camera_setting_on_bounds(
        bounds,
        window.width(),
        window.height(),
        margin)
}

fn get_camera_fixed_settings(center: Vec2, bodies: Vec<&Body>, query_window: Query<&Window>, margin: f32) -> f32 {
    let window = query_window.single();
    let bounds = get_window_bounds(&bodies);
    get_camera_settings_on_center(
        bounds,
        center,
        window.width(),
        window.height(),
        margin)
}