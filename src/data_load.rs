use crate::tower::{TowerDB, TowerDef};
use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
struct TowerDefJson {
    pub id: String,
    pub scene: String,
    pub projectile_scene: String,
    pub fire_interval: f32,
    pub range: f32,
    pub damage: f32,
    pub projectile_speed: f32,
    pub projectile_scale: f32,
    pub offset: [f32; 3],
    pub shot_sound: String,
    pub shot_volume: f32
}

#[derive(serde::Deserialize, Asset, TypePath, Clone)]
struct TowersJsonFile {
    towers: Vec<TowerDefJson>,
}

#[derive(Resource)]
struct TowersJsonHandle(Handle<TowersJsonFile>);

pub struct DataLoadPlugin;

impl Plugin for DataLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(JsonAssetPlugin::<TowersJsonFile>::new(&["json"]))
            .add_systems(Startup, load_tower_json)
            .add_systems(
                Update,
                build_tower_db_once
                    .run_if(towers_json_ready)
                    .run_if(not(resource_exists::<TowerDB>))
            );
    }
}

fn towers_json_ready(
    handle: Option<Res<TowersJsonHandle>>,
    assets: Res<Assets<TowersJsonFile>>,
) -> bool {
    match handle {
        Some(h) => assets.get(&h.0).is_some(),      // true, если ассет уже в Assets
        None => false,
    }
}

fn load_tower_json(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Load Tower Json");
    let handle: Handle<TowersJsonFile> = asset_server.load("data/towers.json");
    commands.insert_resource(TowersJsonHandle(handle));
}

fn build_tower_db_once(
    mut commands: Commands,
    json_handle: Res<TowersJsonHandle>,
    assets: Res<Assets<TowersJsonFile>>,
    asset_server: Res<AssetServer>,
) {
    info!("Load Tower DB");
    let file = assets.get(&json_handle.0).expect("json must be loaded by run_if");

    let mut defs = HashMap::new();
    for j in &file.towers {
        let scene: Handle<Scene>      = asset_server.load(format!("{}#Scene0", j.scene));
        let proj_scene: Handle<Scene> = asset_server.load(format!("{}#Scene0", j.projectile_scene));
        let shot_sound: Handle<AudioSource> = asset_server.load(j.shot_sound.clone());
        defs.insert(j.id.clone(), TowerDef {
            id: j.id.clone(),
            damage: j.damage,
            scene,
            projectile_scene: proj_scene,
            fire_interval: j.fire_interval,
            range: j.range,
            projectile_speed: j.projectile_speed,
            projectile_scale: j.projectile_scale,
            offset: Vec3::from_array(j.offset),
            shot_sound,
            shot_volume: j.shot_volume
        });
    }

    commands.insert_resource(TowerDB { defs });
    info!("TowerDB built from data/towers.json");
}
