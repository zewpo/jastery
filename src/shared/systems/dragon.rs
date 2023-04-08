// use std::sync::Arc;
// use std::{collections::HashSet, process, fs::File};
// use std::io::Write;
// use chrono::Local;
// use std::time::{SystemTime, UNIX_EPOCH};

use bevy::{prelude::*, sprite::collide_aabb::Collision};
// use image::{DynamicImage, GenericImageView};
// use std::collections::{HashMap, HashSet};
use crate::shared::components::{dragon::*, resource_cache::*, wall::*, CollidableImage};

use super::game::Grid;

// fn pixel_collision(
//     cell1_pos: (f32, f32),
//     pixels1: &HashSet<(i32, i32)>,
//     cell2_pos: (f32, f32),
//     pixels2: &HashSet<(i32, i32)>,
// ) -> bool {
//     let (cell1_x, cell1_y) = cell1_pos;
//     let (cell2_x, cell2_y) = cell2_pos;

//     // let dx = cell2_x - cell1_x;
//     // let dy = cell2_y - cell1_y;

//     let mut pixels1_vec: Vec<(i32, i32)> = pixels1.iter().copied().collect();
//     pixels1_vec.sort_unstable_by_key(|&(x, y)| (x, y));
//     let min1_x = pixels1_vec[0].0;
//     let max1_x = pixels1_vec[pixels1_vec.len() - 1].0;
//     pixels1_vec.sort_unstable_by_key(|&(x, y)| (y, x));
//     let min1_y = pixels1_vec[0].1;
//     let max1_y = pixels1_vec[pixels1_vec.len() - 1].1;

//     let mut pixels2_vec: Vec<(i32, i32)> = pixels2.iter().copied().collect();
//     pixels2_vec.sort_unstable_by_key(|&(x, y)| (x, y));
//     let min2_x = pixels2_vec[0].0;
//     let max2_x = pixels2_vec[pixels2_vec.len() - 1].0;
//     pixels2_vec.sort_unstable_by_key(|&(x, y)| (y, x));
//     let min2_y = pixels2_vec[0].1;
//     let max2_y = pixels2_vec[pixels2_vec.len() - 1].1;


//     let output_path = "pixels-output.txt";
//     let mut file = File::create(output_path).unwrap();

//     writeln!(file, "---pixels-1--BEGIN----------------------------------------------------").unwrap();
//     writeln!(file, ": cell1_pos {:?}", cell1_pos).unwrap();
    
//     for j1 in (min1_y..=max1_y).rev() {
//         for i1 in min1_x..=max1_x {
//             if pixels1.get(&(i1, j1)).is_some() {
//                 write!(file, "({:>3},{:>3})", i1, j1).unwrap();
//             } else {
//                 write!(file, "         ").unwrap();
//             }
//         }
//         writeln!(file, "").unwrap();
//     }
//     writeln!(file, "---pixels-1--END----------------------------------------------------").unwrap();

//     writeln!(file, "---pixels-2--BEGIN----------------------------------------------------").unwrap();
//     writeln!(file, ": cell2_pos {:?}", cell2_pos).unwrap();
//     for j2 in (min2_y..=max2_y).rev() {
//         for i2 in min2_x..=max2_x {
//             if pixels2.get(&(i2, j2)).is_some() {
//                 write!(file, "({:>3},{:>3})", i2, j2).unwrap();
//             } else {
//                 write!(file, "         ").unwrap();
//             }
//         }
//         writeln!(file, "").unwrap();
//     }
//     writeln!(file, "---pixels-2--END----------------------------------------------------").unwrap();

//     let mut collision = false;
//     let mut overlapping_pixels = HashSet::new();

//     for (pi1, pj1) in pixels1 {
//         let global_px1 = (cell1_x + *pi1 as f32) as i32;
//         let global_py1 = (cell1_y + *pj1 as f32) as i32; // Changed from subtraction to addition

