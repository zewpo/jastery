//  src\shared\systems\game\play.rs

use bevy::prelude::*;
use crate::{shared::{components::{dragon::*, game::*}, systems::{projectile::*, dragon::*}} };

use super::npc_dragon::*;

pub struct GamePlayPlugin;

impl Plugin for GamePlayPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems((
            dragon_movement_system,
            dragon_dragon_collision_system,
            dragon_wall_collision_system,
            projectile_spawn_system,
            projectile_movement_system,
            projectile_dragon_collision_system,
            projectile_projectile_collision_system,
            npc_dragon_pathfinding_system,
            npc_dragon_movement_system,
            game_over_trigger,
        ).in_set(OnUpdate(GamePhase::Playing)))
        // .add_system(draw_cell_grids_system.in_schedule(OnEnter(GamePhase::Playing)))

        ;
    }
}

fn game_over_trigger(
    dragon_query: Query<(&Dragon, Option<&MyDragon>)>,
    // mut next_screen: ResMut<NextState<AppScreen>>,
    game_phase:  Res<State<GamePhase>>,
    mut next_game_phase:  ResMut<NextState<GamePhase>>,
    // mut game_outcome:  ResMut<NextState<GameOutcome>>,
    mut game_status: ResMut<GameStatus>,
) {

    if game_phase.0 == GamePhase::Paused {
        println!("game_over_trigger. Skipped because game_phase: {:?}", game_phase);
        return;
    }

    let n_dragons_found = dragon_query.iter().collect::<Vec<_>>().len();
    if n_dragons_found < 1 {
        println!("game_over_trigger. Found NO Dragons, game_phase: {:?}", game_phase);
        return;
    }

    let mut my_health = 0;
    let mut npc_dragon_health = 0;
    // if game_status.phase != GamePhase::GameOver{
    for (dragon, my_dragon) in dragon_query.iter() {
        if let Some(_) = my_dragon {
            my_health = dragon.health;
        }
        else {
            npc_dragon_health += dragon.health;
        }
    }

    if my_health <= 0 {
        game_status.outcome = GameOutcome::Lose;
        println!("You Died");
    } else if npc_dragon_health <= 0 {
        game_status.outcome = GameOutcome::Win;
        println!("All Enemies Died!");
    }

    if game_status.outcome != GameOutcome::Undecided {
        next_game_phase.set(GamePhase::GameOver);
        println!("\n*** Game Over! ***\n");
    }
            
}
