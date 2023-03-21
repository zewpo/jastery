mod client;
mod server;
mod shared;

use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    utils::HashMap,
    window::{close_on_esc, PresentMode},
};
use client::systems::{camera::{camera_follow_system, setup_camera}, keyboard::keyboard_input_system};
use image::DynamicImage;

use shared::{components::elemental_theme::ElementalTheme, systems::{projectile::*, dragon::dragon_movement_system, resource_cache::preload_resources}};
//use shared::resource_cache::ResourceCache;
// use shared::*;
use uuid::Uuid;

// use shared::{game_state::GameState, resource_cache::ResourceCache};
use crate::client::components::game_camera::*;

use crate::shared::components::game_state::*;
use crate::shared::components::dragon::*;
use crate::shared::components::projectile::*;
use crate::shared::components::resource_cache::*;
use crate::shared::components::wall::*;
// and so on for other shared items


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Dragsteroids!".into(),
                resolution: (1200., 800.).into(),
                present_mode: PresentMode::AutoVsync,
                // Tells wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_state::<GameState>()
        .insert_resource(CameraScale(1.0))
        .insert_resource(ResourceCache {
                wall_images: HashMap::new(),
                dragon_images: HashMap::new(),
                projectile_images: HashMap::new(),
        })
        .add_systems(  (preload_resources,
                        setup_dragons.after(preload_resources), 
                        setup_camera.after(setup_dragons),
                ).chain().on_startup()
        )
        .add_system(setup_maze.run_if(in_state(GameState::Setup)))
        .add_systems((
                // keyboard_input_system_ice_dragon.run_if(in_state(GameState::Running)),
                keyboard_input_system.run_if(in_state(GameState::Running)),
                dragon_movement_system.run_if(in_state(GameState::Running)), 
                camera_follow_system.run_if(in_state(GameState::Running)),
                projectile_spawn_system.run_if(in_state(GameState::Running)), 
                projectile_movement_system.run_if(in_state(GameState::Running))
            )
        )
        .add_system(close_on_esc)
        .run();
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
                elemental_theme: mydragon_theme
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
                elemental_theme: ice_dragon_theme 
            }
    });
}


fn setup_maze(
    mut commands: Commands,
//     asset_server: Res<AssetServer>,
    _state: ResMut<State<GameState>>,
//     images: Res<Assets<Image>>,
    mut next_state: ResMut<NextState<GameState>>,
//     wall_texture_handle: Res<WallTextureHandle>,
    resource_cache: Res<ResourceCache>
) {
    println!("Setup Maze");
//     let wall_handle = asset_server.load("wall.png");
//     let wall_sizes = &resource_cache.wall_sizes;
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
        next_state.set(GameState::Running);
        // state .set(GameState::Running).unwrap();
    } else {
        println!("Setup Maze - Image not loaded yet...");
    }
}



