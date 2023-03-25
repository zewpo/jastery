use bevy::prelude::*;


use crate::{client::{
    systems::{
        camera::{camera_follow_system, setup_camera},
        keyboard::keyboard_input_system,
    }, components::game_camera::CameraScale,
}, shared::components::game::GamePhase};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
        // .add_plugins(DefaultPlugins.set(WindowPlugin {
        //     primary_window: Some(Window {
        //         title: "Dragsteroids!".into(),
        //         resolution: (1400., 800.).into(),
        //         present_mode: PresentMode::AutoVsync,
        //         // Tells wasm to resize the window according to the available canvas
        //         fit_canvas_to_parent: true,
        //         // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
        //         prevent_default_event_handling: false,
        //         ..default()
        //     }),
        //     ..default()
        // }))
        // .add_state::<AppScreen>()
        .insert_resource(CameraScale(3.0))
        .add_startup_system(setup_camera)
        .add_systems((
            keyboard_input_system,
            camera_follow_system,
        ).in_set(OnUpdate(GamePhase::Playing)))
        ;
    }
}




