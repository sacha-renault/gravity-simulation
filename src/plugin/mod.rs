mod systems;

use bevy::prelude::*;
use systems::*;
use crate::components::CameraState;

pub struct SysPlugin;
impl Plugin for SysPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_plugin)
            .add_systems(Update, (setup_body_visuals, update_body_visuals))
            .add_systems(Update, update_bodies)
            .add_systems(Update, update_camera_position)
            .add_systems(Update, (handle_wheel_event, handle_keyboard_event, handle_mouse_event))
            .insert_resource(CameraState::default());
    }
}
