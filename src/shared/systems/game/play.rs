//  src\shared\systems\game\play.rs

use bevy::prelude::*;
use crate::shared::{components::{dragon::*, game::*}, systems::{projectile::*, dragon::*}};

use super::icedragon_ai::*;

pub struct GamePlayPlugin;


impl Plugin for GamePlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            dragon_movement_system,
            projectile_spawn_system,
            projectile_movement_system,
            projectile_collision_system,
            ice_dragon_ai_system,
            game_over_system
        ).in_set(OnUpdate(AppState::Running)));
    }
}


fn game_over_system(
    dragon_query: Query<(&Dragon, Option<&MyDragon>)>,
    mut next_state: ResMut<NextState<AppState>>,
    mut game_outcome:  ResMut<NextState<GameOutcome>>,
) {

    for (dragon, my_dragon) in dragon_query.iter() {
        if dragon.health <=0 {
            println!("Game Over.");
            if let Some(_) = my_dragon {
                game_outcome.set(GameOutcome::Lose);
                println!("You Lose!");
            } else {
                game_outcome.set(GameOutcome::Win);
                println!("You Win!");
            }
            next_state.set(AppState::GameOver);
        }
    }
}
