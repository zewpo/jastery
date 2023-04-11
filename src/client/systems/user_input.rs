
// use std::cmp::min;


use bevy::{input::{touch::*,mouse::*, ButtonState}, prelude::*};

use crate::client::components::*;
use crate::mutils;
use crate::shared::components::*;
use crate::shared::components::game::*;

use super::*;

pub struct UserInputPlugin;

impl Plugin for UserInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(TouchAssignments::default())
            .insert_resource(DoubleClickState::default())
            .add_systems((
                    keyboard_input_system,
                    mouse_input_system,
                    touch_input_system,
                )
                .distributive_run_if(|game_status: Res<GameStatus>| game_status.phase == GamePhase::Playing)
                .in_set(OnUpdate(AppScreen::InPlay))
            )
            ;
    }
}


//
pub fn mouse_input_system(
    windows: Query<&Window>,
    time: Res<Time>,
    mouse_button_input: Res<Input<MouseButton>>,

    // mut mouse_button_input_events: EventReader<MouseButtonInput>,
    // mut mouse_motion_events: EventReader<MouseMotion>,
    // mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_wheel_events: EventReader<MouseWheel>,

    mut double_click_status: ResMut<DoubleClickState>,
    mut dragon_query: Query<(&mut Dragon, &Transform), (With<MyDragon>, Without<GameCamera>, Without<VirtualJoystick>)>,
    mut camera_query: Query<(&mut GameCamera, &mut Transform), (With<GameCamera>, Without<VirtualJoystick>,Without<MyDragon>,)>,
) {
    let current_time = time.elapsed_seconds_f64();

    let window = windows.single();
    let (mut game_camera, mut camera_transform) = camera_query.single_mut();
    
    let (mut dragon, dragon_transform) = dragon_query.single_mut();
    let dragon_pos = dragon_transform.translation;



    for mouse_wheel_event in mouse_wheel_events.iter() {
        // info!("{:?}", event);
        if mouse_wheel_event.y != 0.0 {
            let zoom_factor = (100.0 + 5.0*mouse_wheel_event.y.signum())/100.0;
            game_camera.scale /= zoom_factor;
            camera_transform.scale = Vec3::splat(game_camera.scale);
            
        }
    }

    // let mut is_move = false;
    // let mut is_shoot = false;

//        let ctrl_pressed = keyboard_input.pressed(KeyCode::LControl) || keyboard_input.pressed(KeyCode::RControl);

    // for (mouse_button_event, _event_id) in mouse_button_input_events.iter_with_id() {
    //     let click_dir_to_dragon: Vec3;
    //     let cursor_position = window.cursor_position().unwrap_or_default();
    //     let cursor_position_world = game_camera.mouse_to_world(cursor_position, window, &camera_transform.translation);
    //     click_dir_to_dragon = (cursor_position_world - dragon_pos.truncate()).normalize_or_zero().extend(0.0);
    //     match mouse_button_event.state {
    //         ButtonState::Pressed => {
                
    //         },
    //         ButtonState::Released => {

    //         },
    //     }
    //     match mouse_button_event.button {
    //         MouseButton::Left => {
    //             dragon.input.shoot_direction = click_dir_to_dragon;
    //             dragon.input.shoot = true;
    //         },
    //         MouseButton::Right => {
    //             dragon.input.move_direction = click_dir_to_dragon;
    //             dragon.input.shoot = false;
    //         },
    //         MouseButton::Middle => {

    //         },
    //         MouseButton::Other(_) => {

    //         },
    //     }
    // }

    if mouse_button_input.pressed(MouseButton::Right) {
        let cursor_position = window.cursor_position().unwrap_or_default();
        let cursor_position_world = game_camera.mouse_to_world(cursor_position, window, &camera_transform.translation);
        let click_dir = (cursor_position_world - dragon_pos.truncate()).normalize_or_zero().extend(0.0);
        dragon.input.move_direction = click_dir;
        dragon.input.shoot = false;
    }

    //     //if mouse_button_input.just_released(MouseButton::Left) {
    //     if let Some(last_click_time) = double_click_status.last_click_time {
    //         if current_time - last_click_time <= double_click_status.max_delay {
    //             println!("Double-click detected!");
    //             double_click_status.last_click_time = None;

    //             let cursor_position = window.cursor_position().unwrap_or_default();
    //             let cursor_position_world = game_camera.screen_to_world(cursor_position, window, &camera_transform.translation);
    //             let mut click_dir = (cursor_position_world - dragon_pos.truncate()).normalize_or_zero().extend(0.0);
    //             click_dir.y *= -1.0;
    //             dragon.input.move_direction = click_dir;
    //             dragon.input.shoot = false;
    //             is_move = true;
    //         } else {
                
    //             if mouse_button_input.just_released(MouseButton::Left) {
    //                 println!("Single-click release detected!");
    //                 double_click_status.last_click_time = Some(current_time);    
    //             }
                
    //         }
    //     } else {
    //         if mouse_button_input.just_released(MouseButton::Left) {
    //             println!("Single-click release detected!");
    //             double_click_status.last_click_time = Some(current_time);
    //         }
    //     }

    
    if mouse_button_input.pressed(MouseButton::Left) {
        
        if let Some(cursor_position) = window.cursor_position(){
            let cursor_position_world = game_camera.mouse_to_world(cursor_position, window, &camera_transform.translation);
            let click_dir = (cursor_position_world - dragon_pos.truncate()).normalize_or_zero().extend(0.0);
            
            if let Some(last_release_time) = double_click_status.last_release_time {
                
                double_click_status.last_release_time = None;
                
                if double_click_status.still_pressed 
                    || current_time - last_release_time <= double_click_status.max_delay 
                {
                    // println!("Double-click detected!");
                    dragon.input.move_direction = click_dir;
                    dragon.input.shoot = false;
                    double_click_status.still_pressed = true;
                }

            } else if double_click_status.still_pressed {
                // println!("Double-click still pressed!");
                dragon.input.move_direction = click_dir;
                dragon.input.shoot = false;
                double_click_status.still_pressed = true;
            } else {
                dragon.input.shoot_direction = click_dir;
                dragon.input.shoot = true;
            }
        }
    }

    if mouse_button_input.just_released(MouseButton::Left) {
            double_click_status.last_release_time = Some(current_time);
            double_click_status.still_pressed = false;
    }

    // if mouse_button_input.pressed(MouseButton::Left) {
    //     // println!("Single-click press detected!");

    //     let cursor_position = window.cursor_position().unwrap_or_default();
    //     let cursor_position_world = game_camera.mouse_to_world(cursor_position, window, &camera_transform.translation);
    //     let click_dir = (cursor_position_world - dragon_pos.truncate()).normalize_or_zero().extend(0.0);
    //     println!("cursor_position: {:?},  Cursor_world: {:?}, Dragon: {:?}", cursor_position, cursor_position_world, dragon_pos.truncate() );
    //     dragon.input.shoot_direction = click_dir;
    //     dragon.input.shoot = true;

    // }

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

    if n_touches == 1 {
        let touch = touches_vec[0];
        if touch_assignments.shoot_touch_id != Some(touch.id()) {
            touch_assignments.shoot_touch_id = None;
        }
        else if touch_assignments.move_touch_id != Some(touch.id()) {
            touch_assignments.move_touch_id = None;
        }
    }



    // let mut using_joystick = false;
 
    if let Ok((mut virtual_joystick,virtual_joystick_style, virtual_joystick_transform)) = joystick_query.get_single_mut(){

        if n_touches == 0 || ( n_touches == 1 && touch_assignments.shoot_touch_id != None ) {
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
                    || ( touch_assignments.move_touch_id == None && touch_assignments.shoot_touch_id != Some(touch.id()) ) {

                    if touch_delta.length() < joystick_radius || !touches.just_pressed(touch.id()) {

                        if touches.just_released(touch.id()) || touches.just_cancelled(touch.id()) {
                            virtual_joystick.direction = Vec3::ZERO;
                            dragon.input.brake = true;
                            touch_assignments.move_touch_id = None;
                        } else {
                            touch_assignments.move_touch_id = Some(touch.id());
                            if touch_delta.length() < 0.1 * joystick_radius {
                                dragon.input.brake = true;
                                virtual_joystick.direction = Vec3::ZERO;
                            } else {
                                dragon.input.brake = false;
                                
                                let mut joystick_direction = touch_delta.normalize_or_zero().extend(0.0);
                                joystick_direction.y *= -1.0;  // flip Y from screen directions to world directions
                                let touch_distance_ratio = (touch_delta.length() / (1.5*joystick_radius)).clamp(0.0, 1.0);
                                let scale = 1.0 - (1.0 - touch_distance_ratio).powi(4);
                                joystick_direction *= scale;
                                virtual_joystick.direction = joystick_direction;
                            }
                        }
                    }
                    dragon.input.move_direction = virtual_joystick.direction;
                }
            }
        }
    }

    for touch in touches_vec.iter() {
        // Assign non-movement touch to shooting.
        if touch_assignments.shoot_touch_id.is_none() && touches.just_pressed(touch.id()) 
            && touch_assignments.move_touch_id != Some(touch.id()) {
            touch_assignments.shoot_touch_id = Some(touch.id());
        }

        // Handle Shooting Touch
        if touch_assignments.shoot_touch_id == Some(touch.id()) {
            // Handle touch release
            if touches.just_released(touch.id()) || touches.just_cancelled(touch.id()) {
                dragon.input.shoot_direction = Vec3::ZERO;
                dragon.input.shoot = false;
            } else {
                let touch_pos_screen = touch.position();
                let touch_pos_world = game_camera.touch_to_world(touch_pos_screen, window, &camera_transform.translation);
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
    mut game_status: ResMut<GameStatus>,
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

    // let ctrl_pressed = keyboard_input.pressed(KeyCode::LControl) || keyboard_input.pressed(KeyCode::RControl);

    // let +,- keys zoom the screen 
    // if ctrl_pressed  {
        let mut scale_change = 1.0;
        if keyboard_input.pressed(KeyCode::Plus) || keyboard_input.pressed(KeyCode::Equals) {
            scale_change = 0.99;
        } else if keyboard_input.pressed(KeyCode::Minus) || keyboard_input.pressed(KeyCode::Underline)  {
            scale_change = 1.01;
        }
        if scale_change != 1.0 {
            let (mut game_camera, mut camera_transform) = camera_query.single_mut();
            game_camera.scale *= scale_change;
            camera_transform.scale *= Vec3::splat(game_camera.scale);
            
        }
        
    // }


    if game_status.phase == GamePhase::Playing
        && keyboard_input.pressed(KeyCode::Escape)
    {
        game_status.phase = GamePhase::Paused;
    }
}

