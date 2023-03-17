use bevy::window::close_on_esc;
use bevy::window::PresentMode;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

#[derive(PartialEq)]
enum MoveDirection {
    Stop,
    Up,
    Down,
    Left,
    Right,
    Brake,
    Home,
    EaseUp,
}


#[derive(Bundle)]
struct Dragon {
    #[bundle]
    sprite_bundle: SpriteBundle,
    input: DragonInput,
    movement: DragonMovement,
}

#[derive(Component)]
struct DragonInput {
    move_direction: MoveDirection,
}

#[derive(Component)]
struct DragonMovement {
    velocity: Vec3,
    max_velocity: f32,
    timer: Timer,
}

#[derive(Bundle)]
struct Fireball {
    #[bundle]
    sprite_bundle: SpriteBundle,
    movement: FireballMovement,
}

#[derive(Component)]
struct FireballMovement {
    speed: f32,
    despawn_timer: Timer,
}

#[derive(Component)]
struct Wall;

#[derive(Bundle)]
struct WallBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    wall: Wall,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Dragsteroids!".into(),
                resolution: (1200., 800.).into(),
                present_mode: PresentMode::AutoVsync,
                // Tells wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(setup)
        .add_system(keyboard_input_system)
        .add_system(dragon_movement_system)
        .add_system(dragon_wall_collision_system)
        .add_system(fireball_spawn_system)
        .add_system(fireball_movement_system)
        .add_system(close_on_esc)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

     // Set the camera scale factor
    let camera_scale = 2.0; // Adjust this value to change the zoom level (e.g., 2.0 for half the size, 0.5 for twice the size)

    // Spawn a Camera2d into the game, so that we can see the game.
    commands.spawn(Camera2dBundle {
        transform: Transform::from_scale(Vec3::splat(camera_scale)),
        ..default()
    });

    // Spawn the Dragon into the game.
    commands.spawn(Dragon {
        sprite_bundle: SpriteBundle {
            texture: asset_server.load("dragon.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
        input: DragonInput {
            move_direction: MoveDirection::Stop,
        },
        movement: DragonMovement {
            velocity: Vec3::ZERO,
            max_velocity: 15.0,
            timer: Timer::from_seconds(0.05, TimerMode::Repeating),
        },
    });
 

    let mut maze = [
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1],
        [1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1],
        [1, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
        [1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1],
        ];

    let wall_width = 200.0; // Adjust this value based on your wall texture size
    let wall_height = 100.0;
    maze.reverse();
    // Spawn Wall blocks into the game.
    for (i, row) in maze.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == 1 {
                spawn_wall(
                    &mut commands,
                    &asset_server,
                    (j as f32 * wall_width) - 1600.0,
                    (i as f32 * wall_height) - 1000.0,
                );
            }
        }
    }

}

fn spawn_wall(commands: &mut Commands, asset_server: &Res<AssetServer>, x: f32, y: f32) {
    commands.spawn(WallBundle {
        sprite_bundle: SpriteBundle {
            texture: asset_server.load("wall.png"), // Load your wall texture here
            transform: Transform::from_xyz(x, y, 0.),
            ..default()
        },
        wall: Wall,
    });
}

fn dragon_wall_collision_system(
    mut dragon_query: Query<(&mut DragonMovement, &Transform, &Handle<Image>)>,
    wall_query: Query<(&Wall, &Transform, &Handle<Image>)>,
    images: Res<Assets<Image>>,
) {
    for (mut dragon_movement, dragon_transform, dragon_image_handle) in dragon_query.iter_mut() {
        if let Some(dragon_image) = images.get(dragon_image_handle) {
            let dragon_size = dragon_image.size().extend(0.0) * dragon_transform.scale.abs();
            let dragon_center_position = dragon_transform.translation;

            let mut has_collision = false;

            for (_, wall_transform, wall_image_handle) in wall_query.iter() {
                if let Some(wall_image) = images.get(wall_image_handle) {
                    let wall_size = wall_image.size().extend(0.0) * wall_transform.scale.abs();
                    let wall_center_position = wall_transform.translation;

                    let collision = collide(
                        dragon_center_position,
                        dragon_size.truncate(),
                        wall_center_position,
                        wall_size.truncate(),
                    );

                    if collision.is_some() {
                        has_collision = true;
                        break;
                    }
                }
            }

            if has_collision {
                if dragon_movement.velocity.x > 0.0 {
                        dragon_movement.velocity.x = -10.0;
                //     dragon_transform.translation.x -= 5.;
                } else {
                        dragon_movement.velocity.x = 10.0;
                //     dragon_transform.translation.x += 5.;
                }
                if dragon_movement.velocity.y > 0.0{
                        dragon_movement.velocity.y = -10.0;
                //     dragon_transform.translation.y -= 5.;
                } else {
                        dragon_movement.velocity.y = 10.0;
                //     dragon_transform.translation.y += 5.;
                }
                // dragon_movement.velocity = Vec3::ZERO;
            }
        }
    }
}


fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut DragonInput>) {
    for mut dragon_input in query.iter_mut() {
        dragon_input.move_direction = 
            if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W)
        {
            MoveDirection::Up
        } else if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            MoveDirection::Down
        } else if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            MoveDirection::Left
        } else if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            MoveDirection::Right
        } else if keyboard_input.pressed(KeyCode::LShift) || keyboard_input.pressed(KeyCode::RShift){
            MoveDirection::Brake
        } else if keyboard_input.pressed(KeyCode::X) {
            MoveDirection::Home
        } else {
            MoveDirection::EaseUp
        };
    }
}

