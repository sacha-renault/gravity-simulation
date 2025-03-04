use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{components::body::Body, physics::Force};

#[derive(Component)]
pub struct SysCamera;

pub fn setup_plugin(mut commands: Commands, _asset_server: Res<AssetServer>) {
    // Spawn a camera for UI
    commands.spawn((Camera2dBundle::default(), SysCamera));

    // Spawn a random body
    commands.spawn(Body::new(Vec2::new(0., 0.), Vec2::new(0., 0.), 10.0, 1e8));
    commands.spawn(Body::new(Vec2::new(100., 100.), Vec2::new(0., 0.), 10.0, 1e8));
}

pub fn update_bodies(
    mut commands: Commands, 
    time: Res<Time>,
    mut body_query: Query<&mut Body>
) {
    let delta = time.delta_seconds();
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
    
    println!("{:?}", sum_force.iter().map(|v| v.clone()*delta));

    // From new acceleration, modify speed and position
    for (index, mut body) in body_query.iter_mut().enumerate() {
        // calculate acceleeratino from newton laws Sum(F) = m*a <=> a = Sum(F) / m
        let acc = sum_force[index] / body.mass();

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