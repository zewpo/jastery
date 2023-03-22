// src\shared\components\game.rs

use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    Setup,
    Running,
    Paused,
    GameOver,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameOutcome {
    #[default]
    TBD,
    Win,
    Lose,
} 