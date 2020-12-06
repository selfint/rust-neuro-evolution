use bevy::prelude::*;
use bevy_rapier3d::rapier::{
    dynamics::RigidBodyBuilder, geometry::ColliderBuilder, math::AngVector,
};
use std::f32::consts::PI;

use crate::Constants;

struct Ground;

struct Wall;

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
    constants: Res<Constants>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ground_mesh = meshes.add(Mesh::from(shape::Plane {
        size: constants.environment_size * 2.,
    }));
    let ground_mat = materials.add(Color::rgb(0.1, 1., 0.).into());
    let wall_mesh = meshes.add(Mesh::from(shape::Plane {
        size: constants.environment_size * 2.,
    }));
    let wall_mat = materials.add(Color::rgba(0., 0., 0., 0.).into());
    spawn_ground(
        &mut commands,
        constants.environment_size,
        ground_mesh,
        ground_mat,
    );

    // left
    spawn_wall(
        &mut commands,
        constants.environment_size,
        wall_mesh.clone(),
        wall_mat.clone(),
        AngVector::new(0., 0., PI / 2.),
        constants.environment_size,
        constants.environment_size,
        0.,
    );

    // right
    spawn_wall(
        &mut commands,
        constants.environment_size,
        wall_mesh.clone(),
        wall_mat.clone(),
        AngVector::new(0., 0., PI / 2.),
        -constants.environment_size,
        constants.environment_size,
        0.,
    );

    // front
    spawn_wall(
        &mut commands,
        constants.environment_size,
        wall_mesh.clone(),
        wall_mat.clone(),
        AngVector::new(PI / 2., 0., 0.),
        0.,
        constants.environment_size,
        constants.environment_size,
    );

    // back
    spawn_wall(
        &mut commands,
        constants.environment_size,
        wall_mesh.clone(),
        wall_mat.clone(),
        AngVector::new(PI / 2., 0., 0.),
        0.,
        constants.environment_size,
        -constants.environment_size,
    );

    // top
    spawn_wall(
        &mut commands,
        constants.environment_size,
        wall_mesh,
        wall_mat,
        AngVector::new(0., 0., 0.),
        0.,
        constants.environment_size * 2.,
        0.,
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

// TODO: implement with less parameters?
fn spawn_wall(
    commands: &mut Commands,
    environment_size: f32,
    wall_mesh: Handle<Mesh>,
    wall_mat: Handle<StandardMaterial>,
    angle: AngVector<f32>,
    x: f32,
    y: f32,
    z: f32,
) {
    commands
        .spawn(PbrComponents {
            mesh: wall_mesh,
            material: wall_mat,
            ..Default::default()
        })
        .with(Wall)
        .with(
            RigidBodyBuilder::new_static()
                .rotation(angle)
                .translation(x, y, z),
        )
        .with(ColliderBuilder::cuboid(
            environment_size,
            0.,
            environment_size,
        ));
}