//         for (pi2, pj2) in pixels2 {
//             let global_px2 = (cell2_x + *pi2 as f32) as i32;
//             let global_py2 = (cell2_y + *pj2 as f32) as i32; // Changed from subtraction to addition

//             if global_px1 == global_px2 && global_py1 == global_py2 {
//                 //println!("cell_pos1:{:?}, cell_pos2:{:?},pi1: {}, pj1: {}, pi2: {}, pj2: {},  global_px1: {}, global_py1:{}", cell_pos1, cell_pos2, pi1, pj1, pi2, pj2, global_px1, global_py1 );
//                 writeln!(file, "Pixel collision detected:").unwrap();
//                 writeln!(file, "cell1_pos:{:?}, p1:({},{}), global_p1:({},{})", cell1_pos, pi1, pj1, global_px1, global_py1).unwrap();
//                 writeln!(file, "cell2_pos:{:?}, p2:({},{}), global_p2:({},{})", cell2_pos, pi2, pj2, global_px2, global_py2).unwrap();
//                 //return true;
//                 overlapping_pixels.insert((global_px1, global_py1));
//                 collision = true;
//             }
//         }
//     }

//     println!("collision = {:?}" , collision);

//     println!("TEST 1234");
//     if overlapping_pixels.len() > 0 {

//         let min_x = overlapping_pixels.iter().map(|&(x, _)| x).min().unwrap();
//         println!("TEST MIN_X");

//         let max_x = overlapping_pixels.iter().map(|&(x, _)| x).max().unwrap();
//         println!("TEST MAX_X");

//         let min_y = overlapping_pixels.iter().map(|&(_, y)| y).min().unwrap();
//         println!("TEST MIN_Y");
        
//         let max_y = overlapping_pixels.iter().map(|&(_, y)| y).max().unwrap();
//         println!("TEST MAX_Y");

//         println!("TEST 5678");

//         if collision{
//             writeln!(file, "---overlapping-pixels------------------------------------").unwrap();
//             for j in (min_y..=max_y).rev() {
//                 for i in min_x..=max_x {
//                     if overlapping_pixels.contains(&(i, j)) {
//                         write!(file, "({},{})", i, j).unwrap();
//                     } else {
//                         write!(file, " ").unwrap();
//                     }
//                 }
//                 writeln!(file,"").unwrap();
//             }
//             writeln!(file, "----------------------------------------------------").unwrap();
//         }

//         process::exit(1);
//     }
    
    
//     println!("TEST 9999");

//         // if collision{
//                 // Exit with code 1
//             // process::exit(1);
//         // }
//     // false
//     collision
// }




fn detect_collision_of_opaque_image_cells(
    transform1: &Transform,
    image1: &CollidableImage,
    transform2: &Transform,
    image2: &CollidableImage,
    // cell_size: i32,
) -> bool {

    let pos1 = transform1.translation;
    let pos2 = transform2.translation;

    let dx = (pos1.x - pos2.x).floor();
    let dy = (pos1.y - pos2.y).floor();
    
    let flipped1 = transform1.scale.x < 0.0;
    let flipped2 = transform2.scale.x < 0.0;

    for (cell1_key, _pixels1) in image1.opaque_pixel_cells.iter() {
        let (cell1_i, cell1_j) = cell1_key;

        // distance relative to center of parent image 1.
        let cell1_x = if flipped1 {
            -1.0 * ((cell1_i + 1) * CELL_SIZE) as f32
        } else {
            (cell1_i * CELL_SIZE) as f32
        };
        let cell1_y = (cell1_j * CELL_SIZE) as f32;

        // distance relative to center of parent image 2.
        let cell2_x = if flipped2 {
           -1.0 * (dx + cell1_x) - CELL_SIZE as f32
        } else {
            dx + cell1_x
        };
        let cell2_y = dy + cell1_y;

        let cell2_i = ((cell2_x) / (CELL_SIZE as f32)) as i32;
        let cell2_j = ((cell2_y) / (CELL_SIZE as f32)) as i32;
        
        let cell2_key = (cell2_i, cell2_j);
        if let Some(_pixels2) = image2.opaque_pixel_cells.get(&cell2_key) {
            // let global_cell1_x = pos1.x + cell1_x;
            // let global_cell1_y = pos1.y + cell1_y;
            
            // let global_cell2_x = pos2.x + cell2_x;
            // let global_cell2_y = pos2.y + cell2_y;

            // if pixel_collision(
            //     (global_cell1_x, global_cell1_y),
            //     pixels1,
            //     (global_cell2_x, global_cell2_y),
            //     pixels2,
            // ) {
                return true;
            // }
        }
    }
    false
}

