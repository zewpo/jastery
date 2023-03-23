use bevy::{prelude::*, sprite::collide_aabb::{collide, Collision}};

use crate::shared::components::{dragon::*, resource_cache::*, wall::*};


pub fn dragon_movement_system(
    time: Res<Time>,
    mut dragon_query: Query<(&Dragon, &mut DragonAction, &DragonInput, &mut Transform)>,
    wall_query: Query<(&Wall, &Transform), Without<Dragon>>,
    resource_cache: Res<ResourceCache>,
) {
    println!("dragon_movement_system running");
    for (dragon, mut dragon_action, dragon_input, mut dragon_transform) in dragon_query.iter_mut() {
        let acceleration = 0.4;

        dragon_action.velocity.x += dragon_input.move_direction.x * acceleration;
        dragon_action.velocity.y += dragon_input.move_direction.y * acceleration;

        // Brake
        if dragon_input.brake && dragon_action.motion_timer.tick(time.delta()).just_finished() {
            dragon_action.velocity *= 0.6;
        }

        // Move to home position
        if dragon_input.home {
            dragon_action.velocity = Vec3::ZERO;
            dragon_transform.translation = dragon_action.spawn_home;
        } else if dragon_input.ease_up && dragon_action.motion_timer.tick(time.delta()).just_finished() {
            dragon_action.velocity *= 0.8;
        }

        // Check for collisions
        if let Some(dragon_image) = resource_cache.dragon_images.get(&dragon.elemental_theme) {
            for (wall, wall_transform) in wall_query.iter() {
                if let Some(wall_image) = resource_cache.wall_images.get(&wall.shape) {
                     // If the collision occurs on multiple sides, the side with the deepest penetration is returned.
                     // If all sides are involved, `Inside` is returned.
                    if let Some(collision) = collide(
                        dragon_transform.translation,
                        dragon_image.size,
                        wall_transform.translation,
                        wall_image.size
                    ) {
                        dragon_action.velocity = Vec3::ZERO;
                        match collision {
                            Collision::Left => {
                                dragon_transform.translation.x = wall_transform.translation.x - (wall_image.size.x + dragon_image.size.x) / 2.0;
                                dragon_action.velocity.x = -0.0;
                            }
                            Collision::Right => {
                                dragon_transform.translation.x = wall_transform.translation.x + (wall_image.size.x + dragon_image.size.x) / 2.0;
                                dragon_action.velocity.x = 0.0;
                            }
                            Collision::Top => {
                                dragon_transform.translation.y = wall_transform.translation.y + (wall_image.size.y + dragon_image.size.y) / 2.0;
                                dragon_action.velocity.y = 0.0;
                            }
                            Collision::Bottom => {
                                dragon_transform.translation.y = wall_transform.translation.y - (wall_image.size.y + dragon_image.size.y) / 2.0;
                                dragon_action.velocity.y = -0.0;
                            }
                            Collision::Inside => {
                                // Handle inside collision as appropriate for your game
                                println!("Dragon inside wall collision!?");
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

        // Flip the dragon with an animation when it changes directions between left to right.
        if dragon_action.flipping {
            if dragon_action.flip_timer.tick(time.delta()).just_finished() {
            // Finish the flipping animation.
                dragon_action.flipping = false;
                if dragon_transform.scale.x < 0.0{
                    dragon_transform.scale.x = 1.0;
                } else {
                    dragon_transform.scale.x = -1.0;
                }
            } else {
                // Continue the flipping animation.
                let progress = dragon_action.flip_timer.percent();
                dragon_transform.scale.x = dragon_transform.scale.x.signum() * (0.5 - 0.5 * (progress * std::f32::consts::PI).sin());
            }
        } else if (dragon_action.velocity.x > 0.0 && dragon_transform.scale.x < 0.0) || (dragon_action.velocity.x < 0.0 && dragon_transform.scale.x > 0.0) {
            // Start the flipping animation.
            dragon_action.flip_timer.reset();
            dragon_action.flipping = true;
        }
    }
}