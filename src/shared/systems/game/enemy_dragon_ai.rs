use bevy::prelude::*;
use crate::shared::components::*;
use rand::Rng;
use pathfinding::prelude::astar;
// use std::cmp::{min, max};
use super::Grid;

// pub fn enemy_dragon_ai_system(
//     // time: Res<Time>,
//     grid: Res<Grid>,
//     mut enemy_dragon_query: Query<(&mut Dragon, &Transform), Without<MyDragon>>,
//     my_dragon_query: Query<&Transform, With<MyDragon>>,
// ) {
//     if let Ok(my_dragon_transform) = my_dragon_query.get_single() {
        
//         // Create a random number generator
//         let mut rng = rand::thread_rng();

//         for (mut enemy_dragon, enemy_dragon_transform) in enemy_dragon_query.iter_mut() {
//             let my_dragon_position = my_dragon_transform.translation;

            // // Calculate direction towards the player dragon
            // let direction = my_dragon_position - enemy_dragon_transform.translation;
            // enemy_dragon.input.move_direction = direction.normalize_or_zero();

//             // Randomly decide when to shoot, using a random probability
//             let shoot_probability = rng.gen_range(0.0..1.0);
//             if shoot_probability < 0.5 {
//                 enemy_dragon.input.fire = true;
//             } else {
//                 enemy_dragon.input.fire = false;
//             }

//         }
//     }
// }

// fn absdiff(a: usize, b: usize) -> usize {
//     max(a, b) - min(a, b)
// }

// fn manhattan_distance(a: &(usize, usize), b: &(usize, usize)) -> u32 {
//     (absdiff(a.0, b.0) + absdiff(a.1, b.1)) as u32
// }

fn manhattan_distance(a: &(i32, i32), b: &(i32, i32)) -> usize {
    let dx = (a.0 as isize - b.0 as isize).abs() as usize;
    let dy = (a.1 as isize - b.1 as isize).abs() as usize;
    dx + dy
}


pub fn enemy_dragon_ai_system(
    grid: Res<Grid>,
    mut enemy_dragon_query: Query<(&mut Dragon, &Transform), Without<MyDragon>>,
    my_dragon_query: Query<&Transform, With<MyDragon>>,
) {

    if let Ok(my_dragon_transform) = my_dragon_query.get_single() {
        // Create a random number generator
        let mut rng = rand::thread_rng();

        for (mut enemy_dragon, enemy_dragon_transform) in enemy_dragon_query.iter_mut() {
            let my_dragon_position = my_dragon_transform.translation;
            let enemy_dragon_position = enemy_dragon_transform.translation;

            // Convert positions to grid coordinates
            let start = grid.world_to_grid(enemy_dragon_position);
            let goal = grid.world_to_grid(my_dragon_position);
            // Check if start or goal is outside the grid
            let is_start_outside_grid = !grid.is_inside_grid(enemy_dragon_transform.translation);
            //let is_goal_outside_grid = !grid.is_inside_grid(my_dragon_transform.translation);
            // println!("Start: {:?}", start); // Debugging line
            // println!("Goal: {:?}", goal); // Debugging line
            
            if is_start_outside_grid {
                // Calculate direction towards the player dragon
                let direction = my_dragon_position - enemy_dragon_transform.translation;
                enemy_dragon.input.move_direction = direction.normalize_or_zero();
                println!("Outside Grid - Direction: {:?}\n", direction.normalize_or_zero()); // Debugging line
            } else {
                // Use pathfinding to navigate the grid
                let result = astar(
                    &start,
                    |pos| grid.neighbors(pos),
                    |pos| manhattan_distance(pos, &goal),
                    |pos| *pos == goal,
                );

                // if let Some((path, _cost)) = result {
                //     // println!("Path: {:?}", path); // Debugging line
                //     println!("Path: {:?}, Cost: {:?}", path, _cost);
                //     if path.len() > 1 {
                //         let next_step_world = grid.grid_to_world(path[1]);
                //         let direction = next_step_world - enemy_dragon_position;
                //         enemy_dragon.input.move_direction = direction.normalize_or_zero();
                //         println!("Direction: {:?}", direction); // Debugging line
                //     }
                // }
                if let Some((path, _cost)) = result {
                    //println!("Start: {:?}:{:?}, Goal: {:?}:{:?}, Path: {:?}, Cost: {:?}", start, enemy_dragon_position, goal, my_dragon_position, path, _cost);
                    if path.len() > 1 {
                        let grid_direction = (path[1].0 as f32 - path[0].0 as f32, path[1].1 as f32 - path[0].1 as f32);
                        let direction = Vec3::new(grid_direction.0, grid_direction.1, 0.0);
                        enemy_dragon.input.move_direction = direction.normalize_or_zero();
                        // let next_step_world = grid.grid_to_world(path[1]);
                        // let direction = next_step_world - enemy_dragon_position;
                        // enemy_dragon.input.move_direction = direction.normalize_or_zero();
                        //println!("Direction: {:?}\n", direction.normalize_or_zero()); // Debugging line
                    }
                }
            }
            // Randomly decide when to shoot, using a random probability
            let shoot_probability = rng.gen_range(0.0..1.0);
            if shoot_probability < enemy_dragon.shooting_frequence {
                enemy_dragon.input.fire = true;
            } else {
                enemy_dragon.input.fire = false;
            }
        }
    }
}


// pub fn enemy_dragon_ai_system(
//     grid: Res<Grid>,
//     mut enemy_dragon_query: Query<(&mut Dragon, &Transform), Without<MyDragon>>,
//     my_dragon_query: Query<&Transform, With<MyDragon>>,
// ) {
//     if let Ok(my_dragon_transform) = my_dragon_query.get_single() {
//         // Create a random number generator
//         let mut rng = rand::thread_rng();

//         for (mut enemy_dragon, enemy_dragon_transform) in enemy_dragon_query.iter_mut() {
//             let my_dragon_position = my_dragon_transform.translation;
//             let enemy_dragon_position = enemy_dragon_transform.translation;

//             // Convert positions to grid coordinates
//             let start = grid.world_to_grid(enemy_dragon_position);
//             let goal = grid.world_to_grid(my_dragon_position);

//             let result = astar(
//                 &start,
//                 |pos| grid.neighbors(pos),
//                 |pos| manhattan_distance(pos, &goal),
//                 |pos| *pos == goal,
//             );

//             if let Some((path, _cost)) = result {
//                 if path.len() > 1 {
//                     let next_step_world = grid.grid_to_world(path[1]);
//                     let direction = next_step_world - enemy_dragon_position;
//                     enemy_dragon.input.move_direction = direction.normalize_or_zero();
//                 }
//             }

//             // Randomly decide when to shoot, using a random probability
//             let shoot_probability = rng.gen_range(0.0..1.0);
//             if shoot_probability < 0.5 {
//                 enemy_dragon.input.fire = true;
//             } else {
//                 enemy_dragon.input.fire = false;
//             }
//         }
//     }
// }
