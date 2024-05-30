use bevy::{app::App, prelude::*};
use logic::GamePlugin;
use states::{AppState, GameModeState};
use ui::GameUI;

mod logic;
mod states;
mod ui;

fn main() {
    App::new()
        .insert_resource(ClearColor::default())
        .insert_resource(ClearColor(Color::hex("353b45").unwrap()))
        .add_plugins(GamePlugin)
        .add_plugins(GameUI)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: [1280., 940.].into(),
                title: "Pro Chess".to_string(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameModeState>()
        .init_state::<AppState>()
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
