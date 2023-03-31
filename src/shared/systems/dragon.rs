use std::sync::Arc;
use std::{collections::HashSet, process, fs::File};
use std::io::Write;

use bevy::{prelude::*, sprite::collide_aabb::Collision};
// use image::{DynamicImage, GenericImageView};
// use std::collections::{HashMap, HashSet};
use crate::shared::components::{dragon::*, resource_cache::*, wall::*, CollidableImage};

fn pixel_collision(
    cell1_pos: (f32, f32),
    pixels1: &HashSet<(i32, i32)>,
    cell2_pos: (f32, f32),
    pixels2: &HashSet<(i32, i32)>,
) -> bool {
    let (cell1_x, cell1_y) = cell1_pos;
    let (cell2_x, cell2_y) = cell2_pos;

    // let dx = cell2_x - cell1_x;
    // let dy = cell2_y - cell1_y;

    let mut pixels1_vec: Vec<(i32, i32)> = pixels1.iter().copied().collect();
    pixels1_vec.sort_unstable_by_key(|&(x, y)| (x, y));
    let min1_x = pixels1_vec[0].0;
    let max1_x = pixels1_vec[pixels1_vec.len() - 1].0;
    pixels1_vec.sort_unstable_by_key(|&(x, y)| (y, x));
    let min1_y = pixels1_vec[0].1;
    let max1_y = pixels1_vec[pixels1_vec.len() - 1].1;

    let mut pixels2_vec: Vec<(i32, i32)> = pixels2.iter().copied().collect();
    pixels2_vec.sort_unstable_by_key(|&(x, y)| (x, y));
    let min2_x = pixels2_vec[0].0;
    let max2_x = pixels2_vec[pixels2_vec.len() - 1].0;
    pixels2_vec.sort_unstable_by_key(|&(x, y)| (y, x));
    let min2_y = pixels2_vec[0].1;
    let max2_y = pixels2_vec[pixels2_vec.len() - 1].1;


    let output_path = "pixels-output.txt";
    let mut file = File::create(output_path).unwrap();

    writeln!(file, "---pixels-1--BEGIN----------------------------------------------------").unwrap();
    writeln!(file, ": cell1_pos {:?}", cell1_pos).unwrap();
    
    for j1 in (min1_y..=max1_y).rev() {
        for i1 in min1_x..=max1_x {
            if pixels1.get(&(i1, j1)).is_some() {
                write!(file, "({:>3},{:>3})", i1, j1).unwrap();
            } else {
                write!(file, "         ").unwrap();
            }
        }
        writeln!(file, "").unwrap();
    }
    writeln!(file, "---pixels-1--END----------------------------------------------------").unwrap();

    writeln!(file, "---pixels-2--BEGIN----------------------------------------------------").unwrap();
    writeln!(file, ": cell2_pos {:?}", cell2_pos).unwrap();
    for j2 in (min2_y..=max2_y).rev() {
        for i2 in min2_x..=max2_x {
            if pixels2.get(&(i2, j2)).is_some() {
                write!(file, "({:>3},{:>3})", i2, j2).unwrap();
            } else {
                write!(file, "         ").unwrap();
            }
        }
        writeln!(file, "").unwrap();
    }
    writeln!(file, "---pixels-2--END----------------------------------------------------").unwrap();

    let mut collision = false;
    let mut overlapping_pixels = HashSet::new();

    for (pi1, pj1) in pixels1 {
        let global_px1 = (cell1_x + *pi1 as f32) as i32;
        let global_py1 = (cell1_y + *pj1 as f32) as i32; // Changed from subtraction to addition

        for (pi2, pj2) in pixels2 {
            let global_px2 = (cell2_x + *pi2 as f32) as i32;
            let global_py2 = (cell2_y + *pj2 as f32) as i32; // Changed from subtraction to addition

            if global_px1 == global_px2 && global_py1 == global_py2 {
                //println!("cell_pos1:{:?}, cell_pos2:{:?},pi1: {}, pj1: {}, pi2: {}, pj2: {},  global_px1: {}, global_py1:{}", cell_pos1, cell_pos2, pi1, pj1, pi2, pj2, global_px1, global_py1 );
                writeln!(file, "Pixel collision detected:").unwrap();
                writeln!(file, "cell1_pos:{:?}, p1:({},{}), global_p1:({},{})", cell1_pos, pi1, pj1, global_px1, global_py1).unwrap();
                writeln!(file, "cell2_pos:{:?}, p2:({},{}), global_p2:({},{})", cell2_pos, pi2, pj2, global_px2, global_py2).unwrap();
                //return true;
                overlapping_pixels.insert((global_px1, global_py1));
                collision = true;
            }
        }
    }

    println!("collision = {:?}" , collision);

    println!("TEST 1234");
    if overlapping_pixels.len() > 0 {

        let min_x = overlapping_pixels.iter().map(|&(x, _)| x).min().unwrap();
        println!("TEST MIN_X");

        let max_x = overlapping_pixels.iter().map(|&(x, _)| x).max().unwrap();
        println!("TEST MAX_X");

        let min_y = overlapping_pixels.iter().map(|&(_, y)| y).min().unwrap();
        println!("TEST MIN_Y");
        
        let max_y = overlapping_pixels.iter().map(|&(_, y)| y).max().unwrap();
        println!("TEST MAX_Y");

        println!("TEST 5678");

        if collision{
            writeln!(file, "---overlapping-pixels------------------------------------").unwrap();
            for j in (min_y..=max_y).rev() {
                for i in min_x..=max_x {
                    if overlapping_pixels.contains(&(i, j)) {
                        write!(file, "({},{})", i, j).unwrap();
                    } else {
                        write!(file, " ").unwrap();
                    }
                }
                writeln!(file,"").unwrap();
            }
            writeln!(file, "----------------------------------------------------").unwrap();
        }

        process::exit(1);
    }
    
    
    println!("TEST 9999");

        // if collision{
                // Exit with code 1
            // process::exit(1);
        // }
    // false
    collision
}




