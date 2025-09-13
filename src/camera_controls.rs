use bevy::input::ButtonInput;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::math::{Quat, Vec3};
use bevy::prelude::*;
use crate::AppState;

pub struct CameraControlsPlugin;


impl Plugin for CameraControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (camera_controls, camera_zoom, camera_rotate)
                .run_if(not(in_state(AppState::Menu))));
    }
}

#[derive(Component)]
pub struct ControllableCamera;

fn camera_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut cameras: Query<&mut Transform, With<ControllableCamera>>,
    time: Res<Time>,
) {

    let speed = 5.0;
    let delta = time.delta_secs();
    let mut movement = Vec3::ZERO;

    for mut camera_tr in &mut cameras {
        let forward = camera_tr.forward().normalize();
        let right = camera_tr.right().normalize();

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
        if keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight) {
            movement -= Vec3::Y;
        }

        if movement != Vec3::ZERO {
            let delta_move = movement.normalize() * speed * delta;
            camera_tr.translation += delta_move;
        }
    }
}

fn camera_zoom(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut cameras: Query<&mut Transform, With<ControllableCamera>>,
    time: Res<Time>,
) {

    for mut camera_tr in &mut cameras {
        let zoom_speed = 30.0;
        let delta = time.delta_secs();

        for ev in mouse_wheel_events.read() {
            let scroll_amount = ev.y;
            if scroll_amount == 0.0 {
                continue;
            }
            let dir = camera_tr.forward();
            let delta_move = dir * scroll_amount * zoom_speed * delta;
            camera_tr.translation += delta_move;
        }
    }
}

fn camera_rotate(
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut cameras: Query<&mut Transform, With<ControllableCamera>>,
) {
    if !mouse_button.pressed(MouseButton::Middle) {
        mouse_motion_events.clear();
        return;
    }
    for mut camera_tr in &mut cameras {
        let rotation_speed = 0.005;
        for ev in mouse_motion_events.read() {
            let delta = ev.delta;

            let yaw = Quat::from_rotation_y(-delta.x * rotation_speed);
            camera_tr.rotate(yaw);

            let right_dir = camera_tr.right();
            let pitch = Quat::from_axis_angle(*right_dir, -delta.y * rotation_speed);
            camera_tr.rotate(pitch);
        }
    }
}