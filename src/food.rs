use bevy::prelude::*;
use bevy_rapier3d::rapier::{geometry::ColliderBuilder, math::Vector};
use bevy_rapier3d::{
    physics::RigidBodyHandleComponent,
    rapier::dynamics::{RigidBodyBuilder, RigidBodySet},
};
use rand::Rng;

use crate::Constants;

struct FoodSpawner;
#[derive(Bundle)]
struct FoodSpawnerBundle {
    food_spawner: FoodSpawner,
}

struct FoodSpawnerTimer(Timer);

struct Food;
#[derive(Bundle)]
struct FoodBundle {
    food: Food,
    rbb: RigidBodyBuilder,
    clb: ColliderBuilder,
}

pub struct FoodPlugin {
    pub food_spawn_rate: f32,
}

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(FoodSpawnerTimer(Timer::from_seconds(
            self.food_spawn_rate,
            true,
        )))
        .add_startup_system(food_startup_system.system())
        .add_system(food_spawner_system.system())
        .add_system(move_system.system());
    }
}

fn move_system(
    mut rigid_bodies: ResMut<RigidBodySet>,
    query: Query<(&Food, &RigidBodyHandleComponent)>,
) {
    let mut rng = rand::thread_rng();
    let mut counter = 0;
    for (_f, handle) in query.iter() {
        if let Some(rb) = rigid_bodies.get_mut(handle.handle()) {
            rb.apply_impulse(
                Vector::new(
                    rng.gen_range(-2., 2.),
                    rng.gen_range(-2., 2.),
                    rng.gen_range(-2., 2.),
                ),
                true,
            );
            counter += 1;
        }
    }

    println!("moved {} foods", counter);
}

fn food_spawner_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut food_spawner_timer: ResMut<FoodSpawnerTimer>,
    time: Res<Time>,
    query: Query<(&FoodSpawner, &Transform)>,
) {
    food_spawner_timer.0.tick(time.delta_seconds);
    if !food_spawner_timer.0.finished {
        return;
    }
    let food_mesh = meshes.add(Mesh::from(shape::Icosphere {
        radius: 0.5,
        subdivisions: 1,
    }));
    let food_mat = materials.add(Color::rgb(1., 0., 1.).into());
    for (_f, transform) in query.iter() {
        let x = transform.translation.x();
        let y = transform.translation.y() + 5.;
        let z = transform.translation.z();
        commands
            .spawn(FoodBundle {
                food: Food,
                rbb: RigidBodyBuilder::new_dynamic().translation(x, y, z),
                clb: ColliderBuilder::ball(0.5),
            })
            .with_bundle(PbrComponents {
                mesh: food_mesh.clone(),
                material: food_mat.clone(),
                ..Default::default()
            });
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
