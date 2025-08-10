use std::ops::Add;
use bevy::asset::Assets;
use bevy::color::{Color, Srgba};
use bevy::math::{FloatOrd, Vec3};
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::*;
use crate::projectile::Projectile;
use crate::target::Target;

pub struct TowerPlugin;

impl Plugin for TowerPlugin{
    fn build(&self,app: &mut App){
        app.register_type::<Tower>()
            .add_systems(Startup, spawn_tower)
            .add_systems(Update, spawn_projectiles);
        
        return;
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tower{
    pub shooting_timer: Timer,
    pub projectile_offset: Vec3,
}

fn spawn_tower(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>){
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::from(Srgba::new(0.6, 0.3, 0.1, 0.8)))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ))
        .insert(Tower{shooting_timer:Timer::from_seconds(1.0, TimerMode::Repeating),
            projectile_offset: Vec3::new(0.0, 0.0, 0.65) })
        .insert(Name::new("Tower1"));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::from(Srgba::new(0.6, 0.3, 0.1, 0.8)))),
        Transform::from_xyz(3.0, 0.5, 0.0),
    ))
        .insert(Tower{shooting_timer:Timer::from_seconds(1.0, TimerMode::Repeating),
            projectile_offset: Vec3::new(0.0, 0.0, 0.65) })
        .insert(Name::new("Tower2"));
    
}

fn spawn_projectiles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut towers: Query<(&GlobalTransform, &mut Tower, Entity)>,
    targets: Query<&GlobalTransform, With<Target>>,
    time: Res<Time>,
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
                commands.entity(tower_entity).with_children(|commands|{
                    commands.spawn((
                        Mesh3d(meshes.add(Sphere::new(0.1))),
                        MeshMaterial3d(materials.add(Color::from(Srgba::new(1.0, 0.0, 0.0, 1.0)))),
                        Transform::from_translation(tower.projectile_offset).with_scale(Vec3::new(0.5, 0.5, 0.5)),
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