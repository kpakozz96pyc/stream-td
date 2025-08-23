use bevy::asset::Assets;
use bevy::color::{Color, Srgba};
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::*;
use bevy::scene::SceneInstanceReady;
use rand::{Rng};
use crate::world::Game;

pub struct TargetPlugin;

impl Plugin for TargetPlugin{
    fn build(&self,app: &mut App){
        app.register_type::<Target>()
            .insert_resource(TargetAssets1::default())
            .insert_resource(TargetAssets2::default())
            .add_systems(Startup, load_assets)
            .add_systems(Update, (spawn_targets, move_targets));

        return;
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Target{
    pub speed: f32,
    pub health: f32,
}

fn spawn_targets(
    mut commands: Commands,
    mut game: Query<&mut Game>,
    //ToDo: разобраться как делать загрузку одних объектов с разными glb и анимациями
    mut target_assets1: ResMut<TargetAssets1>,
    mut target_assets2: ResMut<TargetAssets2>,
    asset_server: Res<AssetServer>,
    time: Res<Time>){
    let mut game = game.single_mut().unwrap();
    target_assets1.scene = asset_server.load(GltfAssetLabel::Scene(0).from_asset(GLF_TARGET_1));
    target_assets2.scene = asset_server.load(GltfAssetLabel::Scene(0).from_asset(GLF_TARGET_2));
    game.target_spawn_timer.tick(time.delta());
    if game.target_spawn_timer.just_finished(){
        let mut rng = rand::rng();
        let n: u8 = rng.random_range(1..=2);

        if n == 1 {
            commands.spawn((
                SceneRoot(target_assets1.scene.clone()),
                Transform::from_xyz(-2.0, 0.0, 2.0).with_scale(Vec3::splat(0.15)).with_rotation(Quat::from_rotation_y(std::f32::consts::PI / 2.0)),
            )).insert(Target { speed: 0.3, health: 100.0 }).insert(Name::new("Target1")).observe(play_animation_when_ready_1);
        }
        if n == 2 {
            commands.spawn((
                SceneRoot(target_assets2.scene.clone()),
                Transform::from_xyz(-2.0, 0.0, 2.0).with_scale(Vec3::splat(0.25)).with_rotation(Quat::from_rotation_y(std::f32::consts::PI / 2.0)),
            )).insert(Target { speed: 0.3, health: 100.0 }).insert(Name::new("Target2")).observe(play_animation_when_ready_2);
        }
    }
}

fn move_targets(mut targets: Query<(&mut Transform, &Target)>, time: Res<Time>){
    for (mut transform, target) in targets.iter_mut(){
        transform.translation.x += target.speed*time.delta_secs();
    }
}

const GLF_TARGET_1: &str = "glb/target_01.glb";
const GLF_TARGET_2: &str = "glb/target_02.glb";

#[derive(Default, Resource)]
struct TargetAssets1 {
    scene: Handle<Scene>,
    animation_graph: Handle<AnimationGraph>,
    animation_index: AnimationNodeIndex,
}

#[derive(Default, Resource)]
struct TargetAssets2 {
    scene: Handle<Scene>,
    animation_graph: Handle<AnimationGraph>,
    animation_index: AnimationNodeIndex,
}

fn load_assets(
    asset_server: Res<AssetServer>,
    mut target_assets1: ResMut<TargetAssets1>,
    mut target_assets2: ResMut<TargetAssets2>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
){
    let (graph1, index1) = AnimationGraph::from_clip(
        asset_server.load(GltfAssetLabel::Animation(6).from_asset(GLF_TARGET_1)),
    );

    let (graph2, index2) = AnimationGraph::from_clip(
        asset_server.load(GltfAssetLabel::Animation(6).from_asset(GLF_TARGET_2)),
    );

    let graph_handle_1 = graphs.add(graph1);
    target_assets1.animation_graph = graph_handle_1;
    target_assets1.animation_index = index1;

    let graph_handle_2 = graphs.add(graph2);
    target_assets2.animation_graph = graph_handle_2;
    target_assets2.animation_index = index2;

}

fn play_animation_when_ready_1(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    target_assets: Res<TargetAssets1>,
    mut players: Query<&mut AnimationPlayer>
){
    for child in children.iter_descendants(trigger.target()){

        if let Ok (mut player) = players.get_mut(child){
            player.play(target_assets.animation_index).repeat();
            commands.entity(child).insert(AnimationGraphHandle(target_assets.animation_graph.clone()));

            return;
        }
    }
}

fn play_animation_when_ready_2(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    target_assets: Res<TargetAssets2>,
    mut players: Query<&mut AnimationPlayer>
){
    for child in children.iter_descendants(trigger.target()){

        if let Ok (mut player) = players.get_mut(child){
            player.play(target_assets.animation_index).repeat();
            commands.entity(child).insert(AnimationGraphHandle(target_assets.animation_graph.clone()));

            return;
        }
    }
}

