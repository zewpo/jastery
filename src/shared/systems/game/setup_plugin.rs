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
        .add_system(start_game.in_set(OnUpdate(GamePhase::Setup)));
    }
}

fn start_game(
    mut game_phase: ResMut<NextState<GamePhase>>,
    mut game_status: ResMut<GameStatus>,
    my_dragon: Query<&Dragon, With<MyDragon>>
){
    let my_dragon = my_dragon.single();
    game_status.my_id = my_dragon.id;
    game_status.outcome = GameOutcome::Undecided;
    game_phase.set(GamePhase::Playing);
}
