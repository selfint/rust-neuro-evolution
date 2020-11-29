use bevy::prelude::*;

pub struct StartupPlugin;
struct Floor {
    height: f32,
}

impl Floor {
    fn new(height: f32) -> Self {
        Floor { height }
    }
}

#[derive(Bundle)]
struct WorldBundle {
    floor: Floor,
}

impl WorldBundle {
    fn new(floor_height: f32) -> Self {
        WorldBundle {
            floor: Floor::new(floor_height),
        }
    }
}

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup_world.system());
    }
}

fn startup_world(mut cmd: Commands) {
    println!("Starting world");
    cmd.spawn(WorldBundle::new(5.0));
}
