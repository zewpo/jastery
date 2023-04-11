use bevy::{prelude::*, sprite::collide_aabb::collide};
use crate::shared::components::{dragon::*, resource_cache::*, projectile::*, wall::*, game::*, CollidableClassifier};

pub fn projectile_spawn_system(
    time: Res<Time>,
    mut dragon_query: Query<(&mut Dragon, &Transform)>,
    mut commands: Commands,
    resource_cache: Res<ResourceCache>,
) {
    for (mut dragon, dragon_transform) in dragon_query.iter_mut() {
        if dragon.health <= 0 {
           continue; 
        }
        if dragon.input.shoot && dragon.action.shooting_rate_timer.tick(time.delta()).just_finished() { 
            // if let Some(projectile_image) = resource_cache.projectile_images.get(&dragon.elemental_theme) {
            let projectile_image = resource_cache.get_collidable_image(CollidableClassifier::Projectile(dragon.elemental_theme));
                //let mut projectile_direction = dragon_action.velocity.normalize_or_zero();

                let mut projectile_direction = dragon.input.shoot_direction.normalize_or_zero();

                if projectile_direction == Vec3::ZERO {
                    if dragon.input.move_direction.normalize_or_zero() == Vec3::ZERO {
                        if dragon.action.velocity.normalize_or_zero() == Vec3::ZERO {
                            projectile_direction.x = 1.0 * dragon_transform.scale.x.signum();
                        } else {
                            projectile_direction = dragon.action.velocity.normalize_or_zero();
                        }
                    } else {
                        projectile_direction = dragon.input.move_direction.normalize_or_zero();
                    }
                }

                if projectile_direction.normalize_or_zero() == Vec3::ZERO {
                    projectile_direction.x = 1.0;
                }


                // Calculate the speed of the projectile based on the dragon's velocity.  and if moving opposite direcrtion.
                let moving_opposite_direction_x = projectile_direction.x.signum() * dragon.action.velocity.x.signum();
                let moving_opposite_direction_y = projectile_direction.y.signum() * dragon.action.velocity.y.signum();

                let mut projectile_speed = (projectile_direction * 500.0) + 90.0 * Vec3::new(dragon.action.velocity.x * moving_opposite_direction_x, dragon.action.velocity.y * moving_opposite_direction_y, 0.0 );//(250.0 + 75.0 * dragon_action.velocity.length());
                
                while projectile_speed.length() < 80.0 {
                    projectile_speed += 20.0;
                }

                // Calculate the rotation of the projectile image, based on its velocity direction.
                let projectile_rotation = Quat::from_rotation_arc(Vec3::new(1.0,0.0,0.0), projectile_direction.truncate().extend(0.));
                
                let dx_spawn = if projectile_speed.y == 0.0 {
                    110.0 * dragon_transform.scale.x.signum()
                } else {
                    0.0
                };

                // Spawn the projectile into the game.
                commands.spawn(ProjectileBundle {
                    game_piece: GamePiece,
                    sprite_bundle: SpriteBundle {
                        texture: projectile_image.handle(),
                        transform: Transform {
                            translation: dragon_transform.translation + Vec3::new(dx_spawn, 30.0, 0.0),
                            rotation: projectile_rotation,
                            ..default()
                        },
                        ..default()
                    },
                    movement: ProjectileMovement { 
                        speed: projectile_speed,
                        despawn_timer: Timer::from_seconds(2.5, TimerMode::Once), // Set the timeout duration here
                    },
                    projectile: Projectile { 
                        elemental_theme: dragon.elemental_theme,
                        image: projectile_image,
                        shot_by: dragon.id, 
                        }
                });
            // }
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

pub fn projectile_projectile_collision_system(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &Transform, &Projectile), Without<Dragon>>,
    // game_status: Res<GameStatus>,
) {

        // iter_combinations_mut checks for all unique pairs.
    let mut combinations = projectile_query.iter_combinations_mut();
    while let Some([(projectile_entity_a, projectile_transform_a, projectile_a), 
                    (projectile_entity_b, projectile_transform_b, projectile_b)]) 
            = combinations.fetch_next() {
                if projectile_a.elemental_theme != projectile_b.elemental_theme {
                    if 50.0 > (projectile_transform_a.translation - projectile_transform_b.translation).length() {
                        commands.entity(projectile_entity_a).despawn();
                        commands.entity(projectile_entity_b).despawn();
                    }
                }
    }
}

pub fn projectile_dragon_collision_system(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &Transform, &Projectile), Without<Dragon>>,
    mut dragon_query: Query<(&Transform, &mut Dragon), With<Dragon>>,
    game_status: Res<GameStatus>,
) {
    let my_dragon_id = game_status.my_id;

    for (projectile_entity, projectile_transform, projectile) in projectile_query.iter_mut() {
        let mut despawned = false;
        for (dragon_transform, mut dragon) in dragon_query.iter_mut() {
            let distance = projectile_transform.translation.distance(dragon_transform.translation);

            // Check if the projectile is close enough to the dragon to be considered a hit
            if distance < 150.0 {
                if projectile.elemental_theme != dragon.elemental_theme { 
                    if dragon.health > 0 {
                        dragon.health -= 1;
                        if let Some(_) = &dragon.my_dragon {
                            // Handle hit on my_dragon
                            info!("Ouch...  Someone hit my dragon! Health: {}",dragon.health);
                        } else {
                            if projectile.shot_by == my_dragon_id {
                                info!("Yay...  I hit an enemy dragon, Health: {}",dragon.health);
                            } else {
                                info!("Yay...  Someone hit an enemy dragon, Health: {}",dragon.health);
                            }
                        }
                    }
                    // Remove the projectile after it has collided with a dragon of another kind.
                    // Note, it might hit more than one other dragon, but can only despawn it once.
                    if !despawned {
                        commands.entity(projectile_entity).despawn();
                        despawned = true;
                    } 
                } else {
                    // dragons projectiles don't hurt their own kind.
                }
            }
        }
    }
}

