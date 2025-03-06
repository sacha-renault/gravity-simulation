use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;

use crate::components::SysCamera;

/// Handles mouse wheel events to implement camera zooming functionality.
///
/// This function processes mouse wheel input events and adjusts the orthographic projection
/// scale to create a zoom effect. Scrolling up zooms in by decreasing the projection scale,
/// while scrolling down zooms out by increasing the scale.
///
/// # Parameters
/// * `mouse_wheel_events` - Event reader for mouse wheel input events
/// * `camera_query` - Query for orthographic projection component with the SysCamera marker
///
/// # Note
/// Zoom in is 17% and Zoom out is 20%
/// Thus one zoom in followed but one zoom out is 1.2 * 0.83333333333 = 1.
pub fn handle_wheel_event(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut camera_query: Query<&mut OrthographicProjection, With<SysCamera>>,
) {
    let mut projection = camera_query.single_mut();
    for event in mouse_wheel_events.read() {
        if event.y == 1. {
            projection.scale /= 1.2; // Zoom out 17%
        } else if event.y == -1. {
            projection.scale *= 1.2; // Zoom in 20%
        }
    }
}

pub fn handle_keyboard_event(keyboard_input: Res<Input<KeyCode>>) {

}