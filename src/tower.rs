use crate::AppState;
use crate::projectile::Projectile;
use crate::target::Target;
use bevy::math::{FloatOrd, Vec3};
use bevy::prelude::*;
use std::collections::HashMap;
use bevy::audio::Volume;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tower>()
            .add_systems(Update, spawn_some_towers.run_if(resource_added::<TowerDB>))
            .add_systems(Update, spawn_projectiles.run_if(in_state(AppState::InGame)));
    }
}

// region struct

pub struct TowerPlugin;
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
    pub shot_sound: Handle<AudioSource>,
    pub shot_volume: f32
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
    pub defs: HashMap<String, TowerDef>,
}

pub struct TowerDef {
    pub id: String,
    pub damage: f32,
    pub scene: Handle<Scene>,
    pub projectile_scene: Handle<Scene>,
    pub fire_interval: f32,
    pub range: f32,
    pub projectile_speed: f32,
    pub projectile_scale: f32,
    pub offset: Vec3,
    pub shot_sound: Handle<AudioSource>,
    pub shot_volume: f32
}
// endregion

// region systems
fn spawn_some_towers(mut commands: Commands, db: Res<TowerDB>) {
    let basic = db.defs.get("basic").unwrap();
    let sniper = db.defs.get("sniper").unwrap();
    spawn_tower_of(
        &mut commands,
        basic,
        Vec3::new(0.0, 0.0, 0.0),
    );
    spawn_tower_of(
        &mut commands,
        sniper,
        Vec3::new(3.0, 0.0, 0.0),
    );
}

fn spawn_tower_of(commands: &mut Commands,
                  def: &TowerDef,
                  pos: Vec3) {
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
            shot_sound: def.shot_sound.clone(),
            shot_volume: def.shot_volume
        },
        scene: SceneRoot(def.scene.clone()),
        transform: Transform::from_translation(pos),
        name: Name::new(def.id.clone()),
    });
}

fn spawn_projectiles(
    mut commands: Commands,
    mut towers: Query<(&GlobalTransform, &mut Tower, &TowerStats)>,
    targets: Query<&GlobalTransform, With<Target>>,
    time: Res<Time>,
) {
    for (gt, mut tower, stats) in &mut towers {
        tower.shooting_timer.tick(time.delta());
        if !tower.shooting_timer.just_finished() {
            continue;
        }

        let muzzle = gt.translation() + stats.projectile_offset;

        let maybe_dir = targets
            .iter()
            .filter_map(|tgt| {
                let to = tgt.translation() + Vec3::Y * 0.3 - muzzle;
                let d2 = to.length_squared();
                if d2 <= stats.range_sq {
                    Some((d2, to))
                } else {
                    None
                }
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
            commands.spawn((
                Name::new("shot_sound"),
                Transform::from_translation(muzzle),
                GlobalTransform::default(),
                AudioPlayer(stats.shot_sound.clone()),
                PlaybackSettings::DESPAWN
                    .with_spatial(true)
                    .with_volume(Volume::Linear(stats.shot_volume))
            ));
        }
    }
}

// endregion
