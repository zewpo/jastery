use bevy::prelude::*;
use crate::{shared::components::dragon::*, client::components::game_camera::*};

pub fn keyboard_input_system (
    keyboard_input: Res<Input<KeyCode>>, 
//     mut dragon_query: Query<&mut DragonInput>,
    mut dragon_query: Query<&mut DragonInput, With<MyDragon>>,
    mut camera_query: Query<(&mut Transform, &mut GameCamera), With<GameCamera>>,
) {
    let mut dragon_input = dragon_query.single_mut();
    dragon_input.move_direction = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::Up) {
        dragon_input.move_direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        dragon_input.move_direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Left)  {
        dragon_input.move_direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        dragon_input.move_direction.x += 1.0;
    }

    dragon_input.fire = keyboard_input.pressed(KeyCode::Space);
    dragon_input.brake = keyboard_input.pressed(KeyCode::RShift);
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

