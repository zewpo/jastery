use bevy::prelude::*;
use crate::shared::{components::{dragon::*, game::*}, systems::{projectile::*, dragon::*}};

use super::icedragon_ai::*;

pub struct GamePlayPlugin;

// impl Plugin for GamePlayPlugin {
//     fn build(&self, app: &mut AppBuilder) {
//         app.add_system_set(
//             SystemSet::on_update(GameState::GamePlay)
//                 .with_system(game_play_system.system())
//                 .with_system(game_over_system.system())
//                 // Add any other systems you want to include in this SystemSet
//         );
//     }
// }

impl Plugin for GamePlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            dragon_movement_system.run_if(in_state(GameState::Running)),
            projectile_spawn_system.run_if(in_state(GameState::Running)), 
            projectile_movement_system.run_if(in_state(GameState::Running)),
            projectile_collision_system.run_if(in_state(GameState::Running)),
            ice_dragon_ai_system.run_if(in_state(GameState::Running)),
            game_over_system.in_set(OnUpdate(GameState::Running))
        ));
    }
}

// pub fn game_play_system_set() -> SystemSet {

//     SystemSet::on_update(GameState::GamePlay)
//         .with_system(game_over_system.system())
//         // Add any other systems you want to include in this SystemSet

//     // for (dragon, my_dragon) in dragon_query.iter() {
//     //     if dragon.health <=0 {
//     //         game_state.set(GameState::GameOver);
//     //         println!("Game Over.");
//     //         if let Some(_) = my_dragon {
//     //             game_outcome.set(GameOutcome::Lose);
//     //             println!("You Lose!");
//     //         } else {
//     //             game_outcome.set(GameOutcome::Win);
//     //             println!("You Win!");
//     //         }
//     //     }
//     // }
// }


fn game_over_system(
    dragon_query: Query<(&Dragon, Option<&MyDragon>)>,
    mut game_state: ResMut<NextState<GameState>>,
    mut game_outcome:  ResMut<NextState<GameOutcome>>,
) {

    for (dragon, my_dragon) in dragon_query.iter() {
        if dragon.health <=0 {
            game_state.set(GameState::GameOver);
            println!("Game Over.");
            if let Some(_) = my_dragon {
                game_outcome.set(GameOutcome::Lose);
                println!("You Lose!");
            } else {
                game_outcome.set(GameOutcome::Win);
                println!("You Win!");
            }
        }
    }
}
