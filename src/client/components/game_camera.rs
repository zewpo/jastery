use bevy::prelude::*;


#[derive(Component)]
pub struct GameCamera {
    pub threshold: f32,  // The threshold before the camera starts scrolling
    pub scale: f32,
}

impl GameCamera {

    /// Touch screen coordinates reference the top left of the screen.
    pub fn touch_to_world(&self, touch_pos: Vec2, window: &Window, camera_translation: &Vec3) -> Vec2 {
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);
        let screen_center = window_size / 2.0;
        let mut screen_space_pos = (touch_pos - screen_center) * self.scale;
        screen_space_pos.y *= -1.0;
        let world_space_pos = Vec2::new(screen_space_pos.x, screen_space_pos.y) + camera_translation.truncate();
        world_space_pos
    }

    /// Mouse click coordinates reference the bottom left of the screen.
    pub fn mouse_to_world(&self, click_pos: Vec2, window: &Window, camera_translation: &Vec3) -> Vec2 {
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);
        let screen_center = window_size / 2.0;
        let screen_space_pos = (click_pos - screen_center) * self.scale;
        // screen_space_pos.y *= -1.0;
        let world_space_pos = Vec2::new(screen_space_pos.x, screen_space_pos.y) + camera_translation.truncate();
        world_space_pos
    }
}

#[derive(Resource)]
pub struct CameraScale(pub f32);
