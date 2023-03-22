// src/shared/systems/game/menu.rs

use bevy::{
    prelude::*,
    ui::Interaction,
};

use crate::shared::components::{game::*, dragon::*, projectile::*, wall::*};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(setup_menu.in_schedule(OnEnter(GameState::MainMenu)))
            .add_system(menu.in_set(OnUpdate(GameState::MainMenu)))
            .add_system(cleanup_menu.in_schedule(OnExit(GameState::MainMenu)))
        ;
    }
}

#[derive(Resource)]
struct MenuData {
    button_entity: Entity,
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button_entity = commands
        .spawn(NodeBundle {
            style: Style {
                // center button
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        })
        .id();
    commands.insert_resource(MenuData { button_entity });
}

fn menu(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                next_state.set(GameState::Setup);
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

fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.button_entity).despawn_recursive();
}



pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(setup_game_over_screen.in_schedule(OnEnter(GameState::GameOver)))
            .add_system(handle_game_over_input.in_set(OnUpdate(GameState::GameOver)))
            .add_system(cleanup_game_over_screen.in_schedule(OnExit(GameState::GameOver)))
        ;
    }
}

#[derive(Resource)]
struct GameOverData {
    message_entity: Entity,
}


fn setup_game_over_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_outcome: Res<State<GameOutcome>>,
) {
    println!("Entering Game Over screen.");
    let message = match &game_outcome.0 {
        GameOutcome::Win => "You Win!",
        GameOutcome::Lose => "You Lose!",
        _ => "Game Over",
    };

    let message_entity = commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                message,
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        })
        .id();

    commands.insert_resource(GameOverData { message_entity });
}


fn handle_game_over_input(
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        next_state.set(GameState::Setup);
    } else if keyboard_input.just_pressed(KeyCode::M) {
        next_state.set(GameState::MainMenu);
    }
}

fn cleanup_game_over_screen(
        mut commands: Commands, 
        game_over_data: Res<GameOverData>,
        // all_entities_query: Query<Entity>,
        dragon_query: Query<Entity, With<Dragon>>,
        projectile_query: Query<Entity, With<Projectile>>,
        wall_query: Query<Entity, With<Wall>>,
    ) {
    commands.entity(game_over_data.message_entity).despawn_recursive();

    // // Despawn all entities without filtering by components
    // for entity in all_entities_query.iter() {
    //     commands.entity(entity).despawn_recursive();
    // }

    // Despawn all Dragon entities
    for entity in dragon_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Despawn all Projectile entities
    for entity in projectile_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Despawn all Wall entities
    for entity in wall_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

}

