mod startup;
use startup::StartupPlugin;

use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(StartupPlugin)
        .run();
}
