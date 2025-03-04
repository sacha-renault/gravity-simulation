use core::f32;

use bevy::prelude::*;

use crate::physics::{G, Force};

#[derive(Debug, Copy, Clone, Component)]
pub struct Body {
    position: Vec2,
    speed: Vec2,
    radius: f32,
    density: f32,
}

impl Default for Body {
    fn default() -> Self {
        Self {
            position: Vec2::default(),
            speed: Vec2::default(),
            radius: 1.0,
            density: 1.0
        }
    }
}

impl Body {
    pub fn volume(&self) -> f32 {
        self.radius.powi(3) * 4. / 3. * f32::consts::PI
    }

    pub fn mass(&self) -> f32 {
        self.volume() * self.density
    }

    pub fn gravity_force(&self, rhs: &Self) -> Force {
        let position_offset = self.position - rhs.position;
        let distance = position_offset.length();

        // Avoid division by zero
        if distance < f32::EPSILON {
            return Vec2::ZERO;
        }

        // Calculate magnitude using Newton's law of gravitation: F = G * m1 * m2 / r²
        let force_magnitude = G * self.mass() * rhs.mass() / (distance * distance);

        // Direction is along the vector between the objects (normalized)
        let direction = -position_offset.normalize();

        // Apply magnitude to direction to get the force vector
        direction * force_magnitude
    }
}