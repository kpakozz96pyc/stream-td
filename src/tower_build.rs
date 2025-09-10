use crate::PlayerState;
use crate::tower::{TowerDB, TowerDef, spawn_tower_of, Tower, TowerStats, TowerBundle};
use bevy::diagnostic::FrameCount;
use bevy::pbr::{NotShadowCaster, NotShadowReceiver};
use bevy::prelude::*;
use bevy::scene::SceneInstance;
use bevy::window::PrimaryWindow;
use bevy_inspector_egui::{bevy_egui, egui};

pub struct TowerBuildPlugin;

impl Plugin for TowerBuildPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(TowerPreviewEntity::default())
            .add_systems(Startup, setup_preview_material)
            .add_event::<SelectTowerToBuildEvent>()
            .init_resource::<SelectedToBuildTower>()
            .add_systems(
                bevy_egui::EguiPrimaryContextPass,
                ui_build_panel
                    .run_if(resource_exists::<crate::egui_setup::EguiConfigured>)
                    .run_if(in_state(PlayerState::Build)),
            )
            .add_systems(
                Update,
                place_selected_tower_on_click.run_if(in_state(PlayerState::Build)),
            )
            .add_systems(Update, on_select_tower)
            .add_systems(Update, (
                on_select_tower,
                update_preview_position.run_if(in_state(PlayerState::Build)),
                tint_scene_preview_white
                    .run_if(tower_to_build_selected_need_to_be_tinted)
                    .run_if(in_state(PlayerState::Build)),
                place_selected_tower_on_click.run_if(in_state(PlayerState::Build)),
            ));;
    }
}

fn tower_to_build_selected_need_to_be_tinted(selected: Res<SelectedToBuildTower>) -> bool {
    selected.def.is_some() && !selected.tinted
}

#[derive(Resource)]
struct PreviewMaterial(pub Handle<StandardMaterial>);

#[derive(Resource, Default)]
struct TowerPreviewEntity(Option<Entity>);

#[derive(Component)]
pub struct TowerPreview;


fn setup_preview_material(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    let mat = materials.add(StandardMaterial {
        base_color: Color::WHITE.with_alpha(0.4), // полупрозрачный
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    });
    commands.insert_resource(PreviewMaterial(mat));
}

fn ui_build_panel(
    mut egui_ctx: bevy_egui::EguiContexts,
    tower_db: Res<TowerDB>,
    mut select_event_writer: EventWriter<SelectTowerToBuildEvent>,
) {
    let ctx = egui_ctx.ctx_mut().unwrap();

    egui::Area::new(egui::Id::new("build_area"))
        .anchor(egui::Align2::RIGHT_TOP, [-12.0, 12.0])
        .interactable(true)
        .show(ctx, |ui| {
            let frame = egui::Frame::window(&ctx.style())
                .fill(ui.visuals().panel_fill)
                .corner_radius(egui::CornerRadius::same(6))
                .inner_margin(egui::Margin::symmetric(10, 8))
                .stroke(ui.visuals().widgets.noninteractive.bg_stroke);

            egui::Frame::show(frame, ui, |ui| {
                ui.heading("Towers: ");
                ui.separator();
                tower_db.defs.iter().for_each(|(name, def)| {
                    ui.label(format!("Name: {} ", name));
                    ui.label(format!("damage: {} ", def.damage));
                    if ui.button("Build!").clicked() {
                        select_event_writer.write(SelectTowerToBuildEvent(def.clone()));
                    };
                    ui.separator();
                });
            });
        });
}

#[derive(Resource, Default)]
struct SelectedToBuildTower {
    def: Option<TowerDef>,
    armed_frame: u32,
    awaiting_click: bool,
    tinted: bool
}

#[derive(Event, Clone)]
struct SelectTowerToBuildEvent(pub TowerDef);

