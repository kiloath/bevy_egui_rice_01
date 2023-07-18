#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::prelude::*;
use bevy_egui::{egui, *};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_systems(Update, ui_example_system)
        .run();
}

fn ui_example_system(mut contexts: EguiContexts) {
    egui::Window::new("Layout").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("horzontal1.label_1");
            ui.label("horzontal1.labe1_2");
        });
        ui.horizontal(|ui| {
            ui.label("horizontal2.label_1");
            ui.label("horizontal2.labe1_2");
        });
        ui.vertical(|ui| {
            ui.label("vertical1.label_1");
            ui.label("vertical1.labe1_2");
        });
        ui.vertical_centered(|ui| {
            ui.label("vertical_centered1.label_1");
            ui.label("vertical_centered2.labe1_2");
        });
    });
}
