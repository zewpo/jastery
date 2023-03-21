// use std::collections::HashMap;
use bevy::prelude::*;
// use image::DynamicImage;
use bevy::utils::HashMap;

use crate::shared::components::wall::{WallShape, WallImage};
use crate::shared::components::dragon::DragonImage;
use crate::shared::components::projectile::ProjectileImage;
use crate::shared::components::elemental_theme::ElementalTheme;


#[derive(Resource)]
pub struct ResourceCache {
    pub wall_images: HashMap<WallShape, WallImage>,
    pub dragon_images: HashMap<ElementalTheme, DragonImage>, // DynamicImage,
    pub projectile_images: HashMap<ElementalTheme, ProjectileImage>,
    // Other resources can be added here, e.g., audio files, character data, etc.
}

