// src\shared\systems\game\setup_plugin.rs
use bevy::prelude::*;

use crate::client::systems::AppScreen;
use crate::shared::components::*;
use crate::shared::systems::game::*;

pub struct GameSetupPlugin;

impl Plugin for GameSetupPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(
            (setup_maze, 
              setup_dragons,)
            .distributive_run_if(|game_status: Res<GameStatus>| game_status.phase == GamePhase::Setup)
            .in_schedule(OnEnter(AppScreen::InPlay)))

        //.add_systems((setup_maze, setup_dragons,).in_schedule(OnExit(AppScreen::GameOver)))
  
        .add_system( 
            check_setup_complete
            .in_set(OnUpdate(AppScreen::InPlay))
            .run_if(|game_status: Res<GameStatus>| game_status.phase == GamePhase::Setup)
        )
        .add_system( 
            start_game
            .in_set(OnUpdate(AppScreen::InPlay))
            .run_if(|game_status: Res<GameStatus>| game_status.phase == GamePhase::Start)
        )
        ;
    }
}

fn check_setup_complete(
    // game_phase: Res<State<GamePhase>>,
    // mut next_game_phase: ResMut<NextState<GamePhase>>,
    mut game_status: ResMut<GameStatus>,
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
    
    if game_status.phase != GamePhase::Start {
        info!("check_setup_complete -- ");
        if my_dragon_found  && walls_found {
                info!(" -- All complete.  set GamePhase::Start.");
                game_status.phase = GamePhase::Start;
                //next_game_phase.set(GamePhase::Playing);
        } else {
            info!(" -- Not complete.  GamePhase: {:?}", game_status.phase );
        }
    } else {
        info!("check_setup_complete: Already Playing");
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
        info!("*** (re)start_game ***");
        let my_dragon = my_dragon_query.single();
        game_status.my_id = my_dragon.id;
        game_status.outcome = GameOutcome::Undecided;
        game_status.phase = GamePhase::Playing;
    } else {
        info!("******* Attempting to Start Game *******");
        // println!("******* Attempting to Start Game *******");
    }
    
}
