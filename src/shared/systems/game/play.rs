//  src\shared\systems\game\play.rs

use bevy::prelude::*;
use crate::shared::{components::{dragon::*, game::*}, systems::{projectile::*, dragon::*}};

use super::icedragon_ai::*;

pub struct GamePlayPlugin;

impl Plugin for GamePlayPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems((
            dragon_movement_system,
            projectile_spawn_system,
            projectile_movement_system,
            projectile_collision_system,
            ice_dragon_ai_system,
            game_over_trigger
        ).in_set(OnUpdate(GamePhase::Playing)));
    }
}

fn game_over_trigger(
    dragon_query: Query<(&Dragon, Option<&MyDragon>)>,
    // mut next_screen: ResMut<NextState<AppScreen>>,
    mut game_phase:  ResMut<NextState<GamePhase>>,
    // mut game_outcome:  ResMut<NextState<GameOutcome>>,
    mut game_status: ResMut<GameStatus>,
) {
    // if game_status.phase != GamePhase::GameOver{
        for (dragon, my_dragon) in dragon_query.iter() {
            if dragon.health <=0 {
                println!("Game Over.");
                if let Some(_) = my_dragon {
                    // game_outcome.set(GameOutcome::Lose);
                    game_status.outcome = GameOutcome::Lose;
                    println!("You Lose!");
                } else {
                    // game_outcome.set(GameOutcome::Win);
                    game_status.outcome = GameOutcome::Win;
                    println!("You Win!");
                }
                // game_status.phase = GamePhase::GameOver;
                game_phase.set(GamePhase::GameOver);
                break;
            }
        }
    // }

    // // todo separate the screen management from the game management
    // if game_status.phase == GamePhase::GameOver{
    //     next_screen.set(AppScreen::GameOver);
    // }
            
}
