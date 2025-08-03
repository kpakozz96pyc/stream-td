use bevy::math::Vec3;
use bevy::prelude::*;

pub struct CustomCameraPlugin;

impl Plugin for CustomCameraPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_controls);

        return;
    }
}

fn spawn_camera(mut commands: Commands){
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-5.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    )).insert(Name::new("Camera"));
}

fn camera_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>
){
    let mut camera_transform = camera_query.single_mut().unwrap();
    let forward = Vec3::new(camera_transform.forward().x, 0.0, camera_transform.forward().z);
    let right = Vec3::new(camera_transform.right().x, 0.0, camera_transform.right().z);

    let speed = 5.0;
    let rotation_speed = 1.5;

    let delta = time.delta_secs();

    let mut movement = Vec3::ZERO;

    if  keyboard.pressed(KeyCode::KeyW)
    {
        movement += forward;
    }
    if  keyboard.pressed(KeyCode::KeyS)
    {
        movement -= forward;
    }
    if  keyboard.pressed(KeyCode::KeyD)
    {
        movement += right;
    }
    if  keyboard.pressed(KeyCode::KeyA){
        movement -= right;
    }

    if movement != Vec3::ZERO{
        movement = movement.normalize()*speed*delta;
        camera_transform.translation += movement;
    }

    if keyboard.pressed(KeyCode::KeyQ){
        camera_transform.rotate(Quat::from_axis_angle(Vec3::Y, rotation_speed*delta));
    }
    if keyboard.pressed(KeyCode::KeyE){
        camera_transform.rotate(Quat::from_axis_angle(Vec3::Y, -rotation_speed*delta));
    }

    return;
}