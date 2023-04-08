// src\client\systems\screen.rs

// use bevy::app::AppExit;
use bevy::{
    app::AppExit,
    input::touch::*,
    prelude::*,
    ui::Interaction,
};

use crate::client::components::*;
use crate::mutils;
use crate::shared::components::*;
use crate::shared::components::game::*;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

const BUTTON_SIZE: Size = Size::new(Val::Px(180.0), Val::Px(65.0));

#[derive(Resource)]
struct ScreenPackage {
    entity: Entity,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppScreen {
    #[default]
    MainMenu,
    Settings,
    InPlay,
    Paused,
    GameOver
}

// All actions that can be triggered from a menu button click
#[derive(Component)]
enum MenuButtonAction {
    Play,
    Quit,
    Restart,
    Settings,
    // SettingsDisplay,
    // SettingsSound,
    MainMenu,
    // BackToSettings, 
}

pub struct ScreenManagerPlugin;

impl Plugin for ScreenManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<AppScreen>()

            .add_system(setup_main_menu_screen.in_schedule(OnEnter(AppScreen::MainMenu)))
            .add_system(handle_menu_interaction.in_set(OnUpdate(AppScreen::MainMenu)))
            .add_system(cleanup_screen.in_schedule(OnExit(AppScreen::MainMenu)))

            .add_system(setup_game_play_screen.in_schedule(OnEnter(AppScreen::InPlay)))
            .add_system(setup_virtual_joystick.in_schedule(OnEnter(AppScreen::InPlay)))
            .add_system(game_monitor.in_set(OnUpdate(AppScreen::InPlay)))
            .add_system(update_virtual_joystick_system.in_set(OnUpdate(GamePhase::Playing)))
            .add_system(dragon_position_text_system.in_set(OnUpdate(GamePhase::Playing)))

            .add_system(setup_game_over_screen.in_schedule(OnEnter(AppScreen::GameOver)))
            .add_system(handle_menu_interaction.in_set(OnUpdate(AppScreen::GameOver)))
            .add_system(cleanup_screen.in_schedule(OnExit(AppScreen::GameOver)));
    }
}


fn setup_main_menu_screen(
    mut commands: Commands,
    resource_cache: Res<ResourceCache>,
) {
    println!("setup_main_menu_screen");

    let font: Handle<Font> =  resource_cache.gui_fonts.get("FiraSans-Bold").unwrap().clone();
    // Common style for all buttons on the screen
    let button_style = Style {
        size: BUTTON_SIZE,
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = TextStyle {
        font: font.clone(),
        font_size: 40.0,
        color: TEXT_COLOR,
    };

    let screen_package = commands.spawn(NodeBundle {
            style: Style {
                // center button
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Play Button
            parent.spawn((MenuButtonAction::Play, ButtonBundle {
                    style: button_style.clone(),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        button_text_style.clone()
                    ));
                });
            // Settings Button
            parent.spawn((MenuButtonAction::Settings, ButtonBundle {
                    style: button_style.clone(),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Settings",
                        button_text_style.clone()
                    ));
                });
            // Quit Button
            parent.spawn((MenuButtonAction::Quit, ButtonBundle {
                    style: button_style.clone(),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Quit",
                        button_text_style.clone()
                    ));
                });
        })
        .id();

        commands.insert_resource(ScreenPackage { entity: screen_package });
}

// fn cleanup_screen(mut commands: Commands, screen_package: Res<ScreenPackage>) {
//     commands.entity(screen_package.entity).despawn_recursive();
// }

// fn handle_main_menu(
//     mut next_state: ResMut<NextState<AppState>>,
//     mut interaction_query: Query<
//         (&Interaction, &MenuButtonAction, &mut BackgroundColor),
//         (Changed<Interaction>, With<Button>),
//     >,
//     mut app_exit_events: EventWriter<AppExit>,
// ) {
//     for (interaction, menu_button_action, mut color) in &mut interaction_query {
//         match *interaction {
//             Interaction::Clicked => {
//                 *color = PRESSED_BUTTON.into();
//                 match menu_button_action {
//                     MenuButtonAction::Play => next_state.set(AppState::Setup),
//                     MenuButtonAction::Quit => app_exit_events.send(AppExit),
//                 }
//             }

//             Interaction::Hovered => {
//                 *color = HOVERED_BUTTON.into();
//             }
//             Interaction::None => {
//                 *color = NORMAL_BUTTON.into();
//             }
//         }
//     }
// }

// fn cleanup_main_menu(
//     mut commands: Commands,   // With<MenuButtonAction>,
//     all_entities_query: Query<Entity,(Without<GameCamera>,Without<PrimaryWindow>)>,
// ) {

