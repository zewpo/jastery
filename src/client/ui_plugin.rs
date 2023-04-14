// src/client/ui_plugin.rs

use bevy::prelude::*;

use crate::client::systems::{
    ScreenManagerPlugin,
    GameCameraPlugin,
    UserInputPlugin,
};
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(ScreenManagerPlugin)
        .add_plugin(GameCameraPlugin)
        .add_plugin(UserInputPlugin);
    }
}
