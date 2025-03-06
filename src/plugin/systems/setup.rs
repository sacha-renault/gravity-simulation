use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::shared::Body;

pub fn setup_plugin(mut commands: Commands) {
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
            Vec2::new(149.6e8, 0.0),   // ~149.6 million km from Sun in meters
            Vec2::new(0.0, 29.78e2),     // Orbital velocity of ~29.78 km/s in m/s
            6.378e7,                   // Radius in meters (~6,378 km)
            5515.0                     // Density in kg/m³
        ),
    ];

    // Spawn a random bodies
    for body in bodies.into_iter() {
        commands.spawn(body);
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