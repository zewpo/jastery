// src/shared/systems/game/mod.rs

pub mod game_plugin;
pub mod setup_plugin;
pub mod setup_dragons;
pub mod setup_maze;
pub mod npc_dragon;
pub mod play;


// pub mod setup_maze;

pub use game_plugin::*;
pub use setup_plugin::*;
pub use setup_dragons::*;
pub use setup_maze::*;
pub use npc_dragon::*;
pub use play::*;

// pub use setup_maze::*;
 