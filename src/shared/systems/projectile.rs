use bevy::{prelude::*, sprite::collide_aabb::collide};
// use image::DynamicImage;
// use uuid::Uuid;
use crate::shared::components::{dragon::*, resource_cache::ResourceCache, projectile::{ProjectileBundle, ProjectileMovement, Projectile}, wall::Wall};

pub fn projectile_spawn_system(
    time: Res<Time>,
    mut dragon_query: Query<(&Dragon, &mut DragonAction, &mut DragonInput, &Transform)>,
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    resource_cache: Res<ResourceCache>,
) {

    for (dragon, mut dragon_action, dragon_input, dragon_transform) in dragon_query.iter_mut() {
    
        if dragon_input.fire && dragon_action.firerate_timer.tick(time.delta()).just_finished() { 
            if let Some(projectile_image) = resource_cache.projectile_images.get(&dragon.elemental_theme) {
            
                // println!("projectile_spawn_system called");
                let mut projectile_direction = dragon_action.velocity.normalize_or_zero();
                if projectile_direction == Vec3::ZERO {
                        projectile_direction.x = 1.0 * dragon_transform.scale.x.signum();
                }

                // Calculate the speed of the projectile based on the dragon's velocity.
                let projectile_speed = projectile_direction * (250.0 + 75.0 * dragon_action.velocity.length());

                // Calculate the rotation of the projectile based on its velocity direction.
                let projectile_rotation = Quat::from_rotation_arc(Vec3::new(1.0,0.0,0.0), projectile_direction);

                // Spawn the projectile into the game.
                commands.spawn(ProjectileBundle {
                    sprite_bundle: SpriteBundle {
                        texture: projectile_image.file_handle.clone(),
                        transform: Transform {
                            translation: dragon_transform.translation + Vec3::new(110.0 * dragon_transform.scale.x.signum(), 30.0, 0.0),
                            rotation: projectile_rotation,
                            ..default()
                        },
                        ..default()
                    },
                    movement: ProjectileMovement { 
                        speed: projectile_speed,
                        despawn_timer: Timer::from_seconds(2.4, TimerMode::Once), // Set the timeout duration here
                    },
                    projectile: Projectile { elemental_theme: dragon.elemental_theme }
                });
            }
        }
    }
}


pub fn projectile_movement_system(
        time: Res<Time>,
        mut commands: Commands,
        mut projectile_query: Query<(Entity, &mut ProjectileMovement, &mut Transform, &Handle<Image>),With<Projectile>>,
        wall_query: Query<(&Wall, &Transform, &Handle<Image>),Without<Projectile>>,
        images: Res<Assets<Image>>,
    ) {
    let delta_time = time.delta_seconds();
    for (   projectile_entity,
            mut projectile_movement, 
            mut projectile_transform,
            projectile_image_handle,
        ) in projectile_query.iter_mut() {

        // Move the projectile sprite by distance of speed * time.
        projectile_transform.translation += projectile_movement.speed * delta_time;

        // Update the despawn timer
        projectile_movement.despawn_timer.tick(time.delta());

        // Despawn the projectile if the timer has finished
        if projectile_movement.despawn_timer.finished() {
            commands.entity(projectile_entity).despawn();
        } else {
            // Check for collisions with walls
            if let Some(projectile_image) = images.get(projectile_image_handle) {
                let projectile_size = Vec2::new(projectile_image.size().x as f32, projectile_image.size().y as f32);
                for (_wall, wall_transform, wall_image_handle) in wall_query.iter() {
                
                    if let Some(wall_image) = images.get(wall_image_handle){
                        let wall_size = Vec2::new(wall_image.size().x as f32, wall_image.size().y as f32);

                        let collision = collide(
                            projectile_transform.translation,
                            projectile_size/5.0,
                            wall_transform.translation,
                            wall_size,
                        );

                        if let Some(_) = collision {
                            commands.entity(projectile_entity).despawn();
                            break;
                        }
                    }
                }
            }
        }
    }
}
