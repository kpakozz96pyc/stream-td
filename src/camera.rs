use crate::pixel_plugin::PostProcessSettings;
use bevy::app::{App, Startup};
use bevy::input::mouse::{MouseButton, MouseMotion, MouseWheel};
use bevy::math::{Quat, Vec3};
use bevy::prelude::{in_state, not, ButtonInput, Camera3d, Commands, EventReader, GlobalTransform, IntoScheduleConfigs, KeyCode, Query, Res, SpatialListener, Time, Transform, Update, With};
use crate::AppState;

pub struct CustomCameraPlugin;

impl bevy::prelude::Plugin for CustomCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-5.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        PostProcessSettings { block_size: 3.0 },
        SpatialListener::new(5.0),
        GlobalTransform::default()
    ));
}