/// This is based on bevy's collide, changed so that it also returns the depth of collision.
/// The return value is a tuple, of (collision side, and depth of collision).
/// With collision as the side of `B` that `A` has collided with.  
/// Where `Left` means that `A` collided with `B`'s left side.  
/// `Top` means that `A` collided with `B`'s top side, etc.
/// If the collision occurs on multiple sides, the side with the deepest penetration is returned.
/// If all sides are involved, `Inside` is returned.
/// The depth of collision is also returned as a Vec2, useful for making adjustments.
pub fn detect_collision_depth(a_pos: Vec3, a_size: Vec2, b_pos: Vec3, b_size: Vec2) -> (Option<Collision>, Vec2)  {
    
    // let now = SystemTime::now();
    // let duration = now.duration_since(UNIX_EPOCH)
    //     .expect("Time went backwards");
    // let subsec_micro = duration.subsec_nanos()/1000;
    //println!("{}", subsec_nanos);

    // info!("At {} running detect_collision_depth:  ", subsec_micro);

    // info!("A ... ");
    let a_min = a_pos.truncate() - a_size / 2.0;
    let a_max = a_pos.truncate() + a_size / 2.0;

    let b_min = b_pos.truncate() - b_size / 2.0;
    let b_max = b_pos.truncate() + b_size / 2.0;
    // info!("B ... ");
    // check to see if the two rectangles are intersecting
    if a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y {
        // println!(" detect_collision_depth ...");
        // info!("C ... ");
        // println!("Sprite - Collision checker...");
        // check to see if we hit on the left or right side
        let (x_collision, x_depth) = if a_min.x < b_min.x && a_max.x > b_min.x && a_max.x < b_max.x
        {
            (Collision::Left, b_min.x - a_max.x)
        } else if a_min.x > b_min.x && a_min.x < b_max.x && a_max.x > b_max.x {
            (Collision::Right, b_max.x - a_min.x)
        } else {
            (Collision::Inside, -f32::INFINITY)
        };

        // check to see if we hit on the top or bottom side
        let (y_collision, y_depth) = if a_min.y < b_min.y && a_max.y > b_min.y && a_max.y < b_max.y
        {
            (Collision::Bottom, b_min.y - a_max.y)
        } else if a_min.y > b_min.y && a_min.y < b_max.y && a_max.y > b_max.y {
            (Collision::Top, b_max.y - a_min.y)
        } else {
            (Collision::Inside, -f32::INFINITY)
        };
        
        // if we had an "x" and a "y" collision, pick the "primary" side using penetration depth
        let collision =
        if y_depth.abs() < x_depth.abs() {
            // info!("D1 ... ");
            Some(y_collision)
        } else {
            // info!("D2 ... ");
            Some(x_collision)
        };
        // info!("E!\n");
        return (collision, Vec2::new(x_depth, y_depth));
    }
    // info!("FFFF!\n");
    (None, Vec2::ZERO)
    
    
}


