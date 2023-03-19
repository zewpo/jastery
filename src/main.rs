use bevy::sprite::collide_aabb::{collide, Collision};
use bevy::window::close_on_esc;
use bevy::window::PresentMode;
use bevy::prelude::*;


#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Setup,
    Running,
}

#[derive(Bundle)]
struct DragonBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    input: DragonInput,
    movement: DragonAction,
    dragon: Dragon,
}

#[derive(Bundle)]
struct FireDragonBundle {
    marker: FireDragon,
    #[bundle]
    dragon_bundle: DragonBundle,
}

#[derive(Bundle)]
struct IceDragonBundle {
    marker: IceDragon,
    #[bundle]
    dragon_bundle: DragonBundle,
}

#[derive(Component)]
struct Dragon;

#[derive(Component)]
struct FireDragon;

#[derive(Component)]
struct IceDragon;

#[derive(Component, Default)]
struct DragonInput {
    move_direction: Vec2,
    brake: bool,
    home: bool,
    ease_up: bool,
    fire: bool,
}


#[derive(Component)]
struct DragonAction {
    velocity: Vec3,
    max_velocity: f32,
    motion_timer: Timer,
    flip_timer: Timer,
    firerate_timer: Timer,
    flipping: bool,
}

#[derive(Component)]
struct Fireball;

#[derive(Bundle)]
struct FireballBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    movement: FireballMovement,
    fireball: Fireball,
}

#[derive(Component)]
struct FireballMovement {
    speed: Vec3,
    despawn_timer: Timer,
}

#[derive(Component)]
struct Wall;

#[derive(Resource)]
struct WallTextureHandle(Handle<Image>);

#[derive(Bundle)]
struct WallBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    wall: Wall,
}

#[derive(Component)]
pub struct GameCamera {
    pub threshold: f32,  // The threshold before the camera starts scrolling
    pub scale: f32,
}

#[derive(Resource)]
struct CameraScale(f32);


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
        .add_state::<GameState>()
        .insert_resource(CameraScale(1.0))
        .add_systems(  (preload_images,
                        setup_dragon, 
                        setup_camera,
                ).chain().on_startup()
        )
        .add_system(setup_maze.run_if(in_state(GameState::Setup)))
        .add_systems((
                keyboard_input_system.run_if(in_state(GameState::Running)), 
                dragon_movement_system.run_if(in_state(GameState::Running)), 
                camera_follow_system.run_if(in_state(GameState::Running)),
                fireball_spawn_system.run_if(in_state(GameState::Running)), 
                fireball_movement_system.run_if(in_state(GameState::Running))
            )
        )
        .add_system(close_on_esc)
        .run();
}

fn preload_images(mut commands: Commands, asset_server: Res<AssetServer>) {
    let wall_texture_handle = asset_server.load("wall.png");
    commands.insert_resource(WallTextureHandle(wall_texture_handle));
}


fn setup_dragon(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Setup Fire Dragon");
    // Spawn the Fire Dragon into the game.
    commands.spawn(FireDragonBundle {
        marker: FireDragon,
        dragon_bundle: DragonBundle {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("dragon.png"),
                transform: Transform::from_xyz(100., 0., 0.),
                ..default()
            },
            input: DragonInput::default(),
            movement: DragonAction {
                velocity: Vec3::ZERO,
                max_velocity: 25.0,
                motion_timer: Timer::from_seconds(0.05, TimerMode::Repeating),
                firerate_timer: Timer::from_seconds(0.15, TimerMode::Repeating),
                flip_timer: Timer::from_seconds(0.2, TimerMode::Once),
                flipping: false,
            },
            dragon: Dragon,
        }
    });


        // Spawn the Ice Dragon into the game.
    commands.spawn(IceDragonBundle {
        marker: IceDragon,
        dragon_bundle: DragonBundle {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("dragon.png"),
                transform: Transform::from_xyz(1200., 0., 0.),
                ..default()
            },
            input: DragonInput::default(),
            movement: DragonAction {
                velocity: Vec3::ZERO,
                max_velocity: 25.0,
                motion_timer: Timer::from_seconds(0.05, TimerMode::Repeating),
                firerate_timer: Timer::from_seconds(0.15, TimerMode::Repeating),
                flip_timer: Timer::from_seconds(0.2, TimerMode::Once),
                flipping: false,
            },
            dragon: Dragon,
        }
    });
}


