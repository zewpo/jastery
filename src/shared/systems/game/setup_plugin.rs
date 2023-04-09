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
        .add_system( check_setup_complete.in_set(OnUpdate(GamePhase::Setup)))
        .add_system( start_game.in_schedule(OnExit(GamePhase::Setup)) )
        
        ;
    }
}

fn check_setup_complete(
    game_phase: Res<State<GamePhase>>,
    mut next_game_phase: ResMut<NextState<GamePhase>>,
    my_dragon_query: Query<&Dragon, With<MyDragon>>,
    wall_query: Query<&Wall, Without<Dragon>>,
){
    // check if all setup systems have completed.
    // At the moment, all setup functions are scheduled when the OnEnter(GamePhase::Setup) event fires,
    // so that 'should' mean they have all been ran by the time the OnUpdate(GamePhase::Setup) events start firing.
    // Otherwise, if we need to add more systems to run multiple times in a we could add marker components to the setup system, 
    // and check if they have all be created/spawned yet.
    
    let my_dragon_found = my_dragon_query.iter().collect::<Vec<_>>().len() == 1;
    let walls_found = wall_query.iter().collect::<Vec<_>>().len() > 0;
    print!("check_setup_complete -- ");
    if my_dragon_found  && walls_found 
        && game_phase.0 != GamePhase::Playing {
            println!(" All complete.  set GamePhase::Playing.");
        next_game_phase.set(GamePhase::Playing);
    } else {
        println!(" Not complete.  GamePhase: {:?}", game_phase );
    }
}
// called when setup is complete.
fn start_game(
    // game_phase: Res<State<GamePhase>>,
    mut game_status: ResMut<GameStatus>,
    my_dragon_query: Query<&Dragon, With<MyDragon>>
){
    let my_dragon_found = my_dragon_query.iter().collect::<Vec<_>>().len() == 1;
    if my_dragon_found {
        info!("*** start_game ***");
        let my_dragon = my_dragon_query.single();
        game_status.my_id = my_dragon.id;
        game_status.outcome = GameOutcome::Undecided;
    } else {
        info!("******* Attempting to Start Game *******");
        // println!("******* Attempting to Start Game *******");
    }
    
}