pub fn dragon_dragon_collision_system(
    mut dragon_query: Query<(&mut Dragon, &mut Transform)>,
) {
    // Check for collisions between dragons.
    // iter_combinations_mut checks for all unique pairs.
    let mut combinations = dragon_query.iter_combinations_mut();
    while let Some([(mut dragon_a, mut dragon_transform_a), 
                    (mut dragon_b, mut dragon_transform_b)]) 
            = combinations.fetch_next() {
                
        let dragon_image_a = dragon_a.image.clone();
        let dragon_image_b = dragon_b.image.clone();
            
        // check for sprite boundary collision.
        if let (Some(collision), mut depth) = detect_collision_depth(
            dragon_transform_a.translation,
            dragon_image_a.size_vec2(),
            dragon_transform_b.translation,
            dragon_image_b.size_vec2()
        ) {
            // Check for cell collision
            if detect_collision_of_opaque_image_cells(
                &dragon_transform_a,
                &dragon_image_a,
                &dragon_transform_b,
                &dragon_image_b
            ) {

                dragon_a.action.velocity = Vec3::ZERO;
                dragon_b.action.velocity = Vec3::ZERO;
                let mut total_adjustment = Vec3::ZERO;
                match collision {
                    Collision::Left | Collision::Right => {
                        total_adjustment.x += depth.x;
                    }
                    // Collision::Right  => {
                    //     total_adjustment.x += depth.x;
                    // }
                    Collision::Bottom | Collision::Top => {
                        total_adjustment.y += depth.y;
                    }
                    // Collision::Top => {
                    //     total_adjustment.y += depth.y;
                    // }
                    Collision::Inside => {
                        // println!("Dragon inside Dragon collision!?");
                        if depth.length() < 1.0 as f32 {
                            depth = depth.normalize_or_zero();
                            if depth == Vec2::ZERO {
                                depth.x = 2.0*CELL_SIZE as f32;
                            }
                        }
                        total_adjustment += depth.extend(0.0);
                    }
                    // _ => {
                    //     total_adjustment += depth.extend(0.0);
                    // }
                }

                if total_adjustment.length() > CELL_SIZE as f32 {
                    total_adjustment = total_adjustment.normalize_or_zero();// * (CELL_SIZE as f32)/3.0;
                }
                // total_adjustment *= 0.2;
                // Apply the total adjustment
                dragon_transform_a.translation += total_adjustment;
                dragon_transform_b.translation -= total_adjustment;

            }
        }
    }
}


pub fn dragon_wall_collision_system(
    grid: Res<Grid>,
    mut dragon_query: Query<(&mut Dragon, &mut Transform)>,
    wall_query: Query<(&Wall, &Transform), Without<Dragon>>,
) {

    for (mut dragon, mut dragon_transform) in dragon_query.iter_mut() {

        // Check for Dragon - Wall collisions
        //if let Some(dragon_image) = resource_cache.dragon_images.get(&dragon.elemental_theme) {
        let dragon_image = dragon.image.clone();

        let mut total_adjustment = Vec3::ZERO;

        // Check for collision with Walls
        for (wall, wall_transform) in wall_query.iter() {

            let wall_image = wall.image.clone();
            // If the collision occurs on multiple sides, the side with the deepest penetration is returned.
            // If all sides are involved, `Inside` is returned.
            if let (Some(collision), mut depth) = detect_collision_depth(
                dragon_transform.translation,
                dragon_image.size_vec2(),
                wall_transform.translation,
                wall_image.size_vec2()
            ) {

                dragon.action.velocity = Vec3::ZERO;

                let mut bumped = false;
                if dragon.my_dragon.is_none(){
                    if let Some(path) = &dragon.action.path {
                        if path.len() > 0 {
                            let cell_pos = grid.grid_to_world(path[0]);
                            total_adjustment += cell_pos - dragon_transform.translation;
                            bumped = true
                        }
                    }
                } 
                if !bumped {

                    // println!("Dragon wall collision... ");
                    // Check for opaque cell collision
                    if detect_collision_of_opaque_image_cells(
                        &dragon_transform,
                        &dragon_image,
                        &wall_transform,
                        &wall_image
                    ) {                        
                        match collision {
                            Collision::Left => {
                                total_adjustment.x += depth.x;
                                // println!("Dragon Left of wall collision!?");
                            }
                            Collision::Right  => {
                                total_adjustment.x += depth.x;
                                // println!("Dragon Right of wall collision!?");
                            }
                            Collision::Bottom => {
                                total_adjustment.y += depth.y;
                                // println!("Dragon Bottom of wall collision!?");
                            }
                            Collision::Top => {
                                total_adjustment.y += depth.y;
                                // println!("Dragon Top of wall collision!?");
                            }
                            Collision::Inside => {
                                // println!("Dragon inside of wall collision!?");
                                if depth.length() < 1.0 as f32 {
                                    depth = depth.normalize_or_zero();
                                    if depth == Vec2::ZERO {
                                        depth.x = 2.0*CELL_SIZE as f32;
                                    }
                                }
                                total_adjustment += depth.extend(0.0);
                            }
                            // _ => {
                            //     total_adjustment += depth.extend(0.0);
                            // }
                        }
                    }
                }
            }
        } // next wall

        if total_adjustment.length() > CELL_SIZE as f32 {
            total_adjustment = total_adjustment.normalize_or_zero() * (CELL_SIZE as f32);
        }
        // Apply the total adjustment
        dragon_transform.translation += total_adjustment;
    } // next dragon
}



