//  src\shared\systems\game\game_plugin.rs

use bevy::prelude::*;

use crate::shared::components::game::*;
use crate::shared::systems::game::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_state::<GamePhase>()
        // .add_state::<GameOutcome>()
        .insert_resource(GameStatus::default())
        .add_plugin(GameSetupPlugin)
        .add_plugin(GamePlayPlugin);
    }
}