fn on_select_tower(
    mut evr: EventReader<SelectTowerToBuildEvent>,
    mut selected: ResMut<SelectedToBuildTower>,
    frames: Res<FrameCount>,
    mut commands: Commands,
    mut preview_ent: ResMut<TowerPreviewEntity>,
) {
    for SelectTowerToBuildEvent(def) in evr.read().cloned() {
        selected.def = Some(def.clone());
        selected.armed_frame = frames.0;
        selected.awaiting_click = true;

        if let Some(e) = preview_ent.0.take() {
            commands.entity(e).despawn();
        }

        let e = commands.spawn(TowerPreviewBundle {
            scene: SceneRoot(def.scene.clone()),
            marker: TowerPreview,
        }
        ).id();

        preview_ent.0 = Some(e);
    }
}

fn update_preview_position(
    selected: Res<SelectedToBuildTower>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut transforms: Query<&mut Transform>,
    preview_ent: Res<TowerPreviewEntity>,
) {
    if !selected.awaiting_click {
        return;
    }
    let Some(e) = preview_ent.0 else { return; };

    let Ok(window) = windows.single() else { return; };
    let Some(cursor_pos) = window.cursor_position() else { return; };

    let mut hit_world: Option<Vec3> = None;
    for (camera, cam_gt) in &cameras {
        let ray = camera.viewport_to_world(cam_gt, cursor_pos).expect("cant cast ray");
            let o = ray.origin;
            let d = ray.direction;
            if d.y.abs() > f32::EPSILON {
                let t = -o.y / d.y;
                if t >= 0.0 {
                    hit_world = Some(o + d * t);
                    break;
                }
            }
    }

    if let (Some(pos), Ok(mut tr)) = (hit_world, transforms.get_mut(e)) {
        tr.translation = pos;
    }
}

fn place_selected_tower_on_click(
    mut commands: Commands,
    mut selected: ResMut<SelectedToBuildTower>,
    frames: Res<FrameCount>,
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    if !selected.awaiting_click {
        return;
    }
    let Some(def) = selected.def.as_ref() else {
        return;
    };

    if frames.0 == selected.armed_frame {
        return;
    }

    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(window) = windows.get_single() else {
        return;
    };
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    let mut hit_world: Option<Vec3> = None;
    for (camera, cam_gt) in &cameras {
        let ray = camera
            .viewport_to_world(cam_gt, cursor_pos)
            .expect("cant cast ray");
        let o = ray.origin;
        let d = ray.direction;
        if d.y.abs() > f32::EPSILON {
            let t = -o.y / d.y;
            if t >= 0.0 {
                hit_world = Some(o + d * t);
                break;
            }
        }
    }

    if let Some(pos) = hit_world {
        spawn_tower_of(&mut commands, def, pos);
        selected.def = None;
        selected.awaiting_click = false;
        selected.tinted = false;
        selected.armed_frame = 0;
    }
}

#[derive(Bundle)]
pub struct TowerPreviewBundle {
    pub scene: SceneRoot,
    pub marker: TowerPreview,

}

fn tint_scene_preview_white(
    mut selected: ResMut<SelectedToBuildTower>,
    preview_ent: Res<TowerPreviewEntity>,
    scene_spawner: Res<SceneSpawner>,
    preview_mat: Res<PreviewMaterial>,
    q_instance: Query<&SceneInstance, With<TowerPreview>>,
    mut mesh_mats: Query<&mut MeshMaterial3d<StandardMaterial>>,
    mut commands: Commands,
) {
    info!("tint_scene_preview_white called");
    let Some(root) = preview_ent.0 else { return; };
    let Ok(instance) = q_instance.get(root) else { return; };

    if !scene_spawner.instance_is_ready(**instance) {
        return;
    }

    for child in scene_spawner.iter_instance_entities(**instance) {
        if let Ok(mut mat_handle) = mesh_mats.get_mut(child) {

            mat_handle.0 = preview_mat.0.clone();
        }
        commands.entity(child).insert((NotShadowCaster, NotShadowReceiver));
    }
    selected.tinted = true;
}

