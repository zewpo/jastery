use std::{collections::HashSet, process, fs::File};
use std::io::Write;

use bevy::{prelude::*, sprite::collide_aabb::{collide, Collision}};
// use image::{DynamicImage, GenericImageView};
// use std::collections::{HashMap, HashSet};
use crate::shared::components::{dragon::*, resource_cache::*, wall::*, CollidableImage};

// fn pixel_collision(
//     cell_pos1: (f32, f32),
//     pixels1: &HashSet<(i32, i32)>,
//     cell_pos2: (f32, f32),
//     pixels2: &HashSet<(i32, i32)>,
//     // cell_size: i32,
// ) -> bool {
    
    
//     // let mut pixels1_vec: Vec<(i32, i32)> = pixels1.iter().copied().collect();
//     // pixels1_vec.sort_unstable_by_key(|&(x, y)| (x, y));
//     // let min1_x = pixels1_vec[0].0;
//     // let max1_x = pixels1_vec[pixels1_vec.len() - 1].0;
//     // pixels1_vec.sort_unstable_by_key(|&(x, y)| (y, x));
//     // let min1_y = pixels1_vec[0].1;
//     // let max1_y = pixels1_vec[pixels1_vec.len() - 1].1;


//     // let mut pixels2_vec: Vec<(i32, i32)> = pixels2.iter().copied().collect();
//     // pixels2_vec.sort_unstable_by_key(|&(x, y)| (x, y));
//     // let min2_x = pixels2_vec[0].0;
//     // let max2_x = pixels2_vec[pixels2_vec.len() - 1].0;
//     // pixels2_vec.sort_unstable_by_key(|&(x, y)| (y, x));
//     // let min2_y = pixels2_vec[0].1;
//     // let max2_y = pixels2_vec[pixels2_vec.len() - 1].1;

//     // println!("cell_pos1:{:?}, cell_pos2:{:?}, cell_size: {}, pixel1 range ({}..{} , {}..{}),  pixel2 range ({}..{} , {}..{})", cell_pos1, cell_pos2, cell_size, min1_x, max1_x, min1_y, max1_y, min2_x, max2_x, min2_y, max2_y);


//     // let (cell1_i, cell1_j) = cell1;
//     // let (cell2_i, cell2_j) = cell2;
//     let (cell1_x, cell1_y) = cell_pos1;
//     let (cell2_x, cell2_y) = cell_pos2;

//     for (px1, py1) in pixels1 {
//         let global_px1 = (cell1_x + (*px1 as f32)) as i32;
//         let global_py1 = (cell1_y - (*py1 as f32)) as i32;

//         for (px2, py2) in pixels2 {
//             let global_px2 = (cell2_x + (*px2 as f32)) as i32;
//             let global_py2 = (cell2_y - (*py2 as f32)) as i32;

//             //println!("p1: ({},{}), p2: ({},{}),  global_px1: {}, global_py1: {}, global_px2: {}, global_py2: {}", px1, py1, px2, py2, global_px1, global_py1, global_px2, global_py2);
//             // println!(
//             //     "global_px1: {}, global_py1: {}, global_px2: {}, global_py2: {}",
//             //     global_px1, global_py1, global_px2, global_py2
//             // );

//             if global_px1 == global_px2 && global_py1 == global_py2 {
//                 return true;
//             }
//         }
//     }
//     false
// }

// fn pixel_collision(
//     cell_pos1: (f32, f32),
//     pixels1: &HashSet<(i32, i32)>,
//     cell_pos2: (f32, f32),
//     pixels2: &HashSet<(i32, i32)>,
// ) -> bool {
//     let (cell1_x, cell1_y) = cell_pos1;
//     let (cell2_x, cell2_y) = cell_pos2;

//     for (px1, py1) in pixels1 {
//         let global_px1 = cell1_x + *px1 as f32;
//         let global_py1 = cell1_y - *py1 as f32;

//         for (px2, py2) in pixels2 {
//             let global_px2 = cell2_x + *px2 as f32;
//             let global_py2 = cell2_y - *py2 as f32;

//             if global_px1 == global_px2 && global_py1 == global_py2 {
//                 return true;
//             }
//         }
//     }
//     false
// }

