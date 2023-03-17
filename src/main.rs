use bevy::window::close_on_esc;
use bevy::window::PresentMode;
use bevy::prelude::*;

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
        .add_system(fireball_spawn_system)
        .add_system(fireball_movement_system)
        .add_system(close_on_esc)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Spawn the Dragon into the game.
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("dragon.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
        DragonInput {
            move_direction: MoveDirection::Stop,
        },
        DragonMovement {
            velocity: Vec3::ZERO,
            max_velocity: 20.0,
            timer: Timer::from_seconds(0.05, TimerMode::Repeating),
        },
    ));
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
    mut query: Query<(&mut DragonMovement, &DragonInput, &mut Transform)>,
) {
    for (mut dragon_movement, dragon_input, mut transform) in query.iter_mut() {
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
                transform.translation = Vec3::ZERO;
            }
            MoveDirection::EaseUp => {
                if dragon_movement.timer.tick(time.delta()).just_finished() {
                dragon_movement.velocity *= 0.8;
                }
            }
        }

        // Move the sprite.
        if dragon_movement.velocity != Vec3::ZERO {
            transform.translation += dragon_movement.velocity.clamp_length_max(dragon_movement.max_velocity);
        }

        // Display direction.  Assume the sprite is normallay facing to the right.
        if dragon_movement.velocity.x > 0.0 && transform.scale.x < 0.0 {
            // Moving right, set the X scale to positive (normal)
            transform.scale.x = transform.scale.x.abs();
        } else if dragon_movement.velocity.x < 0.0 && transform.scale.x > 0.0 {
            // Moving left, set the X scale to negative (flipped)
            transform.scale.x = -transform.scale.x.abs();
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
        for (_,dragon_transform) in dragon_query.iter() {

            // Spawn a fireball into the game.
            commands.spawn(Fireball {
                sprite_bundle: SpriteBundle {
                    texture: asset_server.load("fireball.png"),
                    transform: Transform {
                        translation: dragon_transform.translation + Vec3::new(50.0, 0.0, 0.0),
                        ..default()
                    },
                    ..default()
                },
                movement: FireballMovement { speed: (500.0) } ,
            });
        }
    }
}

fn fireball_movement_system(
        time: Res<Time>,
        mut commands: Commands,
        mut fireball_query: Query<(&mut FireballMovement, &mut Transform)>,
        window_query: Query<&Window>,
    ) {
    let delta_time = time.delta_seconds();
    
    let window_width = window_query.single().width();
    for (fireball_movement, mut fireball_transform) in fireball_query.iter_mut() {
        fireball_transform.translation.x += fireball_movement.speed * delta_time;

        // Despawn the fireball if it has gone further than one-fifth the window width.
        if fireball_transform.translation.x > window_width/5.0 {
        //     commands.entity(entity).despawn();
        }
    }
}



// use bevy::window::close_on_esc;
// use bevy::prelude::*;

// #[derive(PartialEq)]
// enum MoveDirection {
//     Stop,      // Immediatly sets velocity to 0.
//     Up,        // increases y velocity.
//     Down,      // decreases y velocity.
//     Left,      // decreases x velocity.
//     Right,     // increases x velocity.
//     Brake,     // Quickly decelerate towards 0 velocity.
//     Home,      // sets velocity to 0 and resets position to home.
//     EaseUp,    // Gently decelerate towards 0 velocity.
// }

// #[derive(Component)]
// struct Dragon {
//     sprite_bundle: SpriteBundle,
//     velocity: Vec3,
//     time_last_velocity_changed: f32,
//     move_direction: MoveDirection,
//     max_velocity: f32,
// }

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_startup_system(setup)
//         .add_system(keyboard_input_system)
//         .add_system(dragon_movement_system)
//         .add_system(close_on_esc)
//         .run();
// }

// fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

//     commands.spawn(Camera2dBundle::default());

//     let dragon = Dragon {
//         sprite_bundle: SpriteBundle {
//             texture: asset_server.load("dragon.png"),
//             transform: Transform::from_xyz(100., 0., 0.),
//             ..default()
//         },
//         velocity: Vec3::ZERO,
//         time_last_velocity_changed: 0.,
//         move_direction: MoveDirection::Stop,
//         max_velocity: 50.0
//     };

//     commands.spawn((dragon.sprite_bundle.clone(), dragon));

// }


// fn keyboard_input_system(
//     keyboard_input: Res<Input<KeyCode>>,
//     //time: Res<Time>,
//     mut query: Query<&mut Dragon>,
// ) {
//     for mut dragon in query.iter_mut() {

