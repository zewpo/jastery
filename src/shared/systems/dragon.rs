use bevy::{prelude::*, sprite::collide_aabb::{collide, Collision}};
// use image::{DynamicImage, GenericImageView};
use std::collections::{HashMap, HashSet};
use crate::shared::components::{dragon::*, resource_cache::*, wall::*};


// fn pixel_collision(
//     pos1: Vec3,
//     non_transparent_pixels1: &HashMap<(i32, i32), HashSet<(i32, i32)>>,
//     pos2: Vec3,
//     non_transparent_pixels2: &HashMap<(i32, i32), HashSet<(i32, i32)>>,
//     cell_size: i32,
// ) -> bool {
//     let dx = (pos2.x - pos1.x).round() as i32;
//     let dy = (pos2.y - pos1.y).round() as i32;

//     for ((cell_x1, cell_y1), pixels1) in non_transparent_pixels1.iter() {
//         let cell_x2 = *cell_x1 - dx / cell_size;
//         let cell_y2 = *cell_y1 - dy / cell_size;

//         if let Some(pixels2) = non_transparent_pixels2.get(&(cell_x2, cell_y2)) {
//             for (x1, y1) in pixels1 {
//                 let x2 = *x1 + cell_x2 * cell_size;
//                 let y2 = *y1 + cell_y2 * cell_size;

//                 let pixel_in_cell2 = (x2 % cell_size, y2 % cell_size);

//                 if pixels2.contains(&pixel_in_cell2) {
//                     return true;
//                 }
//             }
//         }
//     }
//     false
// }

// fn pixel_collision(
//     pos1: Vec3,
//     non_transparent_pixels1: &HashMap<(i32, i32), HashSet<(i32, i32)>>,
//     pos2: Vec3,
//     non_transparent_pixels2: &HashMap<(i32, i32), HashSet<(i32, i32)>>,
//     cell_size: i32,
// ) -> bool {
//     let dx = (pos2.x - pos1.x).round() as i32;
//     let dy = (pos2.y - pos1.y).round() as i32;

//     for ((cell_x1, cell_y1), pixels1) in non_transparent_pixels1.iter() {
//         for (x1, y1) in pixels1 {
//             let x1_global = *x1 + *cell_x1 * cell_size;
//             let y1_global = *y1 + *cell_y1 * cell_size;

//             let x2_global = x1_global + dx;
//             let y2_global = y1_global + dy;

//             let cell_x2 = x2_global / cell_size;
//             let cell_y2 = y2_global / cell_size;

//             let pixel_in_cell2 = (x2_global % cell_size, y2_global % cell_size);

//             if let Some(pixels2) = non_transparent_pixels2.get(&(cell_x2, cell_y2)) {
//                 if pixels2.contains(&pixel_in_cell2) {
//                     return true;
//                 }
//             }
//         }
//     }
//     false
// }

fn pixel_collision(
    pos1: Vec3,
    non_transparent_pixels1: &HashMap<(i32, i32), HashSet<(i32, i32)>>,
    pos2: Vec3,
    img2_size: Vec2, // We need the size of image 2 to determine if a collision occurs within its bounds
    cell_size: i32,
) -> bool {
    let dx = (pos2.x - pos1.x).round() as i32;
    let dy = (pos2.y - pos1.y).round() as i32;
    let (width2, height2) = (img2_size.x as i32, img2_size.y as i32);

    for ((cell_x1, cell_y1), pixels1) in non_transparent_pixels1.iter() {
        for (x1, y1) in pixels1 {
            let x1_global = *x1 + *cell_x1 * cell_size;
            let y1_global = *y1 + *cell_y1 * cell_size;

            let x2_global = x1_global + dx;
            let y2_global = y1_global + dy;

            // Check if the global position of the pixel from image 1 is within the bounds of image 2
            if x2_global >= 0 && x2_global < width2 && y2_global >= 0 && y2_global < height2 {
                return true;
            }
        }
    }
    false
}


