// src/shared/systems/game/menu.rs

use bevy::app::AppExit;
use bevy::window::PrimaryWindow;
use bevy::{
    prelude::*,
    ui::Interaction,
};

use crate::shared::components::resource_cache::ResourceCache;
use crate::{shared::components::game::*, client::components::game_camera::*};

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

const BUTTON_SIZE: Size = Size::new(Val::Px(180.0), Val::Px(65.0));



// All actions that can be triggered from a Main menu button click
#[derive(Component)]
enum MainMenuButtonAction {
    Play,
    Quit,
}


// All actions that can be triggered from a Game Over menu button click
#[derive(Component)]
enum GameOverMenuButtonAction {
    Restart,
    // Settings,
    // SettingsDisplay,
    // SettingsSound,
    MainMenu,
    // BackToSettings,
    Quit,
}


pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(setup_main_menu.in_schedule(OnEnter(GameState::MainMenu)))
            .add_system(handle_main_menu.in_set(OnUpdate(GameState::MainMenu)))
            .add_system(cleanup_main_menu.in_schedule(OnExit(GameState::MainMenu)))
        ;
    }
}


fn setup_main_menu(
    mut commands: Commands, 
    //asset_server: Res<AssetServer>
    resource_cache: Res<ResourceCache>,
) {
    
    let font: Handle<Font> =  resource_cache.gui_fonts.get("FiraSans-Bold").unwrap().clone(); //asset_server.load("fonts/FiraSans-Bold.ttf");
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

    commands.spawn(NodeBundle {
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
            parent.spawn((MainMenuButtonAction::Play, ButtonBundle {
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
            // Quit Button
            parent.spawn((MainMenuButtonAction::Quit, ButtonBundle {
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
        });
}



fn handle_main_menu(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &MainMenuButtonAction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for (interaction, menu_button_action, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                match menu_button_action {
                    MainMenuButtonAction::Play => next_state.set(GameState::Setup),
                    MainMenuButtonAction::Quit => app_exit_events.send(AppExit),
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


fn cleanup_main_menu(
    mut commands: Commands,   // With<MainMenuButtonAction>,
    all_entities_query: Query<Entity,(Without<GameCamera>,Without<PrimaryWindow>)>,
) {
    for entity in all_entities_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}


pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(setup_game_over_screen.in_schedule(OnEnter(GameState::GameOver)))
            .add_system(handle_game_over_menu.in_set(OnUpdate(GameState::GameOver)))
            //handle_game_over_input
            .add_system(cleanup_game_over_screen.in_schedule(OnExit(GameState::GameOver)))
        ;
    }
}

fn setup_game_over_screen(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    resource_cache: Res<ResourceCache>,
    game_outcome: Res<State<GameOutcome>>,
) {

    println!("Entering Game Over screen.");
    // A semi-transparent black color
    let dim_color = Color::rgba(0.0, 0.0, 0.0, 0.7);


    let message = match &game_outcome.0 {
        GameOutcome::Win => "You Win!",
        GameOutcome::Lose => "You Lose!",
        _ => "Game Over",
    };

    //let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
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

    commands
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
                .spawn((GameOverMenuButtonAction::MainMenu, ButtonBundle {
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
                .spawn((GameOverMenuButtonAction::Restart, ButtonBundle {
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
                .spawn((GameOverMenuButtonAction::Quit, ButtonBundle {
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
        });

}


// fn handle_game_over_input(
//     mut next_state: ResMut<NextState<GameState>>,
//     keyboard_input: Res<Input<KeyCode>>,
// ) {
//     if keyboard_input.just_pressed(KeyCode::R) {
//         next_state.set(GameState::Setup);
//     } else if keyboard_input.just_pressed(KeyCode::M) {
//         next_state.set(GameState::MainMenu);
//     }
// }

fn handle_game_over_menu(
    mut interaction_query: Query<(&Interaction, &GameOverMenuButtonAction, &mut BackgroundColor), 
                                 (Changed<Interaction>, With<Button>) >,
    mut next_state: ResMut<NextState<GameState>>,
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
                    GameOverMenuButtonAction::Quit => app_exit_events.send(AppExit),
                    GameOverMenuButtonAction::Restart => next_state.set(GameState::Setup),
                    GameOverMenuButtonAction::MainMenu => next_state.set(GameState::MainMenu), 
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


fn cleanup_game_over_screen(
    mut commands: Commands,
    all_entities_query: Query<Entity,(Without<GameCamera>,Without<PrimaryWindow>)>,
) {

    for entity in all_entities_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