//         if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
//             dragon.move_direction = MoveDirection::Up;
//             println!("Up");
//             println!("{:?}", dragon.velocity);
//         }

//         if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
//         //     dragon.velocity.y -= 1.0;
//             dragon.move_direction = MoveDirection::Down;
//             println!("Down");
//             println!("{:?}", dragon.velocity);
//         }

//         if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
//             dragon.move_direction = MoveDirection::Left;
//             println!("Left");
//             println!("{:?}", dragon.velocity);
//         }

//         if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
//         //     dragon.velocity.x += 1.0;
//             dragon.move_direction = MoveDirection::Right;
//             println!("Right");
//             println!("{:?}", dragon.velocity);
//         }

//         if keyboard_input.pressed(KeyCode::Space) {
//                 dragon.move_direction = MoveDirection::Brake;
//             println!("Break");
//             println!("{:?}", dragon.velocity);
//         }

//         if keyboard_input.pressed(KeyCode::X) {
//             dragon.move_direction = MoveDirection::Home;
//             println!("Home");
//             println!("{:?}", dragon.velocity);
//         }
//     }
// }

// fn dragon_movement_system(time: Res<Time>, mut query: Query<(&mut Dragon, &mut Transform)>) {
    
//     for (mut dragon, mut transform) in query.iter_mut() {
//         match dragon.move_direction {
//             MoveDirection::Stop => dragon.velocity = Vec3::ZERO,
//             MoveDirection::Up => { 
//                 if dragon.velocity.y >= 0.4 {
//                     dragon.velocity.y *= 1.2;
//                 } else if dragon.velocity.y <= -0.4 {
//                     dragon.velocity.y /= 1.2;
//                 } else {
//                     dragon.velocity.y += 0.2;
//                 }
//             },
//             MoveDirection::Down => { 
//                 if dragon.velocity.y >= 0.4 {
//                     dragon.velocity.y /= 1.2;
//                 } else if dragon.velocity.y <= -0.4 {
//                     dragon.velocity.y *= 1.2;
//                 } else {
//                     dragon.velocity.y -= 0.2;
//                 } 
//             },
//             MoveDirection::Left => { 
//                 if dragon.velocity.x >= 0.4 {
//                     dragon.velocity.x /= 1.2;
//                 } else if dragon.velocity.x <= -0.4 {
//                     dragon.velocity.x *= 1.2;
//                 } else {
//                     dragon.velocity.x -= 0.2;
//                 }
//             },
//             MoveDirection::Right => { 
//                 if dragon.velocity.x >= 0.4 {
//                     dragon.velocity.x *= 1.2;
//                 } else if dragon.velocity.x <= -0.4 {
//                     dragon.velocity.x /= 1.2;
//                 } else {
//                     dragon.velocity.x += 0.2;
//                 }
//             },
//             MoveDirection::Brake => {
//                 if dragon.velocity.x < 0.01 && dragon.velocity.x > -0.01 {
//                     dragon.velocity.x = 0.;
//                 } else {
//                     dragon.velocity.x *= 0.2;
//                 }
//                 if dragon.velocity.y < 0.01 && dragon.velocity.y > -0.01 {
//                     dragon.velocity.y = 0.;
//                 } else {
//                     dragon.velocity.y *= 0.2;
//                 }
//             },
//             MoveDirection::Home => {
//                 dragon.velocity = Vec3::ZERO;
//                 transform.translation.x = 0.;
//                 transform.translation.y = 0.;
//             },
//             MoveDirection::EaseUp => {
//                 let d = time.elapsed_seconds() - dragon.time_last_velocity_changed;
//                 if d > 0.05 {
//                     if dragon.velocity.x < 0.01 && dragon.velocity.x > -0.01 {
//                         dragon.velocity.x = 0.;
//                     } else {
//                         dragon.velocity.x *= 0.9;
//                     }
//                     if dragon.velocity.y < 0.01 && dragon.velocity.y > -0.01 {
//                         dragon.velocity.y = 0.;
//                     } else {
//                         dragon.velocity.y *= 0.9;
//                     }
//                     dragon.time_last_velocity_changed = time.elapsed_seconds();
//                 }
//             }
//         }
//         if dragon.move_direction != MoveDirection::EaseUp {
//             dragon.time_last_velocity_changed = time.elapsed_seconds();
//             dragon.move_direction = MoveDirection::EaseUp;
//         }

//         // Clamp velocity
//         dragon.velocity = dragon.velocity.clamp_length_max(dragon.max_velocity);

//         if dragon.velocity != Vec3::ZERO {
//             transform.translation += dragon.velocity; // * 200.0 * time.delta_seconds();
//         }
//     }
// }