mod client;
mod server;
mod shared;


use bevy::{
    prelude::*,
    window::PresentMode,
};
// use shared::systems::GamePlugin;
// use client::UIPlugin;


use crate::shared::{
    components::{
        game::*,
    },
    systems::{
        // game,
        GamePlugin,
        // game::play::*,
        game::screen::*,
        resource_cache::*,
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Jastery!".into(),
                resolution: (1400., 800.).into(),
                present_mode: PresentMode::AutoVsync,
                // Tells wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_state::<AppScreen>()
        // .add_state::<GameOutcome>()
        .add_plugin(ResourceCachePlugin)
        .add_plugin(ScreenManagerPlugin)
        .add_plugin(client::UIPlugin)
        .add_plugin(GamePlugin)
        // .add_system(close_on_esc)
        .run();
}