//     commands.entity(menu_data.button_entity).despawn_recursive();

//     // for entity in all_entities_query.iter() {
//     //     commands.entity(entity).despawn_recursive();
//     // }
// }

// fn cleanup_main_menu(
//     mut commands: Commands,   // With<MenuButtonAction>,
//     game_piece_query: Query<Entity,(With<GamePiece>,Without<GameCamera>,Without<PrimaryWindow>)>,
//     button_query: Query<Entity,(With<Button>,Without<GameCamera>,Without<PrimaryWindow>)>,
// ) {
//     for entity in game_piece_query.iter() {
//         commands.entity(entity).despawn_recursive();
//     }

//     for entity in button_query.iter() {
//         commands.entity(entity).despawn_recursive();
//     }
// }


// pub struct GameOverMenuScreen;

// impl Plugin for GameOverMenuScreen {
//     fn build(&self, app: &mut App) {
//         app
//             .add_system(setup_game_over_screen.in_schedule(OnEnter(AppScreen::GameOver)))
//             .add_system(handle_menu_interaction.in_set(OnUpdate(AppScreen::GameOver)))
//             .add_system(cleanup_screen.in_schedule(OnExit(AppScreen::GameOver)))
//         ;
//     }
// }

fn setup_game_over_screen(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    resource_cache: Res<ResourceCache>,
    // game_outcome: Res<State<GameOutcome>>,
    game_status: Res<GameStatus>,
) {

    println!("setup_game_over_screen");
    // A semi-transparent black color
    let dim_color = Color::rgba(0.4, 0.4, 0.4, 0.75);

    // game_outcome.0
    let message = match game_status.outcome {
        GameOutcome::Win => "You Win!",
        GameOutcome::Lose => "You Lose!",
        _ => "Game Over",
    };

    let font: Handle<Font> =  resource_cache.gui_fonts.get("FiraSans-Bold").unwrap().clone();
    // Common style for all buttons on the screen
    let button_style = Style {
        size: BUTTON_SIZE,
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = TextStyle {
        font: font.clone(),
        font_size: 40.0,
        color: TEXT_COLOR,
    };

    let screen_package = commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                // flex_direction: FlexDirection::Column,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: bevy::prelude::BackgroundColor(dim_color),
            ..default()
        })
        .with_children(|parent| {
            // Game Outcome message
            parent.spawn(TextBundle::from_section(
                message,
                TextStyle {
                    font: font.clone(),
                    font_size: 80.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
            // Main Menu button
            parent
                .spawn((MenuButtonAction::MainMenu, ButtonBundle {
                    style: button_style.clone(),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                }
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Main Menu",
                        button_text_style.clone(),
                    ));
                });
            // Restart button
            parent
                .spawn((MenuButtonAction::Restart, ButtonBundle {
                    style: button_style.clone(),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                }
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Restart",
                        button_text_style.clone(),
                    ));
                });
            // Quit button
            parent
                .spawn((MenuButtonAction::Quit, ButtonBundle {
                    style: button_style.clone(),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                }
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Quit",
                        button_text_style.clone()
                    ));
                });
        })
        .id();

    commands.insert_resource(ScreenPackage { entity: screen_package });

}


fn dragon_position_text_system(
    windows: Query<&Window>,
    camera_scale: Res<CameraScale>,
    mut text_query: Query<&mut Text>,
    mouse_button_input: Res<Input<MouseButton>>,
    touches: Res<Touches>,
    // mut mouse_motion_events: EventReader<MouseMotion>,
    _cursor_moved_events: EventReader<CursorMoved>,
    dragon_query: Query<(&Dragon, &Transform), (With<MyDragon>,Without<GameCamera>)>,
    camera_query: Query<(&GameCamera, &mut Transform), With<GameCamera>>,
) {
    
    let (game_camera, camera_transform) = camera_query.single();
    let window = windows.single();
    let window_size = Vec2::new(window.width(), window.height());
    
    let (dragon, dragon_transform) = dragon_query.single();

    // for event in mouse_motion_events.iter() {
    //     info!("{:?}", event);
    // 
    // let mut pos = Vec2::ZERO;
    // for event in cursor_moved_events.iter() {
    //     pos = event.position - window_size / 2.0;
    //     info!("{:?}", pos );
    // }

    let mut last_touched_start_pos = Vec2::ZERO;
    for touch in touches.iter() {
        if touches.just_pressed(touch.id()){
            last_touched_start_pos = touch.position();
        }
    }

    if let Some(mut text) = text_query.iter_mut().next() {        
        //text.sections[0].value = format!("Dragon Position: ({:.1}, {:.1})", dragon_transform.translation.x, dragon_transform.translation.y);
        text.sections[0].value = format!("Position: {:.1}, {:.1}", dragon_transform.translation.x, dragon_transform.translation.y);
        text.sections[1].value = format!("\nHealth: {}/{}", dragon.health, dragon.max_health );
        if mouse_button_input.pressed(MouseButton::Left) {
            let cursor_position = window.cursor_position().unwrap_or_default();
            //let world_position =  camera_transform.translation.truncate() + ((cursor_position - window_size / 2.0) * camera_scale.0 ) ;
            let world_position = game_camera.screen_to_world(cursor_position, window, &camera_transform.translation);
            text.sections[2].value = format!("\nMouse Click: {:.1}, {:.1}", world_position.x, world_position.y);
        } 
        else if last_touched_start_pos != Vec2::ZERO {

            let world_position = game_camera.screen_to_world(last_touched_start_pos, window, &camera_transform.translation);
            text.sections[2].value = format!("\nTouch Pos: {:.1}, {:.1}", world_position.x, world_position.y);
        }
    }
}

