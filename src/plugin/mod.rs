mod systems;

use bevy::prelude::*;
use systems::*;
use crate::shared::{Body, CameraState, SimulationState};

pub fn setup_bodies(bodies: Vec<Body>) -> impl for<'a, 'b> Fn(
    Commands<'a, 'b>
) {
    move |mut commands| {
        // Spawn a random bodies
        for body in bodies.iter() {
            commands.spawn(*body);
        }
    }
}

pub struct SysPlugin {
    bodies: Vec<Body>
}

impl SysPlugin {
    pub fn new(bodies: Vec<Body>) -> Self {
        Self { bodies }
    }
}

impl Plugin for SysPlugin {
    fn build(&self, app: &mut App) {
        app
            // First insert the resources
            .insert_resource(CameraState::default())
            .insert_resource(SimulationState::default())

            // Add setup and post setup systems
            .add_systems(Startup, (
                setup_bodies(self.bodies.clone()), setup_text_visual))
            .add_systems(PostStartup, setup_camera)

            // System for update body position and the visual
            // TODO: This `FixedUpdate` was added a bit late in the code
            // And therefore, the UI can look not so good without
            // proper transition handling
            .add_systems(FixedUpdate, update_bodies)

            // System for updating ui
            .add_systems(Update, update_camera_position)
            .add_systems(Update, update_text_visual)
            .add_systems(Update,  (setup_body_visuals, update_body_visuals))

            // System for handling user input
            .add_systems(Update, 
                (handle_wheel_event, handle_keyboard_event, handle_mouse_motion_event));
    }
}
