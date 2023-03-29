// use std::collections::HashMap;
use bevy::prelude::*;
// use image::DynamicImage;
// use bevy::utils::HashMap;
use std::collections::HashMap;
use std::sync::Arc;

use crate::shared::components::{WallShape, CollidableImage};
// use crate::shared::components::DragonImage;
// use crate::shared::components::ProjectileImage;
use crate::shared::components::ElementalTheme;

use super::*;

pub const CELL_SIZE: i32 = 100;

#[derive(Resource)]
pub struct ResourceCache {
    pub wall_images: HashMap<WallShape, Arc<CollidableImage>>,
    pub dragon_images: HashMap<ElementalTheme, Arc<CollidableImage>>,
    pub projectile_images: HashMap<ElementalTheme, Arc<CollidableImage>>,
    pub gui_fonts: HashMap<String, Handle<Font>>,
    // Other resources can be added here, e.g., audio files, character data, etc.
}

impl ResourceCache {
    pub fn get_collidable_image(&self, classifier: CollidableClassifier) -> Arc<CollidableImage> {
        match classifier {
            CollidableClassifier::Dragon(elemental_theme) => {
                self.dragon_images.get(&elemental_theme).unwrap().clone()
            },
            CollidableClassifier::Projectile(elemental_theme) => {
                self.projectile_images.get(&elemental_theme).unwrap().clone()
            },
            CollidableClassifier::Wall(wall_shape) => {
                self.wall_images.get(&wall_shape).unwrap().clone()
            },
        }
    }
}