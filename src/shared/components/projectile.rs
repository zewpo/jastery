 //Projectile, ProjectileBundle, ProjectileMovement, and ProjectileImage
use bevy::prelude::*;
use image::DynamicImage;
use crate::shared::components::elemental_theme::ElementalTheme;
use crate::shared::components::game::GamePiece;
#[derive(Component)]
pub struct Projectile {
    pub elemental_theme: ElementalTheme,
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

pub struct ProjectileImage {
    pub size: Vec2,
    pub image: DynamicImage,
    pub file_handle: Handle<Image>,
    pub elemental_theme: ElementalTheme,
}
