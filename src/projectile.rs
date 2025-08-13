use crate::target::Target;
use bevy::math::Vec3;
use bevy::prelude::*;

const GLTF_PATH: &str = "glb/arrow_2.glb";

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Projectile>()
            .insert_resource(ArrowAssets::default())
            .add_systems(Startup, load_arrow_assets)
            .add_systems(
                Update,
                (projectile_fly, projectile_collision, projectile_despawn),
            );

        return;
    }
}

#[derive(Resource, Default)]
pub struct ArrowAssets {
    pub scene: Handle<Scene>,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Projectile {
    pub speed: f32,
    pub direction: Vec3,
    pub life_timer: Timer,
}

fn projectile_fly(mut projectiles: Query<(&mut Transform, &Projectile)>, time: Res<Time>) {
    for (mut transform, projectile) in projectiles.iter_mut() {
        transform.translation += projectile.direction * projectile.speed * time.delta_secs();
    }
}

fn projectile_collision(
    mut commands: Commands,
    mut param_set: ParamSet<(
        Query<(Entity, &GlobalTransform), With<Projectile>>,
        Query<(Entity, &mut Target, &GlobalTransform), With<Target>>,
    )>,
) {
    let projectiles: Vec<(Entity, Vec3)> = param_set
        .p0()
        .iter()
        .map(|(entity, transform)| (entity, transform.translation()))
        .collect();

    for (projectile, projectile_pos) in projectiles {
        let mut despawn_projectile = false;

        for (te, mut target, target_transform) in &mut param_set.p1() {
            let target_pos = target_transform.translation();
            if Vec3::distance(projectile_pos, target_pos) < 0.5 {
                target.health -= 15.0;
                despawn_projectile = true;
                if target.health <= 0.0 {
                    commands.entity(te).despawn();
                }
            }
        }

        if despawn_projectile {
            commands.entity(projectile).despawn();
        }
    }
    return;
}

fn projectile_despawn(
    mut commands: Commands,
    projectiles: Query<(Entity, &mut Projectile), With<Projectile>>,
    time: Res<Time>,
) {
    for (projectile, mut pr) in projectiles {
        pr.life_timer.tick(time.delta());
        if pr.life_timer.finished() {
            commands.entity(projectile).despawn();
        }
    }
    return;
}

fn load_arrow_assets(asset_server: Res<AssetServer>, mut target_assets: ResMut<ArrowAssets>) {
    target_assets.scene = asset_server.load(GltfAssetLabel::Scene(0).from_asset(GLTF_PATH));
}