use bevy::prelude::*;
use image::DynamicImage;
use image::GenericImageView;
// use bevy::utils::HashMap;
use std::collections::{HashMap, HashSet};
// use std::fs::File;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;

use crate::shared::components::*;
// use crate::shared::components::I32ImageSize;
// // use image::GenericImageView;
// use crate::shared::components::dragon::*;
// use crate::shared::components::elemental_theme::*;
// use crate::shared::components::projectile::*;
// use crate::shared::components::resource_cache::*;
// use crate::shared::components::wall::*;

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


// fn find_opaque_pixel_cells(image: &DynamicImage) -> std::collections::HashSet<(u32, u32)> {
//     let mut opaque_pixel_cells = std::collections::HashSet::new();
//     let width = image.width();
//     let height = image.height();

//     for x in 0..width {
//         for y in 0..height {
//             let pixel = image.get_pixel(x, y);
//             if pixel[3] != 0 { // If the alpha channel is not transparent
//                 opaque_pixel_cells.insert((x, y));
//             }
//         }
//     }
//     opaque_pixel_cells
// }
fn get_opaque_pixel_cells(
    image: &DynamicImage,
) -> HashMap<(i32, i32), HashSet<(i32, i32)>> {
    let cell_size = CELL_SIZE as u32;
    let mut opaque_pixel_cells: HashMap<(i32, i32), HashSet<(i32, i32)>> = HashMap::new();
    let (width, height) = image.dimensions();

    let is_transparent = |x: u32, y: u32| image.get_pixel(x, y)[3] == 0;

    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            if pixel[3] != 0 {
                let neighbors = [
                    (x.checked_sub(1).map_or(true, |nx| is_transparent(nx, y))),
                    (x + 1 >= width || is_transparent(x + 1, y)),
                    (y.checked_sub(1).map_or(true, |ny| is_transparent(x, ny))),
                    (y + 1 >= height || is_transparent(x, y + 1)),
                ];

                let is_edge_pixel = neighbors.iter().any(|&is_transparent| is_transparent);

                if is_edge_pixel {
                    let cell_x = (x / cell_size) as i32;
                    let cell_y = (y / cell_size) as i32;
                    let cell_key = (cell_x, cell_y);

                    let pixel_in_cell = ((x % cell_size) as i32, (y % cell_size) as i32);

                    opaque_pixel_cells
                        .entry(cell_key)
                        .or_insert_with(HashSet::new)
                        .insert(pixel_in_cell);
                }
            }
        }
    }
    opaque_pixel_cells
}



// fn get_opaque_pixel_cells(image: &DynamicImage, cell_size: u32) -> HashMap<(u32, u32), HashSet<(u32, u32)>> {
//     let mut opaque_pixel_cells: HashMap<(u32, u32), HashSet<(u32, u32)>> = HashMap::new();

//     for (x, y, pixel) in image.enumerate_pixels() {
//         if pixel[3] != 0 {
//             let cell_x = x / cell_size;
//             let cell_y = y / cell_size;
//             let cell_key = (cell_x, cell_y);

//             let pixel_in_cell = (x % cell_size, y % cell_size);

//             opaque_pixel_cells.entry(cell_key).or_insert_with(HashSet::new).insert(pixel_in_cell);
//         }
//     }

//     opaque_pixel_cells
// }


// fn precompute_opaque_pixel_cells(image: &DynamicImage, cell_size: u32) -> HashSet<(u32, u32)> {
//     let mut opaque_pixel_cells = HashSet::new();
//     let width = image.width();
//     let height = image.height();

