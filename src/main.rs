use bevy::app::App;
use bevy::prelude::*;
use bevy::color::{Color, Srgba};
use bevy::math::FloatOrd;
use bevy::prelude::{Camera3d, ClearColor, Commands};
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct Target{
    pub speed: f32,
    pub health: f32,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct Projectile{
    pub speed: f32,
    pub direction: Vec3,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct Tower{
    pub shooting_timer: Timer,
    pub projectile_offset: Vec3,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::Srgba(Srgba::new(0.3,0.3,0.3, 1.0))))
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, (spawn_camera, spawn_scene, spawn_tower, spawn_light, spawn_targets))
        .add_systems(Update, (move_targets, spawn_projectiles, projectile_fly))
        .run();
}

fn spawn_camera(mut commands: Commands){
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-5.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    )).insert(Name::new("Camera"));
}

fn spawn_scene(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>){
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10))),
        MeshMaterial3d(materials.add(Color::from(Srgba::new(0.3, 0.5, 0.3, 1.0)))),
    )).insert(Name::new("Scene"));
}

fn spawn_tower(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>){
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::from(Srgba::new(0.6, 0.3, 0.1, 0.8)))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ))
        .insert(Tower{shooting_timer:Timer::from_seconds(1.0, TimerMode::Repeating),
            projectile_offset: Vec3::new(0.0, 0.0, 0.65) })
        .insert(Name::new("Tower"));
}

fn spawn_light(mut commands: Commands){
    commands.spawn((PointLight {
        shadows_enabled: true,
        ..default()
    },
        Transform::from_xyz(5.0, 5.0, 5.0),
    )).insert(Name::new("Sun"));
}

fn spawn_targets(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>){
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
        MeshMaterial3d(materials.add(Color::from(Srgba::new(0.3, 0.3, 0.6, 0.8)))),
        Transform::from_xyz(-2.0, 0.25, 2.0),
    )).insert(Target{speed: 0.5, health: 100.0}).insert(Name::new("Target1"));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
        MeshMaterial3d(materials.add(Color::from(Srgba::new(0.3, 0.3, 0.6, 0.8)))),
        Transform::from_xyz(-4.0, 0.25, 2.0),
    )).insert(Target{speed: 0.3, health: 100.0}).insert(Name::new("Target2"));
}

fn move_targets(mut targets: Query<(&mut Transform, &Target)>, time: Res<Time>){
    for (mut transform, target) in targets.iter_mut(){
        transform.translation.x += target.speed*time.delta_secs();
    }
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
            }).map(|ct|{ct.translation() - bullet_spawn_position});

            if let Some(direction) = direction{
                commands.entity(tower_entity).with_children(|commands|{
                    commands.spawn((
                        Mesh3d(meshes.add(Sphere::new(0.1))),
                        MeshMaterial3d(materials.add(Color::from(Srgba::new(1.0, 0.0, 0.0, 1.0)))),
                        Transform::from_translation(tower.projectile_offset).with_scale(Vec3::new(1.0, 1.0, 1.0)),
                    )).insert(Projectile{speed: 5.0, direction}).insert(Name::new("Projectile"));
                });
            }
        }
    }
}

fn projectile_fly(mut projectiles: Query<(&mut Transform, &Projectile)>, time: Res<Time>){
    for (mut transform, projectile) in projectiles.iter_mut(){
        transform.translation += projectile.direction*projectile.speed*time.delta_secs();
    }
}