pub fn dragon_movement_system(
    time: Res<Time>,
    mut dragon_query: Query<(&mut Dragon, &mut Transform)>,
) {

    for (mut dragon, mut dragon_transform) in dragon_query.iter_mut() {
        
        if dragon.health <= 0 {
            dragon.action.velocity = Vec3::ZERO;
            continue;
        }

        // let previous_velocity = dragon.action.velocity;
        
        // Change in motion
        if dragon.action.motion_timer.tick(time.delta()).just_finished() {
            let acceleration_rate = 2.5;
            let decceleration_rate = 0.7;

            if dragon.input.move_direction != Vec3::ZERO {
                let target_velocity = dragon.input.move_direction * dragon.max_velocity;    

                // Gradually change the velocity towards the target
                dragon.action.velocity = dragon.action.velocity.lerp(target_velocity, acceleration_rate * time.delta().as_secs_f32() );

                if dragon.action.velocity.x.signum() != dragon.input.move_direction.x.signum() {
                    dragon.action.velocity.x *= decceleration_rate;
                }
                if dragon.action.velocity.y.signum() != dragon.input.move_direction.y.signum() {
                    dragon.action.velocity.y *= decceleration_rate;
                }
            } else {
                dragon.action.velocity *= decceleration_rate;
            }

            // if dragon.input.move_direction.x != 0.0 {
            //     dragon.action.velocity.x = dragon.input.move_direction.x * dragon.max_velocity;
            // } else {
            //     dragon.action.velocity.x /= acceleration_rate;
            // }

            // if dragon.input.move_direction.y != 0.0 {
            //     dragon.action.velocity.y = dragon.input.move_direction.y * dragon.max_velocity;
            // } else {
            //         dragon.action.velocity.y /= acceleration_rate;
            // }

            if dragon.input.brake {
                dragon.action.velocity *= 0.5;
            }
        }

        // Move to home position
        if dragon.input.home {
            dragon.action.velocity = Vec3::ZERO;
            dragon_transform.translation = dragon.action.spawn_home;
        }

        // Move the dragon sprite.
        if dragon.action.velocity != Vec3::ZERO {
            dragon.action.velocity = dragon.action.velocity.clamp_length_max(dragon.max_velocity);
            dragon_transform.translation += dragon.action.velocity;
        }

        // if dragon.action.motion_timer.tick(time.delta()).just_finished() {
        //     dragon.action.acceleration = dragon.action.velocity - previous_velocity;
        // }


        // We flip the dragon image with an animation when it changes directions between left to right
        if dragon.action.flipping {
            // we are still flipping...
            if dragon.action.flip_timer.tick(time.delta()).just_finished() {
                // Finish the flipping animation.
                dragon.action.flipping = false;
                if dragon_transform.scale.x < 0.0{
                    dragon_transform.scale.x = 1.0;
                } else {
                    dragon_transform.scale.x = -1.0;
                }
            } else {
                // Continue the flipping animation.
                let progress = dragon.action.flip_timer.percent();
                dragon_transform.scale.x = dragon_transform.scale.x.signum() * (0.5 - 0.5 * (progress * std::f32::consts::PI).sin());
            }
        }
        else {
            let face_dir_x = dragon_transform.scale.x.signum();
            let vel_dir_x = dragon.action.velocity.x.signum();
            // let shoot_dir_x_zero = dragon.input.shoot_direction.x == 0.0;
            let vel_dir_x_min = dragon.action.velocity.x.abs() > 3.0;

            // if (!shoot_dir_x_zero && dragon.my_dragon.is_some() && dragon.input.shoot_direction.x != face_dir_x)
            //     || ((shoot_dir_x_zero || dragon.my_dragon.is_none()) && vel_dir_x_min && vel_dir_x != face_dir_x)
            // {
            if vel_dir_x_min && vel_dir_x != face_dir_x
            {
                dragon.action.flip_timer.reset();
                dragon.action.flipping = true;
            }
        }
    }
}