// fn pixel_collision(
//     cell_pos1: (f32, f32),
//     pixels1: &HashSet<(i32, i32)>,
//     cell_pos2: (f32, f32),
//     pixels2: &HashSet<(i32, i32)>,
// ) -> bool {
//     let (cell1_x, cell1_y) = cell_pos1;
//     let (cell2_x, cell2_y) = cell_pos2;

//     for (pi1, pj1) in pixels1 {
//         let global_px1 = cell1_x + *pi1 as f32;
//         let global_py1 = cell1_y - *pj1 as f32;

//         for (pi2, pj2) in pixels2 {
//             let global_px2 = cell2_x + *pi2 as f32;
//             let global_py2 = cell2_y - *pj2 as f32;

//             if (global_px1 - global_px2).abs() < f32::EPSILON && (global_py1 - global_py2).abs() < f32::EPSILON {
//                 return true;
//             }
//         }
//     }
//     false
// }

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


    // let min_x = overlapping_pixels.iter().map(|&(x, _)| x).min().unwrap();
    // let max_x = overlapping_pixels.iter().map(|&(x, _)| x).max().unwrap();
    // let min_y = overlapping_pixels.iter().map(|&(_, y)| y).min().unwrap();
    // let max_y = overlapping_pixels.iter().map(|&(_, y)| y).max().unwrap();

    // if collision{
    //     writeln!(file, "---overlapping-pixels------------------------------------").unwrap();
    //     for j in min_y..=max_y {
    //         for i in min_x..=max_x {
    //             if overlapping_pixels.contains(&(i, j)) {
    //                 write!(file, "({},{})", i, j).unwrap();
    //             } else {
    //                 write!(file, " ").unwrap();
    //             }
    //         }
    //         writeln!(file,"").unwrap();
    //     }
    //     writeln!(file, "----------------------------------------------------").unwrap();
    // }

    // if collision{
    //         // Exit with code 1
    //     /////////process::exit(1);
    // }
    // false
    collision
}




fn cell_collision(
    pos1: Vec3,
    image1: &CollidableImage,
    pos2: Vec3,
    image2: &CollidableImage,
    // cell_size: i32,
) -> bool {
    let dx = pos1.x - pos2.x;
    let dy = pos1.y - pos2.y;

    // let adjustment1_x = (image1.width_i32() - CELL_SIZE) as f32 / 2.0;
    // let adjustment1_y = (image1.height_i32() - CELL_SIZE) as f32 / 2.0;

    // let adjustment2_x = (image2.width_i32() - CELL_SIZE)  as f32 / 2.0;
    // let adjustment2_y = (image2.height_i32() - CELL_SIZE)  as f32 / 2.0;

    for (cell1_key, pixels1) in image1.opaque_pixel_cells.iter() {
        let (cell1_i, cell1_j) = cell1_key;
        let cell1_x = (cell1_i * CELL_SIZE) as f32;
        let cell1_y = (cell1_j * CELL_SIZE) as f32;

        let cell2_x = dx + cell1_x;
        let cell2_y = dy + cell1_y;

        let cell2_i = ((cell2_x) / (CELL_SIZE as f32)) as i32;
        let cell2_j = ((cell2_y) / (CELL_SIZE as f32)) as i32;
        
        let cell2_key = (cell2_i, cell2_j);
        if let Some(pixels2) = image2.opaque_pixel_cells.get(&cell2_key) {
            // return true;
            let global_cell1_x = pos1.x + cell1_x;
            let global_cell1_y = pos1.y + cell1_y;
            
            let global_cell2_x = pos2.x + cell2_x;
            let global_cell2_y = pos2.y + cell2_y;

            if pixel_collision(
                (global_cell1_x, global_cell1_y),
                pixels1,
                (global_cell2_x, global_cell2_y),
                pixels2,
            ) {
                return true;
            }
        }
    }
    false
}



