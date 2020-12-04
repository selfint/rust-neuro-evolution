use bevy::prelude::*;
use bevy_rapier3d::rapier::dynamics::RigidBodyBuilder;
use bevy_rapier3d::rapier::geometry::ColliderBuilder;

pub struct Creature;

#[derive(Bundle)]
struct CreatureBundle {
    creature: Creature,
}

pub struct CreaturesPlugin;

impl Plugin for CreaturesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(creature_startup_system.system());
    }
}

fn creature_startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let creature_mesh = meshes.add(Mesh::from(shape::Cube { size: 1. }));
    let creature_mat = materials.add(Color::rgb(1., 0., 0.).into());
    commands
        .spawn(CreatureBundle { creature: Creature })
        .with(RigidBodyBuilder::new_dynamic().translation(0.0, 4.0, 0.0))
        .with(ColliderBuilder::cuboid(1.0, 1.0, 1.0))
        .with_bundle(PbrComponents {
            mesh: creature_mesh,
            material: creature_mat,
            ..Default::default()
        });
}
