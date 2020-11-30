mod startup;
use startup::StartupPlugin;

mod environment;
use environment::EnvironmentPlugin;

mod creatures;
use creatures::CreaturesPlugin;

use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(StartupPlugin)
        .add_plugin(EnvironmentPlugin)
        .add_plugin(CreaturesPlugin)
        .run();
}
