use bevy::prelude::*;

use crate::shared::{SimulationState, Body};
use crate::utility::Force;

pub fn update_bodies(
    time: Res<Time>,
    mut body_query: Query<&mut Body>,
    simulation_state: Res<SimulationState>
) {
    // Early exit if simulation is paused
    if simulation_state.paused {
        return;
    }

    let delta = time.delta_seconds() * simulation_state.time_factor;
    let bodies = body_query.iter().collect::<Vec<_>>();
    let mut sum_force: Vec<Force> = vec![default(); bodies.len()];

    // Calculate the sum of force
    for (i1, b1) in bodies.iter().enumerate() {
        for (i2, b2) in bodies.iter().enumerate().skip(i1 + 1) {
            let g_force = b1.gravity_force(b2);
            sum_force[i1] += g_force;
            sum_force[i2] -= g_force;
        }
    }

    // From new acceleration, modify speed and position
    for (index, mut body) in body_query.iter_mut().enumerate() {
        // calculate acceleeratino from newton laws Sum(F) = m*a <=> a = Sum(F) / m
        let force = sum_force[index];
        let acc = body.get_acceleration(force);

        // Calculate the speed difference
        let speed_delta = delta * acc;
        body.add_speed_delta(speed_delta);

        // Calculate the position difference
        let position_delta = delta * delta * acc / 2. + delta * (*body.get_speed());
        body.add_position_delta(position_delta);
    }
}