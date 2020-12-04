use bevy::prelude::*;
use bevy_rapier3d::rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder};

pub struct Ground;

#[derive(Bundle)]
struct GroundBundle {
    ground: Ground,
}

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(environment_startup_system.system());
    }
}

fn environment_startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ground_mesh = meshes.add(Mesh::from(shape::Plane { size: 50. }));
    let ground_mat = materials.add(Color::rgb(0.1, 1., 0.).into());

    commands
        .spawn(GroundBundle { ground: Ground })
        .with(RigidBodyBuilder::new_static().translation(0.0, 0.0, 0.0))
        .with(ColliderBuilder::cuboid(50.0, 1.0, 50.0))
        .with_bundle(PbrComponents {
            mesh: ground_mesh,
            material: ground_mat,
            ..Default::default()
        });
}
