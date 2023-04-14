// src/client/systems/screen/screen_main_menu.rs

use bevy::prelude::*;
use crate::shared::components::*;
use crate::client::systems::screen::*;

pub fn spawn_main_menu_screen(
    mut commands: Commands,
    resource_cache: Res<ResourceCache>,
) {
    info!("spawn_main_menu_screen");

    let font: Handle<Font> =  resource_cache.gui_fonts.get("FiraSans-Bold").unwrap().clone();
    let (button_style, button_text_style) = common_button_style(font.clone());

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
