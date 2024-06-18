use bevy::ecs::{component::Component, schedule::States, system::Resource};

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Menu,
    #[default]
    InGame,
    LoadingScreen,
    BoardSetup,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameModeState {
    #[default]
    NotInGame,
    Singleplayer,
    Multiplayer,
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct GameVolue(pub u32);
