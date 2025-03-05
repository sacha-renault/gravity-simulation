use bevy::prelude::*;

use crate::components::body::Body;

// This system updates the visuals for bodies when they move
pub fn update_body_visuals(
    mut query: Query<(&Body, &mut Transform)>,
) {
    for (body, mut transform) in query.iter_mut() {
        // Update the position to match the body's physics position
        transform.translation.x = body.get_position().x;
        transform.translation.y = body.get_position().y;
    }
}