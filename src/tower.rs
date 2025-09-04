use crate::{AppState};
use crate::projectile::Projectile;
use crate::target::Target;
use bevy::math::{FloatOrd, Vec3};
use bevy::prelude::*;
use std::collections::HashMap;
use bevy::audio::Volume;
use bevy::color::palettes::css::YELLOW;
use bevy::scene::SceneInstanceReady;
use bevy_inspector_egui::{bevy_egui, egui};

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tower>()
            .add_systems(Update, spawn_some_towers.run_if(resource_added::<TowerDB>))
            .add_systems(Update, spawn_projectiles.run_if(in_state(AppState::InGame)))
            .init_resource::<SelectedTower>()
            .init_resource::<TowerClickFlag>()
            .add_systems(Startup, setup_selection_materials)
            .add_systems(Update, apply_tower_selection.run_if(resource_changed::<SelectedTower>))
            .add_systems(
                Update,
                ui_selected_tower_panel
                    .run_if(tower_selected))
            .add_systems(PreUpdate, reset_tower_click_flag)
            .add_systems(PostUpdate, deselect_on_empty_click.run_if(in_state(AppState::InGame)));
    }
}

// region struct

pub struct TowerPlugin;
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tower {
    pub shooting_timer: Timer,
}

#[derive(Component, Clone)]
pub struct TowerStats {
    pub damage: f32,
    pub projectile_offset: Vec3,
    pub range_sq: f32,
    pub projectile_speed: f32,
    pub projectile_scale: f32,
    pub projectile_scene: Handle<Scene>,
    pub shot_sound: Handle<AudioSource>,
    pub shot_volume: f32
}

#[derive(Bundle)]
pub struct TowerBundle {
    pub tower: Tower,
    pub stats: TowerStats,
    pub scene: SceneRoot,
    pub transform: Transform,
    pub name: Name,
    pub pickable: Pickable
}

#[derive(Resource)]
pub struct TowerDB {
    pub defs: HashMap<String, TowerDef>,
}

pub struct TowerDef {
    pub id: String,
    pub damage: f32,
    pub scene: Handle<Scene>,
    pub projectile_scene: Handle<Scene>,
    pub fire_interval: f32,
    pub range: f32,
    pub projectile_speed: f32,
    pub projectile_scale: f32,
    pub offset: Vec3,
    pub shot_sound: Handle<AudioSource>,
    pub shot_volume: f32
}
// endregion

// region systems
fn spawn_some_towers(mut commands: Commands, db: Res<TowerDB>) {
    let basic = db.defs.get("basic").unwrap();
    let sniper = db.defs.get("sniper").unwrap();

    spawn_tower_of(
        &mut commands,
        basic,
        Vec3::new(0.0, 0.0, 0.0),
    );
    spawn_tower_of(
        &mut commands,
        sniper,
        Vec3::new(3.0, 0.0, 0.0),
    );
}

fn spawn_tower_of(commands: &mut Commands,
                  def: &TowerDef,
                  pos: Vec3) {

    commands.spawn(TowerBundle {
        tower: Tower {
            shooting_timer: Timer::from_seconds(def.fire_interval, TimerMode::Repeating),
        },
        stats: TowerStats {
            damage: def.damage,
            projectile_offset: def.offset,
            range_sq: def.range * def.range,
            projectile_speed: def.projectile_speed,
            projectile_scale: def.projectile_scale,
            projectile_scene: def.projectile_scene.clone(),
            shot_sound: def.shot_sound.clone(),
            shot_volume: def.shot_volume
        },
        scene: SceneRoot(def.scene.clone()),
        transform: Transform::from_translation(pos),
        name: Name::new(def.id.clone()),
        pickable: Pickable::default(),
    }).observe(attach_tower_clickables);
}

fn spawn_projectiles(
    mut commands: Commands,
    mut towers: Query<(&GlobalTransform, &mut Tower, &TowerStats)>,
    targets: Query<&GlobalTransform, With<Target>>,
    time: Res<Time>,
) {
    for (gt, mut tower, stats) in &mut towers {
        tower.shooting_timer.tick(time.delta());
        if !tower.shooting_timer.just_finished() {
            continue;
        }

        let muzzle = gt.translation() + stats.projectile_offset;

        let maybe_dir = targets
            .iter()
            .filter_map(|tgt| {
                let to = tgt.translation() + Vec3::Y * 0.3 - muzzle;
                let d2 = to.length_squared();
                if d2 <= stats.range_sq {
                    Some((d2, to))
                } else {
                    None
                }
            })
            .min_by_key(|(d2, _)| FloatOrd(*d2))
            .map(|(_, to)| to);

        if let Some(dir) = maybe_dir {
            let dir_norm = dir.normalize();
            let rot = Quat::from_rotation_arc(-Vec3::Z, dir_norm);

            commands.spawn((
                SceneRoot(stats.projectile_scene.clone()),
                Transform::from_translation(muzzle)
                    .with_rotation(rot)
                    .with_scale(Vec3::splat(stats.projectile_scale)),
                Projectile {
                    speed: stats.projectile_speed,
                    direction: dir_norm,
                    life_timer: Timer::from_seconds(2.0, TimerMode::Once),
                    damage: stats.damage,
                },
                Name::new("Projectile"),
            ));
            commands.spawn((
                Name::new("shot_sound"),
                Transform::from_translation(muzzle),
                GlobalTransform::default(),
                AudioPlayer(stats.shot_sound.clone()),
                PlaybackSettings::DESPAWN
                    .with_spatial(true)
                    .with_volume(Volume::Linear(stats.shot_volume))
            ));
        }
    }
}

