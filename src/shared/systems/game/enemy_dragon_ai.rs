use bevy::prelude::*;
use crate::shared::components::*;
use rand::Rng;
use pathfinding::prelude::astar;
use super::Grid;

// fn manhattan_distance(a: &(i32, i32), b: &(i32, i32)) -> usize {
//     let dx = (a.0 as isize - b.0 as isize).abs() as usize;
//     let dy = (a.1 as isize - b.1 as isize).abs() as usize;
//     dx + dy
// }

pub fn scaled_chebyshev_distance(a: &(i32, i32), b: &(i32, i32)) -> usize {
    let dx = (a.0 - b.0).abs() as usize;
    let dy = (a.1 - b.1).abs() as usize;
    dx.max(dy) * 1000
}


pub fn enemy_dragon_ai_movement_system(
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
            // Calculate direction towards the player dragon
            let direction = my_dragon_position - enemy_dragon_position;
            
            enemy_dragon.input.fire_direction = direction.normalize_or_zero();

 
            if let Some(path) = &enemy_dragon.action.path {
                if path.len() > 0 {
                    // let path_0_pos = grid.grid_to_world(path[0]);
                    // let delta_from_path = path_0_pos - enemy_dragon_position;
                    // let max_distance = 500.0;
                    // // let min_threshold = 100.0;
                    // // let max_threshold = 5000.0;
                    // //let wall_in_neighbor = grid.neighbors(&path[0]).iter().any(|&((x, y), _cost)| !grid.is_walkable((x, y)));

                    // if delta_from_path.length() > min_distance {
                    //     let direction = delta_from_path.normalize_or_zero();
                    //     println!("ai move dragon @: {:?} , start {:?}", enemy_dragon_position, path_0_pos);
                    //     println!("ai move direction: {:?} , delta {:?} : {:?}", direction, delta_from_path.length(), delta_from_path);
                    //     enemy_dragon.input.move_direction = direction;
                    //     enemy_dragon.input.brake = true;
                    // } else 
                    
                    if path.len() > 1 {
                        let grid_direction = (path[1].0 as f32 - path[0].0 as f32, path[1].1 as f32 - path[0].1 as f32);
                        let direction = Vec3::new(grid_direction.0, grid_direction.1, 0.0);
                        enemy_dragon.input.move_direction = direction.normalize_or_zero();
                    }
                } else {
                    enemy_dragon.input.move_direction = direction.normalize_or_zero();
                }
            } else if !grid.is_inside_grid(enemy_dragon_position) {
                enemy_dragon.input.move_direction = direction.normalize_or_zero();
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


pub fn enemy_dragon_ai_pathfinding_system(
    time: Res<Time>,
    grid: Res<Grid>,
    mut enemy_dragon_query: Query<(&mut Dragon, &Transform), Without<MyDragon>>,
    my_dragon_query: Query<&Transform, With<MyDragon>>,
) {

    if let Ok(my_dragon_transform) = my_dragon_query.get_single() {

        for (mut enemy_dragon, enemy_dragon_transform) in enemy_dragon_query.iter_mut() {

            if let Some(pathfinding_timer) = &mut enemy_dragon.action.pathfinding_timer {
                pathfinding_timer.tick(time.delta());
                if !pathfinding_timer.finished() {
                    return;
                }
            }

            let my_dragon_position = my_dragon_transform.translation;
            let enemy_dragon_position = enemy_dragon_transform.translation;

            // Convert positions to grid coordinates
            let start = grid.world_to_grid(enemy_dragon_position);
            let goal = grid.world_to_grid(my_dragon_position);

            let is_goal_walkable = grid.is_walkable(goal);
            if !is_goal_walkable {
                println!("Not walkable: {:?} [{:?}]",goal ,my_dragon_position );
            }
            // Check if start or goal is outside the grid
            let is_start_outside_grid = !grid.is_inside_grid(enemy_dragon_position);
            
            if is_goal_walkable && !is_start_outside_grid {
                // Use pathfinding to navigate the grid
                if let Some((path, _cost)) = astar(
                    &start,
                    |pos| grid.neighbors(pos),
                    |pos| scaled_chebyshev_distance(pos, &goal),
                    |pos| *pos == goal,
                ) {
                    println!("PATH: {:?}",path);
                    enemy_dragon.action.path = Some(path);
                }
            } else {
                println!("PATH: Direct");
                enemy_dragon.action.path = None;
            }
        }
    }
}

