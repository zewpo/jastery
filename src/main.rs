mod client;
mod server;
mod shared;


use bevy::{
    prelude::*,
    window::{close_on_esc, PresentMode},
};

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
    },
    systems::{
        game::*,
        game::play::*,
        game::menu::*,
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
        .add_state::<AppState>()
        .add_state::<GameOutcome>()
        .insert_resource(CameraScale(3.0))
        .add_plugin(MenuPlugin)
        .add_plugin(ResourceCachePlugin)
        .add_startup_system(setup_camera)
        .add_plugin(GameSetupPlugin)
        .add_systems((
                keyboard_input_system.run_if(in_state(AppState::Running)),
                camera_follow_system.run_if(in_state(AppState::Running)),
            )
        )
        .add_plugin(GamePlayPlugin)
        .add_system(close_on_esc)
        .run();
}