#[derive(Component)]
struct TowerRoot(Entity);


fn attach_tower_clickables(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    towers: Query<&Tower>,
    children_q: Query<&Children>,
    mesh_q: Query<&Mesh3d>,
) {
    info!("attach_tower_clickables called");
    let root = trigger.target();
    if towers.get(root).is_err() {
        return;
    }

    for child in children_q.iter_descendants(root) {
        if mesh_q.get(child).is_ok() {
            commands.entity(child)
                .insert(Pickable::default())
                .insert(TowerRoot(root))
                .observe(on_tower_click);
        }
    }
}

fn on_tower_click(
    trigger: Trigger<Pointer<Click>>,
    roots: Query<&TowerRoot>,
    names: Query<&Name>,
    mut selected: ResMut<SelectedTower>
) {
    if let Ok(root) = roots.get(trigger.target()) {
        if selected.0 == Some(root.0) {
            selected.0 = None;
            if let Ok(name) = names.get(root.0) {
                info!("Tower deselected: {}", name.as_str());
            } else {
                info!("Tower deselected");
            }
        } else {
            selected.0 = Some(root.0);
            if let Ok(name) = names.get(root.0) {
                info!("Tower selected: {}", name.as_str());
            } else {
                info!("Tower selected");
            }
        }
    }
}

fn apply_tower_selection(
    selected: Res<SelectedTower>,
    highlight: Res<SelectionMat>,
    mut q_meshes: Query<(Entity, &TowerRoot, &mut MeshMaterial3d<StandardMaterial>, Option<&MeshOriginalMaterial>)>,
    mut commands: Commands,
) {
    for (entity, tower_root, mut mat, original) in &mut q_meshes {
        let should_highlight = selected.0 == Some(tower_root.0);

        match (should_highlight, original) {
            // Включаем подсветку: запоминаем исходный материал и ставим выделенный
            (true, None) => {
                let prev = mat.0.clone();
                commands.entity(entity).insert(MeshOriginalMaterial(prev));
                mat.0 = highlight.0.clone();
            }
            // Уже подсвечен — ничего не делаем
            (true, Some(_)) => {}
            // Снимаем подсветку: восстанавливаем исходный и убираем метку
            (false, Some(orig)) => {
                mat.0 = orig.0.clone();
                commands.entity(entity).remove::<MeshOriginalMaterial>();
            }
            // И так не подсвечен — ничего
            (false, None) => {}
        }
    }
}



#[derive(Component, Clone)]
struct MeshOriginalMaterial(Handle<StandardMaterial>);

#[derive(Resource, Default, Deref, DerefMut)]
struct SelectedTower(Option<Entity>);

#[derive(Resource)]
struct SelectionMat(Handle<StandardMaterial>);

fn setup_selection_materials(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    let highlight = materials.add(Color::from(YELLOW));
    commands.insert_resource(SelectionMat(highlight));
}
fn tower_selected(selected: Res<SelectedTower>) -> bool {
    selected.0.is_some()
}

fn ui_selected_tower_panel(
    mut egui_ctx: bevy_egui::EguiContexts,
    selected: Res<SelectedTower>,
    stats_q: Query<(&TowerStats, Option<&Name>)>,
) {
    let Some(entity) = selected.0 else { return; };

    let ctx = egui_ctx.ctx_mut().unwrap();

    egui::Area::new(egui::Id::new("tower_info_area"))
        .anchor(egui::Align2::RIGHT_BOTTOM, [-12.0, -12.0])
        .show(ctx, |ui| {
            let frame = egui::Frame::window(&ctx.style())
                .fill(ui.visuals().panel_fill)
                .corner_radius(egui::CornerRadius::same(6))
                .inner_margin(egui::Margin::symmetric(10, 8))
                .stroke(ui.visuals().widgets.noninteractive.bg_stroke);

            egui::Frame::show(frame, ui, |ui| {
                ui.heading("Башня");
                ui.separator();

                if let Ok((stats, name)) = stats_q.get(entity) {
                    if let Some(name) = name {
                        ui.label(format!("ID: {}", name.as_str()));
                    }
                    ui.label(format!("Damage: {:.1}", stats.damage));
                    ui.label(format!("Range: {:.1}", stats.range_sq.sqrt()));
                    ui.label(format!("Projectile speed: {:.1}", stats.projectile_speed));
                    ui.label(format!("Proj size: {:.2}", stats.projectile_scale));
                    ui.label(format!("Volume: {:.1}", stats.shot_volume));
                } else {
                    ui.label("Нет данных по выбранной башне");
                }
            });
        });
}

#[derive(Resource, Default)]
struct TowerClickFlag(bool);

fn reset_tower_click_flag(mut flag: ResMut<TowerClickFlag>) {
    flag.0 = false;
}


fn deselect_on_empty_click(
    mouse: Res<ButtonInput<MouseButton>>,
    mut egui_ctx: bevy_egui::EguiContexts,
    mut selected: ResMut<SelectedTower>,
    flag: Res<TowerClickFlag>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    if flag.0 {
        return;
    }

    let ctx_result = egui_ctx.ctx_mut();
    if ctx_result.is_err() { return; }
    if ctx_result.unwrap().wants_pointer_input() {
        return;
    }



    if selected.0.is_some() {
        info!("Tower deselected by empty click");
        selected.0 = None;
    }
}

// endregion
