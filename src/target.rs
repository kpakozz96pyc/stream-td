use bevy::asset::Assets;
use bevy::prelude::*;
use bevy::scene::SceneInstanceReady;
use rand::{Rng};
use crate::AppState;
use crate::world::Game;

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Target>()
            .insert_resource(TargetCatalog::default())
            .add_systems(Startup, load_assets)
            .add_observer(play_animation_when_ready)
            .add_systems(
                Update,
                (
                    spawn_targets,
                    move_targets,
                ).run_if(in_state(AppState::InGame)
            ));
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Target;

fn spawn_targets(
    mut commands: Commands,
    catalog: Res<TargetCatalog>,
    time: Res<Time>,
    mut game: Query<&mut Game>,
) {
    let mut game = game.single_mut().unwrap();
    if !game.target_spawn_timer.tick(time.delta()).just_finished() {
        return;
    }

    if catalog.entries.is_empty() {
        return;
    }
    let mut rng = rand::rng();
    let kind = rng.random_range(0..catalog.entries.len());
    let entry = &catalog.entries[kind];

    commands
        .spawn((
            Name::new(entry.name),
            Target,
            TargetKind(kind),
            Speed(0.3),
            Health(300.0),
            SceneRoot(entry.scene.clone()),
            Transform::from_xyz(-2.0, 0.0, 2.0)
                .with_scale(Vec3::splat(entry.spawn_scale))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI * 0.5)),
        ));
}

fn move_targets(mut q: Query<(&mut Transform, &Speed), With<Target>>, time: Res<Time>) {
    for (mut t, s) in &mut q {
        t.translation.x += s.0 * time.delta_secs();
    }
}

const GLB_TARGET_1: &str = "glb/target_01.glb";
const GLB_TARGET_2: &str = "glb/target_02.glb";

#[derive(Clone)]
struct TargetEntry {
    scene: Handle<Scene>,
    graph: Handle<AnimationGraph>,
    clip_index: AnimationNodeIndex,
    spawn_scale: f32,
    name: &'static str,
}

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct TargetKind(pub usize);

#[derive(Resource, Default)]
struct TargetCatalog {
    entries: Vec<TargetEntry>,
}

const ANIM_WALK_IDX: usize = 6;

fn load_assets(
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut catalog: ResMut<TargetCatalog>,
) {
    // Target 1
    let scene1 = asset_server.load(GltfAssetLabel::Scene(0).from_asset(GLB_TARGET_1));
    let (graph1, idx1) = AnimationGraph::from_clip(
        asset_server.load(GltfAssetLabel::Animation(ANIM_WALK_IDX).from_asset(GLB_TARGET_1)),
    );
    let graph1 = graphs.add(graph1);

    // Target 2
    let scene2 = asset_server.load(GltfAssetLabel::Scene(0).from_asset(GLB_TARGET_2));
    let (graph2, idx2) = AnimationGraph::from_clip(
        asset_server.load(GltfAssetLabel::Animation(ANIM_WALK_IDX).from_asset(GLB_TARGET_2)),
    );
    let graph2 = graphs.add(graph2);

    catalog.entries = vec![
        TargetEntry {
            scene: scene1,
            graph: graph1,
            clip_index: idx1,
            spawn_scale: 0.15,
            name: "Target_1",
        },
        TargetEntry {
            scene: scene2,
            graph: graph2,
            clip_index: idx2,
            spawn_scale: 0.25,
            name: "Target_2",
        },
    ];
}

fn play_animation_when_ready(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,                    // ← добавили
    children: Query<&Children>,
    mut players: Query<&mut AnimationPlayer>,
    graph_handle: Query<&AnimationGraphHandle>,
    catalog: Res<TargetCatalog>,
    kind_q: Query<&TargetKind>,
) {
    let Ok(kind) = kind_q.get(trigger.target()) else { return; };
    let Some(entry) = catalog.entries.get(kind.0) else { return; };

    for child in children.iter_descendants(trigger.target()) {
        if let Ok(mut player) = players.get_mut(child) {
            // гарантированно повесим граф на того же энтити, где player
            if graph_handle.get(child).is_err() {
                commands.entity(child)
                    .insert(AnimationGraphHandle(entry.graph.clone()));
            }
            player.play(entry.clip_index).repeat();
            break;
        }
    }
}

