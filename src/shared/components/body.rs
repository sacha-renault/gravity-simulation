use core::f32;

use bevy::prelude::*;
use serde::{Deserialize, Deserializer};

use crate::utility::{G, DISTANCE_SCALE, MASS_SCALE, Force};

#[derive(Debug, Copy, Clone, Component)]
pub struct Body {
    position: Vec2,
    speed: Vec2,
    radius: f32,
    density: f32,
}

// Define a helper struct that will deserialize without scaling
#[derive(Deserialize)]
struct BodyHelper {
    position: Vec2,
    speed: Vec2,
    radius: f32,
    density: f32,
}

impl<'de> Deserialize<'de> for Body {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize into the helper struct
        let helper = BodyHelper::deserialize(deserializer)?;
        
        // Use Body::new to apply the scaling consistently
        Ok(Body::new(
            helper.position,
            helper.speed,
            helper.radius,
            helper.density
        ))
    }
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
    pub fn new(position: Vec2, speed: Vec2, radius: f32, density: f32) -> Self {
        Self {
            position: position / DISTANCE_SCALE,
            speed: speed / DISTANCE_SCALE,
            radius: radius / DISTANCE_SCALE,
            density: density / MASS_SCALE
        }
    }

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

    pub fn get_acceleration(&self, force: Force) -> Vec2 {
        force / self.mass()
    }

    pub fn get_speed(&self) -> &Vec2 {
        &self.speed
    }

    pub fn get_position(&self) -> &Vec2 {
        &self.position
    }

    pub fn get_density(&self) -> f32 {
        self.density
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }

    pub fn add_speed_delta(&mut self, speed_delta: Vec2) {
        self.speed += speed_delta;
    }

    pub fn add_position_delta(&mut self, position_delta: Vec2) {
        self.position += position_delta;
    }
}