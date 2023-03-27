use bevy::prelude::*;
use crate::shared::components::{dragon::*};

pub fn ice_dragon_ai_system(
    time: Res<Time>,
    mut ice_dragon_query: Query<(&mut Dragon, &Transform), Without<MyDragon>>,
    fire_dragon_query: Query<&Transform, With<MyDragon>>,
) {
    if let Ok(fire_dragon_transform) = fire_dragon_query.get_single() {
        for (mut ice_dragon, ice_dragon_transform) in ice_dragon_query.iter_mut() {
            let fire_dragon_position = fire_dragon_transform.translation;

            // Calculate direction towards the fire dragon
            let direction = fire_dragon_position - ice_dragon_transform.translation;
            ice_dragon.input.move_direction = Vec3::new(
                direction.x.signum(),
                direction.y.signum(),
                0.
            );

            // Randomly decide when to shoot
            if time.elapsed_seconds() % 1.0 < 0.1 {
                ice_dragon.input.fire = true;
            } else {
                ice_dragon.input.fire = false;
            }
        }
    }
}
