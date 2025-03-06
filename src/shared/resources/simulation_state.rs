use bevy::prelude::*;

#[derive(Resource)]
pub struct SimulationState {
    pub paused: bool,
    pub time_factor: f32,
}

impl Default for SimulationState {
    fn default() -> Self {
        Self {
            paused: false,
            time_factor: 1e4
        }
    }
}