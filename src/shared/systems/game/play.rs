//  src\shared\systems\game\play.rs

use bevy::prelude::*;
use crate::{shared::{components::{dragon::*, game::*}, systems::{projectile::*, dragon::*}}, client::systems::AppScreen };

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
        )
        .distributive_run_if(|game_status: Res<GameStatus>| game_status.phase == GamePhase::Playing)
        .in_set(OnUpdate(AppScreen::InPlay))
        )
        // .add_system(draw_cell_grids_system.in_schedule(OnEnter(GamePhase::Playing)))

        ;
    }
}

fn game_over_trigger(
    mut dragon_query: Query<(&Dragon, &mut Sprite, &mut Transform, Option<&MyDragon>)>,
    // mut next_screen: ResMut<NextState<AppScreen>>,
    // mut game_outcome:  ResMut<NextState<GameOutcome>>,
    mut game_status: ResMut<GameStatus>,
) {

    if game_status.phase == GamePhase::Paused {
        println!("game_over_trigger. Skipped because game_status.phase: {:?}", game_status.phase);
        return;
    }

    let n_dragons_found = dragon_query.iter().collect::<Vec<_>>().len();
    if n_dragons_found < 1 {
        println!("game_over_trigger. Found NO Dragons, game_status.phase: {:?}", game_status.phase);
        return;
    }

    let mut my_health = 0;
    let mut npc_dragon_health = 0;
    // if game_status.phase != GamePhase::GameOver{
    for (dragon, mut dragon_sprite, mut dragon_transform, my_dragon) in dragon_query.iter_mut() {
        if let Some(_) = my_dragon {
            my_health = dragon.health;
        }
        else {
            npc_dragon_health += dragon.health;
        }
        if dragon.health <= 0 {
            //show dragon as dead.
            dragon_sprite.color = Color::rgba(0.4, 0.4, 0.4, 1.0);
            if dragon_transform.scale.x == 0.0 {
                dragon_transform.scale.x = 1.0;
            } else {
                dragon_transform.scale.x = dragon_transform.scale.x.signum()
            }
            
            // dragon_sprite.flip_y = true;
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
        //next_game_phase.set(GamePhase::GameOver);
        game_status.phase = GamePhase::GameOver;
        println!("\n*** Game Over! ***\n");
    }
            
}
