use bevy::prelude::*;


#[derive(Component)]
pub struct GameCamera {
    pub threshold: f32,  // The threshold before the camera starts scrolling
    pub scale: f32,
}

#[derive(Resource)]
pub struct CameraScale(pub f32);
