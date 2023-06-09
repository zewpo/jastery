// src/client/systems/camera.rs

use bevy::prelude::*;
use crate::client::components::*;
use crate::shared::components::*;

use super::*;
// use crate::client::components::game_camera::*;
// use crate::shared::components::dragon::*;


pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CameraScale(4.0))
            .add_startup_system(setup_camera)
            .add_system(
                camera_follow_system
                .run_if(|game_status: Res<GameStatus>| game_status.phase == GamePhase::Playing)
                .in_set(OnUpdate(AppScreen::InPlay)));
    }
}

pub fn setup_camera(
    mut commands: Commands,
//     query_window: Query<&Window>,
    camera_scale: Res<CameraScale>,
) {
    info!("Setup Camera");
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_scale(Vec3::splat(camera_scale.0)),
            ..default()
        },
        GameCamera {
            threshold: 250.0,  // distance to edge before camera moves to follow dragon
            scale: camera_scale.0,
        },
    ));

}
pub fn camera_follow_system(
    time: Res<Time>,
    dragon_query: Query<(&mut Dragon, &Transform), (With<MyDragon>, Without<GameCamera>)>,
    mut camera_query: Query<(&GameCamera, &mut Transform), With<GameCamera>>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    let (game_camera, mut camera_transform) = camera_query.single_mut();
    let (dragon, dragon_transform) = dragon_query.single();

    let scaled_dragon_size = Vec2::new(
        dragon.image.width_f32() * dragon_transform.scale.x.abs(),
        dragon.image.height_f32() * dragon_transform.scale.y.abs(),
    );

    let dragon_left_edge = dragon_transform.translation.x - (scaled_dragon_size.x / 2.0);
    let dragon_right_edge = dragon_left_edge + scaled_dragon_size.x;

    let dragon_bottom_edge = dragon_transform.translation.y - (scaled_dragon_size.y / 2.0);
    let dragon_top_edge = dragon_bottom_edge + scaled_dragon_size.y;

    let scaled_window_width = window.width() * game_camera.scale;
    let scaled_window_height = window.height() * game_camera.scale;

    let window_left_edge = camera_transform.translation.x - (scaled_window_width / 2.0);
    let window_right_edge = window_left_edge + scaled_window_width;
    let window_bottom_edge = camera_transform.translation.y - (scaled_window_height / 2.0);
    let window_top_edge = window_bottom_edge + scaled_window_height;

    let margin = game_camera.threshold * game_camera.scale;

    let mut target_translation = camera_transform.translation;

    if dragon_left_edge < window_left_edge + margin {
        target_translation.x -= (dragon_left_edge - (window_left_edge + margin)).abs();
    } else if dragon_right_edge > window_right_edge - margin {
        target_translation.x += (dragon_right_edge - (window_right_edge - margin)).abs();
    }

    if dragon_bottom_edge < window_bottom_edge + margin {
        target_translation.y -= (dragon_bottom_edge - (window_bottom_edge + margin)).abs();
    } else if dragon_top_edge > window_top_edge - margin {
        target_translation.y += (dragon_top_edge - (window_top_edge - margin)).abs();
    }

    let lerp_t = time.delta_seconds() * 7.0;
    let smooth_t = smoothstep(0.0, 1.0, lerp_t);

    camera_transform.translation = camera_transform.translation.lerp(target_translation, smooth_t);
}

// Smoothstep function
fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

