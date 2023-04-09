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
