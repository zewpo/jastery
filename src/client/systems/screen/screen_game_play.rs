// src\client\systems\screen_game_play.rs

use bevy::prelude::*;
use crate::mutils;
use crate::shared::components::*;
use crate::client::components::*;
use crate::client::systems::screen::*;




// // hack for bug in bevy, fix due in version 0.11
// fn ensure_state_schedules<S: States>(app: &mut App) {
//     for variant in S::variants() {
//         let on_enter_label = OnEnter(variant.clone());
//         let on_exit_label = OnExit(variant.clone());

//         if app.get_schedule(on_enter_label.clone()).is_none() {
//             app.add_schedule(on_enter_label, Schedule::new());
//         }

//         if app.get_schedule(on_exit_label.clone()).is_none() {
//             app.add_schedule(on_exit_label, Schedule::new());
//         }
//     }
// }

pub struct GamePlayScreenPlugin;
impl Plugin for GamePlayScreenPlugin {

    fn build(&self, app: &mut App) {

        // ensure_state_schedules::<AppScreen>(app);

        app
            // .add_state::<AppScreen>()

            .add_systems( (spawn_virtual_joystick,
                           spawn_dragon_status_text,
                        //    spawn_screen_game_play,
                        )
                        .distributive_run_if(|game_status: Res<GameStatus>| game_status.phase == GamePhase::Setup)
                        .in_schedule(OnEnter(AppScreen::InPlay)))
            .add_systems( (virtual_joystick_watcher,
                           dragon_status_watcher
                        )
                        .distributive_run_if(|game_status: Res<GameStatus>| game_status.phase == GamePhase::Playing)
                        .in_set(OnUpdate(AppScreen::InPlay)) )
            .add_system(
                game_event_watcher
                .in_set(OnUpdate(AppScreen::InPlay))
            ) 
            
            // .add_system( cleanup_game.in_schedule(OnExit(AppScreen::InPlay)) );
            // .add_system( game_event_watcher.in_schedule(OnExit(GamePhase::Playing)) )
            ;
    }
}



// fn spawn_screen_game_play(
//     // mut commands: Commands,
//     // app_screen: Res<State<AppScreen>>, 
//     // mut next_app_screen: ResMut<NextState<AppScreen>>,
//     mut game_status: ResMut<GameStatus>,
//     // resource_cache: Res<ResourceCache>,
// ){
//     println!("setup_game_play_screen");
//     game_status.phase = GamePhase::Setup;
// }


fn spawn_dragon_status_text(
    mut commands: Commands,
    // app_screen: Res<State<AppScreen>>, 
    // mut next_app_screen: ResMut<NextState<AppScreen>>,
    // game_status: Res<GameStatus>,
    resource_cache: Res<ResourceCache>,
){

    println!("spawn_dragon_status_text");

    let font: Handle<Font> =  resource_cache.gui_fonts.get("FiraSans-Bold").unwrap().clone();
    
    // let dragon_position_text_section: TextSection = TextSection::new(
    //     "",  // placeholder for Dragon Position: 
    //     TextStyle {
    //         font: font.clone(),
    //         font_size: 30.0,
    //         color: Color::WHITE,
    // });

    let dragon_health_text_section: TextSection = TextSection::new(
        "", // placeholder for Dragon Health: 
        TextStyle {
            font: font.clone(),
            font_size: 30.0,
            color: Color::WHITE,
    });

    // let mouse_position_text_section: TextSection = TextSection::new(
    //     "",  // placeholder for \nMouse Position: 
    //     TextStyle {
    //         font: font.clone(),
    //         font_size: 30.0,
    //         color: Color::WHITE,
    // });

    // Create a text element to display things like the dragon health and position
    let _text_bundle_package = commands.spawn(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexStart,
            ..Default::default()
        },
        text: Text::from_sections([
            // dragon_position_text_section,
            dragon_health_text_section,
            // mouse_position_text_section
        ]),
        ..Default::default()
    }).id();

    // commands.insert_resource(DragonStatusText { entity: Some(text_bundle_package) });

}


fn spawn_virtual_joystick(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut touch_assignments: ResMut<TouchAssignments>,
        // game_phase: Res<State<GamePhase>>,
        game_status: Res<GameStatus>,
    ) {

    info!("Setup Joystick. game_status.phase: {:?}", game_status.phase );

    if game_status.phase == GamePhase::Paused {
        println!("Setup Joystick. don't respawn, just game_status.phase: {:?}", game_status.phase);
        return;
    }

    if let Some(_joystick_entity) = touch_assignments.joystick_entity_id{
        info!("joystick_entity already exists...");
        return;
    }
    
    let joystick_diameter = 200.0;
    let joystick_border_left = 25.0;
    let joystick_border_bottom = 25.0;
    
    touch_assignments.move_touch_id = None;
    touch_assignments.shoot_touch_id = None;

    let joystick_background_path = "sprites/joystick-background.png";
    let joystick_background_image_handle: Handle<Image> = asset_server.load(joystick_background_path);

    let joystick_handle_path = "sprites/joystick-handle.png";
    let joystick_handle_image_handle: Handle<Image> = asset_server.load(joystick_handle_path);

    let handle_entity = commands.spawn(ImageBundle {
        style: Style {
            size: Size { 
                width: Val::Px(joystick_diameter/4.0), 
                height: Val::Px(joystick_diameter/4.0) 
            },
            ..default()
        },
        image: UiImage {
            texture: joystick_handle_image_handle, 
            ..default()
        },
        background_color: Color::rgba(1.0, 1.0, 1.0, 0.78).into(),
        ..default()
    }).id();

    let joystick_entity = commands.spawn((
        VirtualJoystick {
            center: Vec2::ZERO,
            direction: Vec3::ZERO,
            handle_entity,
        },
        ImageBundle {
            style: Style {
                size: Size { 
                    width: Val::Px(joystick_diameter), 
                    height: Val::Px(joystick_diameter) 
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(joystick_border_left),
                    bottom: Val::Px(joystick_border_bottom),
                    ..default()
                },
                ..default()
            },
            image: UiImage { 
                texture: joystick_background_image_handle,
                ..default()
            },
            background_color: Color::rgba(1.0, 1.0, 1.0, 0.78).into(),
            ..default()
        },
    ))
    .id();

    commands.entity(joystick_entity).push_children(&[handle_entity]);
    touch_assignments.joystick_entity_id = Some(joystick_entity);

    // commands.insert_resource(ScreenPackage { entity: joystick_entity });

}


