use bevy::math::{FloatOrd, Vec3};
use bevy::prelude::*;
use crate::projectile::{ArrowAssets, Projectile};
use crate::target::{Target};

pub struct TowerPlugin;

const GLTF_PATH: &str = "glb/tower_01.glb";

impl Plugin for TowerPlugin{
    fn build(&self,app: &mut App){
        app.register_type::<Tower>().insert_resource(TowerAssets::default())
            .add_systems(Startup, spawn_tower)
            .add_systems(Update, spawn_projectiles);
        
        return;
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tower{
    pub shooting_timer: Timer,
    pub projectile_offset: Vec3
}

#[derive(Resource, Default)]
pub struct TowerAssets {
    scene: Handle<Scene>
}

fn spawn_tower(mut commands: Commands,
               mut tower_assets: ResMut<TowerAssets>,
               asset_server: Res<AssetServer>){


    tower_assets.scene = asset_server.load(GltfAssetLabel::Scene(0).from_asset(GLTF_PATH));
    commands.spawn((
        SceneRoot(tower_assets.scene.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ))
        .insert(Tower{
            shooting_timer:Timer::from_seconds(1.0, TimerMode::Repeating),
            projectile_offset: Vec3::new(0.0, 1.1, 0.65)
        })
        .insert(Name::new("Tower1"));

    commands.spawn((
        SceneRoot(tower_assets.scene.clone()),
        Transform::from_xyz(3.0, 0.0, 0.0),
    ))
        .insert(Tower{
            shooting_timer:Timer::from_seconds(1.0, TimerMode::Repeating),
            projectile_offset: Vec3::new(0.0, 1.1, 0.0)
        })
        .insert(Name::new("Tower2"));
    
}

fn spawn_projectiles(
    mut commands: Commands,
    mut towers: Query<(&GlobalTransform, &mut Tower, Entity)>,
    targets: Query<&GlobalTransform, With<Target>>,
    time: Res<Time>,
    arrow_assets: Res<ArrowAssets>

){

    for (transform, mut tower, tower_entity) in towers.iter_mut(){
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished(){
            let bullet_spawn_position = transform.translation() + tower.projectile_offset;

            let direction = targets.iter().min_by_key(|tt|{
                FloatOrd(Vec3::distance(tt.translation(), bullet_spawn_position))
            }).map(|ct|{
                let mut target_pos = ct.translation();
                target_pos.y += 0.25;
                target_pos - bullet_spawn_position
            });

            if let Some(direction) = direction{

                let tower_forward = transform.rotation() * -Vec3::Z;

                // Кватернион, чтобы повернуть forward башни в нужное направление
                let normalized_tower_forward = tower_forward.normalize();
                let normalized_direction = direction.normalize();
                let rotation_to_target = Quat::from_rotation_arc(normalized_tower_forward, normalized_direction);

                // Итоговая ориентация стрелы = поворот башни + корректировка на цель
                let rotation = rotation_to_target * transform.rotation();

                commands.entity(tower_entity).with_children(|commands|{
                    
                    commands.spawn((
                        SceneRoot(arrow_assets.scene.clone()),
                        Transform::from_translation(tower.projectile_offset)
                            .with_rotation(rotation)
                            .with_scale(Vec3::new(1.0, 1.0, 1.0)),
                    )).insert(Projectile{
                        speed: 3.0,
                        direction,
                        life_timer: Timer::from_seconds(2.0, TimerMode::Once)
                    }).insert(Name::new("Projectile"));
                });
            }
        }
    }
}