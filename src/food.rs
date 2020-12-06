use bevy::prelude::*;
use bevy_rapier3d::rapier::dynamics::RigidBodyBuilder;
use bevy_rapier3d::rapier::geometry::ColliderBuilder;
use rand::Rng;

use crate::Constants;

struct FoodSpawner;
#[derive(Bundle)]
struct FoodSpawnerBundle {
    food_spawner: FoodSpawner,
}

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(food_startup_system.system());
    }
}

fn food_startup_system(
    mut commands: Commands,
    constants: Res<Constants>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let food_spawner_mesh = meshes.add(Mesh::from(shape::Icosphere {
        radius: 3.,
        subdivisions: 3,
    }));
    let food_spawner_mat = materials.add(Color::rgb(0., 1., 1.).into());
    let env_size = constants.environment_size;
    let y = 1.;
    let mut rng = rand::thread_rng();
    for _ in 0..constants.food_spawners {
        let x = rng.gen_range(-env_size, env_size);
        let z = rng.gen_range(-env_size, env_size);
        commands
            .spawn(FoodSpawnerBundle {
                food_spawner: FoodSpawner,
            })
            .with(RigidBodyBuilder::new_static().translation(x, y, z))
            .with(ColliderBuilder::ball(3.))
            .with_bundle(PbrComponents {
                mesh: food_spawner_mesh.clone(),
                material: food_spawner_mat.clone(),
                ..Default::default()
            });
    }
}
