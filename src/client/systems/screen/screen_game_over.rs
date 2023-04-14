// src/client/systems/screen/screen_game_over.rs

use bevy::prelude::*;
use crate::shared::components::*;
use crate::client::systems::screen::*;



pub struct GameOverScreenPlugin;
impl Plugin for GameOverScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system( spawn_screen_game_over.in_schedule(OnEnter(AppScreen::GameOver)) )
            .add_system( cleanup_game.in_schedule(OnExit(AppScreen::GameOver)) )
            ;
    }
}



pub fn spawn_screen_game_over(
    mut commands: Commands,
    resource_cache: Res<ResourceCache>,
    game_status: Res<GameStatus>,
) {

    info!("screen_game_over");
    // A semi-transparent black color
    let dim_color = Color::rgba(0.4, 0.4, 0.4, 0.75);

    // game_outcome.0
    let message = match game_status.outcome {
        GameOutcome::Win => "You Win!",
        GameOutcome::Lose => "You Lose!",
        _ => "Game Over",
    };

    let font: Handle<Font> = resource_cache.gui_fonts.get("FiraSans-Bold").unwrap().clone();
    let (button_style, button_text_style) = common_button_style(font.clone());

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
            // Reset button
            parent
                .spawn((MenuButtonAction::Reset, ButtonBundle {
                    style: button_style.clone(),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                }
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Reset",
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