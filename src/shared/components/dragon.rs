use bevy::prelude::*;
use image::DynamicImage;
use uuid::Uuid;
use crate::shared::components::elemental_theme::ElementalTheme;
//Dragon, DragonImage, MyDragon, MyDragonBundle, DragonBundle, DragonInput, and DragonAction 


#[derive(Component)]
pub struct Dragon{
    pub id: Uuid,
    pub elemental_theme: ElementalTheme,
}

pub struct DragonImage {
    pub size: Vec2,
    pub image: DynamicImage,
    pub file_handle: Handle<Image>,
    pub elemental_theme: ElementalTheme,
}

// Marker to query the dragon to control by the local system.
#[derive(Component)]
pub struct MyDragon;

#[derive(Bundle)]
pub struct MyDragonBundle {
    #[bundle]
    pub dragon_bundle: DragonBundle,
    pub my_dragon: MyDragon,
}

#[derive(Bundle)]
pub struct DragonBundle {
    #[bundle]
    pub sprite_bundle: SpriteBundle,
    pub input: DragonInput,
    pub movement: DragonAction,
    pub dragon: Dragon,
}

#[derive(Component, Default)]
pub struct DragonInput {
    pub move_direction: Vec2,
    pub brake: bool,
    pub home: bool,
    pub ease_up: bool,
    pub fire: bool,
}

#[derive(Component)]
pub struct DragonAction {
    pub spawn_home: Vec3,
    pub velocity: Vec3,
    pub max_velocity: f32,
    pub motion_timer: Timer,
    pub flip_timer: Timer,
    pub firerate_timer: Timer,
    pub flipping: bool,
}
