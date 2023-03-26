// src\shared\components\collideable_image.rs

use bevy::prelude::*;
use image::DynamicImage;
use std::collections::{HashMap,HashSet};
use std::convert::From;

pub struct I32ImageSize {
    pub width: i32,
    pub height: i32,
}

trait ToVec2 {
    fn to_vec2(self) -> Vec2;
}

impl ToVec2 for I32ImageSize {
    fn to_vec2(self) -> Vec2 {
        Vec2::new(self.width as f32, self.height as f32)
    }
}

impl From<(i32, i32)> for I32ImageSize {
    fn from(tuple: (i32, i32)) -> Self {
        Self {
            width: tuple.0,
            height: tuple.1,
        }
    }
}

impl From<Vec2> for I32ImageSize {
    fn from(vec2: Vec2) -> Self {
        Self {
            width: vec2.x as i32,
            height: vec2.y as i32,
        }
    }
}

impl From<&DynamicImage> for I32ImageSize {
    fn from(image: &DynamicImage) -> Self {
        Self {
            width: image.width() as i32,
            height: image.height() as i32,
        }
    }
}

impl CollidableImage {
    pub fn height(&self) -> i32 {
        self.size.height
    }

    pub fn width(&self) -> i32 {
        self.size.width
    }
}

// trait FromTuple {
// }


pub struct CollidableImage {
    pub size: I32ImageSize,
    pub pixel_data: DynamicImage,
    pub file_handle: Handle<Image>,
    pub opaque_pixel_cells: HashMap<(i32, i32), HashSet<(i32, i32)>>, // cell to non-transparent pixels map
}
