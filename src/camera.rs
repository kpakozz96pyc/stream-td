use bevy::app::{App, Startup};
use bevy::input::mouse::{MouseButton, MouseMotion, MouseWheel};
use bevy::math::{Quat, Vec3};
use bevy::prelude::{ButtonInput, Camera3d, Commands, EventReader, KeyCode, Query, Res, Time, Transform, Update, With};
use crate::pixel_plugin::PostProcessSettings;

pub struct CustomCameraPlugin;

impl bevy::prelude::Plugin for CustomCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, (camera_controls, camera_zoom, camera_rotate));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-5.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        PostProcessSettings {
            block_size: 2.5,
        },
    ));
}

fn camera_controls(keyboard: Res<ButtonInput<KeyCode>>, mut camera_query: Query<&mut Transform, With<Camera3d>>, time: Res<Time>){
    let mut camera = camera_query.single_mut().unwrap();

    let forward = Vec3::new(camera.forward().x, camera.forward().y, camera.forward().z).normalize();
    let right = Vec3::new(camera.right().x, camera.right().y, camera.right().z).normalize();

    let speed = 5.0;
    let rotation_speed = 1.5;
    let delta = time.delta_secs();

    let mut movement = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        movement += forward;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        movement -= forward;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        movement -= right;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        movement += right;
    }
    if keyboard.pressed(KeyCode::Space) {
        movement += Vec3::Y;
    }

    if movement != Vec3::ZERO {
        movement = movement.normalize() * speed * delta;
        camera.translation += movement;
    }


    if keyboard.pressed(KeyCode::KeyQ) {
        let rotation = bevy::math::Quat::from_rotation_y(rotation_speed * delta);
        camera.rotate(rotation);
    }
    if keyboard.pressed(KeyCode::KeyE) {
        let rotation = Quat::from_rotation_y(-rotation_speed * delta);
        camera.rotate(rotation);
    }
}

fn camera_zoom(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let mut camera_transform = camera_query.single_mut().unwrap();
    let zoom_speed = 10.0;
    let delta = time.delta_secs();


    for event in mouse_wheel_events.read() {

        let scroll_amount = event.y;

        let zoom_direction = camera_transform.forward();
        camera_transform.translation += zoom_direction * scroll_amount * zoom_speed * delta;
    }
}

fn camera_rotate(
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
) {

    if mouse_button.pressed(MouseButton::Middle) {
        let mut camera_transform = camera_query.single_mut().unwrap();
        let rotation_speed = 0.005;
        for event in mouse_motion_events.read() {
            let mouse_delta = event.delta;
            let y_rotation = Quat::from_rotation_y(-mouse_delta.x * rotation_speed);
            camera_transform.rotate(y_rotation);
            let right_dir = Vec3::new(camera_transform.right().x, camera_transform.right().y, camera_transform.right().z).normalize();
            let x_rotation = Quat::from_axis_angle(right_dir, -mouse_delta.y * rotation_speed);
            camera_transform.rotate(x_rotation);
        }
    } else {
        mouse_motion_events.clear();
    }
}
