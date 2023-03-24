// src\shared\components\game.rs

use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppScreen {
    #[default]
    MainMenu,
    Settings,
    GamePlay,
    Paused,
    GameOver
}

// #[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
// pub enum GameState {
//     #[default]
//     Build,
//     Running,
//     Paused,
//     GameOver,
// }

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameOutcome {
    #[default]
    TBD,
    Win,
    Lose,
}

// useful marker for pieces that can be despawned to clear out the game.
#[derive(Component)]
pub struct GamePiece;