fn setup_maze(
    mut commands: Commands,
//     asset_server: Res<AssetServer>,
    _state: ResMut<State<GameState>>,
    images: Res<Assets<Image>>,
    mut next_state: ResMut<NextState<GameState>>,
    wall_texture_handle: Res<WallTextureHandle>,
) {
    println!("Setup Maze");
//     let wall_handle = asset_server.load("wall.png");

    if let Some(wall_image) = images.get(&wall_texture_handle.0) {
        let wall_width = wall_image.size().x;
        let wall_height = wall_image.size().y;

        let mut maze = [
                [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1],
        ];

        maze.reverse();
        // Spawn Wall blocks into the game.
        for (i, row) in maze.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if *cell == 1 {
                    let x = (j as f32 * wall_width) - 1600.0;
                    let y = (i as f32 * wall_height) - 1000.0;
                    commands.spawn(WallBundle {
                        sprite_bundle: SpriteBundle {
                            texture: wall_texture_handle.0.clone(),
                            transform: Transform::from_xyz(x, y, -1.0),
                            ..default()
                        },
                        wall: Wall,
                    });
                }
            }
        }
        next_state.set(GameState::Running);
        // state .set(GameState::Running).unwrap();
    } else {
        println!("Setup Maze - Image not loaded yet...");
    }
}


fn setup_camera(
    mut commands: Commands,
//     query_window: Query<&Window>,
    camera_scale: Res<CameraScale>,
) {
    println!("Setup Camera");
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_scale(Vec3::splat(camera_scale.0)),
            ..default()
        },
        GameCamera {
            threshold: 250.0,
            scale: camera_scale.0,
        },
    ));
}

fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>, 
//     mut dragon_query: Query<&mut DragonInput>,
    mut dragon_query: Query<&mut DragonInput, With<FireDragon>>,
    mut camera_query: Query<(&mut Transform, &mut GameCamera), With<GameCamera>>,
) {
    let mut dragon_input = dragon_query.single_mut();
    dragon_input.move_direction = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
        dragon_input.move_direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
        dragon_input.move_direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        dragon_input.move_direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        dragon_input.move_direction.x += 1.0;
    }

    dragon_input.fire = keyboard_input.pressed(KeyCode::Space);
    dragon_input.brake = keyboard_input.pressed(KeyCode::LShift) || keyboard_input.pressed(KeyCode::RShift);
    dragon_input.home = keyboard_input.pressed(KeyCode::X);
    dragon_input.ease_up = !dragon_input.brake && !dragon_input.home && dragon_input.move_direction == Vec2::ZERO;
    

    let ctrl_pressed = keyboard_input.pressed(KeyCode::LControl) || keyboard_input.pressed(KeyCode::RControl);

    if ctrl_pressed {
        let mut scale_change = 1.0;
        if keyboard_input.pressed(KeyCode::Plus) || keyboard_input.pressed(KeyCode::Equals) {
            scale_change = 0.99;
        } else if keyboard_input.pressed(KeyCode::Minus) {
            scale_change = 1.01;
        }

        let (mut camera_transform, mut game_camera) = camera_query.single_mut();
        camera_transform.scale *= Vec3::splat(scale_change);
        game_camera.scale *= scale_change;
    }
}



