// src\shared\components\game.rs

use bevy::prelude::*;
use uuid::Uuid;

#[derive(Resource, Default)]

pub struct GameStatus {
    pub phase: GamePhase,
    pub outcome: GameOutcome,
    pub my_id: Uuid,
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

// useful marker for pieces that should be despawned when clearing out the game.
#[derive(Component, Default)]
pub struct GamePiece;