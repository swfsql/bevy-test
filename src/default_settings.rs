use bevy::prelude::*;
use bevy::window::*;

pub struct DefaultSettingsPlugin;

impl Plugin for DefaultSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            bevy::DefaultPlugins //
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        cursor: Cursor::default(),
                        present_mode: PresentMode::default(),
                        mode: WindowMode::default(),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: WindowResolution::new(800.0, 600.0),
                        title: "my app".to_string(),
                        composite_alpha_mode: CompositeAlphaMode::default(),
                        resize_constraints: WindowResizeConstraints::default(),
                        resizable: true,
                        decorations: true,
                        transparent: false,
                        focused: true,
                        window_level: WindowLevel::Normal,
                        canvas: None,
                        fit_canvas_to_parent: false,
                        prevent_default_event_handling: true,
                        internal: Default::default(),
                        ime_enabled: false,
                        ime_position: Default::default(),
                    }),
                    exit_condition: ExitCondition::OnAllClosed,
                    close_when_requested: true,
                })
                // prevents blurry sprites
                .set(bevy::render::texture::ImagePlugin::default_nearest()),
        );
    }
}
