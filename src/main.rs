mod visualization;
use visualization::VisualizationPlugin;

mod environment;
use environment::EnvironmentPlugin;

mod creatures;
use creatures::CreaturesPlugin;

use bevy::prelude::*;
use bevy_rapier3d::physics::RapierPhysicsPlugin;

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_resource(WindowDescriptor {
            title: "Creatures!".to_string(),
            width: 800,
            height: 800,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(VisualizationPlugin)
        .add_plugin(EnvironmentPlugin)
        .add_plugin(CreaturesPlugin)
        .run();
}
