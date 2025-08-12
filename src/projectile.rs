use crate::target::Target;
use bevy::math::Vec3;
use bevy::prelude::*;
use crate::world::Game;

const GLTF_PATH: &str = "glb/arrow_2.glb";

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Projectile>()
            .insert_resource(ArrowAssets::default())
            .add_systems(Startup, (load_arrow_assets,load_blood_assets))
            .add_systems(
                Update,
                (projectile_fly, projectile_collision, projectile_despawn, splashes_systems),
            );

        return;
    }
}

#[derive(Resource)]
pub struct BloodAssets {
    pub splash: Handle<Image>,
}

#[derive(Component)]
struct BloodSplash;

fn load_blood_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(BloodAssets {
        splash: asset_server.load("png/blood.png"),
    });
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

#[derive(Component)]
struct SplashTimer(Timer);

fn projectile_fly(mut projectiles: Query<(&mut Transform, &Projectile)>, time: Res<Time>) {
    for (mut transform, projectile) in projectiles.iter_mut() {
        transform.translation += projectile.direction * projectile.speed * time.delta_secs();
    }
}

fn projectile_collision(
    blood_assets: Res<BloodAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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

                commands.spawn((
                    Mesh3d(meshes.add(Plane3d::default().mesh().size(0.5, 0.5))),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(blood_assets.splash.clone()),
                        alpha_mode: AlphaMode::Blend,
                        unlit: true,
                        ..default()
                    })),
                    Transform {
                        translation: projectile_pos + Vec3::Y * 0.25,
                        rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
                        ..default()
                    })
                ).insert(BloodSplash).insert(SplashTimer(Timer::from_seconds(0.3, TimerMode::Once)));

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

fn splashes_systems(
    mut commands: Commands,
    time: Res<Time>,
    mut param_set: ParamSet<(
        Query<&Transform, With<Camera3d>>,
        Query<(Entity, &mut Transform, &mut SplashTimer, &BloodSplash)>,
    )>,
) {
    // Get camera transform first
    let cam_pos = {
        let camera_q = param_set.p0();
        match camera_q.get_single() {
            Ok(t) => t.translation,
            Err(_) => return,
        }
    };

    // Then process splashes
    let mut splashes = param_set.p1();
    for (entity, mut transform, mut timer, _) in &mut splashes {
        // Таймер
        timer.0.tick(time.delta());
        if timer.0.finished() {
            commands.entity(entity).despawn();
            continue;
        }

        // Билборд — поворачиваем к камере
        let to_camera = cam_pos - transform.translation;
        let normalized_to_camera = to_camera.normalize();
        let look_rotation = Quat::from_rotation_arc(Vec3::Z, normalized_to_camera);
        transform.rotation = look_rotation;
    }
}
