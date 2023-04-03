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


fn load_collidable_image(
    asset_server: &AssetServer,
    path: &str,
    classifier: CollidableClassifier,
) -> Arc<CollidableImage> {

    let image_handle: Handle<Image> = asset_server.load(path);
    let image_bytes = std::fs::read( "assets/".to_owned() + path).expect("Failed to read image file");
    let pixel_data = image::load_from_memory(&image_bytes).expect("Failed to load image data");

    let size: CollidableImageSize = CollidableImageSize::from(&pixel_data);

    let opaque_pixel_cells: HashMap<(i32, i32), HashSet<(i32, i32)>> = get_opaque_pixel_cells(&pixel_data);

    Arc::new(CollidableImage {
        size,
        pixel_data,
        image_handle,
        opaque_pixel_cells,
        classifier,
    })

}

fn get_opaque_pixel_cells(
    image: &DynamicImage,
) -> HashMap<(i32, i32), HashSet<(i32, i32)>> {
    // let cell_size = CELL_SIZE as u32;
    let mut opaque_pixel_cells: HashMap<(i32, i32), HashSet<(i32, i32)>> = HashMap::new();
    let (width, height) = image.dimensions();

    let half_width = width as f32 / 2.0;
    let half_height = height as f32 / 2.0;

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
                    let cell_i = ((x as f32 - half_width) / CELL_SIZE as f32).floor() as i32;
                    let cell_j = ((half_height - y as f32) / CELL_SIZE as f32).floor() as i32;
                    let cell_key = (cell_i, cell_j);

                    let pixel_in_cell = ((x as i32 - half_width as i32) % CELL_SIZE, (half_height as i32 - y as i32) % CELL_SIZE);

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


fn debug_opaque_pixel_cells(
    opaque_pixel_cells: &HashMap<(i32, i32), HashSet<(i32, i32)>>,
    filename: &str,
) {
        // debugging where the opaque cells are
        let mut cell_keys: Vec<&(i32, i32)> = opaque_pixel_cells.keys().collect();
        cell_keys.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

        let min_i = cell_keys.iter().min_by_key(|key| key.0).unwrap().0;

        let mut current_cell_j = None;
        let mut prev_cell_i: Option<i32> = None;

        // Create a new file or truncate an existing file
        let mut file = File::create(filename).expect("Unable to create file");

        writeln!(&mut file, "\n\n").expect("Unable to write to file");
        for cell_key in cell_keys {
            let (i, j) = (cell_key.0,cell_key.1);

            if current_cell_j.is_some() && current_cell_j != Some(j) {
                writeln!(&mut file, "").expect("Unable to write to file");
                prev_cell_i = None;
            }

            // Print min_i to i number of tabs at the beginning of the line when starting a new line
            if current_cell_j != Some(j) {
                for _ in min_i..i {
                    write!(&mut file, "\t").expect("Unable to write to file");
                }
            }

            // Insert tab characters based on the difference in i values of consecutive cells
            if let Some(prev_i) = prev_cell_i {
                for _ in 0..(i - prev_i - 1) {
                    write!(&mut file, "\t").expect("Unable to write to file");
                }
            }

            current_cell_j = Some(j);
            prev_cell_i = Some(i);

            // Format cell_key with fixed width of 8 characters
            let formatted_cell_key = format!("({:2},{:2})", i, j);
            write!(&mut file, "{:8}", formatted_cell_key).expect("Unable to write to file");
        }
        writeln!(&mut file, "\n\n").expect("Unable to write to file");
}

pub fn preload_resources(
    mut _commands: Commands, 
    asset_server: Res<AssetServer>,
    mut resource_cache: ResMut<ResourceCache>
) { 


    let wall_shape_file_names = vec![
        // (WallShape::Straight, "sprites/wall-straight.png"),
        (WallShape::Straight, "sprites/wall-block.png"),
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
    for (wall_shape, path) in wall_shape_file_names {

        let wall_image = load_collidable_image(&asset_server, path, CollidableClassifier::Wall(WallShape::Straight));
        /////////////////////////////////////////////////////
        // debugging where the opaque cells are
        debug_opaque_pixel_cells(&wall_image.opaque_pixel_cells,"wall_output.txt");
        /////////////////////////////////////////////////////
        resource_cache.wall_images.insert(wall_shape, wall_image);
    }

    // Preload the Dragons and their themed Projectiles
    for (elemental_theme, dragon_image_file_path, projectile_image_file_path) in theme_image_file_names {
        
        let dragon_image = load_collidable_image(&asset_server, dragon_image_file_path, CollidableClassifier::Dragon(elemental_theme));
        //////////////////////////////////////////////////////////////
        // debugging where the opaque cells are
        debug_opaque_pixel_cells(&dragon_image.opaque_pixel_cells,"dragon_output.txt");
        //////////////////////////////////////////////////////////////
        resource_cache.dragon_images.insert(elemental_theme, dragon_image);


        let projectile_image = load_collidable_image(&asset_server, projectile_image_file_path, CollidableClassifier::Projectile(elemental_theme));
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