fn cell_collision(
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

    // let adjustment1_x = (image1.width_i32() - CELL_SIZE) as f32 / 2.0;
    // let adjustment1_y = (image1.height_i32() - CELL_SIZE) as f32 / 2.0;

    // let adjustment2_x = (image2.width_i32() - CELL_SIZE)  as f32 / 2.0;
    // let adjustment2_y = (image2.height_i32() - CELL_SIZE)  as f32 / 2.0;

    for (cell1_key, _pixels1) in image1.opaque_pixel_cells.iter() {
        let (cell1_i, cell1_j) = cell1_key;
        let cell1_x = 
            if transform1.scale.x < 0.0 && transform2.scale.x > 0.0 {
                -1.0 * ((cell1_i+1) * CELL_SIZE) as f32
            } else {
                (cell1_i * CELL_SIZE) as f32
        };
        let cell1_y = (cell1_j * CELL_SIZE) as f32;

        let cell2_x = dx + cell1_x;
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
pub fn collide_detail(a_pos: Vec3, a_size: Vec2, b_pos: Vec3, b_size: Vec2) -> (Option<Collision>, Vec2)  {
    let a_min = a_pos.truncate() - a_size / 2.0;
    let a_max = a_pos.truncate() + a_size / 2.0;

    let b_min = b_pos.truncate() - b_size / 2.0;
    let b_max = b_pos.truncate() + b_size / 2.0;

    // check to see if the two rectangles are intersecting
    if a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y {
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
            Some(y_collision)
        } else {
            Some(x_collision)
        };
        (collision, Vec2::new(x_depth, y_depth))
    } else {
        (None, Vec2::ZERO)
    }
}


pub fn dragon_movement_system(
    time: Res<Time>,
    mut dragon_query: Query<(Entity, &mut Dragon, &mut Transform)>,
    wall_query: Query<(&Wall, &Transform), Without<Dragon>>,
    // resource_cache: Res<ResourceCache>,
) {

    // let mut dragons:  Vec<(Entity, Arc<Dragon>, Transform)> = Vec::new();
    // for (dragon_entity, dragon, dragon_transform) in dragon_query.iter() {
    //     dragons.push((dragon_entity, Arc::new(dragon), *dragon_transform));
    // }


    let mut combinations = dragon_query.iter_combinations_mut();
    while let Some([(dragon_entity_a, mut dragon_a, mut dragon_transform_a), 
                    (dragon_entity_b, mut dragon_b, mut dragon_transform_b)]) 
            = combinations.fetch_next() {
                
        let dragon_image_a = dragon_a.image.clone();
        let dragon_image_b = dragon_b.image.clone();
            
                //         // check for sprite boundary collision.
        if let (Some(collision), mut depth) = collide_detail(
            dragon_transform_a.translation,
            dragon_image_a.size_vec2(),
            dragon_transform_b.translation,
            dragon_image_b.size_vec2()
        ) {
            // Check for cell collision
            if cell_collision(
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
                        println!("Dragon inside wall collision!?");
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

                // Apply the total adjustment
                dragon_transform_a.translation += total_adjustment;
                dragon_transform_b.translation -= total_adjustment;

            }
        }
        
    }


    for (_i,(_dragon_entity, mut dragon, mut dragon_transform)) in dragon_query.iter_mut().enumerate() {
    //for ((dragon_entity, mut dragon, mut dragon_transform), (other_dragon_entity, other_dragon, other_dragon_transform)) in dragon_query.iter_combinations_mut() {
        
        if dragon.health <= 0 {
            dragon.action.velocity = Vec3::ZERO;
            continue;
        }

        let previous_velocity = dragon.action.velocity;
        
        // Change in motion
        if dragon.action.motion_timer.tick(time.delta()).just_finished() {
            let acceleration_rate = 0.45;
            let decceleration_rate = acceleration_rate;

            if dragon.input.move_direction.x != 0.0 {
                dragon.action.velocity.x += dragon.input.move_direction.x * acceleration_rate;
            } else {
                dragon.action.velocity.x *= decceleration_rate;
            }

            if dragon.input.move_direction.y != 0.0 {
                dragon.action.velocity.y += dragon.input.move_direction.y * acceleration_rate;
            } else {
                dragon.action.velocity.y *= decceleration_rate;
            }

            if dragon.input.move_direction.z != 0.0 {
                dragon.action.velocity.z += dragon.input.move_direction.z * acceleration_rate;
            } else {
                dragon.action.velocity.z *= decceleration_rate;
            }

            if dragon.input.brake {
                dragon.action.velocity *= 0.5;
            }

        }


        if dragon.input.home {
            // Move to home position
            dragon.action.velocity = Vec3::ZERO;
            dragon_transform.translation = dragon.action.spawn_home;
        }
        
        // let mut collisions: Vec<(Entity, Vec3, Vec3)> = Vec::new();

        // Check for collisions
        //if let Some(dragon_image) = resource_cache.dragon_images.get(&dragon.elemental_theme) {
        let dragon_image = dragon.image.clone();

        let mut total_adjustment = Vec3::ZERO;

        // // Check for collision with Other dragons
        // for (dragon_entity_other, dragon_other, dragon_transform_other, ) in dragons[i + 1..].iter_mut() {

        //     //let mut dragon_other = dragon_query.get_component_mut::<Dragon>(*dragon_entity_other).unwrap();

        //     if let Some(dragon_image_other) = resource_cache.dragon_images.get(&dragon_other.elemental_theme) {
        //         // check for sprite boundary collision.
        //         if let (Some(collision), mut depth) = collide_detail(
        //             dragon_transform.translation,
        //             dragon_image.size_vec2(),
        //             dragon_transform_other.translation,
        //             dragon_image_other.size_vec2()
        //         ) {
        //             // Check for cell collision
        //             if cell_collision(
        //                 &dragon_transform,
        //                 &dragon_image,
        //                 &dragon_transform_other,
        //                 &dragon_image_other
        //             ) {
        //                 dragon.action.velocity = Vec3::ZERO;
        //                 match collision {
        //                     Collision::Left | Collision::Right => {
        //                         total_adjustment.x += depth.x;
        //                     }
        //                     // Collision::Right  => {
        //                     //     total_adjustment.x += depth.x;
        //                     // }
        //                     Collision::Bottom | Collision::Top => {
        //                         total_adjustment.y += depth.y;
        //                     }
        //                     // Collision::Top => {
        //                     //     total_adjustment.y += depth.y;
        //                     // }
        //                     Collision::Inside => {
        //                         println!("Dragon inside wall collision!?");
        //                         if depth.length() < 1.0 as f32 {
        //                             depth = depth.normalize_or_zero();
        //                             if depth == Vec2::ZERO {
        //                                 depth.x = 2.0*CELL_SIZE as f32;
        //                             }
        //                         }
        //                         total_adjustment += depth.extend(0.0);
        //                     }
        //                     // _ => {
        //                     //     total_adjustment += depth.extend(0.0);
        //                     // }
        //                 }
        //             }
        //         }
        //     }

        //     collisions.push((*dragon_entity_other,total_adjustment,Vec3::ZERO));

        //         // if total_adjustment.length() > CELL_SIZE as f32 {
        //         //     total_adjustment = total_adjustment.normalize_or_zero();// * (CELL_SIZE as f32)/3.0;
        //         // }

        //         // // Apply the total adjustment
        //         // dragon_transform_other.translation += total_adjustment;
        // }

        // Check for collision with Walls
        for (wall, wall_transform) in wall_query.iter() {
            // if let Some(wall_image) = resource_cache.wall_images.get(&wall.shape) {
            let wall_image = wall.image.clone();
            // If the collision occurs on multiple sides, the side with the deepest penetration is returned.
            // If all sides are involved, `Inside` is returned.
            if let (Some(collision), mut depth) = collide_detail(
                dragon_transform.translation,
                dragon_image.size_vec2(),
                wall_transform.translation,
                wall_image.size_vec2()
            ) {
                // Check for cell collision
                // if dragon.my_dragon.is_some() && cell_collision(
                if cell_collision(
                    &dragon_transform,
                    &dragon_image,
                    &wall_transform,
                    &wall_image
                ) {
                    dragon.action.velocity = Vec3::ZERO;
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
                            println!("Dragon inside wall collision!?");
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
        
        //total_adjustment = total_adjustment.clamp_length(1.0, CELL_SIZE);

        if total_adjustment.length() > CELL_SIZE as f32 {
            total_adjustment = total_adjustment.normalize_or_zero();// * (CELL_SIZE as f32)/3.0;
        }

        // Apply the total adjustment
        dragon_transform.translation += total_adjustment;
    

        // Move the dragon sprite.
        if dragon.action.velocity != Vec3::ZERO {
            dragon.action.velocity = dragon.action.velocity.clamp_length_max(dragon.action.max_velocity);
            dragon_transform.translation += dragon.action.velocity;
        }
        if dragon.action.motion_timer.tick(time.delta()).just_finished() {
            dragon.action.acceleration = dragon.action.velocity - previous_velocity;
        }


        // Flip the dragon with an animation when it changes directions between left to right.
        // let facing_dir_x = dragon_transform.scale.x.signum();  // -1.0 or +1.0
        // let velocity_dir_x = dragon.action.velocity.x.signum();   // -1.0 or +1.0
        // let fire_dir_x_or_0 = if dragon.input.fire_direction.x == 0.0 { 0.0 } 
        //                                 else { dragon.input.fire_direction.x.signum() };
        
        // let fire_direction_zero = dragon.input.fire_direction.x == 0.0;
        // let fire_opposite_facing = !fire_direction_zero && dragon.input.fire_direction.x.signum() != facing_dir_x;
        // let velocity_opposite_facing = dragon.action.velocity.x.signum() != facing_dir_x;

        // let facing_dir_x = dragon_transform.scale.x.signum();  // -1.0 or +1.0
        // let velocity_opposite_scale = dragon.action.velocity.x.signum() != facing_dir_x;
        // let fire_direction_opposite_scale = dragon.input.fire_direction.x.signum() != facing_dir_x;

        

        // let new_face_dir = if dragon.input.fire_direction.x != 0.0 {
        //     dragon.input.fire_direction.x.signum()
        // } else if dragon.action.velocity.x != 0.0 {
        //     dragon.action.velocity.x.signum()
        // } else {
        //     0.0
        // };

        

        // let fire_direction_opposite_scale = dragon.input.fire_direction.x.signum() != dragon_transform.scale.x.signum();
        // let velocity_opposite_scale = dragon.action.velocity.x.signum() != dragon_transform.scale.x.signum();


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
        // else if fire_opposite_facing || (fire_direction_zero && velocity_opposite_facing)
        //  else if (((dragon.action.velocity.x > 0.0 && dragon.input.fire_direction.x >= 0.0) || ( dragon.action.velocity.x < 0.0 && dragon.input.fire_direction.x > 0.0)) && dragon_transform.scale.x < 0.0)
        //         ||(((dragon.action.velocity.x < 0.0 && dragon.input.fire_direction.x <= 0.0) || ( dragon.action.velocity.x > 0.0 && dragon.input.fire_direction.x < 0.0)) && dragon_transform.scale.x > 0.0) 
        // else if (dragon.input.fire_direction.x != 0.0 && fire_direction_opposite_scale) 
        //      || (dragon.input.fire_direction.x == 0.0 && velocity_opposite_scale)
        // else if new_face_dir != current_face_dir
        // else if (dragon.input.fire_direction.x != 0.0 && fire_direction_opposite_scale)
        //      || (dragon.input.fire_direction.x == 0.0 && velocity_opposite_scale)
        // else if ((dragon.input.fire_direction.x != 0.0 && dragon.input.fire_direction.x.signum() != dragon_transform.scale.x.signum())
        //     || (dragon.input.fire_direction.x == 0.0 && dragon.action.velocity.x.signum() != dragon_transform.scale.x.signum())) 

        else {
            let face_dir_x = dragon_transform.scale.x.signum();
            let vel_dir_x = dragon.action.velocity.x.signum();
            let fire_dir_x_zero = dragon.input.fire_direction.x == 0.0;
            let vel_dir_x_zero = dragon.action.velocity.x == 0.0;

            if (!fire_dir_x_zero && dragon.input.fire_direction.x != face_dir_x)
                || (fire_dir_x_zero && !vel_dir_x_zero && vel_dir_x != face_dir_x)
            {
                dragon.action.flip_timer.reset();
                dragon.action.flipping = true;
            }
        }
    }
}

pub fn draw_cell_grids_system(
    mut commands: Commands,
    dragon_query: Query<(Entity, &Transform, &Dragon)>,
    wall_query: Query<(&Transform, &Wall)>,
) {
    //let cell_size = CELL_SIZE as f32; // Assuming CELL_SIZE is a constant defined elsewhere
    let grid_color = Color::rgba(0.8, 0.2, 0.2, 0.7);

    for (dragon_entity, _transform, dragon) in dragon_query.iter() {

        for (cell_key, _) in dragon.image.opaque_pixel_cells.iter() {
            let (i ,j) = (cell_key.0, cell_key.1);
            let x = (i * CELL_SIZE) as f32  + (CELL_SIZE/2) as f32;
            let y = (j * CELL_SIZE) as f32  + (CELL_SIZE/2) as f32;

            //let position = transform.translation + Vec3::new(x, y, 0.0);
            let position = Vec3::new(x, y, 0.0);
            draw_cell_grid(&mut commands, position, grid_color, Some(dragon_entity));
        }
    }

    for (transform, wall) in wall_query.iter() {
        for (cell_key, _) in wall.image.opaque_pixel_cells.iter() {
            let (i ,j) = (cell_key.0, cell_key.1);
            let x = (i * CELL_SIZE) as f32  + (CELL_SIZE/2) as f32;
            let y = (j * CELL_SIZE) as f32  + (CELL_SIZE/2) as f32;

            let position = transform.translation + Vec3::new(x, y, 0.2);
            draw_cell_grid(&mut commands, position, grid_color, None);
        }
    }
}

fn draw_cell_grid(
    commands: &mut Commands,
    position: Vec3,
    grid_color: Color,
    parent: Option<Entity>,
) {
    let mut cell_grid_entity = commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: grid_color,
            custom_size: Some(Vec2::new(CELL_SIZE as f32, CELL_SIZE as f32)),
            ..default()
        },
        transform: Transform::from_translation(position),
        ..default()
    });

    if let Some(parent_entity) = parent {
        cell_grid_entity.set_parent(parent_entity);
    }
}
