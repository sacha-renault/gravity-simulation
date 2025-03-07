pub mod shared;
pub mod utility;
pub mod plugin;

use bevy::prelude::*;
use plugin::SysPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_plugins(SysPlugin)
        .run();
}
