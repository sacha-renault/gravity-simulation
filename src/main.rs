pub mod shared;
pub mod utility;
pub mod plugin;

use serde_json;

use bevy::prelude::*;
use plugin::SysPlugin;
use shared::Body;

fn main() {
    // Setup the body we wanna put in the simulation
    let json_content = std::fs::read_to_string("bodies.json")
        .expect("Couldn't read bodies.json file");
    let bodies: Vec<Body> = serde_json::from_str(&json_content)
        .expect("Couldn't parse bodies in bodies.json content");

    // Cancel if no bodies are found
    if bodies.is_empty() {
        panic!("Cannot start a simulation with 0 bodies");
    }
    
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_plugins(SysPlugin::new(bodies))
        .run();
}
