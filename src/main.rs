mod client;
mod server;
mod shared;


use bevy::{
    prelude::*,
    window::PresentMode,
};
use client::UIPlugin;
use shared::systems::{
        GamePlugin,
        ResourceCachePlugin,
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
        .add_plugin(UIPlugin)
        .add_plugin(ResourceCachePlugin)
        .add_plugin(GamePlugin)
        .run();
}


