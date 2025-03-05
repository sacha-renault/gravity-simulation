use bevy::prelude::*;

use crate::components::body::Body;
use crate::utility::utility_funcs::*;
use crate::components::{CameraFocusType, SysCamera};

// This system updates the visuals for bodies when they move
pub fn update_camera_position(
    query_window: Query<&Window>,
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), With<SysCamera>>,
    focus_query: Query<&CameraFocusType>,
    body_query: Query<(Entity, &Body)>
) {
    // Get the state of the focus
    let focus_type = focus_query.single();

    // Get camera and update fields
    let (mut transform, mut projection) = camera_query.single_mut();

    // Get new center / scale
    let (center, scale) = match focus_type {
        CameraFocusType::Global(_) => {
            // Get only the body in this case
            let bodies = body_query.iter().map(|t| t.1).collect::<Vec<_>>();
            update_camera_global(bodies, query_window)
        },
        _ => return // Base case we don't update anything
    };

    transform.translation = Vec3::new(center.x, center.y, transform.translation.z);
    projection.scale = scale;
}

fn update_camera_global(bodies: Vec<&Body>, query_window: Query<&Window>) -> (Vec2, f32){
    let window = query_window.single();
    let bounds = get_window_bounds(&bodies);
    get_camera_setting_on_bounds(
        bounds,
        window.width(),
        window.height(),
        1.2)
}