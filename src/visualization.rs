use bevy::prelude::*;
use bevy::render::camera::Camera;

pub struct VisualizationPlugin;

impl Plugin for VisualizationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(visualization_startup_system.system())
            .add_system(camera_pointer_system.system())
            .add_system(camera_rotator_system.system());
    }
}

fn visualization_startup_system(mut commands: Commands) {
    commands
        // Camera
        .spawn(Camera3dComponents {
            transform: Transform::from_translation(Vec3::new(-50., 50., -50.)),
            ..Default::default()
        })
        // Light
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(-40.0, 80.0, 30.0)),
            ..Default::default()
        });
}

fn camera_pointer_system(mut query: Query<(&Camera, &mut Transform)>) {
    for (_c, mut transform) in query.iter_mut() {
        transform.look_at(Vec3::zero(), Vec3::unit_y());
    }
}

fn camera_rotator_system(time: Res<Time>, mut query: Query<(&Camera, &mut Transform)>) {
    // TOOD: there must be a better way to implement
    for (_camera, mut transform) in query.iter_mut() {
        let initial_y = transform.translation.y();
        let left = transform
            .forward()
            .normalize()
            .cross(Vec3::unit_y())
            .normalize();
        transform.translation += left * time.delta_seconds * 5.;
        transform.translation.set_y(initial_y);
    }
}
