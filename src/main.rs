pub mod shared;
pub mod utility;
pub mod plugin;

use bevy::prelude::*;
use plugin::SysPlugin;
use shared::Body;

fn main() {
    // Setup the body we wanna put in the simulation
    let bodies = vec![
        Body::new(
            Vec2::new(0.0, 0.0),
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
    
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_plugins(SysPlugin::new(bodies))
        .run();
}
