use bevy::prelude::*;
use bevy_rapier3d::rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder};

pub struct Ground;
pub const ENVIRONMENT_SIZE: f32 = 50.0;

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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ground_mesh = meshes.add(Mesh::from(shape::Plane {
        size: ENVIRONMENT_SIZE,
    }));
    let ground_mat = materials.add(Color::rgb(0.1, 1., 0.).into());

    commands
        .spawn(GroundBundle { ground: Ground })
        .with(RigidBodyBuilder::new_static().translation(0.0, 0.0, 0.0))
        .with(ColliderBuilder::cuboid(
            ENVIRONMENT_SIZE,
            0.,
            ENVIRONMENT_SIZE,
        ))
        .with_bundle(PbrComponents {
            mesh: ground_mesh,
            material: ground_mat,
            ..Default::default()
        });
}

fn clamp_system(mut query: Query<&mut Transform>) {
    for mut transform in query.iter_mut() {
        let entity_x = transform.translation.x();
        let entity_y = transform.translation.y();
        if entity_x.abs() > ENVIRONMENT_SIZE {
            match entity_x > 0. {
                true => transform.translation.set_x(ENVIRONMENT_SIZE),
                false => transform.translation.set_x(-ENVIRONMENT_SIZE),
            }
        }
        if entity_y.abs() > ENVIRONMENT_SIZE {
            match entity_y > 0. {
                true => transform.translation.set_y(ENVIRONMENT_SIZE),
                false => transform.translation.set_y(-ENVIRONMENT_SIZE),
            }
        }
    }
}
