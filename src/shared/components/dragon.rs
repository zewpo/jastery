use bevy::prelude::*;

use uuid::Uuid;
use crate::shared::components::{
    ElementalTheme,
    CollidableImage,
    GamePiece
};
//Dragon, DragonImage, MyDragon, MyDragonBundle, DragonBundle, DragonInput, and DragonAction 



pub struct DragonImage {
    pub elemental_theme: ElementalTheme,
    pub image: CollidableImage
}

impl DragonImage {
    pub fn height(&self) -> i32 {
        self.image.height()
    }
    pub fn width(&self) -> i32 {
        self.image.width()
    }
    pub fn size(&self) -> Vec2 {
        Vec2::new(self.image.width() as f32, self.image.height() as f32)
    }
}

#[derive(Component)]
pub struct Dragon{
    pub id: Uuid,
    pub elemental_theme: ElementalTheme,
    pub health: i32,
    pub max_health: i32,
}

// struct HealthText {
//     dragon: Entity,
// }

#[derive(Component)]
pub struct HealthText;

// Marker to query the dragon to control by the local system.
#[derive(Component)]
pub struct MyDragon;

#[derive(Bundle)]
pub struct MyDragonBundle {
    #[bundle]
    pub dragon_bundle: DragonBundle,
    pub my_dragon: MyDragon,
}

#[derive(Bundle)]
pub struct DragonBundle {
    pub game_piece: GamePiece,
    #[bundle]
    pub sprite_bundle: SpriteBundle,
    pub input: DragonInput,
    pub movement: DragonAction,
    pub dragon: Dragon,
}

#[derive(Component, Default)]
pub struct DragonInput {
    pub move_direction: Vec3,
    pub fire_direction: Vec3,
    pub brake: bool,
    pub home: bool,
    pub fire: bool,
}

#[derive(Component)]
pub struct DragonAction {
    pub spawn_home: Vec3,
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub max_velocity: f32,
    pub motion_timer: Timer,
    pub flip_timer: Timer,
    pub firerate_timer: Timer,
    pub flipping: bool,
}
