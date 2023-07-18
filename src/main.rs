#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::prelude::*;
use bevy_egui::{egui, *};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_systems(Startup, setup_fonts)
        .add_systems(Update, ui_example_system)
        .run();
}

fn setup_fonts(mut contexts: EguiContexts) {
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters):
    fonts.font_data.insert(
        "mono_font".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "../assets/fonts/sarasa-fixed-slab-tc-regular.ttf"
        )),
    ); // .ttf and .otf supported

    /*
    fonts.font_data.insert(
        "latin_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/fonts/CascadiaMono-Regular.otf")),
    );

    fonts.font_data.insert(
        "tc_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/fonts/NotoSerifTC-Regular.otf")),
    );*/

    // Put my font first (highest priority):
    /*
        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "tc_font".to_owned()); // 等下會被推到第二順位

        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "latin_font".to_owned()); // latin字型要在中文字型前
    */

    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "mono_font".to_owned()); // latin字型要在中文字型前

    // Put my font as last fallback for monospace:
    // 不知道什麼情況才會用到Monospace
    fonts
        .families
        .get_mut(&egui::FontFamily::Monospace)
        .unwrap()
        .push("mono_font".to_owned());

    contexts.ctx_mut().set_fonts(fonts);
}

fn ui_example_system(mut contexts: EguiContexts) {
    egui::Window::new("世界").show(contexts.ctx_mut(), |ui| {
        ui.label("和平");
        ui.label("0oOli1Kk");
    });
}
