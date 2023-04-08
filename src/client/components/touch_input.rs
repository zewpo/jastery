use bevy::prelude::*;

#[derive(Resource)]
pub struct TouchAssignments {
    pub move_touch_id: Option<u64>,
    pub shoot_touch_id: Option<u64>,
}

impl Default for TouchAssignments {
    fn default() -> Self {
        Self {
            move_touch_id: None,
            shoot_touch_id: None,
        }
    }
}

#[derive(Component)]
pub struct VirtualJoystick {
    pub center: Vec2,
    pub direction: Vec3,
    pub handle_entity: Entity,
}
