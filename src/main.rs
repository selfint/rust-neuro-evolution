mod visualization;
use visualization::VisualizationPlugin;

mod environment;
use environment::EnvironmentPlugin;

mod creatures;
use creatures::CreaturesPlugin;

mod food;
use food::FoodPlugin;

use bevy::prelude::*;
use bevy_rapier3d::physics::RapierPhysicsPlugin;

struct Constants {
    environment_size: f32,
    initial_creatures: u32,
    food_spawners: u32,
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
            food_spawners: 3,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(VisualizationPlugin)
        .add_plugin(EnvironmentPlugin)
        .add_plugin(CreaturesPlugin)
        .add_plugin(FoodPlugin {
            food_spawn_rate: 1.,
        })
        .run();
}
