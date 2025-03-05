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
    let (center, scale) = match &camera_state.focus_type {
        CameraFocusType::Global(_) => {
            // Get only the body in this case
            let bodies = body_query.iter().map(|t| t.1).collect::<Vec<_>>();
            update_camera_global(bodies, query_window)
        },
        CameraFocusType::BodyCentered(id) => {
            if let Ok(body) = body_query.get_component::<Body>(Entity::from_raw(*id)) {
                (*body.get_position(), projection.scale) // we actually don't have to change the scale here
            } else {
                (Vec2::default(), projection.scale)
            }
        },
        CameraFocusType::Fixed(_) => {
            // TODO
            // We have to ensure position isn't the same ...
            return
        }
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