pub fn dragon_movement_system(
    time: Res<Time>,
    mut dragon_query: Query<(&mut Dragon, &mut Transform)>,
    wall_query: Query<(&Wall, &Transform), Without<Dragon>>,
    resource_cache: Res<ResourceCache>,
) {
    for (mut dragon, mut dragon_transform) in dragon_query.iter_mut() {
        
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

        // Check for collisions
        if let Some(dragon_image) = resource_cache.dragon_images.get(&dragon.elemental_theme) {
            let mut total_adjustment = Vec3::ZERO;

            for (wall, wall_transform) in wall_query.iter() {
                if let Some(wall_image) = resource_cache.wall_images.get(&wall.shape) {
                    // If the collision occurs on multiple sides, the side with the deepest penetration is returned.
                    // If all sides are involved, `Inside` is returned.
                    if let Some(collision) = collide(
                        dragon_transform.translation,
                        dragon_image.size_vec2(),
                        wall_transform.translation,
                        wall_image.size_vec2()
                    ) {
                        // Check for cell collision
                        if dragon.my_dragon.is_some() && cell_collision(
                            dragon_transform.translation,
                            &dragon_image,
                            wall_transform.translation,
                            &wall_image
                        ) {
                            dragon.action.velocity = Vec3::ZERO;
                            match collision {
                                Collision::Left => {
                                    total_adjustment.x -= 0.2;
                                }
                                Collision::Right => {
                                    total_adjustment.x += 0.2;
                                }
                                Collision::Top => {
                                    total_adjustment.y += 0.2;
                                }
                                Collision::Bottom => {
                                    total_adjustment.y -= 0.2;
                                }
                                Collision::Inside => {
                                    println!("Dragon inside wall collision!?");
                                    total_adjustment.x += 0.5;
                                }
                            }
                        }
                    }
                }
            }

            // Apply the total adjustment
            dragon_transform.translation += total_adjustment;
        }

        // Move the dragon sprite.
        if dragon.action.velocity != Vec3::ZERO {
            dragon.action.velocity = dragon.action.velocity.clamp_length_max(dragon.action.max_velocity);
            dragon_transform.translation += dragon.action.velocity;
        }
        if dragon.action.motion_timer.tick(time.delta()).just_finished() {
            dragon.action.acceleration = dragon.action.velocity - previous_velocity;
        }

        // // Flip the dragon with an animation when it changes directions between left to right.
        // if dragon.action.flipping {
        //     if dragon.action.flip_timer.tick(time.delta()).just_finished() {
        //     // Finish the flipping animation.
        //         dragon.action.flipping = false;
        //         if dragon_transform.scale.x < 0.0{
        //             dragon_transform.scale.x = 1.0;
        //         } else {
        //             dragon_transform.scale.x = -1.0;
        //         }
        //     } else {
        //         // Continue the flipping animation.
        //         let progress = dragon.action.flip_timer.percent();
        //         dragon_transform.scale.x = dragon_transform.scale.x.signum() * (0.5 - 0.5 * (progress * std::f32::consts::PI).sin());
        //     }
        // } else if (dragon.action.velocity.x > 0.0 && dragon_transform.scale.x < 0.0) || (dragon.action.velocity.x < 0.0 && dragon_transform.scale.x > 0.0) {
        //     // Start the flipping animation.
        //     dragon.action.flip_timer.reset();
        //     dragon.action.flipping = true;
        // }
    }
}

pub fn draw_cell_grids_system(
    mut commands: Commands,
    dragon_query: Query<(Entity, &Transform, &Dragon), With<MyDragon>>,
    wall_query: Query<(&Transform, &Wall)>,
) {
    //let cell_size = CELL_SIZE as f32; // Assuming CELL_SIZE is a constant defined elsewhere
    let grid_color = Color::rgba(0.8, 0.2, 0.2, 0.7);

    for (dragon_entity, transform, my_dragon) in dragon_query.iter() {

        for (cell_key, _) in my_dragon.image.opaque_pixel_cells.iter() {
            let (i ,j) = (cell_key.0, cell_key.1);
            let x = (i * CELL_SIZE) as f32  + (CELL_SIZE/2) as f32;
            let y = (j * CELL_SIZE) as f32  + (CELL_SIZE/2) as f32;

            let position = transform.translation + Vec3::new(x, y, 0.0);
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
