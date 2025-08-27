use bevy::math::{FloatOrd, Vec3};
use bevy::prelude::*;
use crate::AppState;
use crate::projectile::{Projectile, ProjectileAssets};
use crate::target::Target;

pub struct TowerPlugin;

impl Plugin for TowerPlugin{
    fn build(&self,app: &mut App){
        app.register_type::<Tower>()
            .insert_resource(TowerAssets::default())
            .add_systems(Startup, spawn_tower)
            .add_systems(Update, spawn_projectiles.run_if(in_state(AppState::InGame)));
        
        return;
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tower{
    pub shooting_timer: Timer,
    pub projectile_offset: Vec3,
}

fn spawn_tower(mut commands: Commands,
               mut tower_assets: ResMut<TowerAssets>,
               asset_server: Res<AssetServer>,
){
    tower_assets.scene_1 = asset_server.load(GltfAssetLabel::Scene(0).from_asset(GLF_TOWER_1));
    tower_assets.scene_2 = asset_server.load(GltfAssetLabel::Scene(0).from_asset(GLF_TOWER_2));
    commands.spawn((
        SceneRoot(tower_assets.scene_1.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ))
        .insert(Tower{shooting_timer:Timer::from_seconds(1.0, TimerMode::Repeating),
            projectile_offset: Vec3::new(0.0, 1.0, 0.2) })
        .insert(Name::new("Tower0"));
    
    commands.spawn((
        SceneRoot(tower_assets.scene_2.clone()),
        Transform::from_xyz(3.0, 0.0, 0.0),
    ))
        .insert(Tower{shooting_timer:Timer::from_seconds(1.0, TimerMode::Repeating),
            projectile_offset: Vec3::new(0.0, 0.8, 0.2) })
        .insert(Name::new("Tower2"));
}

fn spawn_projectiles(
    mut commands: Commands,
    mut towers: Query<(&GlobalTransform, &mut Tower, Entity)>,
    targets: Query<&GlobalTransform, With<Target>>,
    time: Res<Time>,
    projectile_assets: ResMut<ProjectileAssets>,
){

    for (transform, mut tower, tower_entity) in towers.iter_mut(){
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished(){
            let bullet_spawn_position = transform.translation() + tower.projectile_offset;

            let direction = targets.iter().min_by_key(|tt|{
                FloatOrd(Vec3::distance(tt.translation(), bullet_spawn_position))
            }).map(|ct|{
                let mut target_position = ct.translation();
                target_position.y += 0.3;
                return target_position - bullet_spawn_position
            });

            if let Some(direction) = direction{

                let tower_forward = transform.rotation() * -Vec3::Z;
                // Ensure vectors are properly normalized by normalizing twice
                let normalized_tower_forward = tower_forward.normalize().normalize();
                let normalized_direction = direction.normalize().normalize();
                let rotation_to_target = Quat::from_rotation_arc(normalized_tower_forward, normalized_direction);
                let rotation = rotation_to_target * transform.rotation();
                
                
                commands.entity(tower_entity).with_children(|commands|{
                    commands.spawn((
                        SceneRoot(projectile_assets.scene.clone()),
                        Transform::from_translation(tower.projectile_offset).with_rotation(rotation),
                    )).insert(Projectile{
                        speed: 2.0,
                        direction,
                        life_timer: Timer::from_seconds(2.0, TimerMode::Once)
                    }).insert(Name::new("Projectile"));                    
                });



                
            }
        }
    }
}

const GLF_TOWER_1: &str = "glb/tower_01.glb";
const GLF_TOWER_2: &str = "glb/tower_02.glb";

#[derive(Default, Resource)]
struct TowerAssets{
    scene_1: Handle<Scene>,
    scene_2: Handle<Scene>,
}