// src\client\systems\screen.rs

use bevy::prelude::*;
use crate::{shared::components::*, client::systems::*};
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
            
            // Play Button
            parent.spawn((MenuButtonAction::Play, ButtonBundle {
                    style: button_style.clone(),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Resume",
                        button_text_style.clone()
                    ));
                });
        })
        .id();

        commands.insert_resource(ScreenPackage { entity: pause_screen_package });

}


pub fn unpause_screen(
    mut commands: Commands,
    screen_package: Res<ScreenPackage>,
    mut next_game_phase: ResMut<NextState<GamePhase>>,
) {
    commands.entity(screen_package.entity).despawn_recursive();
    next_game_phase.set(GamePhase::Playing);
}
