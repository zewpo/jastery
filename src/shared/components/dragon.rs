use std::sync::Arc;

use bevy::prelude::*;

use uuid::Uuid;
use super::*;

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

// // Not in use yet!
#[derive(Component)]
pub struct DragonStatusText;



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

// pub enum PlayerControllerType {
//     Keyboard,
//     Mouse,
//     GamePad,
//     TouchScreen,
// }
// pub enum AiControllerType {
//     BasicAi,
//     AdvancedAi,
//     // Add more AI types as needed
// }

// pub enum DragonController {
//     PlayerControlled(PlayerControllerType),
//     NonPlayerControlled(AiControllerType),
// }


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