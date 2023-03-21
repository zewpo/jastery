use bevy::prelude::*;

// src/shared/game_state.rs
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Setup,
    Running,
}
