// src\shared\systems\game\setup.rs

use bevy::prelude::*;
use uuid::Uuid;

use crate::shared::components::{resource_cache::*, elemental_theme::*, dragon::*, game::*, wall::*};

pub struct GameConstructionPlugin;


impl Plugin for GameConstructionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            build_maze,
            spawn_dragons,
        ).in_schedule(OnEnter(AppScreen::GamePlay)));
    }
}

fn spawn_dragons(
        mut commands: Commands,
        resource_cache: Res<ResourceCache>,
    ) {
    println!("Setup Dragons.");
    let dragon_images = &resource_cache.dragon_images;

    // Spawn the Fire Dragon into the game.
    let mydragon_spawn_home = Vec3::new(100., 0., 0.);
    let mydragon_theme = ElementalTheme::Fire;
    let _dragon_entity = commands.spawn(MyDragonBundle {
        my_dragon: MyDragon,
        dragon_bundle: DragonBundle {
            game_piece: GamePiece,
            sprite_bundle: SpriteBundle {
                texture: dragon_images.get(&mydragon_theme).unwrap().file_handle.clone(),
                transform: Transform::from_translation(mydragon_spawn_home),
                ..default()
            },
            input: DragonInput::default(),
            movement: DragonAction {
                spawn_home: mydragon_spawn_home,
                velocity: Vec3::ZERO,
                acceleration: Vec3::ZERO,
                max_velocity: 15.0,
                motion_timer: Timer::from_seconds(0.02, TimerMode::Repeating),
                firerate_timer: Timer::from_seconds(0.15, TimerMode::Repeating),
                flip_timer: Timer::from_seconds(0.2, TimerMode::Once),
                flipping: false,
            },
            dragon: Dragon {
                id: Uuid::new_v4(),
                elemental_theme: mydragon_theme,
                health: 10,
                max_health: 20,
            },
            // health_text_bundle: Text2dBundle { 
            //     text: Text::from_section(
            //         format!("HealthX: {}", 10),
            //         TextStyle {
            //             font:  resource_cache.gui_fonts.get("FiraSans-Bold").unwrap().clone(),   //font.clone(),
            //             font_size: 20.0,
            //             color: Color::WHITE,
            //           }
            //         // TextAlignment::default(),
            //         ), 
            //         ..default()
            //     // text_anchor: (), 
            //     // text_2d_bounds: (), 
            //     // transform: (), 
            //     // global_transform: (), 
            //     // visibility: (), 
            //     // computed_visibility: () 
            // },
        },
    }).id();



    // Spawn an Ice Dragon into the game.
    let icedragon_spawn_home = Vec3::new(1400., 0., 0.);
    let ice_dragon_theme = ElementalTheme::Ice;

    commands.spawn( DragonBundle {
            game_piece: GamePiece,
            sprite_bundle: SpriteBundle {
                // texture: asset_server.load("sprites/ice-dragon.png"),
                texture: dragon_images.get(&ice_dragon_theme).unwrap().file_handle.clone(),
                transform: Transform::from_translation(icedragon_spawn_home),  //from_xyz(1200., 0., 0.),
                ..default()
            },
            input: DragonInput::default(),
            movement: DragonAction {
                spawn_home: icedragon_spawn_home,
                velocity: Vec3::ZERO,
                acceleration: Vec3::ZERO,
                max_velocity: 8.0,
                motion_timer: Timer::from_seconds(0.05, TimerMode::Repeating),
                firerate_timer: Timer::from_seconds(0.15, TimerMode::Repeating),
                flip_timer: Timer::from_seconds(0.2, TimerMode::Once),
                flipping: false,
            },
            dragon: Dragon { 
                id: Uuid::new_v4(), 
                elemental_theme: ice_dragon_theme,
                health: 10,
                max_health: 20,
            },
            // health_text_bundle: Text2dBundle { 
            //     text: Text::from_section(
            //         format!("Health: {}", 10),
            //         TextStyle {
            //             font:  resource_cache.gui_fonts.get("FiraSans-Bold").unwrap().clone(), // .clone(),   //font.clone(),
            //             font_size: 20.0,
            //             color: Color::WHITE,
            //           }
            //         // TextAlignment::default(),
            //         ),
            //         ..default()
            //     // text_anchor: (), 
            //     // text_2d_bounds: (), 
            //     // transform: (), 
            //     // global_transform: (), 
            //     // visibility: (), 
            //     // computed_visibility: () 
            // }
    });
    println!("Setup Dragons DONE.");
}


fn build_maze(
    mut commands: Commands,
    resource_cache: Res<ResourceCache>,
) {
    println!("Setup Maze");
    let wall_images = &resource_cache.wall_images;

    let mut maze = [
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1],
    ];
    maze.reverse();

    if let Some(wall_image) =  wall_images.get(&WallShape::Straight) {
        let wall_width = wall_image.size.x;
        let wall_height = wall_image.size.y;

        // Spawn Wall blocks into the game.
        for (i, row) in maze.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if *cell == 1 {
                    let x = (j as f32 * wall_width) - 1600.0;
                    let y = (i as f32 * wall_height) - 1000.0;
                    commands.spawn(WallBundle {
                        game_piece: GamePiece,
                        sprite_bundle: SpriteBundle {
                            texture: wall_image.file_handle.clone(),
                            transform: Transform::from_xyz(x, y, -1.0),
                            ..default()
                        },
                        wall: Wall { shape: WallShape::Straight },
                    });
                }
            }
        }

       
    } else {
        println!("Setup Maze - Image not loaded yet...");
    }
    println!("Setup Maze DONE.");
    // next_state.set(AppState::Running);
}


