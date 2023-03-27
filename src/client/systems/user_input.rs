use bevy::prelude::*;
use crate::{shared::components::{dragon::*, game::GamePhase}, client::components::game_camera::*};

pub struct UserInputPlugin;

impl Plugin for UserInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(keyboard_input_system.in_set(OnUpdate(GamePhase::Playing)));
    }
}

pub fn keyboard_input_system (
    keyboard_input: Res<Input<KeyCode>>, 
    mut dragon_query: Query<&mut Dragon, With<MyDragon>>,
    mut camera_query: Query<(&mut GameCamera, &mut Transform), With<GameCamera>>,
) {
    let mut dragon = dragon_query.single_mut();
    dragon.input.move_direction = Vec3::ZERO;
    dragon.input.fire_direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::W) {
        dragon.input.move_direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        dragon.input.move_direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::A)  {
        dragon.input.move_direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        dragon.input.move_direction.x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        dragon.input.fire_direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        dragon.input.fire_direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Left)  {
        dragon.input.fire_direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        dragon.input.fire_direction.x += 1.0;
    }

    dragon.input.fire = keyboard_input.pressed(KeyCode::Space);
    dragon.input.brake = keyboard_input.pressed(KeyCode::RShift);
    dragon.input.home = keyboard_input.pressed(KeyCode::X);

    let ctrl_pressed = keyboard_input.pressed(KeyCode::LControl) || keyboard_input.pressed(KeyCode::RControl);

    if ctrl_pressed {
        let mut scale_change = 1.0;
        if keyboard_input.pressed(KeyCode::Plus) || keyboard_input.pressed(KeyCode::Equals) {
            scale_change = 0.99;
        } else if keyboard_input.pressed(KeyCode::Minus) {
            scale_change = 1.01;
        }

        let (mut game_camera, mut camera_transform) = camera_query.single_mut();
        camera_transform.scale *= Vec3::splat(scale_change);
        game_camera.scale *= scale_change;
    }
}

