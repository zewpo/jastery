use bevy::prelude::*;
use std::sync::Arc;

use crate::shared::components::*;

#[derive(Component)]
pub struct Wall {
    pub shape: WallShape,
    pub image: Arc<CollidableImage>
}

#[derive(Bundle)]
pub struct WallBundle {
    pub game_piece: GamePiece,
    #[bundle]
    pub sprite_bundle: SpriteBundle,
    pub wall: Wall,
}

// pub struct WallImage {
//     pub shape: WallShape,
//     pub image: CollidableImage,
// }

// impl WallImage {
//     pub fn height(&self) -> i32 {
//         self.image.height()
//     }
//     pub fn width(&self) -> i32 {
//         self.image.width()
//     }
//     pub fn size_vec2(&self) -> Vec2 {
//         Vec2::new(self.image.width() as f32, self.image.height() as f32)
//     }
//     // pub fn size_i32(&self) -> (i32, i32) {
//     //     (self.image.width(), self.image.height())
//     // }
// }

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum WallShape {
    Straight,
    // Corner,
    // TJunction,
    // Cross,
    // ShortStraight,
    // LongStraight,
    // Diagonal,
    // Curved,
    // Narrow,
}
