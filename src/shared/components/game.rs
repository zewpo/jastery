// src\shared\components\game.rs

use bevy::prelude::*;

#[derive(Resource, Default)]

pub struct GameStatus {
    pub phase: GamePhase,
    pub outcome: GameOutcome,
}


#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GamePhase {
    #[default]
    ToBeDefined,
    Setup,
    Playing,
    Paused,
    GameOver,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum GameOutcome {
    #[default]
    Undecided,
    Win,
    Lose,
}

// useful marker for pieces that can be despawned to clear out the game.
#[derive(Component, Default)]
pub struct GamePiece;