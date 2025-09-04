mod tower;
mod world;
mod projectile;
mod target;
mod camera;
mod pixel_plugin;
mod blood;
mod main_menu;
mod data_load;
mod egui_setup;
mod input_system;
mod tower_build;

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
use crate::blood::BloodPlugin;
use crate::data_load::DataLoadPlugin;
use crate::egui_setup::EguiConfigurePlugin;
use crate::input_system::PlayerInputPlugin;
use crate::main_menu::MainMenuPlugin;
use crate::pixel_plugin::PixelPlugin;
use crate::StartupStage::{Build, Load, Processing};
use crate::tower_build::TowerBuildPlugin;

fn main() {
    App::new()
        .configure_sets(Startup, (Load, Processing, Build).chain())
        .configure_sets(Update, (Load, Processing, Build).chain())
        .insert_resource(ClearColor(Color::Srgba(Srgba::new(0.3,0.3,0.3, 1.0))))
        .add_plugins(DefaultPlugins)
        .add_plugins(MeshPickingPlugin)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(DataLoadPlugin)
        .add_plugins(TowerPlugin)
        .add_plugins(TargetPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(ProjectilePlugin)
        .add_plugins(CustomCameraPlugin)
        .add_plugins(PixelPlugin)
        .add_plugins(BloodPlugin)
        .add_plugins(MainMenuPlugin)
        .add_plugins(EguiConfigurePlugin)
        .add_plugins(PlayerInputPlugin)
        .add_plugins(TowerBuildPlugin)
        .init_state::<AppState>()
        .init_state::<PlayerState>()
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
    Paused,
}
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum PlayerState {
    #[default]
    Build,
    None
}


#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum StartupStage {
    Load,
    Processing,
    Build,
}



