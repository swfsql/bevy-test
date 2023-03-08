use crate::game::AppState;
use bevy::prelude::*;
use bevy::window;
use bevy_egui::{egui, EguiContext};

pub struct MainMenu;

pub fn enter() {
    info!("enter");
    // TODO: egui 'enter' necessary stuff?
}

pub fn update(mut next: ResMut<NextState<AppState>>) {
    // info!("update");
    // next.set(AppState::InGame);
}

pub fn exit(mut egui_ctx: Query<&mut EguiContext, With<window::PrimaryWindow>>) {
    info!("exit");

    // TODO: egui cleanup?
}

pub fn ui_example_system(
    mut egui_ctx: Query<&mut EguiContext, With<window::PrimaryWindow>>,
    mut app_exit_events: EventWriter<bevy::app::AppExit>,
    mut next: ResMut<NextState<AppState>>,
) {
    egui::Window::new("Game Menu").show(egui_ctx.single_mut().get_mut(), |ui| {
        let start_btn = ui.button("Start Game");
        let quit_btn = ui.button("Quit Game");

        if start_btn.clicked() {
            next.set(AppState::InGame);
        }
        if quit_btn.clicked() {
            app_exit_events.send(bevy::app::AppExit);
        }
    });
}
