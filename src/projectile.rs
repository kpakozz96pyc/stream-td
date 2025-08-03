use bevy::math::{ Vec3};
use bevy::prelude::*;
use crate::target::Target;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App){
        app.register_type::<Projectile>()
            .add_systems(Update, (projectile_fly, projectile_collision, projectile_despawn));

        return;
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Projectile{
    pub speed: f32,
    pub direction: Vec3,
    pub life_timer: Timer,
}

fn projectile_fly(mut projectiles: Query<(&mut Transform, &Projectile)>, time: Res<Time>){
    for (mut transform, projectile) in projectiles.iter_mut(){
        transform.translation += projectile.direction*projectile.speed*time.delta_secs();
    }
}

fn projectile_collision(
    mut commands: Commands,
    projectiles: Query<(Entity,&mut GlobalTransform), With<Projectile>>,
    mut targets: Query<(Entity,&mut Target, &mut Transform), With<Target>>,
){
    for (projectile, projectile_transform) in projectiles{
        for (te ,mut target, target_transform) in &mut targets{
            if Vec3::distance(projectile_transform.translation(), target_transform.translation) < 0.2{
                target.health -= 15.0;
                commands.entity(projectile).despawn();
                if target.health <= 0.0 {
                    commands.entity(te).despawn()
                }
            }
        }
    }
    return;
}

fn projectile_despawn(mut commands: Commands,
                      projectiles: Query<(Entity, &mut Projectile), With<Projectile>>,
                      time: Res<Time>){
    for (projectile, mut pr  ) in projectiles{
        pr.life_timer.tick(time.delta());
        if pr.life_timer.finished() {
            commands.entity(projectile).despawn();
        }
    }
    return;
}