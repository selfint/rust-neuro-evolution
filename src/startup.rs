use bevy::{prelude::*, render::camera::Camera};

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup_system.system())
            .add_system(camera_rotater_system.system());
    }
}

fn startup_system(mut commands: Commands) {
    commands
        // Camera
        .spawn(Camera3dComponents {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(0., -90., 0., 0.).normalize(),
                Vec3::new(0.0, 1.0, -3.0),
            )),
            ..Default::default()
        })
        // Light
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });
}

fn camera_rotater_system(mut query: Query<(&Camera, &mut Transform)>) {
    for (_camera, mut transform) in query.iter_mut() {
        transform.rotate(Quat::from_rotation_y(0.01));
    }
}
