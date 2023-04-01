// src\shared\components\collideable_image.rs

use bevy::prelude::*;
use image::DynamicImage;
use std::collections::{HashMap,HashSet};
use std::convert::From;

use super::*;
// use super::WallShape;

pub struct CollidableImageSize {
    pub width: i32,
    pub height: i32,
}

trait ToSizeVec2 {
    fn size_vec2(self) -> Vec2;
}

impl ToSizeVec2 for CollidableImageSize {
    fn size_vec2(self) -> Vec2 {
        Vec2::new(self.width as f32, self.height as f32)
    }
}

impl From<(i32, i32)> for CollidableImageSize {
    fn from(tuple: (i32, i32)) -> Self {
        Self {
            width: tuple.0,
            height: tuple.1,
        }
    }
}

impl From<Vec2> for CollidableImageSize {
    fn from(vec2: Vec2) -> Self {
        Self {
            width: vec2.x as i32,
            height: vec2.y as i32,
        }
    }
}

impl From<&DynamicImage> for CollidableImageSize {
    fn from(image: &DynamicImage) -> Self {
        Self {
            width: image.width() as i32,
            height: image.height() as i32,
        }
    }
}

pub enum CollidableClassifier {
    Dragon(ElementalTheme),
    Projectile(ElementalTheme),
    Wall(WallShape),
}

pub struct CollidableImage {
    pub classifier: CollidableClassifier,
    // pub size: (i32, i32),
    pub size: CollidableImageSize,
    pub pixel_data: DynamicImage,
    pub image_handle: Handle<Image>,
    pub opaque_pixel_cells: HashMap<(i32, i32), HashSet<(i32, i32)>>,
}

impl CollidableImage {
    pub fn height_i32(&self) -> i32 {
        self.size.height
    }

    pub fn width_i32(&self) -> i32 {
        self.size.width
    }

    pub fn height_f32(&self) -> f32 {
        self.size.height as f32
    }

    pub fn width_f32(&self) -> f32 {
        self.size.width as f32
    }

    pub fn size_vec2(&self) -> Vec2 {
        Vec2::new(self.width_f32() , self.height_f32())
    }

    // pub fn size_i32(&self) -> (i32, i32) {
    //     (self.size.width , self.size.height)
    // }

    pub fn handle(&self) -> Handle<Image> {
        self.image_handle.clone()
    }
}