mod visualization;
use visualization::VisualizationPlugin;

mod environment;
use environment::EnvironmentPlugin;

mod creatures;
use creatures::CreaturesPlugin;

use bevy::prelude::*;
use bevy_rapier3d::physics::RapierPhysicsPlugin;

struct Constants {
    environment_size: f32,
    initial_creatures: u32,
    max_creatures: u32,
}

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_resource(WindowDescriptor {
            title: "Creatures!".to_string(),
            width: 800,
            height: 800,
            ..Default::default()
        })
        .add_resource(Constants {
            environment_size: 50.,
            initial_creatures: 10,
            max_creatures: 50,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(VisualizationPlugin)
        .add_plugin(EnvironmentPlugin)
        .add_plugin(CreaturesPlugin)
        .run();
}
