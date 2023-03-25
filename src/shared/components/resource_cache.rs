// use std::collections::HashMap;
use bevy::prelude::*;
// use image::DynamicImage;
// use bevy::utils::HashMap;
use std::collections::HashMap;

use crate::shared::components::{WallShape, WallImage};
use crate::shared::components::DragonImage;
use crate::shared::components::ProjectileImage;
use crate::shared::components::ElementalTheme;

pub const CELL_SIZE: i32 = 8;

#[derive(Resource)]
pub struct ResourceCache {
    pub wall_images: HashMap<WallShape, WallImage>,
    pub dragon_images: HashMap<ElementalTheme, DragonImage>, // DynamicImage,
    pub projectile_images: HashMap<ElementalTheme, ProjectileImage>,
    pub gui_fonts: HashMap<String, Handle<Font>>,
    // Other resources can be added here, e.g., audio files, character data, etc.
}

