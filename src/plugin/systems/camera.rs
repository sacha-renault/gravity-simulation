use bevy::prelude::*;

use crate::components::body::Body;
use crate::utility::utility_funcs::*;
use crate::components::SysCamera;

// This system updates the visuals for bodies when they move
pub fn update_camera_position(
    query_window: Query<&Window>,
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), With<SysCamera>>,
    body_query: Query<&Body>
) {
    // Get min and max for both x and y
    let bodies = body_query.iter().collect::<Vec<_>>();
    let window = query_window.single();
    let bounds = get_window_bounds(&bodies);
    let (center, scale) = get_camera_setting_on_bounds(
        bounds,
        window.width(),
        window.height(),
        1.2);

    // Get camera
    let (mut transform, mut projection) = camera_query.single_mut();
    transform.translation = Vec3::new(center.x, center.y, transform.translation.z);
    projection.scale = scale;
}