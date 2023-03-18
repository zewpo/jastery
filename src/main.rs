use bevy::sprite::collide_aabb::Collision;
use bevy::window::close_on_esc;
use bevy::window::PresentMode;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;


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
    movement: DragonMovement,
    dragon: Dragon,
}

#[derive(Component)]
struct Dragon;


#[derive(Component)]
struct DragonInput {
    move_direction: Vec2,
    brake: bool,
    home: bool,
    ease_up: bool,
}

impl Default for DragonInput {
    fn default() -> Self {
        Self {
            move_direction: Vec2::ZERO,
            brake: false,
            home: false,
            ease_up: false,
        }
    }
}


#[derive(Component)]
struct DragonMovement {
    velocity: Vec3,
    max_velocity: f32,
    timer: Timer,
    flip_timer: Timer,
    flipping: bool,
}

#[derive(Bundle)]
struct Fireball {
    #[bundle]
    sprite_bundle: SpriteBundle,
    movement: FireballMovement,
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
        .add_systems((
                setup_dragon, 
                setup_camera,
            ).chain().on_startup() //  in_set(OnUpdate(GameState::Setup))
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


fn setup_dragon(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Setup Dragon");
    // Spawn the Dragon into the game.
    commands.spawn(DragonBundle {
        sprite_bundle: SpriteBundle {
            texture: asset_server.load("dragon.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
        input: DragonInput::default(),
        movement: DragonMovement {
            velocity: Vec3::ZERO,
            max_velocity: 25.0,
            timer: Timer::from_seconds(0.05, TimerMode::Repeating),
            flip_timer: Timer::from_seconds(0.2, TimerMode::Once),
            flipping: false,
        },
        dragon: Dragon,
    });

    let wall_texture_handle = asset_server.load("wall.png");
    commands.insert_resource(WallTextureHandle(wall_texture_handle));
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
                            transform: Transform::from_xyz(x, y, 0.),
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
    mut dragon_query: Query<&mut DragonInput>,
    mut camera_query: Query<(&mut Transform, &mut GameCamera), With<GameCamera>>,
) {
    for mut dragon_input in dragon_query.iter_mut() {
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
        
        dragon_input.brake = keyboard_input.pressed(KeyCode::LShift) || keyboard_input.pressed(KeyCode::RShift);
        dragon_input.home = keyboard_input.pressed(KeyCode::X);
        dragon_input.ease_up = !dragon_input.brake && !dragon_input.home && dragon_input.move_direction == Vec2::ZERO;
    }

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
    dragon_query: Query<(&Transform, &Handle<Image>, &DragonMovement), Without<GameCamera>>,
    mut camera_query: Query<(&mut Transform, &GameCamera), With<GameCamera>>,
    windows: Query<&Window>,
    images: Res<Assets<Image>>,
) {
    let window = windows.single();
    let (mut camera_transform, game_camera) = camera_query.single_mut();
    let (dragon_transform, dragon_handle, _dragon_movement) = dragon_query.single();

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

    let lerp_speed = 7.0;
    camera_transform.translation = camera_transform.translation.lerp(target_translation, time.delta_seconds() * lerp_speed);
}


fn dragon_movement_system(
    time: Res<Time>,
    mut dragon_query: Query<(&mut DragonMovement, &DragonInput, &mut Transform, &Handle<Image>)>,
    wall_query: Query<(&Wall, &Transform, &Handle<Image>), Without<DragonMovement>>,
    images: Res<Assets<Image>>,
) {
    for (mut dragon_movement, dragon_input, mut dragon_transform, dragon_image_handle) in dragon_query.iter_mut() {
        let acceleration = 0.4;

        dragon_movement.velocity.x += dragon_input.move_direction.x * acceleration;
        dragon_movement.velocity.y += dragon_input.move_direction.y * acceleration;

        // Brake if both LShift and RShift are pressed
        if dragon_input.brake && dragon_movement.timer.tick(time.delta()).just_finished() {
            dragon_movement.velocity *= 0.6;
        }

        // Move to home position if X is pressed
        if dragon_input.home {
            dragon_movement.velocity = Vec3::ZERO;
            dragon_transform.translation = Vec3::ZERO;
        } else if dragon_input.ease_up && dragon_movement.timer.tick(time.delta()).just_finished() {
            dragon_movement.velocity *= 0.8;
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
                        dragon_movement.velocity = Vec3::ZERO;
                        match collision {
                            Collision::Left => {
                                dragon_transform.translation.x = wall_center_position.x - (wall_size.x + dragon_size.x) / 2.0;
                                dragon_movement.velocity.x = -1.0;
                            }
                            Collision::Right => {
                                dragon_transform.translation.x = wall_center_position.x + (wall_size.x + dragon_size.x) / 2.0;
                                dragon_movement.velocity.x = 1.0;
                            }
                            Collision::Top => {
                                dragon_transform.translation.y = wall_center_position.y + (wall_size.y + dragon_size.y) / 2.0;
                                dragon_movement.velocity.y = 1.0;
                            }
                            Collision::Bottom => {
                                dragon_transform.translation.y = wall_center_position.y - (wall_size.y + dragon_size.y) / 2.0;
                                dragon_movement.velocity.y = -1.0;
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
        if dragon_movement.velocity != Vec3::ZERO {
            dragon_movement.velocity = dragon_movement.velocity.clamp_length_max(dragon_movement.max_velocity);
            dragon_transform.translation += dragon_movement.velocity;
        }

        if dragon_movement.flipping {
            if dragon_movement.flip_timer.tick(time.delta()).just_finished() {
            // Finish the flipping animation.
                dragon_movement.flipping = false;
                if dragon_transform.scale.x < 0.0{
                    dragon_transform.scale.x = 1.0;
                } else {
                    dragon_transform.scale.x = -1.0;
                }
            } else {
                // Continue the flipping animation.
                let progress = dragon_movement.flip_timer.percent();
                dragon_transform.scale.x = dragon_transform.scale.x.signum() * (0.5 - 0.5 * (progress * std::f32::consts::PI).sin());
            }
        } else if (dragon_movement.velocity.x > 0.0 && dragon_transform.scale.x < 0.0) || (dragon_movement.velocity.x < 0.0 && dragon_transform.scale.x > 0.0) {
            // Start the flipping animation.
            dragon_movement.flip_timer.reset();
            dragon_movement.flipping = true;
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
            
            let mut scale_x: f32 = 1.0;
            let mut trans_x: f32 = 120.0;

            if dragon_transform.scale.x < 0.0 { 
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
                    speed: 5.0 + (dragon_movement.velocity * 3.0),
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

        // Move the fireball sprite by distance of speed * time.
        fireball_transform.translation.x += fireball_movement.speed.x * delta_time;
        fireball_transform.translation.y += fireball_movement.speed.y * delta_time;
        fireball_transform.translation.z += fireball_movement.speed.z * delta_time;

        // Update the despawn timer
        fireball_movement.despawn_timer.tick(time.delta());

        // Despawn the fireball if the timer has finished
        if fireball_movement.despawn_timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}