fn dragon_movement_system(
    time: Res<Time>,
    mut dragon_query: Query<(&mut DragonMovement, &DragonInput, &mut Transform, &Handle<Image>)>,
    window_query: Query<&Window>,
//     camera_query: Query<(&Camera2d,&Transform)>,
    images: Res<Assets<Image>>,
) {

    let window = window_query.single();
    //let (camera, camera_transform) = camera_query.single();
    let camera_scale = 2.0;
//     for (_,camera_transform) in camera_query.iter() {
//         camera_scale = camera_transform.scale.x;
//     }

    for (mut dragon_movement, dragon_input, mut dragon_transform, image_handle) in dragon_query.iter_mut() {
        match dragon_input.move_direction {
            MoveDirection::Stop => dragon_movement.velocity = Vec3::ZERO,
            MoveDirection::Up => dragon_movement.velocity.y += (dragon_movement.velocity.y.abs()/7.0) + 0.4,
            MoveDirection::Down => dragon_movement.velocity.y -= (dragon_movement.velocity.y.abs()/7.0) + 0.4,
            MoveDirection::Left => dragon_movement.velocity.x -= (dragon_movement.velocity.x.abs()/7.0) + 0.4,
            MoveDirection::Right => dragon_movement.velocity.x += (dragon_movement.velocity.x.abs()/7.0) + 0.4,
            MoveDirection::Brake => {
                dragon_movement.velocity *= 0.4;
            }
            MoveDirection::Home => {
                dragon_movement.velocity = Vec3::ZERO;
                dragon_transform.translation = Vec3::ZERO;
            }
            MoveDirection::EaseUp => {
                if dragon_movement.timer.tick(time.delta()).just_finished() {
                dragon_movement.velocity *= 0.8;
                }
            }
        }


        if let Some(image) = images.get(image_handle) {
            
            let scaled_window_width = window.width() * camera_scale;
            let scaled_window_height = window.height() * camera_scale;

            // Clamp the dragon's position within the screen boundaries.
            let image_width = image.size().x * dragon_transform.scale.x.abs();
            let image_height = image.size().y * dragon_transform.scale.y.abs();
            if dragon_transform.translation.x.abs() > (scaled_window_width - image_width).abs()/2.0 {
                dragon_movement.velocity.x = 0.0;
                dragon_transform.translation.x -= 5.0 * dragon_transform.translation.x/dragon_transform.translation.x.abs();
            }
            if dragon_transform.translation.y.abs() > (scaled_window_height - image_height).abs()/2.0 {
                dragon_movement.velocity.y = 0.0;
                dragon_transform.translation.y -= 5.0 * dragon_transform.translation.y/dragon_transform.translation.y.abs();
            }
        }

        // Move the sprite.
        if dragon_movement.velocity != Vec3::ZERO {
            dragon_transform.translation += dragon_movement.velocity.clamp_length_max(dragon_movement.max_velocity);
        }

        // Display direction.  Assume the sprite is normallay facing to the right.
        if dragon_movement.velocity.x > 0.0 && dragon_transform.scale.x < 0.0 {
            // Moving right, set the X scale to positive (normal)
            dragon_transform.scale.x = dragon_transform.scale.x.abs();
        } else if dragon_movement.velocity.x < 0.0 && dragon_transform.scale.x > 0.0 {
            // Moving left, set the X scale to negative (flipped)
            dragon_transform.scale.x = -dragon_transform.scale.x.abs();
        }

    }
}

fn fireball_spawn_system(
    keyboard_input: Res<Input<KeyCode>>,
    dragon_query: Query<(&DragonMovement, &Transform)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for (dragon_movement, dragon_transform) in dragon_query.iter() {
            
            let mut speed_x: f32 = f32::min(1500.0, 100.0 + (20.0 * dragon_movement.velocity.x.abs()));
            let mut scale_x: f32 = 1.0;
            let mut trans_x: f32 = 120.0;

            if dragon_transform.scale.x < 0.0 { 
                speed_x *= -1.0;
                scale_x *= -1.0;
                trans_x *= -1.0;
            }

            // Spawn a fireball into the game.
            commands.spawn(Fireball {
                sprite_bundle: SpriteBundle {
                    texture: asset_server.load("fireball.png"),
                    transform: Transform {
                        translation: dragon_transform.translation + Vec3::new(trans_x, 30.0, 0.0),
                        scale: Vec3::new(scale_x,1.0,1.0),
                        ..default()
                    },
                    ..default()
                },
                movement: FireballMovement { 
                    speed: speed_x,
                    despawn_timer: Timer::from_seconds(2.4, TimerMode::Once), // Set the timeout duration here
                },
            });
        }
    }
}

fn fireball_movement_system(
        time: Res<Time>,
        mut commands: Commands,
        mut fireball_query: Query<(Entity, &mut FireballMovement, &mut Transform)>,
    ) {
    let delta_time = time.delta_seconds();
    for (   entity,
            mut fireball_movement, 
            mut fireball_transform
        ) in fireball_query.iter_mut() {

        // Move the sprite by distance of speed * time.
        fireball_transform.translation.x += fireball_movement.speed * delta_time;
        
        // Update the despawn timer
        fireball_movement.despawn_timer.tick(time.delta());

        // Despawn the fireball if the timer has finished
        if fireball_movement.despawn_timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}
