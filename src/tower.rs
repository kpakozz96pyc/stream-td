use bevy::math::{FloatOrd, Vec3};
use bevy::prelude::*;
use crate::AppState;
use crate::projectile::{Projectile};
use crate::target::Target;
use std::collections::HashMap;

pub struct TowerPlugin;

fn spawn_projectiles(
    mut commands: Commands,
    mut towers: Query<(&GlobalTransform, &mut Tower, &TowerStats)>,
    targets: Query<&GlobalTransform, With<Target>>,
    time: Res<Time>,
) {
    for (gt, mut tower, stats) in &mut towers {
        tower.shooting_timer.tick(time.delta());
        if !tower.shooting_timer.just_finished() { continue; }

        let muzzle = gt.translation() + stats.projectile_offset;

        let maybe_dir = targets.iter()
            .filter_map(|tgt| {
                let to = tgt.translation() + Vec3::Y * 0.3 - muzzle;
                let d2 = to.length_squared();
                if d2 <= stats.range_sq { Some((d2, to)) } else { None }
            })
            .min_by_key(|(d2, _)| FloatOrd(*d2))
            .map(|(_, to)| to);

        if let Some(dir) = maybe_dir {
            let dir_norm = dir.normalize();
            let rot = Quat::from_rotation_arc(-Vec3::Z, dir_norm);

            commands.spawn((
                SceneRoot(stats.projectile_scene.clone()),
                Transform::from_translation(muzzle)
                    .with_rotation(rot)
                    .with_scale(Vec3::splat(stats.projectile_scale)),
                Projectile {
                    speed: stats.projectile_speed,
                    direction: dir_norm,
                    life_timer: Timer::from_seconds(2.0, TimerMode::Once),
                    damage: stats.damage,
                },
                Name::new("Projectile"),
            ));
        }
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tower {
    pub shooting_timer: Timer,
}

#[derive(Component, Clone)]
pub struct TowerStats {
    pub damage: f32,
    pub projectile_offset: Vec3,
    pub range_sq: f32,
    pub projectile_speed: f32,
    pub projectile_scale: f32,
    pub projectile_scene: Handle<Scene>,
}

#[derive(Bundle)]
pub struct TowerBundle {
    pub tower: Tower,
    pub stats: TowerStats,
    pub scene: SceneRoot,
    pub transform: Transform,
    pub name: Name,
}

#[derive(Resource)]
pub struct TowerDB {
    defs: HashMap<&'static str, TowerDef>,
}

pub struct TowerDef {
    pub damage: f32,
    pub scene: Handle<Scene>,
    pub projectile_scene: Handle<Scene>,
    pub fire_interval: f32,
    pub range: f32,
    pub projectile_speed: f32,
    pub projectile_scale: f32,
    pub offset: Vec3,
}

impl FromWorld for TowerDB {
    fn from_world(world: &mut World) -> Self {
        let server = world.resource::<AssetServer>();

        let tower1 = TowerDef {
            scene: server.load(GltfAssetLabel::Scene(0).from_asset("glb/tower_01.glb")),
            projectile_scene: server.load(GltfAssetLabel::Scene(0).from_asset("glb/projectile_02.glb")),
            fire_interval: 0.5,
            range: 5.0,
            damage: 10.0,
            projectile_speed: 4.0,
            projectile_scale: 0.4,
            offset: Vec3::new(0.0, 1.0, 0.2),
        };

        let tower2 = TowerDef {
            scene: server.load(GltfAssetLabel::Scene(0).from_asset("glb/tower_02.glb")),
            projectile_scene: server.load(GltfAssetLabel::Scene(0).from_asset("glb/projectile_02.glb")),
            fire_interval: 1.5,
            damage: 15.0,
            range: 7.5,
            projectile_speed: 6.0,
            projectile_scale: 1.0,
            offset: Vec3::new(0.0, 0.8, 0.2),
        };

        let mut defs = HashMap::new();
        defs.insert("basic", tower1);
        defs.insert("sniper", tower2);
        Self { defs }
    }
}

fn spawn_tower_of(
    commands: &mut Commands,
    def: &TowerDef,
    pos: Vec3,
    name: String,
) {
    commands.spawn(TowerBundle {
        tower: Tower {
            shooting_timer: Timer::from_seconds(def.fire_interval, TimerMode::Repeating),
        },
        stats: TowerStats {
            damage: def.damage,
            projectile_offset: def.offset,
            range_sq: def.range * def.range,
            projectile_speed: def.projectile_speed,
            projectile_scale: def.projectile_scale,
            projectile_scene: def.projectile_scene.clone(),
        },
        scene: SceneRoot(def.scene.clone()),
        transform: Transform::from_translation(pos),
        name: Name::new(name),
    });
}

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tower>()
            .init_resource::<TowerDB>()
            .add_systems(Startup, spawn_some_towers)
            .add_systems(Update, spawn_projectiles.run_if(in_state(AppState::InGame)));
    }
}

fn spawn_some_towers(mut commands: Commands, db: Res<TowerDB>) {
    let basic = db.defs.get("basic").unwrap();
    let sniper = db.defs.get("sniper").unwrap();
    spawn_tower_of(&mut commands, basic, Vec3::new(0.0, 0.0, 0.0), "T_basic_0".to_string());
    spawn_tower_of(&mut commands, sniper, Vec3::new(3.0, 0.0, 0.0), "T_sniper_0".to_string());
}