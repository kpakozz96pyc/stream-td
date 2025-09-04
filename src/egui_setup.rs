use bevy::app::{App};
use bevy::log::info;
use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui, egui};

pub struct EguiConfigurePlugin;

impl Plugin for EguiConfigurePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(bevy_egui::EguiPrimaryContextPass, setup_egui_fonts.run_if(
                not(resource_exists::<EguiConfigured>)));
    }
}

#[derive(Resource)]
pub(crate) struct EguiConfigured;

pub fn setup_egui_fonts(mut egui_ctx: bevy_egui::EguiContexts, mut commands: Commands) {
    info!("setup_egui_fonts called");

    let ctx = egui_ctx.ctx_mut().unwrap();

    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "ui_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/fonts/font_regular.ttf")).into(),
    );

    fonts
        .families
        .entry(Proportional)
        .or_default()
        .insert(0, "ui_font".to_owned());

    ctx.set_fonts(fonts);

    let mut style = (*ctx.style()).clone();
    use egui::{FontFamily::Proportional, FontId, TextStyle};
    style.text_styles = [
        (TextStyle::Small,      FontId::new(12.0, Proportional)),
        (TextStyle::Body,       FontId::new(16.0, Proportional)),
        (TextStyle::Button,     FontId::new(16.0, Proportional)),
        (TextStyle::Heading,    FontId::new(20.0, Proportional)),
        (TextStyle::Monospace,  FontId::new(14.0, Proportional)), // или Monospace
    ]
        .into();
    ctx.set_style(style);
    commands.insert_resource(EguiConfigured);
}