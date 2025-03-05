pub mod camera;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use camera::*;
use crate::components::body::Body;
use crate::components::SysCamera;
use crate::utility::Force;
use crate::utility::utility_funcs::*;

pub fn setup_plugin(mut commands: Commands, query_window: Query<&Window>) {
    // Body init
    // TODO
    // Find a better way that hardcoding those values in a vec
    let bodies = vec![
        // Sun - normal astronomical values
        Body::new(
            Vec2::new(0.0, 0.0),       // Position at origin
            Vec2::new(0.0, 0.0),       // Not moving
            6.96340e8,                 // Radius in meters (~696,340 km)
            1408.0                     // Density in kg/m³
        ),
        // Earth - normal astronomical values
        Body::new(
            Vec2::new(149.6e9, 0.0),   // ~149.6 million km from Sun in meters
            Vec2::new(0.0, 29.78e3),   // Orbital velocity of ~29.78 km/s in m/s
            6.378e6,                   // Radius in meters (~6,378 km)
            5515.0                     // Density in kg/m³
        ),
    ];

    // Get min and max for both x and y
    let window = query_window.single();
    let bounds = get_window_bounds(&bodies.iter().collect());
    let (center, scale) = get_camera_setting_on_bounds(
        bounds,
        window.width(),
        window.height(),
        1.2);

    // Spawn a random bodies
    for body in bodies.into_iter() {
        commands.spawn(body);
    }

    // Spawn a camera for UI
    // AFTER Bodies are created so we can actually
    // Size it correctly
    let mut cam = Camera2dBundle::default();
    cam.transform.translation = Vec3::new(center.x, center.y, cam.transform.translation.z);
    cam.projection.scale = scale;
    commands.spawn((cam, SysCamera));
}

pub fn update_bodies(
    time: Res<Time>,
    mut body_query: Query<&mut Body>
) {
    let delta = time.delta_seconds() * 1e6;
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

pub fn setup_body_visuals(
    mut commands: Commands,
    query: Query<(Entity, &Body), Added<Body>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, body) in query.iter() {
        // Calculate color based on density (you can customize this)
        let hue = (body.get_density() * 0.5) % 1.0;
        let color = Color::hsl(hue * 360.0, 0.8, 0.5);

        // Create a circular mesh for the body
        commands.entity(entity).insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(body.get_radius() * 1.).into()).into(),
            material: materials.add(ColorMaterial::from(color)),
            transform: Transform::from_translation(Vec3::new(body.get_position().x, body.get_position().y, 0.0)),
            ..default()
        });
    }
}

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