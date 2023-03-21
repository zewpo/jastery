use bevy::prelude::*;
use image::DynamicImage;
use crate::shared::components::dragon::*;
use crate::shared::components::elemental_theme::*;
use crate::shared::components::projectile::*;
use crate::shared::components::resource_cache::*;
use crate::shared::components::wall::*;

fn load_image_data(path: &str) -> DynamicImage {
    let image_bytes = std::fs::read( "assets/".to_owned() + path).expect("Failed to read image file");
    let image_data = image::load_from_memory(&image_bytes).expect("Failed to load image data");
    
    image_data
}

// //, mut materials: ResMut<Assets<ColorMaterial>>) {
pub fn preload_resources(mut _commands: Commands, asset_server: Res<AssetServer>, mut resource_cache: ResMut<ResourceCache>) { 
    
    let wall_shape_file_names = vec![
        (WallShape::Straight, "sprites/wall-straight.png"),
        // Add more wall types and their paths here
    ];

    let theme_image_file_names = vec![
        (ElementalTheme::Fire, "sprites/fire-dragon.png", "sprites/fire-projectile.png"),
        (ElementalTheme::Ice, "sprites/ice-dragon.png", "sprites/ice-projectile.png"),
        (ElementalTheme::Water, "sprites/water-dragon.png", "sprites/water-projectile.png"),
        (ElementalTheme::Rock, "sprites/rock-dragon.png", "sprites/rock-projectile.png"),
        // Add more themes and their file paths here
    ];

    // let mut resource_cache = ResourceCache {
    //     wall_images: HashMap::new(),
    //     dragon_images: HashMap::new(),
    //     projectile_images: HashMap::new(),
    // };

    // Preload the walls
    for (shape, path) in wall_shape_file_names {
        let wall_handle: Handle<Image> = asset_server.load(path);
        let wall_image = load_image_data(path);
        let wall_size = Vec2::new(wall_image.width() as f32, wall_image.height() as f32);
        //let wall_size = wall_image.size().extend(0.0) * wall_transform.scale.abs();
        
        let wall_data = WallImage {
            size: wall_size,
            image: wall_image,
            file_handle: wall_handle,
            shape,
        };

        resource_cache.wall_images.insert(shape, wall_data);
    }

    // Preloading projectiles
    for (elemental_theme, dragon_image_file_path, projectile_image_file_path) in theme_image_file_names {
        
        let dragon_handle: Handle<Image> = asset_server.load(dragon_image_file_path);
        let dragon_image_data = load_image_data(dragon_image_file_path);
        let dragon_size = Vec2::new(dragon_image_data.width() as f32, dragon_image_data.height() as f32);

        let dragon_image = DragonImage {
            size: dragon_size,
            image: dragon_image_data,
            file_handle: dragon_handle,
            elemental_theme,
        };
        resource_cache.dragon_images.insert(elemental_theme, dragon_image);

        let projectile_handle: Handle<Image> = asset_server.load(projectile_image_file_path);
        let projectile_image_data = load_image_data(projectile_image_file_path);
        let projectile_size = Vec2::new(projectile_image_data.width() as f32, projectile_image_data.height() as f32);

        let projectile_image = ProjectileImage {
            size: projectile_size,
            image: projectile_image_data,
            file_handle: projectile_handle,
            elemental_theme,
        };
        resource_cache.projectile_images.insert(elemental_theme, projectile_image);
    }

    // commands.insert_resource(resource_cache);

    
//     materials.set(dragon_handle.clone(), ColorMaterial::from(Handle::from(dragon_handle)));
//     materials.set(wall_handle.clone(), ColorMaterial::from(Handle::from(wall_handle)));

}
