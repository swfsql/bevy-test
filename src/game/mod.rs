use crate::state_plugin;
use bevy::prelude::*;

pub mod in_game;
pub mod main_menu;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_plugin(
                state_plugin::on_variant::<main_menu::MainMenu, _>(AppState::MainMenu)
                    .with_enter(main_menu::enter)
                    .with_update(main_menu::update)
                    .with_update(main_menu::ui_example_system)
                    .with_exit(main_menu::exit),
            )
            .add_plugin(
                state_plugin::on_variant::<in_game::InGame, _>(AppState::InGame)
                    .with_startup(in_game::startup)
                    .with_enter(in_game::enter)
                    .with_update(in_game::update)
                    .with_update(in_game::ui_example_system)
                    .with_exit(in_game::exit),
            );
    }
}

#[derive(States, PartialEq, Eq, Debug, Default, Hash, Clone)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
}
