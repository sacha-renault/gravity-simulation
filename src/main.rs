pub mod components;
pub mod physics;
pub mod plugin;

use bevy::prelude::*;
use plugin::SysPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SysPlugin)
        .run();
}
