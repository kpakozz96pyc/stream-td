use bevy::asset::Assets;
use bevy::prelude::*;
use bevy::scene::SceneInstanceReady;
use rand::Rng;
use crate::world::Game;

const GLTF_PATH: &str = "glb/enemy_01.glb";

pub struct TargetPlugin;

impl Plugin for TargetPlugin{
    fn build(&self,app: &mut App){
        app.register_type::<Target>()
            .insert_resource(TargetAssets::default())
            .add_systems(Startup, load_target_assets)
            .add_systems(Update, (spawn_targets, move_targets, despawn_targets));

        return;
    }
}


#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Target{
    pub speed: f32,
    pub health: f32,
}

#[derive(Resource, Default)]
pub struct TargetAssets {
    scene: Handle<Scene>,
    animation_graph: Handle<AnimationGraph>,
    animation_index: AnimationNodeIndex,
}

fn load_target_assets(
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut target_assets: ResMut<TargetAssets>,
) {
    let (graph, index) = AnimationGraph::from_clip(
        asset_server.load(GltfAssetLabel::Animation(0).from_asset(GLTF_PATH)),
    );
    let graph_handle = graphs.add(graph);

    target_assets.scene = asset_server.load(GltfAssetLabel::Scene(0).from_asset(GLTF_PATH));
    target_assets.animation_graph = graph_handle;
    target_assets.animation_index = index;
}

fn spawn_targets(
    mut commands: Commands,
    mut game: Query<&mut Game>, 
    time: Res<Time>,
    target_assets: Res<TargetAssets>,
){

    let mut game = game.single_mut().unwrap();
    game.target_spawn_timer.tick(time.delta());
    if game.target_spawn_timer.just_finished(){


        commands
            .spawn((
                    SceneRoot(target_assets.scene.clone()),
                    Transform::from_xyz(-2.0, 0.0, 2.0).with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)).with_scale(Vec3::splat(0.15))))
            .insert(Target{speed: 0.3, health: 100.0}).insert(Name::new("Target"))
            .observe(play_animation_when_ready);
    }
}

fn play_animation_when_ready(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    target_assets: Res<TargetAssets>,
    mut players: Query<&mut AnimationPlayer>,
) {


        for child in children.iter_descendants(trigger.target()) {
            if let Ok(mut player) = players.get_mut(child) {
                let mut rng = rand::rng();
                let speed = rng.random_range(0.5..1.5);

                player.play(target_assets.animation_index).repeat().set_seek_time(speed);
                commands
                    .entity(child)
                    .insert(AnimationGraphHandle(target_assets.animation_graph.clone()));
            }
        }

}

fn move_targets(mut targets: Query<(&mut Transform, &Target)>, time: Res<Time>){
    for (mut transform, target) in targets.iter_mut(){
        transform.translation.x += target.speed*time.delta_secs();
    }
}

fn despawn_targets(mut commands: Commands, mut targets: Query<(&Transform, Entity)>){
    for (transform, entity) in targets.iter_mut(){
        if transform.translation.x > 30.0{
            commands.entity(entity).despawn();
        }
    }
}

