mod visualization;
use visualization::VisualizationPlugin;

mod environment;
use environment::EnvironmentPlugin;

mod creatures;
use creatures::CreaturesPlugin;

use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(VisualizationPlugin)
        .add_plugin(EnvironmentPlugin)
        .add_plugin(CreaturesPlugin)
        .run();
}