//     for y in (0..height).step_by(cell_size as usize) {
//         for x in (0..width).step_by(cell_size as usize) {
//             if cell_has_opaque_pixel_cells(image, x, y, cell_size) {
//                 opaque_pixel_cells.insert((x, y));
//             }
//         }
//     }
//     opaque_pixel_cells
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
        let image_handle: Handle<Image> = asset_server.load(path);
        let pixel_data = load_image_data(path);
        let size = ImageSizeI32::from(&pixel_data);// (pixel_data.width() as i32, pixel_data.height() as i32);
        //let wall_size = wall_image.size().extend(0.0) * wall_transform.scale.abs();
        
        let opaque_pixel_cells = get_opaque_pixel_cells(&pixel_data);
        
        //////////////////////////////////////////////////////////////
        // debugging where the opaque cells are
        let mut cell_keys: Vec<&(i32, i32)> = opaque_pixel_cells.keys().collect();
        cell_keys.sort_by(|a, b| (a.1, a.0).cmp(&(b.1, b.0)));

        let mut current_cell_y = None;
        let mut prev_cell_x: Option<i32> = None;

        // Create a new file or truncate an existing file
        let mut file = File::create("wall_output.txt").expect("Unable to create file");

        writeln!(&mut file, "\n\n").expect("Unable to write to file");
        for cell_key in cell_keys {
            if current_cell_y.is_some() && current_cell_y != Some(cell_key.1) {
                writeln!(&mut file, "").expect("Unable to write to file");
                prev_cell_x = None;
            }

            // Print X number of tabs at the beginning of the line when starting a new line
            if current_cell_y != Some(cell_key.1) {
                for _ in 0..cell_key.0 {
                    write!(&mut file, "\t").expect("Unable to write to file");
                }
            }

            // Insert tab characters based on the difference in x values of consecutive cells
            if let Some(prev_x) = prev_cell_x {
                for _ in 0..(cell_key.0 - prev_x - 1) {
                    write!(&mut file, "\t").expect("Unable to write to file");
                }
            }

            current_cell_y = Some(cell_key.1);
            prev_cell_x = Some(cell_key.0);

            // Format cell_key with fixed width of 8 characters
            let formatted_cell_key = format!("({:2},{:2})", cell_key.0, cell_key.1);
            write!(&mut file, "{:8}", formatted_cell_key).expect("Unable to write to file");
        }
        writeln!(&mut file, "\n\n").expect("Unable to write to file");


        /////////////////////////////////////////////////////



        // let mut cell_keys: Vec<&(i32, i32)> = opaque_pixel_cells.keys().collect();
        // cell_keys.sort();

        // for cell_key in cell_keys {
        //     println!("Cell key: {:?}", cell_key);
        // }

        // for (cell_key, pixel_set) in opaque_pixel_cells.iter() {
        //     println!("Cell key: {:?}", cell_key);
        //     // println!("Pixel set: {:?}", pixel_set);
        // }
        
        let wall_image = Arc::new(CollidableImage {
                size,
                pixel_data,
                image_handle,
                opaque_pixel_cells,
                classifier: CollidableClassifier::Wall(WallShape::Straight),
        });
        resource_cache.wall_images.insert(shape, wall_image);
    }

    // Preload the Dragons and the Projectiles
    for (elemental_theme, dragon_image_file_path, projectile_image_file_path) in theme_image_file_names {
        
        let dragon_image_handle: Handle<Image> = asset_server.load(dragon_image_file_path);
        let dragon_image_pixel_data = load_image_data(dragon_image_file_path);
        let dragon_image_size = ImageSizeI32::from(&dragon_image_pixel_data);  //  (pixel_data.width() as i32, pixel_data.height() as i32);

        let dragon_image_opaque_pixel_cells = get_opaque_pixel_cells(&dragon_image_pixel_data);

        //////////////////////////////////////////////////////////////
        // debugging where the opaque cells are
        let mut cell_keys: Vec<&(i32, i32)> = dragon_image_opaque_pixel_cells.keys().collect();
        cell_keys.sort_by(|a, b| (a.1, a.0).cmp(&(b.1, b.0)));

        let mut current_cell_y = None;
        let mut prev_cell_x: Option<i32> = None;

        // Create a new file or truncate an existing file
        let mut file = File::create("dragon_output.txt").expect("Unable to create file");

        writeln!(&mut file, "\n\n").expect("Unable to write to file");
        for cell_key in cell_keys {
            if current_cell_y.is_some() && current_cell_y != Some(cell_key.1) {
                writeln!(&mut file, "").expect("Unable to write to file");
                prev_cell_x = None;
            }

            // Print X number of tabs at the beginning of the line when starting a new line
            if current_cell_y != Some(cell_key.1) {
                for _ in 0..cell_key.0 {
                    write!(&mut file, "\t").expect("Unable to write to file");
                }
            }

            // Insert tab characters based on the difference in x values of consecutive cells
            if let Some(prev_x) = prev_cell_x {
                for _ in 0..(cell_key.0 - prev_x - 1) {
                    write!(&mut file, "\t").expect("Unable to write to file");
                }
            }

            current_cell_y = Some(cell_key.1);
            prev_cell_x = Some(cell_key.0);

            // Format cell_key with fixed width of 8 characters
            let formatted_cell_key = format!("({:2},{:2})", cell_key.0, cell_key.1);
            write!(&mut file, "{:8}", formatted_cell_key).expect("Unable to write to file");
        }
        writeln!(&mut file, "\n\n").expect("Unable to write to file");


        /////////////////////////////////////////////////////


        let dragon_image = Arc::new(CollidableImage {
            classifier: CollidableClassifier::Dragon(elemental_theme),
            size: dragon_image_size,
            pixel_data: dragon_image_pixel_data,
            image_handle: dragon_image_handle,
            opaque_pixel_cells: dragon_image_opaque_pixel_cells,
        });
        resource_cache.dragon_images.insert(elemental_theme, dragon_image);


        let projectile_image_handle: Handle<Image> = asset_server.load(projectile_image_file_path);
        let projectile_image_pixel_data = load_image_data(projectile_image_file_path);
        //let projectile_size = Vec2::new(projectile_image_data.width() as f32, projectile_image_data.height() as f32);
        let projectile_image_size = ImageSizeI32::from(&projectile_image_pixel_data);
        
        let projectile_image_opaque_pixel_cells = get_opaque_pixel_cells(&projectile_image_pixel_data);

        let projectile_image = Arc::new (CollidableImage {
            classifier: CollidableClassifier::Projectile(elemental_theme),
            size: projectile_image_size,
            image_handle: projectile_image_handle,
            pixel_data: projectile_image_pixel_data,
            opaque_pixel_cells: projectile_image_opaque_pixel_cells,
        });
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