fn game_event_watcher( 
    mut next_app_screen: ResMut<NextState<AppScreen>>,
    // game_phase: Res<State<GamePhase>>,
    game_status: Res<GameStatus>,
){

    match game_status.phase {
        GamePhase::GameOver => {
            next_app_screen.set(AppScreen::GameOver);
        }
        GamePhase::Paused  => {
            next_app_screen.set(AppScreen::Paused);
            // next_game_phase.set(GamePhase::Paused);
        }
        _ => {
            // println!(" -- Game Phase: {:?}",game_phase.0 );
        }
    }

}

fn dragon_status_watcher(
    // windows: Query<&Window>,
    mut text_query: Query<&mut Text>,
    // mouse_button_input: Res<Input<MouseButton>>,
    // touches: Res<Touches>,
    _cursor_moved_events: EventReader<CursorMoved>,
    dragon_query: Query<(&Dragon, &Transform), (With<MyDragon>,Without<GameCamera>)>,
    // camera_query: Query<(&GameCamera, &mut Transform), With<GameCamera>>,
    //game_phase: Res<State<GamePhase>>,
    game_status: Res<GameStatus>,
) {
    // if game_phase.0 == GamePhase::Paused {
    //     println!("dragon_status_watcher. skip because game_phase: {:?}", game_phase);
    //     return;
    // }
    // if game_phase.0 != GamePhase::Playing {
    //     println!("dragon_status_watcher. skip because game_phase: {:?}", game_phase);
    //     return;
    // }

    let n_dragons_found = dragon_query.iter().collect::<Vec<_>>().len();
    if n_dragons_found < 1 {
        println!("dragon_status_watcher. Found NO Dragons, game_status.phase : {:?}", game_status.phase);
        return;
    }

    // let (game_camera, camera_transform) = camera_query.single();
    // let window = windows.single();
    
    let (dragon, _dragon_transform) = dragon_query.single();
    
    // let mut last_touched_start_pos = Vec2::ZERO;
    // for touch in touches.iter() {
    //     if touches.just_pressed(touch.id()){
    //         last_touched_start_pos = touch.position();
    //     }
    // }

    // let n_text_found = text_query.iter().collect::<Vec<_>>().len();
    // let mut i = 0;
    if let Some(mut text) = text_query.iter_mut().next() {
        // i += 1;
        // println!("text component {} of {} text components on screen", i, n_text_found);
        //text.sections[0].value = format!("Position: {:.1}, {:.1}", dragon_transform.translation.x, dragon_transform.translation.y);
        text.sections[0].value = format!("\nHealth: {}/{}", dragon.health, dragon.max_health );
        // if mouse_button_input.pressed(MouseButton::Left) {
        //     let cursor_position = window.cursor_position().unwrap_or_default();
        //     let world_position = game_camera.screen_to_world(cursor_position, window, &camera_transform.translation);
        //     text.sections[2].value = format!("\nMouse Click: {:.1}, {:.1}", world_position.x, world_position.y);
        // } 
        // else if last_touched_start_pos != Vec2::ZERO {

        //     let world_position = game_camera.screen_to_world(last_touched_start_pos, window, &camera_transform.translation);
        //     text.sections[2].value = format!("\nTouch Pos: {:.1}, {:.1}", world_position.x, world_position.y);
        // }
    }
}


fn virtual_joystick_watcher(
    virtual_joystick_query: Query<(&VirtualJoystick, &Style), With<VirtualJoystick>>,
    mut handle_style_query: Query<&mut Style, (With<UiImage>, Without<VirtualJoystick>)>,
) {
    for (joystick, background_style) in virtual_joystick_query.iter() {
        if let Ok(mut handle_style) = handle_style_query.get_mut(joystick.handle_entity) {
            let half_size = mutils::size_to_vec2(background_style.size) / 2.0;
            let handle_half_size = mutils::size_to_vec2(handle_style.size) / 2.0;
            let joystick_direction = joystick.direction.truncate().normalize_or_zero();
            let offset = joystick_direction * (half_size - handle_half_size) * 0.5;
            handle_style.position.left = Val::Px(offset.x);
            handle_style.position.bottom = Val::Px(offset.y);
            // println!("--------------------------------");
            // println!("background_style.position.left {:?}", background_style.position.left);
            // println!("half_size.x {:?}", half_size.x);
            // println!("offset.x {:?}", offset.x);
            // println!("handle_half_size.x {:?}", handle_half_size.x);

            // println!("background_style.position.bottom {:?}", background_style.position.bottom);
            // println!("half_size.y {:?}", half_size.y);
            // println!("offset.y {:?}", offset.y);
            // println!("handle_half_size.y {:?}", handle_half_size.y);

            // println!("handle_style.position {:?}", handle_style.position);
            // println!("======================================");
        }
    }
}
