// src\shared\systems\game\setup_dragons.rs

use bevy::prelude::*;
use uuid::Uuid;

use crate::shared::components::*;

pub fn setup_dragons(
        mut commands: Commands,
        resource_cache: Res<ResourceCache>,
    ) {
    println!("Setup Dragons.");
    //let dragon_images = &resource_cache.dragon_images;
    
    // Spawn My Fire Dragon into the game.
    let my_dragon_spawn_home = Vec3::new(0., 0., 0.);
    let my_dragon_theme = ElementalTheme::Fire;
    let my_dragon_image = resource_cache.get_collidable_image(CollidableClassifier::Dragon(my_dragon_theme));
    let _my_dragon_entity = commands.spawn( (MyDragon, DragonBundle {
        game_piece: GamePiece,
        sprite_bundle: SpriteBundle {
            texture: my_dragon_image.handle(), //dragon_images.get(&mydragon_theme).unwrap().image.file_handle.clone(),
            transform: Transform::from_translation(my_dragon_spawn_home),
            ..default()
        },
        dragon: Dragon {
            my_dragon: Some(MyDragon),
            id: Uuid::new_v4(),
            elemental_theme: my_dragon_theme,
            health: 10,
            max_health: 10,
            max_velocity: 35.0,
            shooting_frequence: 1.0, // 100%
            image: my_dragon_image,
            input: DragonInput::default(),
            action: DragonAction {
                spawn_home: my_dragon_spawn_home,
                ..default()
            },
        },
         
            // },
            
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
    } )).id();

    // let new_path: Option<Vec<(i32, i32)>> = Some(Vec::new());

    // Spawn an enemy Ice Dragon into the game.
    // let ice_dragon_spawn_home = Vec3::new(1400., 0., 0.);
    let ice_dragon_spawn_home = Vec3::new(0., -800., 0.);
    let ice_dragon_theme = ElementalTheme::Ice;
    let ice_dragon_image = resource_cache.get_collidable_image(CollidableClassifier::Dragon(ice_dragon_theme));
    commands.spawn( DragonBundle {
            game_piece: GamePiece,
            sprite_bundle: SpriteBundle {
                texture: ice_dragon_image.handle(), // dragon_images.get(&ice_dragon_theme).unwrap().image.file_handle.clone(),
                transform: Transform::from_translation(ice_dragon_spawn_home),  //from_xyz(1200., 0., 0.),
                ..default()
            },
            dragon: Dragon {
                my_dragon: None,
                id: Uuid::new_v4(), 
                elemental_theme: ice_dragon_theme,
                health: 1,
                max_health: 20,
                max_velocity: 0.0,
                shooting_frequence: 0.3,
                image: ice_dragon_image,
            
                input: DragonInput::default(),
                action: DragonAction {
                    spawn_home: ice_dragon_spawn_home,
                    path: Some(Vec::new()),
                    pathfinding_timer: Some(Timer::from_seconds(0.05, TimerMode::Repeating)),
                    ..default()
                },
                
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



    // Spawn an enemy Rock Dragon into the game.
    let rock_dragon_spawn_home = Vec3::new(1000., -400., 0.);
    let rock_dragon_theme = ElementalTheme::Rock;
    let rock_dragon_image = resource_cache.get_collidable_image(CollidableClassifier::Dragon(rock_dragon_theme));
    commands.spawn( DragonBundle {
            game_piece: GamePiece,
            sprite_bundle: SpriteBundle {
                texture: rock_dragon_image.handle(),
                transform: Transform::from_translation(rock_dragon_spawn_home),
                ..default()
            },
            dragon: Dragon {
                my_dragon: None,
                id: Uuid::new_v4(), 
                elemental_theme: rock_dragon_theme,
                health: 1,
                max_health: 20,
                max_velocity: 0.0,
                shooting_frequence: 0.3,
                image: rock_dragon_image,
            
                input: DragonInput::default(),
                action: DragonAction {
                    spawn_home: rock_dragon_spawn_home,
                    path: Some(Vec::new()),
                    pathfinding_timer: Some(Timer::from_seconds(0.06, TimerMode::Repeating)),
                    ..default()
                },
            },
    });



    // Spawn an enemy Water Dragon into the game.
    let water_dragon_spawn_home = Vec3::new(-1000., 400., 0.);
    let water_dragon_theme = ElementalTheme::Water;
    let water_dragon_image = resource_cache.get_collidable_image(CollidableClassifier::Dragon(water_dragon_theme));
    commands.spawn( DragonBundle {
            game_piece: GamePiece,
            sprite_bundle: SpriteBundle {
                texture: water_dragon_image.handle(),
                transform: Transform::from_translation(water_dragon_spawn_home),
                ..default()
            },
            dragon: Dragon {
                my_dragon: None,
                id: Uuid::new_v4(), 
                elemental_theme: water_dragon_theme,
                health: 1,
                max_health: 20,
                max_velocity: 0.0,
                shooting_frequence: 0.3,
                image: water_dragon_image,
            
                input: DragonInput::default(),
                action: DragonAction {
                    spawn_home: water_dragon_spawn_home,
                    path: Some(Vec::new()),
                    pathfinding_timer: Some(Timer::from_seconds(0.07, TimerMode::Repeating)),
                    ..default()
                },
            },
    });


    println!("Setup Dragons DONE.");
}

