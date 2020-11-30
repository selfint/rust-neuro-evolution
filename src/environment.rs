use bevy::prelude::*;

struct Ground;

#[derive(Bundle)]
struct GroundBundle {
    ground: Ground,
    transform: Transform,
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
    let ground_mesh = meshes.add(Mesh::from(shape::Plane { size: 10. }));
    let ground_mat = materials.add(Color::rgb(0., 1., 0.).into());

    commands
        .spawn(GroundBundle {
            ground: Ground,
            transform: Transform::from_translation(Vec3::zero()),
        })
        .with_bundle(PbrComponents {
            mesh: ground_mesh,
            material: ground_mat,
            ..Default::default()
        });
}
