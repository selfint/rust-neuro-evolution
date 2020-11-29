use bevy::prelude::*;

pub struct StartupPlugin;
struct Floor;
impl Floor {
    fn new() -> Self {
        Floor {}
    }
}

#[derive(Bundle)]
struct World {
    floor: Floor,
}

impl World {
    fn new() -> Self {
        World {floor: Floor::new()}
    }
}

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup_world.system());
    }
}

fn startup_world(mut cmd: Commands) {
    println!("Starting world");
    cmd.spawn(World::new());
}
