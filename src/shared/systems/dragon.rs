use bevy::{prelude::*, sprite::collide_aabb::{collide, Collision}};
// use image::{DynamicImage, GenericImageView};
use std::collections::{HashMap, HashSet};
use crate::shared::components::{dragon::*, resource_cache::*, wall::*};


// fn pixel_collision(
//     pos1: Vec3,
//     opaque_pixel_cells1: &HashMap<(i32, i32), HashSet<(i32, i32)>>,
//     pos2: Vec3,
//     img2_size: Vec2, // We need the size of image 2 to determine if a collision occurs within its bounds
//     cell_size: i32,
// ) -> bool {
//     let dx = (pos2.x - pos1.x).round() as i32;
//     let dy = (pos2.y - pos1.y).round() as i32;
//     let (width2, height2) = (img2_size.x as i32, img2_size.y as i32);

//     for ((cell_x1, cell_y1), pixels1) in opaque_pixel_cells1.iter() {
//         for (x1, y1) in pixels1 {
//             let x1_global = *x1 + *cell_x1 * cell_size;
//             let y1_global = *y1 + *cell_y1 * cell_size;

//             let x2_global = x1_global + dx;
//             let y2_global = y1_global + dy;

//             // Check if the global position of the pixel from image 1 is within the bounds of image 2
//             if x2_global >= 0 && x2_global < width2 && y2_global >= 0 && y2_global < height2 {
//                 return true;
//             }
//         }
//     }
//     false
// }


fn pixel_collision(
    pos1: Vec3,
    opaque_pixel_cells1: &HashMap<(i32, i32), HashSet<(i32, i32)>>,
    pos2: Vec3,
    img2_size: Vec2,
    cell_size: i32,
) -> bool {
    let dx = (pos2.x - pos1.x).round() as i32;
    let dy = (pos2.y - pos1.y).round() as i32;
    let (width2, height2) = (img2_size.x as i32, img2_size.y as i32);

    // Check if the images overlap at all
    if pos1.x + (cell_size as f32) <= pos2.x || pos1.x >= pos2.x + (img2_size.x as f32) ||
        pos1.y + (cell_size as f32) <= pos2.y || pos1.y >= pos2.y + (img2_size.y as f32)
    {
        return false;
    }

    for ((cell_x1, cell_y1), pixels1) in opaque_pixel_cells1.iter() {
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


// fn cell_collision(
//     pos1: Vec3,
//     opaque_pixel_cells1: &HashMap<(i32, i32), HashSet<(i32, i32)>>,
//     pos2: Vec3,
//     img2_size: Vec2,
//     cell_size: i32,
// ) -> bool {
//     let (width2, height2) = (img2_size.x as i32, img2_size.y as i32);
//     let (cell_width2, cell_height2) = (width2 / cell_size, height2 / cell_size);

//     // Iterate over each cell of the first object
//     for ((cell_x1, cell_y1), _pixels1) in opaque_pixel_cells1.iter() {
//         let cell_x1 = *cell_x1;
//         let cell_y1 = *cell_y1;

//         // Compute the global position of the top-left corner of the current cell of the first object
//         let x1_global = pos1.x as i32 + cell_x1 * cell_size;
//         let y1_global = pos1.y as i32 + cell_y1 * cell_size;

//         // Compute the indices of the cell of the second object that contains the top-left corner of the current cell of the first object
//         let cell_x2 = (x1_global - pos2.x as i32) / cell_size;
//         let cell_y2 = (y1_global - pos2.y as i32) / cell_size;

//         // Check if the indices of the cell of the second object match the current cell of the first object
//         if cell_x2 >= 0 && cell_x2 < cell_width2 && cell_y2 >= 0 && cell_y2 < cell_height2 && (cell_x2, cell_y2) == (cell_x1, cell_y1) {
//             return true;
//         }
//     }

//     // No overlapping cells were found
//     false
// }


fn cell_collision(
    pos1: Vec3,
    opaque_pixel_cells1: &HashMap<(i32, i32), HashSet<(i32, i32)>>,
    pos2: Vec3,
    img2_size: Vec2,
    cell_size: i32,
) -> bool {
    let (width2, height2) = (img2_size.x as i32, img2_size.y as i32);

    for ((cell_x1, cell_y1), _pixels1) in opaque_pixel_cells1.iter() {
        let cell_x2 = (pos2.x as i32 + cell_x1 * cell_size - pos1.x as i32) / cell_size;
        let cell_y2 = (pos2.y as i32 + cell_y1 * cell_size - pos1.y as i32) / cell_size;

        if cell_x2 >= 0 && cell_x2 < width2 / cell_size && cell_y2 >= 0 && cell_y2 < height2 / cell_size {
            return true;
        }
    }
    false
}


// fn cell_collision(
//     pos1: Vec3,
//     cells1: &HashSet<(i32, i32)>,
//     pos2: Vec3,
//     cells2: &HashSet<(i32, i32)>,
//     cell_size: i32,
// ) -> bool {
//     let dx = (pos2.x - pos1.x).round() as i32;
//     let dy = (pos2.y - pos1.y).round() as i32;

//     // Calculate the bounds of the two images in terms of cells
//     let (min_x1, max_x1, min_y1, max_y1) = get_bounds(cells1);
//     let (min_x2, max_x2, min_y2, max_y2) = get_bounds(cells2);

//     // Check if the bounds of the two images overlap
//     if min_x1 + dx <= max_x2 && max_x1 + dx >= min_x2 && min_y1 + dy <= max_y2 && max_y1 + dy >= min_y2 {
//         // Loop through each cell of the first image and check if it overlaps with any cells of the second image
//         for (cell_x1, cell_y1) in cells1 {
//             let x1_global = cell_x1 * cell_size;
//             let y1_global = cell_y1 * cell_size;

//             for (cell_x2, cell_y2) in cells2 {
//                 let x2_global = cell_x2 * cell_size + dx;
//                 let y2_global = cell_y2 * cell_size + dy;

//                 // Check if the two cells overlap
//                 if x1_global <= x2_global + cell_size && x1_global + cell_size >= x2_global && y1_global <= y2_global + cell_size && y1_global + cell_size >= y2_global {
//                     return true;
//                 }
//             }
//         }
//     }

//     false
// }

// fn get_bounds(cells: &HashSet<(i32, i32)>) -> (i32, i32, i32, i32) {
//     let mut min_x = i32::MAX;
//     let mut max_x = i32::MIN;
//     let mut min_y = i32::MAX;
//     let mut max_y = i32::MIN;

//     for (x, y) in cells {
//         if *x < min_x {
//             min_x = *x;
//         }
//         if *x > max_x {
//             max_x = *x;
//         }
//         if *y < min_y {
//             min_y = *y;
//         }
//         if *y > max_y {
//             max_y = *y;
//         }
//     }

//     (min_x, max_x, min_y, max_y)
// }



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
                        // if pixel_collision(
                        if cell_collision(
                            dragon_transform.translation,
                            // dragon_image.image.size,
                            &dragon_image.image.opaque_pixel_cells,
                            wall_transform.translation,
                            wall_image.size(),
                            // &wall_image.image.opaque_pixel_cells,
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