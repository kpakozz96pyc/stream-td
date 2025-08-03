use bevy::color::{ Srgba};
use bevy::math::{ Vec3};
use bevy::pbr::{ StandardMaterial};
use bevy::prelude::*;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App){
        app.register_type::<Projectile>()
            .add_systems(Update, projectile_fly);

        return;
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Projectile{
    pub speed: f32,
    pub direction: Vec3,
}

fn projectile_fly(mut projectiles: Query<(&mut Transform, &Projectile)>, time: Res<Time>){
    for (mut transform, projectile) in projectiles.iter_mut(){
        transform.translation += projectile.direction*projectile.speed*time.delta_secs();
    }
}