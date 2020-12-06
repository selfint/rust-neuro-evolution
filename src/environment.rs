use bevy::prelude::*;
use bevy_rapier3d::{
    physics::RigidBodyHandleComponent,
    rapier::{
        dynamics::{RigidBodyBuilder, RigidBodySet},
        geometry::ColliderBuilder,
    },
};

use crate::Constants;

struct Ground;

#[derive(Bundle)]
struct GroundBundle {
    ground: Ground,
}

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(environment_startup_system.system())
            .add_system(clamp_system.system());
    }
}

fn environment_startup_system(
    mut commands: Commands,
    constants: Res<Constants>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ground_mesh = meshes.add(Mesh::from(shape::Plane {
        size: constants.environment_size * 2.,
    }));
    let ground_mat = materials.add(Color::rgb(0.1, 1., 0.).into());
    spawn_ground(
        &mut commands,
        constants.environment_size,
        ground_mesh,
        ground_mat,
    );
}

fn spawn_ground(
    commands: &mut Commands,
    environment_size: f32,
    ground_mesh: Handle<Mesh>,
    ground_mat: Handle<StandardMaterial>,
) {
    commands
        .spawn(GroundBundle { ground: Ground })
        .with(RigidBodyBuilder::new_static().translation(0.0, 0.0, 0.0))
        .with(ColliderBuilder::cuboid(
            environment_size,
            0.,
            environment_size,
        ))
        .with_bundle(PbrComponents {
            mesh: ground_mesh,
            material: ground_mat,
            ..Default::default()
        });
}

fn clamp_system(
    mut rigib_bodies: ResMut<RigidBodySet>,
    constants: Res<Constants>,
    query: Query<&RigidBodyHandleComponent>,
) {
    let ground_size = constants.environment_size - 2.;
    for rb_handle in query.iter() {
        if let Some(rb) = rigib_bodies.get_mut(rb_handle.handle()) {
            let mut new_position = *rb.position();
            let mut changed = false;
            if rb.position().translation.vector.x > ground_size {
                new_position.translation.x = ground_size;
                changed = true;
            } else if rb.position().translation.vector.x < -ground_size {
                new_position.translation.x = -ground_size;
                changed = true;
            }
            if rb.position().translation.vector.z > ground_size {
                new_position.translation.z = ground_size;
                changed = true;
            } else if rb.position().translation.vector.z < -ground_size {
                new_position.translation.z = -ground_size;
                changed = true;
            }
            if changed {
                rb.set_position(new_position, true);
            }
        }
    }
}
