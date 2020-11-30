mod startup;
use startup::StartupPlugin;

mod environment;
use environment::EnvironmentPlugin;

use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(StartupPlugin)
        .add_plugin(EnvironmentPlugin)
        .run();
}
