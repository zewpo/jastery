// src/shared/components/projectile.rs

 use std::sync::Arc;

//Projectile, ProjectileBundle, ProjectileMovement, and ProjectileImage
use bevy::prelude::*;
use uuid::Uuid;

use super::*;

#[derive(Component)]
pub struct Projectile {
    pub elemental_theme: ElementalTheme,
    pub image: Arc<CollidableImage>,
    pub shot_by: Uuid,
}

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub game_piece: GamePiece,
    #[bundle]
    pub sprite_bundle: SpriteBundle,
    pub movement: ProjectileMovement,
    pub projectile: Projectile,
}

#[derive(Component)]
pub struct ProjectileMovement {
    pub speed: Vec3,
    pub despawn_timer: Timer,
}

// pub struct ProjectileImage {
//     // pub size: Vec2,
//     // pub image: DynamicImage,
//     // pub file_handle: Handle<Image>,
//     pub elemental_theme: ElementalTheme,
//     pub image: CollidableImage
// }

// impl ProjectileImage {
//     pub fn height(&self) -> i32 {
//         self.image.height()
//     }
//     pub fn width(&self) -> i32 {
//         self.image.width()
//     }
//     pub fn size_vec2(&self) -> Vec2 {
//         Vec2::new(self.image.width() as f32, self.image.height() as f32)
//     }
//     pub fn handle(&self) -> Handle<Image> {
//         self.image.file_handle
//     }
//     // pub fn size_i32(&self) -> (i32, i32) {
//     //     (self.image.width(), self.image.height())
//     // }
// }