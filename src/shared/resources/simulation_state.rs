use bevy::prelude::*;

#[derive(Resource)]
pub struct SimulationState {
    pub paused: bool,
    pub time_factor: f32,
    pub selected_body_id: Option<Entity>
}

impl Default for SimulationState {
    fn default() -> Self {
        Self {
            paused: true,
            time_factor: 1e2,
            selected_body_id: default()
        }
    }
}