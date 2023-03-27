// src\shared\systems\game\setup_plugin.rs
use bevy::prelude::*;

use crate::shared::components::*;
use crate::shared::systems::game::*;

pub struct GameSetupPlugin;

impl Plugin for GameSetupPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems((
            setup_maze,
            setup_dragons,
        ).in_schedule(OnEnter(GamePhase::Setup)))
        .add_system(setup_completion.in_set(OnUpdate(GamePhase::Setup)));
    }
}

fn setup_completion(mut game_phase: ResMut<NextState<GamePhase>>){
    game_phase.set(GamePhase::Playing);
}
