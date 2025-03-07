use bevy::prelude::*;

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
    for (body_entity, body) in query.iter() {
        // Calculate color based on density
        let hue = (body.get_density() * 0.5) % 1.0;

        // Create handles
        let mesh_handle = meshes.add(Circle::new(body.get_radius()));
        let color = materials.add(Color::hsl(hue * 360.0, 0.8, 0.5));
        
        // Create visual representation
        commands.entity(body_entity).insert((
            Mesh2d(mesh_handle),
            MeshMaterial2d(color)
        ))
        .observe(update_material_on::<Pointer<Over>>());
    }
}

fn update_material_on<E>() 
    -> impl Fn(Trigger<E>) {
        move |trigger| {
            
        }
}