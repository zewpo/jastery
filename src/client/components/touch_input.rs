// src/client/components/touch_input.rs

use bevy::prelude::*;

// use crate::shared::components::GamePiece;

#[derive(Resource)]
pub struct TouchAssignments {
    pub joystick_entity_id: Option<Entity>,
    pub move_touch_id: Option<u64>,
    pub shoot_touch_id: Option<u64>,
}

impl Default for TouchAssignments {
    fn default() -> Self {
        Self {
            joystick_entity_id: None,
            move_touch_id: None,
            shoot_touch_id: None,
        }
    }
}

#[derive(Component)]
pub struct VirtualJoystick {
    // pub game_piece: GamePiece,
    pub center: Vec2,
    pub direction: Vec3,
    pub handle_entity: Entity,
}


// #[derive(Resource)]
// pub struct MouseDoubleClickTimer(pub Timer);
// impl Default for MouseDoubleClickTimer {
//     fn default() -> Self {
//         Self (
//             Timer::from_seconds(0.3, TimerMode::Once)
//         )
//     }
// }


#[derive(Resource)]
pub struct DoubleClickState {
    pub last_release_time: Option<f64>,
    pub max_delay: f64,
    pub still_pressed: bool,
}

impl Default for DoubleClickState {
    fn default() -> Self {
        DoubleClickState {
            last_release_time: None,
            max_delay: 0.25, // The maximum delay between clicks for a double-click, in seconds
            still_pressed: false,
        }
    }
}