fn camera_follow_system(
    time: Res<Time>,
    dragon_query: Query<(&Transform, &Handle<Image>, &DragonAction), (With<FireDragon>,Without<GameCamera>)>,
    mut camera_query: Query<(&mut Transform, &GameCamera), With<GameCamera>>,
    windows: Query<&Window>,
    images: Res<Assets<Image>>,
) {
    let window = windows.single();
    let (mut camera_transform, game_camera) = camera_query.single_mut();
    let (dragon_transform, dragon_handle, _dragon_action) = dragon_query.single();

    let dragon_image = images.get(dragon_handle).unwrap();
    let scaled_dragon_size = Vec2::new(dragon_image.size().x * dragon_transform.scale.x.abs(), dragon_image.size().y * dragon_transform.scale.y.abs());

    let dragon_left_edge = dragon_transform.translation.x - (scaled_dragon_size.x / 2.0);
    let dragon_right_edge = dragon_left_edge + scaled_dragon_size.x;

    let dragon_bottom_edge = dragon_transform.translation.y - (scaled_dragon_size.y / 2.0);
    let dragon_top_edge = dragon_bottom_edge + scaled_dragon_size.y;

    let scaled_window_width = window.width() * game_camera.scale;
    let scaled_window_height = window.height() * game_camera.scale;

    let window_left_edge = camera_transform.translation.x - (scaled_window_width / 2.0);
    let window_right_edge = window_left_edge + scaled_window_width;
    let window_bottom_edge = camera_transform.translation.y - (scaled_window_height / 2.0);
    let window_top_edge = window_bottom_edge + scaled_window_height;

    let margin = game_camera.threshold * game_camera.scale;

    let mut target_translation = camera_transform.translation;

    if dragon_left_edge < window_left_edge + margin {
        target_translation.x -= (dragon_left_edge - (window_left_edge + margin)).abs();
    } else if dragon_right_edge > window_right_edge - margin {
        target_translation.x += (dragon_right_edge - (window_right_edge - margin)).abs();
    }

    if dragon_bottom_edge < window_bottom_edge + margin {
        target_translation.y -= (dragon_bottom_edge - (window_bottom_edge + margin)).abs();
    } else if dragon_top_edge > window_top_edge - margin {
        target_translation.y += (dragon_top_edge - (window_top_edge - margin)).abs();
    }

    let lerp_rate = 7.0;
    camera_transform.translation = camera_transform.translation.lerp(target_translation, time.delta_seconds() * lerp_rate);
}


