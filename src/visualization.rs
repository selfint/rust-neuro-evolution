use bevy::prelude::*;
use bevy::render::camera::Camera;

pub struct VisualizationPlugin;

impl Plugin for VisualizationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(visualization_startup_system.system())
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

fn camera_rotator_system(mut query: Query<(&Camera, &mut Transform)>) {
    for (_camera, mut transform) in query.iter_mut() {
        // TODO: move camera around 0,0,0
        transform.look_at(Vec3::zero(), Vec3::new(0., 1., 0.));
    }
}
