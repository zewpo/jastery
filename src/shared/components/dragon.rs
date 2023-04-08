use std::sync::Arc;

use bevy::prelude::*;

use uuid::Uuid;
use super::*;
// use crate::shared::components::{
//     ElementalTheme,
//     CollidableImage,
//     GamePiece
// };
//Dragon, DragonImage, MyDragon, MyDragonBundle, DragonBundle, DragonInput, and DragonAction 



// pub struct DragonImage {
//     pub elemental_theme: ElementalTheme,
//     pub image: CollidableImage
// }

// // impl DragonImage {
//     pub fn height(&self) -> i32 {
//         self.image.height()
//     }
//     pub fn width(&self) -> i32 {
//         self.image.width()
//     }
//     pub fn size_vec2(&self) -> Vec2 {
//         Vec2::new(self.image.width() as f32, self.image.height() as f32)
//     }
//     pub fn size_i32(&self) -> (i32, i32) {
//         (self.image.width(), self.image.height())
//     }
//     pub fn handle(&self) -> Handle<Image> {
//         self.image.file_handle
//     }
// // }


#[derive(Component, Clone)]
pub struct Dragon {
    pub input: DragonInput,
    pub action: DragonAction,
    pub id: Uuid,
    pub elemental_theme: ElementalTheme,
    pub health: i32,
    pub max_health: i32,
    pub max_velocity: f32,
    pub shooting_frequence: f32, // a decimal number representing percent of time it can shoot for.
    pub image: Arc<CollidableImage>,
    pub my_dragon: Option<MyDragon>,
}

// struct HealthText {
//     dragon: Entity,
// }

#[derive(Component)]
pub struct HealthText;

// Marker to query the dragon to control by the local system.
#[derive(Component, Clone)]
pub struct MyDragon;

// #[derive(Bundle)]
// pub struct MyDragonBundle {
//     #[bundle]
//     pub dragon_bundle: DragonBundle,
//     pub my_dragon: MyDragon,
// }

#[derive(Bundle)]
pub struct DragonBundle {
    #[bundle]
    pub sprite_bundle: SpriteBundle,
    pub dragon: Dragon,
    pub game_piece: GamePiece,
}

pub enum PlayerControllerType {
    Keyboard,
    Joystick,
    Mouse,
}
pub enum AiControllerType {
    BasicAi,
    AdvancedAi,
    // Add more AI types as needed
}

pub enum DragonController {
    PlayerControlled(PlayerControllerType),
    NonPlayerControlled(AiControllerType),
}


#[derive(Default, Clone)]
pub struct DragonInput {
    pub move_direction: Vec3,
    pub shoot_direction: Vec3,
    pub brake: bool,
    pub home: bool,
    pub shoot: bool,
}

#[derive(Clone)]
pub struct DragonAction {
    pub spawn_home: Vec3,
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub motion_timer: Timer,
    pub flip_timer: Timer,
    pub shooting_rate_timer: Timer,
    pub flipping: bool,
    pub pathfinding_timer: Option<Timer>,
    pub path: Option<Vec<(i32, i32)>>,
}

impl Default for DragonAction {
    fn default() -> Self {
        Self {
            spawn_home: Vec3::ZERO,
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            motion_timer: Timer::from_seconds(0.05, TimerMode::Repeating),
            shooting_rate_timer: Timer::from_seconds(0.15, TimerMode::Repeating),
            flip_timer: Timer::from_seconds(0.1, TimerMode::Once),
            flipping: false,
            path: None,
            pathfinding_timer: None,
        }
    }
}