fn dragon_movement_system(
    time: Res<Time>,
    mut dragon_query: Query<(&mut DragonAction, &DragonInput, &mut Transform, &Handle<Image>)>,
    wall_query: Query<(&Wall, &Transform, &Handle<Image>), Without<DragonAction>>,
    images: Res<Assets<Image>>,
) {
    for (mut dragon_action, dragon_input, mut dragon_transform, dragon_image_handle) in dragon_query.iter_mut() {
        let acceleration = 0.4;

        dragon_action.velocity.x += dragon_input.move_direction.x * acceleration;
        dragon_action.velocity.y += dragon_input.move_direction.y * acceleration;

        // Brake if both LShift and RShift are pressed
        if dragon_input.brake && dragon_action.motion_timer.tick(time.delta()).just_finished() {
            dragon_action.velocity *= 0.6;
        }

        // Move to home position if X is pressed
        if dragon_input.home {
            dragon_action.velocity = Vec3::ZERO;
            dragon_transform.translation = Vec3::ZERO;
        } else if dragon_input.ease_up && dragon_action.motion_timer.tick(time.delta()).just_finished() {
            dragon_action.velocity *= 0.8;
        }

        // Check for collisions
        if let Some(dragon_image) = images.get(dragon_image_handle) {

            // Check for wall collisions
            let dragon_size = dragon_image.size().extend(0.0) * dragon_transform.scale.abs();
            let dragon_center_position = dragon_transform.translation;

            for (_, wall_transform, wall_image_handle) in wall_query.iter() {
                if let Some(wall_image) = images.get(wall_image_handle) {
                    let wall_size = wall_image.size().extend(0.0) * wall_transform.scale.abs();
                    let wall_center_position = wall_transform.translation;
                     // If the collision occurs on multiple sides, the side with the deepest penetration is returned.
                     // If all sides are involved, `Inside` is returned.
                    if let Some(collision) = collide(
                        dragon_center_position,
                        dragon_size.truncate(),
                        wall_center_position,
                        wall_size.truncate(),
                    ) {
                        dragon_action.velocity = Vec3::ZERO;
                        match collision {
                            Collision::Left => {
                                dragon_transform.translation.x = wall_center_position.x - (wall_size.x + dragon_size.x) / 2.0;
                                dragon_action.velocity.x = -0.0;
                            }
                            Collision::Right => {
                                dragon_transform.translation.x = wall_center_position.x + (wall_size.x + dragon_size.x) / 2.0;
                                dragon_action.velocity.x = 0.0;
                            }
                            Collision::Top => {
                                dragon_transform.translation.y = wall_center_position.y + (wall_size.y + dragon_size.y) / 2.0;
                                dragon_action.velocity.y = 0.0;
                            }
                            Collision::Bottom => {
                                dragon_transform.translation.y = wall_center_position.y - (wall_size.y + dragon_size.y) / 2.0;
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


fn fireball_spawn_system(
    time: Res<Time>,
    mut dragon_query: Query<(&mut DragonAction, &mut DragonInput, &Transform)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

    for (mut dragon_action, dragon_input, dragon_transform) in dragon_query.iter_mut(){
    
        if dragon_input.fire && dragon_action.firerate_timer.tick(time.delta()).just_finished() {
        
            // println!("fireball_spawn_system called");
            let mut fireball_direction = dragon_action.velocity.normalize_or_zero();
            if fireball_direction == Vec3::ZERO {
                    fireball_direction.x = 1.0 * dragon_transform.scale.x.signum();
            }

            // Calculate the speed of the fireball based on the dragon's velocity.
            let fireball_speed = fireball_direction * (250.0 + 75.0 * dragon_action.velocity.length());

            // Calculate the rotation of the fireball based on its velocity direction.
            let fireball_rotation = Quat::from_rotation_arc(Vec3::new(1.0,0.0,0.0), fireball_direction);

            // Spawn the fireball into the game.
            commands.spawn(FireballBundle {
                sprite_bundle: SpriteBundle {
                    texture: asset_server.load("fireball.png"),
                    transform: Transform {
                        translation: dragon_transform.translation + Vec3::new(110.0 * dragon_transform.scale.x.signum(), 30.0, 0.0),
                        rotation: fireball_rotation,
                        ..default()
                    },
                    ..default()
                },
                movement: FireballMovement { 
                    speed: fireball_speed,
                    despawn_timer: Timer::from_seconds(2.4, TimerMode::Once), // Set the timeout duration here
                },
                fireball: Fireball,
            });
        }
    }
}


fn fireball_movement_system(
        time: Res<Time>,
        mut commands: Commands,
        mut fireball_query: Query<(Entity, &mut FireballMovement, &mut Transform, &Handle<Image>),With<Fireball>>,
        wall_query: Query<(&Wall, &Transform, &Handle<Image>),Without<Fireball>>,
        images: Res<Assets<Image>>,
    ) {
    let delta_time = time.delta_seconds();
    for (   fireball_entity,
            mut fireball_movement, 
            mut fireball_transform,
            fireball_image_handle,
        ) in fireball_query.iter_mut() {

        // Move the fireball sprite by distance of speed * time.
        fireball_transform.translation += fireball_movement.speed * delta_time;

        // Update the despawn timer
        fireball_movement.despawn_timer.tick(time.delta());

        // Despawn the fireball if the timer has finished
        if fireball_movement.despawn_timer.finished() {
            commands.entity(fireball_entity).despawn();
        } else {
            // Check for collisions with walls
            if let Some(fireball_image) = images.get(fireball_image_handle) {
                let fireball_size = Vec2::new(fireball_image.size().x as f32, fireball_image.size().y as f32);
                for (_wall, wall_transform, wall_image_handle) in wall_query.iter() {
                
                    if let Some(wall_image) = images.get(wall_image_handle){
                        let wall_size = Vec2::new(wall_image.size().x as f32, wall_image.size().y as f32);

                        let collision = collide(
                            fireball_transform.translation,
                            fireball_size/5.0,
                            wall_transform.translation,
                            wall_size,
                        );

                        if let Some(_) = collision {
                            commands.entity(fireball_entity).despawn();
                            break;
                        }
                    }
                }
            }
        }
    }
}
