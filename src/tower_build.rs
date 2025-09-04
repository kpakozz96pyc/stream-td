use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui, egui};
use crate::{PlayerState};

pub struct TowerBuildPlugin;

impl Plugin for TowerBuildPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(bevy_egui::EguiPrimaryContextPass, ui_build_panel
            .run_if(resource_exists::<crate::egui_setup::EguiConfigured>)
            .run_if(in_state(PlayerState::Build)));
    }
}

fn ui_build_panel(
    mut egui_ctx: bevy_egui::EguiContexts,
) {
    let ctx = egui_ctx.ctx_mut().unwrap();

    egui::Area::new(egui::Id::new("build_area"))
        .anchor(egui::Align2::RIGHT_TOP, [-12.0, 12.0])
        .interactable(false)
        .show(ctx, |ui| {
            let frame = egui::Frame::window(&ctx.style())
                .fill(ui.visuals().panel_fill)
                .corner_radius(egui::CornerRadius::same(6))
                .inner_margin(egui::Margin::symmetric(10, 8))
                .stroke(ui.visuals().widgets.noninteractive.bg_stroke);

            egui::Frame::show(frame, ui, |ui| {
                ui.heading("Башня");
                ui.separator();
            });
        });
}