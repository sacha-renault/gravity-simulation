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
            Vec2::new(0.0, 0.0),
            6.96340e8,
            1408.0
        ),
        Body::new(
            Vec2::new(149.6e8, 0.0),
            Vec2::new(0.0, 29.78e2),
            6.378e7,
            5515.0
        ),
    ];
    
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_plugins(SysPlugin::new(bodies))
        .run();
}
