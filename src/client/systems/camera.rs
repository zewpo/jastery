use bevy::prelude::*;
use crate::client::components::game_camera::*;
use crate::shared::components::dragon::*;

pub fn setup_camera(
    mut commands: Commands,
//     query_window: Query<&Window>,
    camera_scale: Res<CameraScale>,
) {
    println!("Setup Camera");
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_scale(Vec3::splat(camera_scale.0)),
            ..default()
        },
        GameCamera {
            threshold: 250.0,
            scale: camera_scale.0,
        },
    ));
}

pub fn camera_follow_system(
    time: Res<Time>,
    dragon_query: Query<(&Transform, &Handle<Image>, &DragonAction), (With<MyDragon>,Without<GameCamera>)>,
    mut camera_query: Query<(&mut Transform, &GameCamera), With<GameCamera>>,
    windows: Query<&Window>,
    images: Res<Assets<Image>>,
) {
    let window = windows.single();
    let (mut camera_transform, game_camera) = camera_query.single_mut();
    let (dragon_transform, dragon_handle, _dragon_action) = dragon_query.single();

    let dragon_image = images.get(dragon_handle).unwrap();
    let scaled_dragon_size = Vec2::new(dragon_image.size().x * dragon_transform.scale.x.abs(), dragon_image.size().y * dragon_transform.scale.y.abs());

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

    let lerp_rate = 7.0;
    camera_transform.translation = camera_transform.translation.lerp(target_translation, time.delta_seconds() * lerp_rate);
}