use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::components::body::Body;
use crate::components::SysCamera;
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
            Vec2::new(149.6e8, 0.0),   // ~149.6 million km from Sun in meters
            Vec2::new(0.0, 29.78e2),     // Orbital velocity of ~29.78 km/s in m/s
            6.378e7,                   // Radius in meters (~6,378 km)
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