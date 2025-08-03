mod tower;
mod world;
mod projectile;
mod target;

use bevy::app::App;
use bevy::prelude::*;
use bevy::color::{Color, Srgba};
use bevy::prelude::{Camera3d, ClearColor, Commands};
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::projectile::ProjectilePlugin;
use crate::target::TargetPlugin;
use crate::tower::TowerPlugin;
use crate::world::WorldPlugin;



fn main() {
    App::new()
        .insert_resource(ClearColor(Color::Srgba(Srgba::new(0.3,0.3,0.3, 1.0))))
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(TowerPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(ProjectilePlugin)
        .add_plugins(TargetPlugin)
        .add_systems(Startup, spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands){
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-5.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    )).insert(Name::new("Camera"));
}