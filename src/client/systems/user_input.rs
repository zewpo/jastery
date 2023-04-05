
use bevy::{input::touch::*, prelude::*};
use crate::{shared::components::{dragon::*, game::GamePhase}, client::components::game_camera::*};

pub struct UserInputPlugin;

impl Plugin for UserInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(keyboard_input_system.in_set(OnUpdate(GamePhase::Playing)))
            .add_system(touch_input_system.in_set(OnUpdate(GamePhase::Playing)))
            ;
    }
}

pub fn touch_input_system(
    windows: Query<&Window>,
    touches: Res<Touches>,
    mut dragon_query: Query<(&mut Dragon, &Transform), (With<MyDragon>,Without<GameCamera>)>,
    mut camera_query: Query<(&mut GameCamera, &mut Transform), With<GameCamera>>,

) {
    let (mut dragon, dragon_transform) = dragon_query.single_mut();
    let dragon_pos = dragon_transform.translation;
    // dragon.input.move_direction = Vec3::ZERO;
    // dragon.input.fire_direction = Vec3::ZERO;

    let (mut game_camera, mut camera_transform) = camera_query.single_mut();
    let window = windows.single();


    
    let mut multi_touches = false;
    // let x = touches.iter().collect()::<Vec<_>>();
    let touches_vec = touches.iter().collect::<Vec<_>>();
    if touches_vec.len() >= 2 {
        multi_touches = true;
    }

    if multi_touches {
        let touch1_pos = touches_vec[0].position();
        let touch2_pos = touches_vec[1].position();

        let prev_touch1_pos = touches_vec[0].previous_position();
        let prev_touch2_pos = touches_vec[1].previous_position();

        let prev_distance = prev_touch1_pos.distance(prev_touch2_pos);
        let current_distance = touch1_pos.distance(touch2_pos);

        if prev_distance != 0.0 
            && current_distance != 0.0 
            && touches_vec[0].delta() != Vec2::ZERO 
            && touches_vec[1].delta() != Vec2::ZERO  {
            let zoom_factor = current_distance / prev_distance;
            game_camera.scale /= zoom_factor; // Modify scale directly
            camera_transform.scale = Vec3::splat(game_camera.scale);
        }

        if touches_vec[0].delta() == Vec2::ZERO 
           || touches_vec[1].delta() == Vec2::ZERO {

            let move_touch: &Touch;
            let fire_touch: &Touch;
            if touches_vec[0].delta() != Vec2::ZERO {
                move_touch = touches_vec[1];
                fire_touch = touches_vec[0];
            } else {
                move_touch = touches_vec[0];
                fire_touch = touches_vec[1];
            };
            let touch_pos_screen = move_touch.position();
            let touch_pos_world = game_camera.screen_to_world(touch_pos_screen, window, &camera_transform.translation);
            let move_dir = (touch_pos_world - dragon_pos.truncate()).normalize_or_zero().extend(0.0);
            dragon.input.move_direction = move_dir;

            let touch_pos_screen = fire_touch.position();
            let touch_pos_world = game_camera.screen_to_world(touch_pos_screen, window, &camera_transform.translation);
            let fire_dir = (touch_pos_world - dragon_pos.truncate()).normalize_or_zero().extend(0.0);
            dragon.input.fire_direction = fire_dir;
            dragon.input.fire = true;
          }
        
    } else {
        // Handle touch input here
        for touch in touches.iter() {
            if touches.just_pressed(touch.id()) {
                dragon.input.brake = false;
                // Handle touch start (ie., start moving towards this position)
                let touch_pos_screen = touch.position();
                let touch_pos_world = game_camera.screen_to_world(touch_pos_screen, window, &camera_transform.translation);
                let move_dir = (touch_pos_world - dragon_pos.truncate()).normalize_or_zero().extend(0.0);
                dragon.input.move_direction = move_dir; //(touch_pos_world - dragon_pos.truncate()).normalize_or_zero().extend(0.0);
            }
            else if touch.delta() != Vec2::ZERO {
                // Handle touch move (ie., start firing in this direction)
                let touch_pos_screen = touch.position();
                let touch_pos_world = game_camera.screen_to_world(touch_pos_screen, window, &camera_transform.translation);
                let fire_dir = (touch_pos_world - dragon_pos.truncate()).normalize_or_zero().extend(0.0);
                dragon.input.fire_direction = fire_dir;
                dragon.input.fire = true;
                
            }
            else if touches.just_released(touch.id()) || touches.just_cancelled(touch.id()) {
                dragon.input.fire = false;
                dragon.input.brake = true;
            } else {
                let touch_pos_screen = touch.position();
                let touch_pos_world = game_camera.screen_to_world(touch_pos_screen, window, &camera_transform.translation);
                let move_dir = (touch_pos_world - dragon_pos.truncate()).normalize_or_zero().extend(0.0);
                dragon.input.move_direction = move_dir;
            }
        }
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
    

    let mut wasd_movement = false;
    if  keyboard_input.pressed(KeyCode::W)
      || keyboard_input.pressed(KeyCode::A)
      || keyboard_input.pressed(KeyCode::S)
      || keyboard_input.pressed(KeyCode::D) {
        wasd_movement = true;
    }

    if keyboard_input.pressed(KeyCode::W) 
        || ( !wasd_movement && keyboard_input.pressed(KeyCode::Up) ) {
        dragon.input.move_direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) 
        || ( !wasd_movement && keyboard_input.pressed(KeyCode::Down) ) {
        dragon.input.move_direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::A)  
        || ( !wasd_movement && keyboard_input.pressed(KeyCode::Left) ) {
        dragon.input.move_direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D)  
        || ( !wasd_movement && keyboard_input.pressed(KeyCode::Right) ) {
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

