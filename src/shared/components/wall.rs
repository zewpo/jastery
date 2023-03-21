use bevy::prelude::*;
use image::DynamicImage;

#[derive(Component)]
pub struct Wall {
    pub shape: WallShape
}

#[derive(Bundle)]
pub struct WallBundle {
    #[bundle]
    pub sprite_bundle: SpriteBundle,
    pub wall: Wall,
}

pub struct WallImage {
    pub size: Vec2,
    pub image: DynamicImage,
    pub file_handle: Handle<Image>,
    pub shape: WallShape,
}


#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum WallShape {
    Straight,
    Corner,
    TJunction,
    Cross,
    ShortStraight,
    LongStraight,
    Diagonal,
    Curved,
    Narrow,
}
