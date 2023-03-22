mod client;
mod server;
mod shared;


use bevy::{
    prelude::*,
    utils::HashMap,
    window::{close_on_esc, PresentMode},
};
// use shared::systems::game::play::*;

use crate::client::{
    components::game_camera::*,
    systems::{
        camera::{camera_follow_system, setup_camera},
        keyboard::keyboard_input_system,
    },
};

use crate::shared::{
    components::{
        game::*,
        resource_cache::*,
    },
    systems::{
        game::*,
        game::play::*,
        resource_cache::*,
    },
};


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
        .add_state::<GameOutcome>()
        .insert_resource(CameraScale(3.0))
        .insert_resource(ResourceCache {
                wall_images: HashMap::new(),
                dragon_images: HashMap::new(),
                projectile_images: HashMap::new(),
        })
        .add_startup_systems( (
            preload_resources,
            setup_camera))
        .add_plugin(GameSetupPlugin)
        // .add_system(setup_maze.run_if(in_state(GameState::Setup)))
        .add_systems((
                // keyboard_input_system_ice_dragon.run_if(in_state(GameState::Running)),
                keyboard_input_system.run_if(in_state(GameState::Running)),
                camera_follow_system.run_if(in_state(GameState::Running)),
                // game_play_system
            )
        )
        .add_plugin(GamePlayPlugin)
        .add_system(close_on_esc)
        .run();
}