// fn handle_game_over_input(
//     mut next_state: ResMut<NextState<AppState>>,
//     keyboard_input: Res<Input<KeyCode>>,
// ) {
//     if keyboard_input.just_pressed(KeyCode::R) {
//         next_state.set(AppState::Setup);
//     } else if keyboard_input.just_pressed(KeyCode::M) {
//         next_state.set(AppState::MainMenu);
//     }
// }

fn handle_menu_interaction(
    mut interaction_query: Query<(&Interaction, &MenuButtonAction, &mut BackgroundColor), 
                                 (Changed<Interaction>, With<Button>) >,
    mut next_screen: ResMut<NextState<AppScreen>>,
    mut app_exit_events: EventWriter<AppExit>,
) {

    // println!("Handling Game Over menu...");

    for (   interaction, 
            menu_button_action, 
            mut color
        ) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                match menu_button_action {
                    MenuButtonAction::Play => next_screen.set(AppScreen::InPlay),
                    MenuButtonAction::Settings => next_screen.set(AppScreen::Settings),
                    
                    MenuButtonAction::MainMenu => next_screen.set(AppScreen::MainMenu),
                    MenuButtonAction::Restart => next_screen.set(AppScreen::InPlay),

                    MenuButtonAction::Quit => app_exit_events.send(AppExit),
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn setup_game_play_screen(
    mut commands: Commands,
    // app_screen: Res<State<AppScreen>>, 
    // mut next_app_screen: ResMut<NextState<AppScreen>>,
    mut game_phase: ResMut<NextState<GamePhase>>,
    resource_cache: Res<ResourceCache>,
){
    println!("setup_game_play_screen");

    let font: Handle<Font> =  resource_cache.gui_fonts.get("FiraSans-Bold").unwrap().clone();
    
    let dragon_position_text_section: TextSection = TextSection::new(
        "",  // placeholder for Dragon Position: 
        TextStyle {
            font: font.clone(),
            font_size: 30.0,
            color: Color::WHITE,
    });

    let dragon_health_text_section: TextSection = TextSection::new(
        "", // placeholder for Dragon Health: 
        TextStyle {
            font: font.clone(),
            font_size: 30.0,
            color: Color::WHITE,
    });

    let mouse_position_text_section: TextSection = TextSection::new(
        "",  // placeholder for \nMouse Position: 
        TextStyle {
            font: font.clone(),
            font_size: 30.0,
            color: Color::WHITE,
    });

    // Create a text element to display things like the dragon position
    commands.spawn(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexStart,
            ..Default::default()
        },
        text: Text::from_sections([
            dragon_position_text_section,
            dragon_health_text_section,
            mouse_position_text_section
        ]),
        ..Default::default()
    });


    // if game_status.phase == GamePhase::ToBeDefined {
        game_phase.set(GamePhase::Setup);
    // }
}

fn game_monitor( 
    // app_screen: Res<State<AppScreen>>, 
    mut next_app_screen: ResMut<NextState<AppScreen>>,
    game_phase: Res<State<GamePhase>>,
    mut next_game_phase: ResMut<NextState<GamePhase>>,
){

    match game_phase.0 {
        GamePhase::GameOver => {
            next_app_screen.set(AppScreen::GameOver);
            next_game_phase.set(GamePhase::ToBeDefined);
            //game_status.phase = GamePhase::ToBeDefined;
        }
        _ => {}
    }

}


fn cleanup_screen(
    mut commands: Commands,
    game_piece_query: Query<Entity,With<GamePiece>>,
    screen_package: Res<ScreenPackage>
) {
    for game_piece in game_piece_query.iter() {
        commands.entity(game_piece).despawn_recursive();
    }
    commands.entity(screen_package.entity).despawn_recursive();

}


