use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};

pub struct VisualizationPlugin;

impl Plugin for VisualizationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(FlyCameraPlugin)
            .add_startup_system(visualization_startup_system.system())
            .add_system(camera_pointer_system.system())
            // .add_system(camera_rotator_system.system())
            ;
    }
}

fn visualization_startup_system(mut commands: Commands) {
    commands
        // Camera
        .spawn(Camera3dComponents {
            transform: Transform::from_translation(Vec3::new(-100., 75., -100.)),
            ..Default::default()
        })
        .with(FlyCamera::default())
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
