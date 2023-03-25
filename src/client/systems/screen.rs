// src\client\systems\screen.rs

use bevy::app::AppExit;
use bevy::{
    prelude::*,
    ui::Interaction,
};

use crate::shared::components::resource_cache::ResourceCache;
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
            .add_system(game_monitor.in_set(OnUpdate(AppScreen::InPlay)))

            .add_system(setup_game_over_screen.in_schedule(OnEnter(AppScreen::GameOver)))
            .add_system(handle_menu_interaction.in_set(OnUpdate(AppScreen::GameOver)))
            .add_system(cleanup_screen.in_schedule(OnExit(AppScreen::GameOver)));
    }
}


// pub struct MainMenuScreen;

// impl Plugin for MainMenuScreen {
//     fn build(&self, app: &mut App) {
//         app
//             .add_system(setup_main_menu_screen.in_schedule(OnEnter(AppScreen::MainMenu)))
//             .add_system(handle_menu_interaction.in_set(OnUpdate(AppScreen::MainMenu)))
//             .add_system(cleanup_screen.in_schedule(OnExit(AppScreen::MainMenu)))

//             .add_system(setup_game_over_screen.in_schedule(OnEnter(AppScreen::GameOver)))
//             .add_system(handle_menu_interaction.in_set(OnUpdate(AppScreen::GameOver)))
//             .add_system(cleanup_screen.in_schedule(OnExit(AppScreen::GameOver)))
//         ;
//     }
// }


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
    // app_screen: Res<State<AppScreen>>, 
    // mut next_app_screen: ResMut<NextState<AppScreen>>,
    mut game_phase: ResMut<NextState<GamePhase>>,
    // game_status: Res<GameStatus>,
){
    println!("setup_game_play_screen");
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
    for entity in game_piece_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    commands.entity(screen_package.entity).despawn_recursive();

}