pub fn dragon_movement_system(
    time: Res<Time>,
    mut dragon_query: Query<(&Dragon, &mut DragonAction, &DragonInput, &mut Transform)>,
    wall_query: Query<(&Wall, &Transform), Without<Dragon>>,
    resource_cache: Res<ResourceCache>,
) {
    for (dragon, mut dragon_action, dragon_input, mut dragon_transform) in dragon_query.iter_mut() {
        
        let previous_velocity = dragon_action.velocity;
        
        // Change in motion
        if dragon_action.motion_timer.tick(time.delta()).just_finished() {
            let acceleration_rate = 0.45;
            let mut decceleration_rate = acceleration_rate;

            if dragon_input.brake {
                decceleration_rate *= 0.5;
            }

            if dragon_input.move_direction.x != 0.0 {
                dragon_action.velocity.x += dragon_input.move_direction.x * acceleration_rate;
            } else {
                dragon_action.velocity.x *= decceleration_rate;
            }

            if dragon_input.move_direction.y != 0.0 {
                dragon_action.velocity.y += dragon_input.move_direction.y * acceleration_rate;
            } else {
                dragon_action.velocity.y *= decceleration_rate;
            }

            if dragon_input.move_direction.z != 0.0 {
                dragon_action.velocity.z += dragon_input.move_direction.z * acceleration_rate;
            } else {
                dragon_action.velocity.z *= decceleration_rate;
            }
        }


        if dragon_input.home {
            // Move to home position
            dragon_action.velocity = Vec3::ZERO;
            dragon_transform.translation = dragon_action.spawn_home;
        } 
        // else if dragon_action.motion_timer.tick(time.delta()).just_finished() {
            
        // }

        // Check for collisions
        if let Some(dragon_image) = resource_cache.dragon_images.get(&dragon.elemental_theme) {
            for (wall, wall_transform) in wall_query.iter() {
                if let Some(wall_image) = resource_cache.wall_images.get(&wall.shape) {
                     // If the collision occurs on multiple sides, the side with the deepest penetration is returned.
                     // If all sides are involved, `Inside` is returned.
                    if let Some(collision) = collide(
                        dragon_transform.translation,
                        dragon_image.size(),
                        wall_transform.translation,
                        wall_image.size()
                    ) {
                        // Check for pixel-perfect collision
                        if pixel_collision(
                            dragon_transform.translation,
                            // dragon_image.image.size,
                            &dragon_image.image.non_transparent_pixels,
                            wall_transform.translation,
                            wall_image.size(),
                            // &wall_image.image.non_transparent_pixels,
                            CELL_SIZE
                        ) {
                            let mut opposite_direction = -0.5 * dragon_action.velocity.normalize_or_zero();
                            if opposite_direction == Vec3::ZERO {
                                opposite_direction = Vec3::new(5.0, 0.0, 0.0);
                            }
                            // dragon_action.velocity = Vec3::ZERO;
                            dragon_action.velocity = opposite_direction;
                            match collision {
                                Collision::Left => {
                                    dragon_transform.translation.x = wall_transform.translation.x - ((wall_image.width() + dragon_image.width()) as f32) / 2.0;
                                    // dragon_action.velocity.x = -0.0;
                                }
                                Collision::Right => {
                                    dragon_transform.translation.x = wall_transform.translation.x + ((wall_image.width() + dragon_image.width()) as f32) / 2.0;
                                    // dragon_action.velocity.x = 0.0;
                                }
                                Collision::Top => {
                                    dragon_transform.translation.y = wall_transform.translation.y + ((wall_image.height() + dragon_image.height()) as f32) / 2.0;
                                    // dragon_action.velocity.y = 0.0;
                                }
                                Collision::Bottom => {
                                    dragon_transform.translation.y = wall_transform.translation.y - ((wall_image.height() + dragon_image.height()) as f32) / 2.0;
                                    // dragon_action.velocity.y = -0.0;
                                }
                                Collision::Inside => {
                                    // Handle inside collision as appropriate for your game
                                    println!("Dragon inside wall collision!?");
                                    dragon_transform.translation.x = wall_transform.translation.x + ((wall_image.width() + dragon_image.width()) as f32) / 2.0;
                                }
                            }
                        }
                        break;
                    }
                }
            }
        }

        // Move the dragon sprite.
        if dragon_action.velocity != Vec3::ZERO {
            dragon_action.velocity = dragon_action.velocity.clamp_length_max(dragon_action.max_velocity);
            dragon_transform.translation += dragon_action.velocity;
        }
        if dragon_action.motion_timer.tick(time.delta()).just_finished() {
            dragon_action.acceleration = previous_velocity - previous_velocity;
        }

        // // Flip the dragon with an animation when it changes directions between left to right.
        // if dragon_action.flipping {
        //     if dragon_action.flip_timer.tick(time.delta()).just_finished() {
        //     // Finish the flipping animation.
        //         dragon_action.flipping = false;
        //         if dragon_transform.scale.x < 0.0{
        //             dragon_transform.scale.x = 1.0;
        //         } else {
        //             dragon_transform.scale.x = -1.0;
        //         }
        //     } else {
        //         // Continue the flipping animation.
        //         let progress = dragon_action.flip_timer.percent();
        //         dragon_transform.scale.x = dragon_transform.scale.x.signum() * (0.5 - 0.5 * (progress * std::f32::consts::PI).sin());
        //     }
        // } else if (dragon_action.velocity.x > 0.0 && dragon_transform.scale.x < 0.0) || (dragon_action.velocity.x < 0.0 && dragon_transform.scale.x > 0.0) {
        //     // Start the flipping animation.
        //     dragon_action.flip_timer.reset();
        //     dragon_action.flipping = true;
        // }
    }
}