use bevy::asset::Assets;
use bevy::color::{ Srgba};
use bevy::pbr::{ StandardMaterial};
use bevy::prelude::*;
use bevy::scene::SceneInstanceReady;
use bevy::sprite::{Sprite};
use crate::world::Game;

const GLTF_PATH: &str = "glb/enemy_01.glb";

pub struct TargetPlugin;

impl Plugin for TargetPlugin{
    fn build(&self,app: &mut App){
        app.register_type::<Target>()
            .add_systems(Update, (spawn_targets, move_targets, update_health_bars));
        return;
    }
}


#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Target{
    pub speed: f32,
    pub health: f32,
}

#[derive(Component)]
struct HealthBar {
    max_health: f32,
}

#[derive(Component)]
pub struct TargetAnimation{
    graph_handle: Handle<AnimationGraph>,
    index: AnimationNodeIndex
}

fn spawn_targets(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game: Query<&mut Game>, 
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>){

    let mut game = game.single_mut().unwrap();
    game.target_spawn_timer.tick(time.delta());
    if game.target_spawn_timer.just_finished(){

        let (graph, index) = AnimationGraph::from_clip(
            asset_server.load(GltfAssetLabel::Animation(0).from_asset(GLTF_PATH)),
        );

        let graph_handle = graphs.add(graph);

        let animation_to_play = TargetAnimation {
            graph_handle,
            index,
        };

        let mesh_scene = SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(GLTF_PATH)));
        commands
            .spawn((animation_to_play, mesh_scene, Transform::from_xyz(-2.0, 0.0, 2.0).with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)).with_scale(Vec3::splat(0.15))))
            .insert(Target{speed: 0.3, health: 100.0}).insert(Name::new("Target"))
            .observe(play_animation_when_ready).with_children(|parent| {
            // Background health bar (black)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(3.0, 0.4, 0.1))),
                MeshMaterial3d(materials.add(Color::srgb(0.0, 0.0, 0.0))),
                Transform::from_translation(Vec3::new(0.0, 4.0, 0.0)),
                Visibility::Visible,
                InheritedVisibility::default(),
                ViewVisibility::default(),
            ));

            // Foreground health bar (red)
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(3.0, 0.4, 0.1))),
                MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
                Transform::from_translation(Vec3::new(0.0, 4.0, 0.01)),
                Visibility::Visible,
                InheritedVisibility::default(),
                ViewVisibility::default(),
                HealthBar { max_health: 100.0 },
            ));
        })
        ;
    }
}

fn update_health_bars(
    targets: Query<&Target>,
    mut bars: Query<(&mut Transform, &HealthBar, &ChildOf)>,
) {
    for (mut transform, bar, parent) in &mut bars {
        if let Ok(target) = targets.get(parent.parent()) {
            let health_ratio = (target.health / bar.max_health).clamp(0.0, 1.0);
            // Scale the health bar based on current health percentage
            transform.scale.x = health_ratio;
            // Adjust position to keep the bar aligned to the left
            transform.translation.x = (health_ratio - 1.0) * 1.5;
        }
    }
}

fn play_animation_when_ready(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    animations_to_play: Query<&TargetAnimation>,
    mut players: Query<&mut AnimationPlayer>,
) {

    if let Ok(animation_to_play) = animations_to_play.get(trigger.target()) {
        for child in children.iter_descendants(trigger.target()) {
            if let Ok(mut player) = players.get_mut(child) {

                player.play(animation_to_play.index).repeat();
                commands
                    .entity(child)
                    .insert(AnimationGraphHandle(animation_to_play.graph_handle.clone()));
            }
        }
    }
}

fn move_targets(mut targets: Query<(&mut Transform, &Target)>, time: Res<Time>){
    for (mut transform, target) in targets.iter_mut(){
        transform.translation.x += target.speed*time.delta_secs();
    }
}

