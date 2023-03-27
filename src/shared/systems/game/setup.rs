// src\shared\systems\game\setup.rs

use bevy::prelude::*;
use uuid::Uuid;

use crate::shared::components::*;

pub struct GameSetupPlugin;

impl Plugin for GameSetupPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems((
            setup_maze,
            setup_dragons,
        ).in_schedule(OnEnter(GamePhase::Setup)))
        .add_system(setup_completion.in_set(OnUpdate(GamePhase::Setup)));
    }
}

fn setup_completion(mut game_phase: ResMut<NextState<GamePhase>>){
    game_phase.set(GamePhase::Playing);
}

fn setup_dragons(
        mut commands: Commands,
        resource_cache: Res<ResourceCache>,
    ) {
    println!("Setup Dragons.");
    //let dragon_images = &resource_cache.dragon_images;
    
    // Spawn the Fire Dragon into the game.
    let my_dragon_spawn_home = Vec3::new(0., 0., 0.);
    let my_dragon_theme = ElementalTheme::Fire;
    let my_dragon_image = resource_cache.get_collidable_image(CollidableClassifier::Dragon(my_dragon_theme));
    let _my_dragon_entity = commands.spawn( (MyDragon, DragonBundle {
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
            max_health: 20,
            image: my_dragon_image,
            game_piece: GamePiece,
            input: DragonInput::default(),
            action: DragonAction {
                spawn_home: my_dragon_spawn_home,
                velocity: Vec3::ZERO,
                acceleration: Vec3::ZERO,
                max_velocity: 15.0,
                motion_timer: Timer::from_seconds(0.02, TimerMode::Repeating),
                firerate_timer: Timer::from_seconds(0.15, TimerMode::Repeating),
                flip_timer: Timer::from_seconds(0.2, TimerMode::Once),
                flipping: false,
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



    // Spawn an enemy Ice Dragon into the game.
    let ice_dragon_spawn_home = Vec3::new(1400., 0., 0.);
    let ice_dragon_theme = ElementalTheme::Ice;
    let ice_dragon_image = resource_cache.get_collidable_image(CollidableClassifier::Dragon(ice_dragon_theme));
    commands.spawn( DragonBundle {
            sprite_bundle: SpriteBundle {
                texture: ice_dragon_image.handle(), // dragon_images.get(&ice_dragon_theme).unwrap().image.file_handle.clone(),
                transform: Transform::from_translation(ice_dragon_spawn_home),  //from_xyz(1200., 0., 0.),
                ..default()
            },
            dragon: Dragon {
                my_dragon: None,
                game_piece: GamePiece,
                id: Uuid::new_v4(), 
                elemental_theme: ice_dragon_theme,
                health: 10,
                max_health: 20,
                image: ice_dragon_image,
            
                input: DragonInput::default(),
                action: DragonAction {
                    spawn_home: ice_dragon_spawn_home,
                    velocity: Vec3::ZERO,
                    acceleration: Vec3::ZERO,
                    max_velocity: 0.0,
                    motion_timer: Timer::from_seconds(0.05, TimerMode::Repeating),
                    firerate_timer: Timer::from_seconds(0.15, TimerMode::Repeating),
                    flip_timer: Timer::from_seconds(0.2, TimerMode::Once),
                    flipping: false,
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



    // Spawn an enemy Fire Dragon into the game.
    let fire_dragon_spawn_home = Vec3::new(1000., -400., 0.);
    let fire_dragon_theme = ElementalTheme::Fire;
    let fire_dragon_image = resource_cache.get_collidable_image(CollidableClassifier::Dragon(fire_dragon_theme));
    commands.spawn( DragonBundle {
            sprite_bundle: SpriteBundle {
                texture: fire_dragon_image.handle(),
                transform: Transform::from_translation(fire_dragon_spawn_home),
                ..default()
            },
            dragon: Dragon {
                my_dragon: None,
                game_piece: GamePiece,
                id: Uuid::new_v4(), 
                elemental_theme: fire_dragon_theme,
                health: 10,
                max_health: 20,
                image: fire_dragon_image,
            
                input: DragonInput::default(),
                action: DragonAction {
                    spawn_home: fire_dragon_spawn_home,
                    velocity: Vec3::ZERO,
                    acceleration: Vec3::ZERO,
                    max_velocity: 0.0,
                    motion_timer: Timer::from_seconds(0.05, TimerMode::Repeating),
                    firerate_timer: Timer::from_seconds(0.15, TimerMode::Repeating),
                    flip_timer: Timer::from_seconds(0.2, TimerMode::Once),
                    flipping: false,
                },
            },
    });
    println!("Setup Dragons DONE.");
}


fn setup_maze(
    mut commands: Commands,
    resource_cache: Res<ResourceCache>,
) {
    println!("Setup Maze");
    //let wall_images = &resource_cache.wall_images;

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
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
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

    let wall_image = resource_cache.get_collidable_image(CollidableClassifier::Wall(WallShape::Straight));
    let wall_width = wall_image.width_i32();
    let wall_height = wall_image.height_i32();

    // Spawn Wall blocks into the game.
    for (i, row) in maze.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == 1 {
                let x = (j * wall_width as usize) as f32 - 1600.0;
                let y = (i * wall_height as usize) as f32 - 1000.0;
                commands.spawn(WallBundle {
                    game_piece: GamePiece,
                    sprite_bundle: SpriteBundle {
                        texture: wall_image.handle(),
                        transform: Transform::from_xyz(x, y, -1.0),
                        ..default()
                    },
                    wall: Wall { 
                        shape: WallShape::Straight,
                        image: wall_image.clone(),
                    },
                });
            }
        }
    }
}


