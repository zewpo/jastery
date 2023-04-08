
// use std::cmp::min;

use bevy::{input::touch::*, prelude::*};

use crate::client::components::*;
use crate::mutils;
use crate::shared::components::*;
use crate::shared::components::game::*;

pub struct UserInputPlugin;

impl Plugin for UserInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(TouchAssignments::default())
            .add_system(keyboard_input_system.in_set(OnUpdate(GamePhase::Playing)))
            .add_system(touch_input_system.in_set(OnUpdate(GamePhase::Playing)))
            ;
    }
}


pub fn touch_input_system(
    windows: Query<&Window>,
    touches: Res<Touches>,
    mut dragon_query: Query<(&mut Dragon, &Transform), (With<MyDragon>, Without<GameCamera>, Without<VirtualJoystick>)>,
    mut camera_query: Query<(&mut GameCamera, &mut Transform), (With<GameCamera>, Without<VirtualJoystick>,Without<MyDragon>,)>,
    mut touch_assignments: ResMut<TouchAssignments>,
    mut joystick_query: Query<(&mut VirtualJoystick, &Style, &mut Transform),(With<VirtualJoystick>,Without<MyDragon>, Without<GameCamera>)>,
) {
    let (mut dragon, dragon_transform) = dragon_query.single_mut();
    let dragon_pos = dragon_transform.translation;

    let (mut game_camera, mut camera_transform) = camera_query.single_mut();
    let window = windows.single();
    
    let touches_vec = touches.iter().collect::<Vec<_>>();
    let n_touches = touches_vec.len();
    if n_touches == 0 {
        touch_assignments.move_touch_id = None;
        touch_assignments.shoot_touch_id = None;
        // joystick_direction = Vec3::ZERO;
    }

    // let mut using_joystick = false;
 
    if let Ok((mut virtual_joystick,virtual_joystick_style, virtual_joystick_transform)) = joystick_query.get_single_mut(){

        if n_touches == 0 {
            virtual_joystick.direction = Vec3::ZERO;
            virtual_joystick.center = virtual_joystick_transform.translation.truncate();
        }
        else {
            let joystick_size = mutils::size_to_vec2(virtual_joystick_style.size);
            let joystick_diameter = (joystick_size.x + joystick_size.y)/2.0;
            let joystick_radius = joystick_diameter / 2.0;

            for touch in touches_vec.iter() {
                let touch_delta = touch.position() - virtual_joystick.center;
                // dont worry about touches that are assigned to firing.
                if touch_assignments.move_touch_id == Some(touch.id())
                    || (touch_assignments.move_touch_id == None && (touch_assignments.shoot_touch_id != Some(touch.id()) || touch_assignments.shoot_touch_id == None )) {
                    
                    
                    touch_assignments.move_touch_id = Some(touch.id());
                    if touch_delta.length() < joystick_radius || !touches.just_pressed(touch.id()) {
                        if touch_delta.length() < 0.1 * joystick_radius {
                            dragon.input.brake = true;
                            virtual_joystick.direction = Vec3::ZERO;
                        } else {
                            dragon.input.brake = false;
                            
                            let mut joystick_direction = touch_delta.normalize_or_zero().extend(0.0);
                            joystick_direction.y *= -1.0;  // flip Y from screen directions to world directions
                            // joystick_direction = mutils::vec3_round(joystick_direction);

                            let touch_distance_ratio = (touch_delta.length() / joystick_radius).clamp(0.0, 1.0);
                            let scale = 1.0 - (1.0 - touch_distance_ratio).powi(2);

                            joystick_direction *= scale;
                            virtual_joystick.direction = joystick_direction;
                        }
                    }
                    dragon.input.move_direction = virtual_joystick.direction;
                }
            }
        }
    }

    for touch in touches_vec.iter() {
        // //Assign touch to movment, if first touch.
        // if touch_assignments.move_touch_id.is_none() && touches.just_pressed(touch.id()) {
        //     touch_assignments.move_touch_id = Some(touch.id());
        // }

        // Assign non-movement touch to shooting.
        if touch_assignments.shoot_touch_id.is_none() && touches.just_pressed(touch.id()) 
            && touch_assignments.move_touch_id != Some(touch.id()) {
            touch_assignments.shoot_touch_id = Some(touch.id());
        }

        // // Handle Movement Touch
        // if touch_assignments.move_touch_id == Some(touch.id()) {
        //     if touches.just_released(touch.id()) || touches.just_cancelled(touch.id()) {
        //         dragon.input.move_direction = Vec3::ZERO;
        //         dragon.input.brake = true;
        //     }
        //     else {
        //         // if using_joystick {
        //             dragon.input.move_direction = joystick_direction;
        //             dragon.input.brake = false;
        //         // } 
        //         // else {
        //         //     let touch_pos_screen = touch.position();
        //         //     let touch_pos_world = game_camera.screen_to_world(touch_pos_screen, window, &camera_transform.translation);
        //         //     let move_dir = (touch_pos_world - dragon_pos.truncate()).normalize_or_zero().extend(0.0);
        //         //     dragon.input.move_direction = move_dir;
        //         // }
        //     }
        // }

        // // Handle Movement Touch
        // if touch_assignments.move_touch_id == Some(touch.id()) {
        //     if touches.just_released(touch.id()) || touches.just_cancelled(touch.id()) {
        //         dragon.input.move_direction = Vec3::ZERO;
        //         dragon.input.brake = true;
        //     }
        //     else {
        //         if using_joystick {
        //             dragon.input.move_direction = joystick_direction;
        //         } else {
        //             let touch_pos_screen = touch.position();
        //             let touch_pos_world = game_camera.screen_to_world(touch_pos_screen, window, &camera_transform.translation);
        //             let move_dir = (touch_pos_world - dragon_pos.truncate()).normalize_or_zero().extend(0.0);
        //             dragon.input.move_direction = move_dir;
        //         }
        //     }
        // }

        // Handle Shooting Touch
        if touch_assignments.shoot_touch_id == Some(touch.id()) {
            // Handle touch release
            if touches.just_released(touch.id()) || touches.just_cancelled(touch.id()) {
                dragon.input.shoot_direction = Vec3::ZERO;
                dragon.input.shoot = false;
            } else {
                let touch_pos_screen = touch.position();
                let touch_pos_world = game_camera.screen_to_world(touch_pos_screen, window, &camera_transform.translation);
                let shoot_dir = (touch_pos_world - dragon_pos.truncate()).normalize_or_zero().extend(0.0);
                dragon.input.shoot_direction = shoot_dir;
                dragon.input.shoot = true;
            }
        }
    }

    // Calculate Pinch to Zoom
    if touches_vec.len() >= 2 {
        let touch1 = touches_vec[0];
        let touch2 = touches_vec[1];
        let touch1_pos = touch1.position();
        let touch2_pos = touch2.position();

        let prev_touch1_pos = touch1.previous_position();
        let prev_touch2_pos = touch2.previous_position();

        let prev_distance = prev_touch1_pos.distance(prev_touch2_pos);
        let current_distance = touch1_pos.distance(touch2_pos);
        
        // if both touches are moving, and the distance is changing
        if prev_distance != 0.0 && current_distance != 0.0 
        && touch1.delta().length() > 1.0 && touch2.delta().length() > 1.0 {
            let zoom_factor = current_distance / prev_distance;
            game_camera.scale /= zoom_factor; // Modify scale directly
            camera_transform.scale = Vec3::splat(game_camera.scale);
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
    dragon.input.shoot_direction = Vec3::ZERO;
    

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
        dragon.input.shoot_direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        dragon.input.shoot_direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Left)  {
        dragon.input.shoot_direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        dragon.input.shoot_direction.x += 1.0;
    }

    dragon.input.shoot = keyboard_input.pressed(KeyCode::Space);
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