// pub fn draw_cell_grids_system(
//     mut commands: Commands,
//     dragon_query: Query<(Entity, &Transform, &Dragon)>,
//     wall_query: Query<(&Transform, &Wall)>,
// ) {
//     //let cell_size = CELL_SIZE as f32; // Assuming CELL_SIZE is a constant defined elsewhere
//     let grid_color = Color::rgba(0.6, 0.6, 0.9, 0.7);

//     for (dragon_entity, _transform, dragon) in dragon_query.iter() {

//         for (cell_key, _) in dragon.image.opaque_pixel_cells.iter() {
//             let (i ,j) = (cell_key.0, cell_key.1);
//             let x = (i * CELL_SIZE) as f32  + (CELL_SIZE/2) as f32;
//             let y = (j * CELL_SIZE) as f32  + (CELL_SIZE/2) as f32;

//             //let position = transform.translation + Vec3::new(x, y, 0.0);
//             let position = Vec3::new(x, y, 0.0);
//             draw_cell_grid(&mut commands, position, grid_color, Some(dragon_entity));
//         }
//     }

//     for (transform, wall) in wall_query.iter() {
//         for (cell_key, _) in wall.image.opaque_pixel_cells.iter() {
//             let (i ,j) = (cell_key.0, cell_key.1);
//             let x = (i * CELL_SIZE) as f32  + (CELL_SIZE/2) as f32;
//             let y = (j * CELL_SIZE) as f32  + (CELL_SIZE/2) as f32;

//             let position = transform.translation + Vec3::new(x, y, 0.2);
//             draw_cell_grid(&mut commands, position, grid_color, None);
//         }
//     }
// }

// fn draw_cell_grid(
//     commands: &mut Commands,
//     position: Vec3,
//     grid_color: Color,
//     parent: Option<Entity>,
// ) {
//     let mut cell_grid_entity = commands.spawn(SpriteBundle {
//         sprite: Sprite {
//             color: grid_color,
//             custom_size: Some(Vec2::new(CELL_SIZE as f32, CELL_SIZE as f32)),
//             ..default()
//         },
//         transform: Transform::from_translation(position),
//         ..default()
//     });

//     if let Some(parent_entity) = parent {
//         cell_grid_entity.set_parent(parent_entity);
//     }
// }
