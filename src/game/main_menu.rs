use super::*;

pub struct MainMenu;

pub fn enter() {
    info!("enter");
}

pub fn update(mut next: ResMut<NextState<AppState>>) {
    info!("update");
    next.set(AppState::InGame);
}

pub fn exit() {
    info!("exit");
}
