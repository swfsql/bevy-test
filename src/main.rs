use bevy::prelude::*;

pub mod default_settings;
pub mod game;
pub mod hello;
pub mod state_plugin;

fn main() {
    println!("before-run");
    App::new()
        .add_plugin(default_settings::DefaultSettingsPlugin)
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        //
        .add_plugin(bevy_egui::EguiPlugin)
        //
        .add_plugin(hello::HelloPlugin)
        .add_plugin(game::GamePlugin)
        .run();
    println!("after-run");
}
