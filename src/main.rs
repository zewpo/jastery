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
        // #[bundle]
    sprite_bundle: SpriteBundle,
    movement: FireballMovement,
}

#[derive(Component)]
struct FireballMovement {
    speed: f32,
    despawn_timer: Timer,
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
            max_velocity: 15.0,
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
        for (dragon_movement, dragon_transform) in dragon_query.iter() {
            
            let mut speed_x: f32 = 70.0 + (20.0 * dragon_movement.velocity.x.abs());
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
                    despawn_timer: Timer::from_seconds(1.2, TimerMode::Once), // Set the timeout duration here
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
