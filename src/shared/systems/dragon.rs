use bevy::{prelude::*, sprite::collide_aabb::{collide, Collision}};
// use image::{DynamicImage, GenericImageView};
use std::collections::{HashMap, HashSet};
use crate::shared::components::{dragon::*, resource_cache::*, wall::*};



// fn pixel_collision(
//     pos1: Vec3,
//     opaque_pixel_cells1: &HashMap<(i32, i32), HashSet<(i32, i32)>>,
//     pos2: Vec3,
//     img2_size: Vec2,
//     cell_size: i32,
// ) -> bool {
//     let dx = (pos2.x - pos1.x).round() as i32;
//     let dy = (pos2.y - pos1.y).round() as i32;
//     let (width2, height2) = (img2_size.x as i32, img2_size.y as i32);

//     // Check if the images overlap at all
//     if pos1.x + (cell_size as f32) <= pos2.x || pos1.x >= pos2.x + (img2_size.x as f32) ||
//         pos1.y + (cell_size as f32) <= pos2.y || pos1.y >= pos2.y + (img2_size.y as f32)
//     {
//         return false;
//     }

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


fn cell_collision(
    pos1: (i32, i32),
    opaque_pixel_cells1: &HashMap<(i32, i32), HashSet<(i32, i32)>>,
    pos2: (i32, i32),
    opaque_pixel_cells2: &HashMap<(i32, i32), HashSet<(i32, i32)>>,
    cell_size: i32,
) -> bool {

    let dx = (pos1.0 - pos2.0) / cell_size;
    let dy = (pos1.1 - pos2.1) / cell_size;

    for ((cell_x1, cell_y1), _) in opaque_pixel_cells1.iter() {
        let cell_x2 = dx + cell_x1;
        let cell_y2 = dy + cell_y1;
        if opaque_pixel_cells2.contains_key(&(cell_x2, cell_y2)) {
            return true;
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
                        if cell_collision(
                            (dragon_transform.translation.x as i32, dragon_transform.translation.y as i32) ,
                            &dragon_image.opaque_pixel_cells ,
                            (wall_transform.translation.x as i32, wall_transform.translation.y as i32),
                            &wall_image.opaque_pixel_cells,
                            CELL_SIZE
                        ) {
                            let mut opposite_direction = -0.5 * dragon.action.velocity.normalize_or_zero();
                            if opposite_direction == Vec3::ZERO {
                                opposite_direction = Vec3::new(5.0, 0.0, 0.0);
                            }
                            // dragon.action.velocity = Vec3::ZERO;
                            dragon.action.velocity = opposite_direction;
                            match collision {
                                Collision::Left => {
                                    dragon_transform.translation.x = wall_transform.translation.x - ((wall_image.width_i32() + dragon_image.width_i32()) as f32) / 2.0;
                                    // dragon.action.velocity.x = -0.0;
                                }
                                Collision::Right => {
                                    dragon_transform.translation.x = wall_transform.translation.x + ((wall_image.width_i32() + dragon_image.width_i32()) as f32) / 2.0;
                                    // dragon.action.velocity.x = 0.0;
                                }
                                Collision::Top => {
                                    dragon_transform.translation.y = wall_transform.translation.y + ((wall_image.height_i32() + dragon_image.height_i32()) as f32) / 2.0;
                                    // dragon.action.velocity.y = 0.0;
                                }
                                Collision::Bottom => {
                                    dragon_transform.translation.y = wall_transform.translation.y - ((wall_image.height_i32() + dragon_image.height_i32()) as f32) / 2.0;
                                    // dragon.action.velocity.y = -0.0;
                                }
                                Collision::Inside => {
                                    // Handle inside collision as appropriate for your game
                                    println!("Dragon inside wall collision!?");
                                    dragon_transform.translation.x = wall_transform.translation.x + ((wall_image.width_i32() + dragon_image.width_i32()) as f32) / 2.0;
                                }
                            }
                        }
                        break;
                    }
                }
            }
        }

        // Move the dragon sprite.
        if dragon.action.velocity != Vec3::ZERO {
            dragon.action.velocity = dragon.action.velocity.clamp_length_max(dragon.action.max_velocity);
            dragon_transform.translation += dragon.action.velocity;
        }
        if dragon.action.motion_timer.tick(time.delta()).just_finished() {
            dragon.action.acceleration = dragon.action.velocity - previous_velocity;
        }

        // Flip the dragon with an animation when it changes directions between left to right.
        if dragon.action.flipping {
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
        } else if (dragon.action.velocity.x > 0.0 && dragon_transform.scale.x < 0.0) || (dragon.action.velocity.x < 0.0 && dragon_transform.scale.x > 0.0) {
            // Start the flipping animation.
            dragon.action.flip_timer.reset();
            dragon.action.flipping = true;
        }
    }
}



// fn draw_cell_grids_system(
//     mut commands: Commands,
//     // asset_server: Res<AssetServer>,
//     // mut materials: ResMut<Assets<ColorMaterial>>,
//     dragon_query: Query<(&Transform, &Dragon), With<MyDragon>>,
//     wall_query: Query<(&Transform, &Wall)>,
// ) {
//     let cell_size = CELL_SIZE as f32; // Assuming CELL_SIZE is a constant defined elsewhere
//     let grid_color = Color::rgba(0.8, 0.2, 0.2, 0.5);

