use bevy::math::{ Vec3};
use bevy::prelude::*;
use crate::blood::SpawnBlood;
use crate::target::{Health, Target};

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App){
        app.register_type::<Projectile>()
            .insert_resource(ProjectileAssets::default())
            .add_event::<DeathEvent>()
            .add_systems(Startup, load_assets)
            .add_systems(Update, (projectile_fly, projectile_collision, projectile_despawn, play_death_sound));

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
    mut targets: Query<(Entity, &mut Transform, &mut Health), With<Target>>,
    mut blood_ev: EventWriter<SpawnBlood>,
    mut death_ev: EventWriter<DeathEvent>,
){
    for (projectile, projectile_transform) in projectiles{
        for (te, target_transform, mut health) in &mut targets{
            let target_pos = target_transform.translation;
            if Vec3::distance(projectile_transform.translation(), target_pos) < 0.4{
                health.0 -= 15.0;
                commands.entity(projectile).despawn();

                if health.0 <= 0.0 {
                    commands.entity(te).despawn();

                    blood_ev.write(SpawnBlood{pos: target_pos});
                    death_ev.write(DeathEvent{});
                }
            }
        }
    }
    return;
}
#[derive(Event, Default)]
struct DeathEvent;

#[derive(Resource, Deref)]
struct DeathSound(Handle<AudioSource>);


fn play_death_sound(
    mut commands: Commands,
    mut death_events: EventReader<DeathEvent>,
    sound: Res<DeathSound>,
) {
    if !death_events.is_empty() {
        death_events.clear();
        commands.spawn((AudioPlayer(sound.clone()), PlaybackSettings::DESPAWN));
    }
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

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut projectile_assets: ResMut<ProjectileAssets>,
    ){
    projectile_assets.scene = asset_server.load(GltfAssetLabel::Scene(0).from_asset(GLF_PROJECTILE));

    let death_sound = asset_server.load("ogg/death_01.ogg");
    commands.insert_resource(DeathSound(death_sound));

}

pub const GLF_PROJECTILE: &str = "glb/projectile_02.glb";

#[derive(Default, Resource)]
pub struct ProjectileAssets{
    pub scene: Handle<Scene>,
}