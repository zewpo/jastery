// src\client\systems\screen.rs

use bevy::{
    app::AppExit,
    prelude::*,
    ui::Interaction,
};

use crate::{shared::components::game::*, client::components::*};

use super::*;

pub const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
pub const BUTTON_SIZE: Size = Size::new(Val::Px(180.0), Val::Px(65.0));


// entity marker for ScreenPackage, used for despawning a screen.
#[derive(Resource)]
pub struct ScreenPackage {
    pub entity: Entity,
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
pub enum MenuButtonAction {
    Play,
    // Resume,
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
            .add_state::<GamePhase>()

            .add_system( handle_screen_interaction )

            .add_system( spawn_main_menu_screen.in_schedule(OnEnter(AppScreen::MainMenu)) )
            .add_system( cleanup_screen.in_schedule(OnExit(AppScreen::MainMenu)))

            .add_system( spawn_settings_screen.in_schedule(OnEnter(AppScreen::Settings)) )
            .add_system( cleanup_screen.in_schedule(OnExit(AppScreen::Settings)))

            .add_system( spawn_paused_screen.in_schedule(OnEnter(AppScreen::Paused)) )
            .add_system( unpause_screen.in_schedule(OnExit(AppScreen::Paused)))

            .add_plugin( GamePlayScreenPlugin )
            // .add_system( cleanup_screen.in_schedule(OnExit(AppScreen::InPlay)) )

            .add_plugin(GameOverScreenPlugin)
            ;
    }
}


fn handle_screen_interaction(
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
                    // MenuButtonAction::Resume => next_screen.set(AppScreen::InPlay),
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


pub fn cleanup_screen(
    mut commands: Commands,
    game_piece_query: Query<Entity,With<GamePiece>>,
    screen_package: Res<ScreenPackage>,
    mut touch_assignments: ResMut<TouchAssignments>,
) {
    println!("1.0  cleanup_screen");
    for game_piece in game_piece_query.iter() {
        commands.entity(game_piece).despawn_recursive();
    }
    println!("3.0  cleanup_screen");
    commands.entity(screen_package.entity).despawn_recursive();

    println!("4.0  cleanup_screen");
    if let Some(joystick_entity) = touch_assignments.joystick_entity_id {
        println!("5.0  cleanup_screen");
        commands.entity(joystick_entity).despawn_recursive();
        println!("6.0  cleanup_screen");
        touch_assignments.joystick_entity_id = None;
        println!("7.0  cleanup_screen");
    }
}



pub fn common_button_style(font: Handle<Font>) -> (Style, TextStyle) {(
    Style {
        size: BUTTON_SIZE,
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    },
    TextStyle {
        font,
        font_size: 40.0,
        color: TEXT_COLOR,
    }
)}