//     for (transform, my_dragon) in dragon_query.iter() {
//         for (cell_key, _) in my_dragon.image.opaque_pixel_cells.iter() {
//             let x = cell_key.0 as f32 * cell_size;
//             let y = cell_key.1 as f32 * cell_size;
//             let position = transform.translation + Vec3::new(x, y, 0.0);
//             draw_cell_grid(&mut commands, position, cell_size, grid_color);
//         }
//     }

//     for (transform, wall) in wall_query.iter() {
//         for (cell_key, _) in wall.image.opaque_pixel_cells.iter() {
//             let x = cell_key.0 as f32 * cell_size;
//             let y = cell_key.1 as f32 * cell_size;
//             let position = transform.translation + Vec3::new(x, y, 0.0);
//             draw_cell_grid(&mut commands, position, cell_size, grid_color);
//         }
//     }
// }

// fn draw_cell_grid(
//     commands: &mut Commands,
//     position: Vec3,
//     cell_size: f32,
//     grid_color: Color,
// ) {
//     commands.spawn(SpriteBundle {
//         sprite: Sprite {
//             color: grid_color,
//             custom_size: Some(Vec2::new(cell_size, cell_size)),
//             ..default()
//         },
//         transform: Transform::from_translation(position),
//         // material: materials.add(grid_color.into()),
//         ..default()
//     });
// }


// fn draw_cell_grids_system(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     dragon_query: Query<(&Transform, &Dragon),With<MyDragon>>,
//     wall_query: Query<(&Transform, &Wall)>,
// ) {
//     let cell_size = CELL_SIZE as f32; // Assuming CELL_SIZE is a constant defined elsewhere
//     let grid_color = Color::rgba(0.8, 0.2, 0.2, 0.5);

//     for (transform, my_dragon) in dragon_query.iter() {
//         for (cell_key, _) in my_dragon.collidable.opaque_pixel_cells.iter() {
//             let x = cell_key.0 as f32 * cell_size;
//             let y = cell_key.1 as f32 * cell_size;
//             let position = transform.translation + Vec3::new(x, y, 0.0);
//             draw_cell_grid(&mut commands, &mut materials, position, cell_size, grid_color);
//         }
//     }

//     for (transform, wall) in wall_query.iter() {
//         for (cell_key, _) in wall.collidable.opaque_pixel_cells.iter() {
//             let x = cell_key.0 as f32 * cell_size;
//             let y = cell_key.1 as f32 * cell_size;
//             let position = transform.translation + Vec3::new(x, y, 0.0);
//             draw_cell_grid(&mut commands, &mut materials, position, cell_size, grid_color);
//         }
//     }
// }

// fn draw_cell_grid(
//     commands: &mut Commands,
//     materials: &mut ResMut<Assets<ColorMaterial>>,
//     position: Vec3,
//     cell_size: f32,
//     grid_color: Color,
// ) {
//     commands.spawn(SpriteBundle {
//         sprite: Sprite {
//             color: grid_color,
//             custom_size: Some(Vec2::new(cell_size, cell_size)),
//             ..default()
//         },
//         transform: Transform::from_translation(position),
//         material: materials.add(grid_color.into()),
//         ..default()
//     });
// }


// fn draw_cell_grids_system(
//     mut lines: ResMut<DebugLines>,
//     time: Res<Time>,
//     dragon_query: Query<(&Transform, &DragonImage)>,
//     wall_query: Query<(&Transform, &WallImage)>,
// ) {
//     let cell_size = CELL_SIZE as f32; // Assuming CELL_SIZE is a constant defined elsewhere

//     for (transform, dragon_image) in dragon_query.iter() {
//         for (cell_key, _) in dragon_image.image.opaque_pixel_cells.iter() {
//             let x = cell_key.0 as f32 * cell_size;
//             let y = cell_key.1 as f32 * cell_size;
//             let position = transform.translation + Vec3::new(x, y, 0.0);
//             draw_cell_grid(&mut lines, position, cell_size);
//         }
//     }

//     for (transform, wall_image) in wall_query.iter() {
//         for (cell_key, _) in wall_image.image.opaque_pixel_cells.iter() {
//             let x = cell_key.0 as f32 * cell_size;
//             let y = cell_key.1 as f32 * cell_size;
//             let position = transform.translation + Vec3::new(x, y, 0.0);
//             draw_cell_grid(&mut lines, position, cell_size);
//         }
//     }
// }

// fn draw_cell_grid(lines: &mut ResMut<DebugLines>, position: Vec3, cell_size: f32) {
//     let half_size = cell_size / 2.0;

//     let top_left = position + Vec3::new(-half_size, half_size, 0.0);
//     let top_right = position + Vec3::new(half_size, half_size, 0.0);
//     let bottom_left = position + Vec3::new(-half_size, -half_size, 0.0);
//     let bottom_right = position + Vec3::new(half_size, -half_size, 0.0);

//     let color = Color::GREEN;

//     lines.line(top_left, top_right, color);
//     lines.line(top_right, bottom_right, color);
//     lines.line(bottom_right, bottom_left, color);
//     lines.line(bottom_left, top_left, color);
// }
