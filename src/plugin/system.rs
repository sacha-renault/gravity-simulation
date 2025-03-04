use bevy::render::view::window;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::components::body::Body;
use crate::utility::Force;
use crate::utility::utility_funcs::*;

#[derive(Component)]
pub struct SysCamera;

pub fn setup_plugin(mut commands: Commands, query_window: Query<&Window>) {
    // Body init
    // TODO
    // Find a better way that hardcoding those values in a vec
    let bodies = vec![
        // Sun
        Body::new(
            Vec2::new(0.0, 0.0), 
            Vec2::new(0.0, 0.0), 
            0.696340,  // 1.989e30 / 1e24
            1408.0
        ),
        // Earth
        Body::new(
            Vec2::new(149.6, 0.0),  // 149.6e9 / 1e9
            Vec2::new(0.0, 0.02978),  // 29.78e3 / 1e3 (scaled velocity)
            5.972,                  // 5.972e24 / 1e24
            5515.0
        ),
    ];

    // Get min and max for both x and y
    let window = query_window.single();
    let bounds = get_window_bounds(&bodies);
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
    cam.projection.scale = scale * 10.;
    println!("{}, {}", center, scale);
    commands.spawn((cam, SysCamera));
}

pub fn update_bodies(
    mut commands: Commands, 
    time: Res<Time>,
    mut body_query: Query<&mut Body>
) {
    let delta = time.delta_seconds() * 100.;
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
        let sum = sum_force[index];
        let total_mass = body.mass();
        let acc = sum / total_mass;

        // Calculate the speed difference
        let speed_delta = delta * acc;
        body.add_speed_delta(speed_delta);

        // Calculate the position difference
        let position_delta = delta * delta * acc / 2. + delta * (*body.get_speed());
        body.add_position_delta(position_delta);
    }

    if let Some(body) = body_query.iter().next() {
        println!("{:?}", body.get_position());
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
            mesh: meshes.add(shape::Circle::new(body.get_radius()).into()).into(),
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