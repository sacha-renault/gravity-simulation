use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};

use crate::shared::{CameraState, SimulationState};

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
    mut camera_state: ResMut<CameraState>,
) {
    for event in mouse_wheel_events.read() {
        if event.y == 1. {
            camera_state.set_scale_by(1. / 1.2); // Zoom out 17%
        } else if event.y == -1. {
            camera_state.set_scale_by(1.2); // Zoom in 20%
        }
    }
}

/// Handles mouse movement for camera panning when the left mouse button is pressed.
///
/// This function captures mouse motion events and applies them to the camera's transform
/// when the left mouse button is being held down, enabling camera panning in a 2D scene.
/// Motion events are cleared when the button is released to prevent unintended movement.
///
/// # Parameters
/// * `mouse_button_input` - Resource for detecting mouse button press states
/// * `mouse_motion_events` - Event reader for tracking mouse movement deltas
/// * `camera_query` - Query for the camera's transform component with the SysCamera marker
///
/// # Behavior
/// * When left mouse button is pressed: Accumulates all motion deltas and updates camera position
/// * When left mouse button is released: Clears motion events to reset tracking
pub fn handle_mouse_motion_event(
    mouse_button_input: Res<Input<MouseButton>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut camera_state: ResMut<CameraState>,
) {
    // Calculate total mouse delta since last frame
    if mouse_button_input.pressed(MouseButton::Left) {
        let raw_delta: Vec2 = mouse_motion_events
            .read()
            .map(|event| event.delta)
            .sum();
        let scale = camera_state.scale();
        let delta = Vec2::new(-raw_delta.x, raw_delta.y);
        camera_state.translate_position_by(delta * scale);
    } else {
        mouse_motion_events.clear();
    }
}

pub fn handle_keyboard_event(
    keyboard_input: Res<Input<KeyCode>>,
    mut state: ResMut<SimulationState>,
) {
    if keyboard_input.pressed(KeyCode::NumpadAdd) {
        state.time_factor *= 1.01;
    }
    if keyboard_input.pressed(KeyCode::NumpadSubtract) {
        state.time_factor /= 1.01;
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        state.paused = !state.paused;
    }
}