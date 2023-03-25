use bevy::prelude::*;
use image::DynamicImage;
use image::GenericImageView;
// use bevy::utils::HashMap;
use std::collections::{HashMap, HashSet};
use crate::shared::components::CollidableImage;
use crate::shared::components::I32ImageSize;
// use image::GenericImageView;
use crate::shared::components::dragon::*;
use crate::shared::components::elemental_theme::*;
use crate::shared::components::projectile::*;
use crate::shared::components::resource_cache::*;
use crate::shared::components::wall::*;

pub struct ResourceCachePlugin;


impl Plugin for ResourceCachePlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(ResourceCache {
            wall_images: HashMap::new(),
            dragon_images: HashMap::new(),
            projectile_images: HashMap::new(),
            gui_fonts: HashMap::new(),
        })
        .add_startup_system(preload_resources);
    }
}

fn load_image_data(path: &str) -> DynamicImage {
    let image_bytes = std::fs::read( "assets/".to_owned() + path).expect("Failed to read image file");
    let image_data = image::load_from_memory(&image_bytes).expect("Failed to load image data");
    
    image_data
}


// fn find_non_transparent_pixels(image: &DynamicImage) -> std::collections::HashSet<(u32, u32)> {
//     let mut non_transparent_pixels = std::collections::HashSet::new();
//     let width = image.width();
//     let height = image.height();

//     for x in 0..width {
//         for y in 0..height {
//             let pixel = image.get_pixel(x, y);
//             if pixel[3] != 0 { // If the alpha channel is not transparent
//                 non_transparent_pixels.insert((x, y));
//             }
//         }
//     }
//     non_transparent_pixels
// }


fn get_non_transparent_pixels_per_cell(
    image: &DynamicImage,
    cell_size: i32,
) -> HashMap<(i32, i32), HashSet<(i32, i32)>> {
    let cell_size = cell_size as u32;
    let mut non_transparent_pixels: HashMap<(i32, i32), HashSet<(i32, i32)>> = HashMap::new();
    let (width, height) = image.dimensions();

    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            //let rgba = pixel.to_rgba8();
            if pixel[3] != 0 {
                let cell_x = (x / cell_size) as i32;
                let cell_y = (y / cell_size) as i32;
                let cell_key = (cell_x, cell_y);

                let pixel_in_cell = ((x % cell_size) as i32, (y % cell_size) as i32);

                non_transparent_pixels
                    .entry(cell_key)
                    .or_insert_with(HashSet::new)
                    .insert(pixel_in_cell);
            }
        }
    }
    non_transparent_pixels
}

//In this function, we use the GenericImageView::dimensions() method to get the width and height of the image, and then we iterate over the pixels using nested for loops. We use the GenericImageView::get_pixel() method to get the pixel at the current position, and then we check if its alpha value is not zero. If it's not transparent, we calculate the cell position and the pixel position within the cell, and then we store this information in the non_transparent_pixels HashMap.







// fn get_non_transparent_pixels_per_cell(image: &DynamicImage, cell_size: u32) -> HashMap<(u32, u32), HashSet<(u32, u32)>> {
//     let mut non_transparent_pixels: HashMap<(u32, u32), HashSet<(u32, u32)>> = HashMap::new();

//     for (x, y, pixel) in image.enumerate_pixels() {
//         if pixel[3] != 0 {
//             let cell_x = x / cell_size;
//             let cell_y = y / cell_size;
//             let cell_key = (cell_x, cell_y);

//             let pixel_in_cell = (x % cell_size, y % cell_size);

//             non_transparent_pixels.entry(cell_key).or_insert_with(HashSet::new).insert(pixel_in_cell);
//         }
//     }

//     non_transparent_pixels
// }


// fn precompute_non_transparent_pixels(image: &DynamicImage, cell_size: u32) -> HashSet<(u32, u32)> {
//     let mut non_transparent_pixels = HashSet::new();
//     let width = image.width();
//     let height = image.height();

//     for y in (0..height).step_by(cell_size as usize) {
//         for x in (0..width).step_by(cell_size as usize) {
//             if cell_has_non_transparent_pixels(image, x, y, cell_size) {
//                 non_transparent_pixels.insert((x, y));
//             }
//         }
//     }
//     non_transparent_pixels
// }

pub fn preload_resources(
    mut _commands: Commands, 
    asset_server: Res<AssetServer>,
    mut resource_cache: ResMut<ResourceCache>
) { 


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

    // Preload the walls
    for (shape, path) in wall_shape_file_names {
        let file_handle: Handle<Image> = asset_server.load(path);
        let pixel_data = load_image_data(path);
        let size = I32ImageSize::from(&pixel_data);// (pixel_data.width() as i32, pixel_data.height() as i32);
        //let wall_size = wall_image.size().extend(0.0) * wall_transform.scale.abs();
        
        let non_transparent_pixels = get_non_transparent_pixels_per_cell(&pixel_data, CELL_SIZE);
        let wall_image = WallImage {
            shape,
            image:  CollidableImage {
                size,
                pixel_data,
                file_handle,
                non_transparent_pixels
            }
            // size: wall_size,
            // image: wall_image_data,
            // file_handle: wall_handle,
            // non_transparent_pixels,
        };

        resource_cache.wall_images.insert(shape, wall_image);
    }

    // Preloading dragons and projectiles
    for (elemental_theme, dragon_image_file_path, projectile_image_file_path) in theme_image_file_names {
        
        let file_handle: Handle<Image> = asset_server.load(dragon_image_file_path);
        let pixel_data = load_image_data(dragon_image_file_path);
        let size = I32ImageSize::from(&pixel_data);  //  (pixel_data.width() as i32, pixel_data.height() as i32);

        let non_transparent_pixels = get_non_transparent_pixels_per_cell(&pixel_data, CELL_SIZE);
        let dragon_image = DragonImage {
            elemental_theme,
            image:  CollidableImage {
                size,
                pixel_data,
                file_handle,
                non_transparent_pixels
            }
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


    // preloads all the ttf file handles from the assets/fonts/ directory.
    let dir = std::path::Path::new("assets/fonts/");
    let paths = std::fs::read_dir(dir).unwrap();

    for entry in paths {
        let path = entry.unwrap().path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "ttf" {
                    if let Some(file_name) = path.file_name() {
                        if let Some(file_name_str) = file_name.to_str() {
                            let handle: Handle<Font> = asset_server.load("fonts/".to_owned() + file_name_str);
                            if let Some(file_stem) = path.file_stem() {
                                if let Some(file_stem_str) = file_stem.to_str() {
                                    resource_cache.gui_fonts.insert(file_stem_str.to_string(), handle);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