pub fn setup_virtual_joystick(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
    ) {
    println!("Setup Joystick.");

    let joystick_diameter = 200.0;
    let joystick_border_left = 25.0;
    let joystick_border_bottom = 25.0;
    
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

    let joystick_entity = commands.spawn( (
        VirtualJoystick {
            center: Vec2::ZERO,
            direction: Vec3::ZERO,
            handle_entity
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
}




fn update_virtual_joystick_system(
    virtual_joysticks: Query<(&VirtualJoystick, &Style), With<VirtualJoystick>>,
    mut handle_style_query: Query<&mut Style, (With<UiImage>, Without<VirtualJoystick>)>,
) {
    for (joystick, background_style) in virtual_joysticks.iter() {
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

// fn update_virtual_joystick_system(
//     virtual_joysticks: Query<(&VirtualJoystick, &Style), With<VirtualJoystick>>,
//     mut handle_style_query: Query<&mut Style, (With<UiImage>, Without<VirtualJoystick>)>,
// ) {
//     for (joystick, background_style) in virtual_joysticks.iter() {
//         if let Ok(mut handle_style) = handle_style_query.get_mut(joystick.handle_entity) {
//             let half_size = size_to_vec2(background_style.size) / 2.0;
//             let handle_half_size = size_to_vec2(handle_style.size) / 2.0;
//             let joystick_direction = joystick.delta.truncate();
//             let offset = joystick_direction * (half_size - handle_half_size) * 0.5;
//             handle_style.position.left = Val::Px(half_size.x + offset.x - handle_half_size.x);
//             handle_style.position.bottom = Val::Px(half_size.y + offset.y - handle_half_size.y);
//         }
//     }
// }


// fn update_virtual_joystick_system(
//     virtual_joysticks: Query<(&VirtualJoystick, &Style), With<VirtualJoystick>>,
//     mut handle_style_query: Query<&mut Style, (With<UiImage>, Without<VirtualJoystick>)>,
// ) {
//     for (joystick, background_style) in virtual_joysticks.iter() {
//         if let Ok(mut handle_style) = handle_style_query.get_mut(joystick.handle_entity) {
//             //let (Val::Px(width), Val::Px(height)) = (background_style.size.width, background_style.size.height);
//             let Val::Px(width) = background_style.size.width else { return; };
//             let Val::Px(height) = background_style.size.height else { return; };
//             let Val::Px(handle_width) = handle_style.size.width else { return; };
//             let Val::Px(handle_height) = handle_style.size.height else { return; };
//             let half_size = Vec2::new(width, height) / 2.0;
//             let handle_half_size = Vec2::new(handle_width, handle_height) / 2.0;
//             let joystick_direction = joystick.delta.truncate();
//             let offset = joystick_direction * (half_size - handle_half_size) * 0.5;
//             handle_style.position.left = Val::Px(half_size.x + offset.x - handle_half_size.x);
//             handle_style.position.bottom = Val::Px(half_size.y + offset.y - handle_half_size.y);
        
//         }
//     }
// }


// fn update_virtual_joystick_system(
//     virtual_joysticks: Query<(&VirtualJoystick, Entity)>,
//     mut query: Query<(&Children, &Style), (With<VirtualJoystick>, Without<UiImage>)>,
//     mut handle_style_query: Query<&mut Style, With<UiImage>>,
// ) {
//     println!("1.0  update_virtual_joystick_system");
//     for (virtual_joystick, joystick_entity) in virtual_joysticks.iter() {
//         println!("1.1  update_virtual_joystick_system");
//         if let Ok((children, background_style)) = query.get(joystick_entity) {
//             println!("1.2  update_virtual_joystick_system");

//             if let Some(handle_entity) = children.iter().next() {
//                 println!("1.3  update_virtual_joystick_system");
//                 if let Ok(mut handle_style) = handle_style_query.get_mut(*handle_entity) {
//                     println!("1.4  update_virtual_joystick_system");
//                     if let (Val::Px(width), Val::Px(height)) = (background_style.size.width, background_style.size.height) {
//                         let delta_2d = virtual_joystick.delta.truncate();
//                         let half_size = Vec2::new(width, height) / 2.0; // Half the size of the joystick background

//                         println!("Virtual Joystick Delta: {:?}", virtual_joystick.delta);
//                         println!("Handle Style Position: {:?}", handle_style.position);

//                         handle_style.position = UiRect {
//                             left: Val::Px(delta_2d.x * half_size.x),
//                             bottom: Val::Px(delta_2d.y * half_size.y),
//                             ..Default::default()
//                         };
//                     }
//                 }
//             }
//         }
//     }
// }


