mod tower;
mod world;
mod projectile;
mod target;
mod camera;

use bevy::app::App;
use bevy::prelude::*;
use bevy::color::{Color, Srgba};
use bevy::prelude::{ClearColor};
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::camera::CustomCameraPlugin;
use crate::projectile::ProjectilePlugin;
use crate::target::TargetPlugin;
use crate::tower::TowerPlugin;
use crate::world::WorldPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::Srgba(Srgba::new(0.3,0.3,0.3, 1.0))))
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(TowerPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(ProjectilePlugin)
        .add_plugins(TargetPlugin)
        .add_plugins(CustomCameraPlugin)
        .run();
}
