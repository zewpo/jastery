use bevy::window::close_on_esc;
use bevy::prelude::*;

#[derive(PartialEq)]
enum MoveDirection {
    Stop,      // Immediatly sets velocity to 0.
    Up,        // increases y velocity.
    Down,      // decreases y velocity.
    Left,      // decreases x velocity.
    Right,     // increases x velocity.
    Brake,     // Quickly decelerate towards 0 velocity.
    Home,      // sets velocity to 0 and resets position to home.
    EaseUp,    // Gently decelerate towards 0 velocity.
}

#[derive(Component)]
struct Dragon {
    sprite_bundle: SpriteBundle,
    velocity: Vec3,
    time_last_velocity_changed: f32,
    move_direction: MoveDirection,
    max_velocity: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(keyboard_input_system)
        .add_system(dragon_movement_system)
        .add_system(close_on_esc)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    commands.spawn(Camera2dBundle::default());

    let dragon = Dragon {
        sprite_bundle: SpriteBundle {
            texture: asset_server.load("dragon.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
        velocity: Vec3::ZERO,
        time_last_velocity_changed: 0.,
        move_direction: MoveDirection::Stop,
        max_velocity: 50.0
    };

    commands.spawn((dragon.sprite_bundle.clone(), dragon));

}


fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    //time: Res<Time>,
    mut query: Query<&mut Dragon>,
) {
    for mut dragon in query.iter_mut() {

        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            dragon.move_direction = MoveDirection::Up;
            println!("Up");
            println!("{:?}", dragon.velocity);
        }

        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
        //     dragon.velocity.y -= 1.0;
            dragon.move_direction = MoveDirection::Down;
            println!("Down");
            println!("{:?}", dragon.velocity);
        }

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            dragon.move_direction = MoveDirection::Left;
            println!("Left");
            println!("{:?}", dragon.velocity);
        }

        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        //     dragon.velocity.x += 1.0;
            dragon.move_direction = MoveDirection::Right;
            println!("Right");
            println!("{:?}", dragon.velocity);
        }

        if keyboard_input.pressed(KeyCode::Space) {
                dragon.move_direction = MoveDirection::Brake;
            println!("Break");
            println!("{:?}", dragon.velocity);
        }

        if keyboard_input.pressed(KeyCode::X) {
            dragon.move_direction = MoveDirection::Home;
            println!("Home");
            println!("{:?}", dragon.velocity);
        }
    }
}

fn dragon_movement_system(time: Res<Time>, mut query: Query<(&mut Dragon, &mut Transform)>) {
    
    for (mut dragon, mut transform) in query.iter_mut() {
        match dragon.move_direction {
            MoveDirection::Stop => dragon.velocity = Vec3::ZERO,
            MoveDirection::Up => { 
                if dragon.velocity.y >= 0.4 {
                    dragon.velocity.y *= 1.2;
                } else if dragon.velocity.y <= -0.4 {
                    dragon.velocity.y /= 1.2;
                } else {
                    dragon.velocity.y += 0.2;
                }
            },
            MoveDirection::Down => { 
                if dragon.velocity.y >= 0.4 {
                    dragon.velocity.y /= 1.2;
                } else if dragon.velocity.y <= -0.4 {
                    dragon.velocity.y *= 1.2;
                } else {
                    dragon.velocity.y -= 0.2;
                } 
            },
            MoveDirection::Left => { 
                if dragon.velocity.x >= 0.4 {
                    dragon.velocity.x /= 1.2;
                } else if dragon.velocity.x <= -0.4 {
                    dragon.velocity.x *= 1.2;
                } else {
                    dragon.velocity.x -= 0.2;
                }
            },
            MoveDirection::Right => { 
                if dragon.velocity.x >= 0.4 {
                    dragon.velocity.x *= 1.2;
                } else if dragon.velocity.x <= -0.4 {
                    dragon.velocity.x /= 1.2;
                } else {
                    dragon.velocity.x += 0.2;
                }
            },
            MoveDirection::Brake => {
                if dragon.velocity.x < 0.01 && dragon.velocity.x > -0.01 {
                    dragon.velocity.x = 0.;
                } else {
                    dragon.velocity.x *= 0.2;
                }
                if dragon.velocity.y < 0.01 && dragon.velocity.y > -0.01 {
                    dragon.velocity.y = 0.;
                } else {
                    dragon.velocity.y *= 0.2;
                }
            },
            MoveDirection::Home => {
                dragon.velocity = Vec3::ZERO;
                transform.translation.x = 0.;
                transform.translation.y = 0.;
            },
            MoveDirection::EaseUp => {
                let d = time.elapsed_seconds() - dragon.time_last_velocity_changed;
                if d > 0.05 {
                    if dragon.velocity.x < 0.01 && dragon.velocity.x > -0.01 {
                        dragon.velocity.x = 0.;
                    } else {
                        dragon.velocity.x *= 0.9;
                    }
                    if dragon.velocity.y < 0.01 && dragon.velocity.y > -0.01 {
                        dragon.velocity.y = 0.;
                    } else {
                        dragon.velocity.y *= 0.9;
                    }
                    dragon.time_last_velocity_changed = time.elapsed_seconds();
                }
            }
        }
        if dragon.move_direction != MoveDirection::EaseUp {
            dragon.time_last_velocity_changed = time.elapsed_seconds();
            dragon.move_direction = MoveDirection::EaseUp;
        }

        // Clamp velocity
        dragon.velocity = dragon.velocity.clamp_length_max(dragon.max_velocity);

        if dragon.velocity != Vec3::ZERO {
            transform.translation += dragon.velocity; // * 200.0 * time.delta_seconds();
        }
    }
}