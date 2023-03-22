use bevy::prelude::*;
use uuid::Uuid;

use crate::shared::components::{resource_cache::*, elemental_theme::*, dragon::*, game::*, wall::*};

pub struct GameSetupPlugin;


impl Plugin for GameSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            setup_maze,
            setup_dragons,
        ) .in_schedule(OnEnter(GameState::Setup)));
    }
}

fn setup_dragons(
        mut commands: Commands,
        resource_cache: Res<ResourceCache>,
    ) {
    
    let dragon_images = &resource_cache.dragon_images;

    // Spawn the Fire Dragon into the game.
    let mydragon_spawn_home = Vec3::new(100., 0., 0.);
    let mydragon_theme = ElementalTheme::Fire;
    commands.spawn(MyDragonBundle {
        my_dragon: MyDragon,
        dragon_bundle: DragonBundle {
            sprite_bundle: SpriteBundle {
                texture: dragon_images.get(&mydragon_theme).unwrap().file_handle.clone(),
                transform: Transform::from_translation(mydragon_spawn_home),
                ..default()
            },
            input: DragonInput::default(),
            movement: DragonAction {
                spawn_home: mydragon_spawn_home,
                velocity: Vec3::ZERO,
                max_velocity: 25.0,
                motion_timer: Timer::from_seconds(0.05, TimerMode::Repeating),
                firerate_timer: Timer::from_seconds(0.15, TimerMode::Repeating),
                flip_timer: Timer::from_seconds(0.2, TimerMode::Once),
                flipping: false,
            },
            dragon: Dragon {
                id: Uuid::new_v4(),
                elemental_theme: mydragon_theme,
                health: 10,
            },
        },
    });

    // Spawn an Ice Dragon into the game.
    let icedragon_spawn_home = Vec3::new(1400., 0., 0.);
    let ice_dragon_theme = ElementalTheme::Ice;

    commands.spawn( DragonBundle {
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
                max_velocity: 25.0,
                motion_timer: Timer::from_seconds(0.05, TimerMode::Repeating),
                firerate_timer: Timer::from_seconds(0.15, TimerMode::Repeating),
                flip_timer: Timer::from_seconds(0.2, TimerMode::Once),
                flipping: false,
            },
            dragon: Dragon { 
                id: Uuid::new_v4(), 
                elemental_theme: ice_dragon_theme,
                health: 10,
            }
    });
    
}


fn setup_maze(
    mut commands: Commands,
    resource_cache: Res<ResourceCache>,
    mut state: ResMut<NextState<GameState>>,
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
    state.set(GameState::Running);
}


