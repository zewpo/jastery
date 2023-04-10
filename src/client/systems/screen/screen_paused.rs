// src\client\systems\screen.rs

use bevy::prelude::*;
use crate::{shared::components::*, client::{systems::*, components::TouchAssignments}};
// use crate::client::systems::screen::*;


pub fn spawn_paused_screen(
    mut commands: Commands,
    resource_cache: Res<ResourceCache>,
) {
    println!("spawn_paused_screen");

    let font: Handle<Font> =  resource_cache.gui_fonts.get("FiraSans-Bold").unwrap().clone();
    let (button_style, button_text_style) = common_button_style(font.clone());

    let pause_screen_package = commands.spawn(NodeBundle {
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
            
            // UnPause Button
            parent.spawn((MenuButtonAction::Play, ButtonBundle {
                    style: button_style.clone(),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Unpause",
                        button_text_style.clone()
                    ));
                });

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

        })
        .id();

        commands.insert_resource(ScreenPackage { entity: pause_screen_package });

}


pub fn unpause_game(
    mut commands: Commands,
    screen_package: Res<ScreenPackage>,
    // dragon_status_text: Res<DragonStatusText>,
    dragon_status_text_query: Query<Entity,With<DragonStatusText>>,
    game_status: Res<GameStatus>,
    // mut next_app_screen: ResMut<NextState<AppScreen>>,
    game_piece_query: Query<Entity,With<GamePiece>>,
    touch_assignments: ResMut<TouchAssignments>,
) {

    if game_status.phase == GamePhase::Playing {
        // just clean the Pause Screen entities, not the whole game.
        commands.entity(screen_package.entity).despawn_recursive();
    } else {
        // clean the whole game.
        cleanup_game(commands, game_piece_query, screen_package, touch_assignments, dragon_status_text_query);
    }

}
