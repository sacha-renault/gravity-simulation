use bevy::prelude::*;

use crate::shared::{Body, SimulationState};

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
    // Create a color for hover
    let hover_handle = materials.add(Color::from(Color::WHITE));

    for (body_entity, body) in query.iter() {
        // Calculate color based on density
        let hue = (body.get_density() * 0.5) % 1.0;

        // Create handles
        let mesh_handle = meshes.add(Circle::new(body.get_radius()));
        let color_handle = materials.add(Color::hsl(hue * 360.0, 0.8, 0.5));
        
        // Create visual representation
        commands.entity(body_entity).insert((
            Mesh2d(mesh_handle),
            MeshMaterial2d(color_handle.clone())
        ))
        .observe(change_color_on::<Pointer<Over>>(hover_handle.clone()))
        .observe(base_color_on::<Pointer<Out>>(color_handle.clone()))
        .observe(select_body_on::<Pointer<Down>>());        
    }
}

/// Changes the material of an entity to a highlighted color when triggered by event `E`.
/// 
/// # Type Parameters
/// 
/// * `E` - The event type that triggers this function, typically a pointer exit event
/// 
/// # Parameters
/// 
/// * `color_handle` - Handle to the color material to apply on trigger
/// 
/// # Returns
/// 
/// A function that can be used with `.observe()` to change an entity's color on event
fn change_color_on<E>(color_handle: Handle<ColorMaterial>) -> impl Fn(
    Trigger<E>, 
    Query<&mut MeshMaterial2d<ColorMaterial>>
) {
    move |trigger, mut query| {
        if let Ok(mut material) = query.get_mut(trigger.entity()) {
            material.0 = color_handle.clone();
        }
    }
}

/// Resets the material of an entity to its base color when triggered by event `E`.
/// 
/// # Type Parameters
/// 
/// * `E` - The event type that triggers this function, typically a pointer exit event
/// 
/// # Parameters
/// 
/// * `color_handle` - Handle to the original color material to restore on trigger
/// 
/// # Returns
/// 
/// A function that can be used with `.observe()` to reset an entity's color on event
fn base_color_on<E: Event>(color_handle: Handle<ColorMaterial>) -> impl Fn(
    Trigger<E>, 
    Query<&mut MeshMaterial2d<ColorMaterial>>
) {
    move |trigger, mut query| {
        if let Ok(mut material) = query.get_mut(trigger.entity()) {
            material.0 = color_handle.clone();
        }
    }
}

/// Sets the selected body in the simulation state when triggered by event `E`.
/// 
/// This function updates the simulation state to track which body has been
/// selected by the user interaction, enabling focused operations on that entity.
/// 
/// # Type Parameters
/// 
/// * `E` - The event type that triggers this function, typically a pointer selection event
/// 
/// # Returns
/// 
/// A function that can be used with `.observe()` to update the selected body on event
fn select_body_on<E: Event>() -> impl Fn(
    Trigger<E>,
    ResMut<SimulationState>
) {
    |trigger, mut game_state| {
        game_state.selected_body_id = Some(trigger.entity());
    }
}