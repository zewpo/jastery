use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    utils::HashMap,
    window::{close_on_esc, PresentMode},
};
use image::DynamicImage;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Setup,
    Running,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum ElementalTheme {
    Fire,
    Ice,
    Water,
    Rock,
    //? Air,
    // Add more elemental themes here as needed
}


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

// #[derive(Component)]
// struct FireDragon;

// #[derive(Component)]
// struct IceDragon;

// Marker to query the dragon to control by the local system.
#[derive(Component)]
pub struct MyDragon;

#[derive(Bundle)]
struct MyDragonBundle {
    #[bundle]
    dragon_bundle: DragonBundle,
    my_dragon: MyDragon,
}

#[derive(Bundle)]
struct DragonBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    input: DragonInput,
    movement: DragonAction,
    dragon: Dragon,
}

// #[derive(Bundle)]
// struct FireDragonBundle {
//     marker: FireDragon,
//     #[bundle]
//     dragon_bundle: DragonBundle,
// }

// #[derive(Bundle)]
// struct IceDragonBundle {
//     marker: IceDragon,
//     #[bundle]
//     dragon_bundle: DragonBundle,
// }

#[derive(Component, Default)]
struct DragonInput {
    move_direction: Vec2,
    brake: bool,
    home: bool,
    ease_up: bool,
    fire: bool,
}

#[derive(Component)]
struct DragonAction {
    spawn_home: Vec3,
    velocity: Vec3,
    max_velocity: f32,
    motion_timer: Timer,
    flip_timer: Timer,
    firerate_timer: Timer,
    flipping: bool,
}



#[derive(Component)]
pub struct Projectile {
    pub elemental_theme: ElementalTheme,
}

#[derive(Bundle)]
struct ProjectileBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    movement: ProjectileMovement,
    projectile: Projectile,
}

#[derive(Component)]
struct ProjectileMovement {
    speed: Vec3,
    despawn_timer: Timer,
}

pub struct ProjectileImage {
    pub size: Vec2,
    pub image: DynamicImage,
    pub file_handle: Handle<Image>,
    pub elemental_theme: ElementalTheme,
}


#[derive(Component)]
pub struct Wall {
    shape: WallShape
}

#[derive(Bundle)]
struct WallBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    wall: Wall,
}

pub struct WallImage {
    pub size: Vec2,
    pub image: DynamicImage,
    pub file_handle: Handle<Image>,
    pub shape: WallShape,
}


#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum WallShape {
    Straight,
    Corner,
    TJunction,
    Cross,
    ShortStraight,
    LongStraight,
    Diagonal,
    Curved,
    Narrow,
}



#[derive(Component)]
pub struct GameCamera {
    pub threshold: f32,  // The threshold before the camera starts scrolling
    pub scale: f32,
}

#[derive(Resource)]
struct CameraScale(f32);


#[derive(Resource)]
pub struct ResourceCache {
    pub wall_images: HashMap<WallShape, WallImage>,
    pub dragon_images: HashMap<ElementalTheme, DragonImage>, // DynamicImage,
    pub projectile_images: HashMap<ElementalTheme, ProjectileImage>,
    // Other resources can be added here, e.g., audio files, character data, etc.
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Dragsteroids!".into(),
                resolution: (1200., 800.).into(),
                present_mode: PresentMode::AutoVsync,
                // Tells wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_state::<GameState>()
        .insert_resource(CameraScale(1.0))
        .insert_resource(ResourceCache {
                wall_images: HashMap::new(),
                dragon_images: HashMap::new(),
                projectile_images: HashMap::new(),
        })
        .add_systems(  (preload_resources,
                        setup_dragons.after(preload_resources), 
                        setup_camera.after(setup_dragons),
                ).chain().on_startup()
        )
        .add_system(setup_maze.run_if(in_state(GameState::Setup)))
        .add_systems((
                // keyboard_input_system_ice_dragon.run_if(in_state(GameState::Running)),
                keyboard_input_system.run_if(in_state(GameState::Running)),
                dragon_movement_system.run_if(in_state(GameState::Running)), 
                camera_follow_system.run_if(in_state(GameState::Running)),
                projectile_spawn_system.run_if(in_state(GameState::Running)), 
                projectile_movement_system.run_if(in_state(GameState::Running))
            )
        )
        .add_system(close_on_esc)
        .run();
}

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



fn setup_dragons(
        mut commands: Commands,
        resource_cache: Res<ResourceCache>,
    ) {
    
    let dragon_images = &resource_cache.dragon_images;

    // Spawn the Fire Dragon into the game.
    let mydragon_spawn_home = Vec3::new(100., 0., 0.);
    let mydragon_theme = ElementalTheme::Fire;
    commands.spawn(MyDragonBundle {
        my_dragon: MyDragon,
        dragon_bundle: DragonBundle {
            sprite_bundle: SpriteBundle {
                texture: dragon_images.get(&mydragon_theme).unwrap().file_handle.clone(),
                transform: Transform::from_translation(mydragon_spawn_home),
                ..default()
            },
            input: DragonInput::default(),
            movement: DragonAction {
                spawn_home: mydragon_spawn_home,
                velocity: Vec3::ZERO,
                max_velocity: 25.0,
                motion_timer: Timer::from_seconds(0.05, TimerMode::Repeating),
                firerate_timer: Timer::from_seconds(0.15, TimerMode::Repeating),
                flip_timer: Timer::from_seconds(0.2, TimerMode::Once),
                flipping: false,
            },
            dragon: Dragon {
                id: Uuid::new_v4(),
                elemental_theme: mydragon_theme
            },
        },
    });

    // Spawn an Ice Dragon into the game.
    let icedragon_spawn_home = Vec3::new(1400., 0., 0.);
    let ice_dragon_theme = ElementalTheme::Ice;

    commands.spawn( DragonBundle {
                sprite_bundle: SpriteBundle {
                    // texture: asset_server.load("sprites/ice-dragon.png"),
                    texture: dragon_images.get(&ice_dragon_theme).unwrap().file_handle.clone(),
                    transform: Transform::from_translation(icedragon_spawn_home),  //from_xyz(1200., 0., 0.),
                    ..default()
            },
            input: DragonInput::default(),
            movement: DragonAction {
                spawn_home: icedragon_spawn_home,
                velocity: Vec3::ZERO,
                max_velocity: 25.0,
                motion_timer: Timer::from_seconds(0.05, TimerMode::Repeating),
                firerate_timer: Timer::from_seconds(0.15, TimerMode::Repeating),
                flip_timer: Timer::from_seconds(0.2, TimerMode::Once),
                flipping: false,
            },
            dragon: Dragon { 
                id: Uuid::new_v4(), 
                elemental_theme: ice_dragon_theme 
            }
    });
}


fn setup_maze(
    mut commands: Commands,
//     asset_server: Res<AssetServer>,
    _state: ResMut<State<GameState>>,
//     images: Res<Assets<Image>>,
    mut next_state: ResMut<NextState<GameState>>,
//     wall_texture_handle: Res<WallTextureHandle>,
    resource_cache: Res<ResourceCache>
) {
    println!("Setup Maze");
//     let wall_handle = asset_server.load("wall.png");
//     let wall_sizes = &resource_cache.wall_sizes;
    let wall_images = &resource_cache.wall_images;

    let mut maze = [
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1],
    ];
    maze.reverse();

    if let Some(wall_image) =  wall_images.get(&WallShape::Straight) {
        let wall_width = wall_image.size.x;
        let wall_height = wall_image.size.y;

        // Spawn Wall blocks into the game.
        for (i, row) in maze.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if *cell == 1 {
                    let x = (j as f32 * wall_width) - 1600.0;
                    let y = (i as f32 * wall_height) - 1000.0;
                    commands.spawn(WallBundle {
                        sprite_bundle: SpriteBundle {
                            texture: wall_image.file_handle.clone(),
                            transform: Transform::from_xyz(x, y, -1.0),
                            ..default()
                        },
                        wall: Wall { shape: WallShape::Straight },
                    });
                }
            }
        }
        next_state.set(GameState::Running);
        // state .set(GameState::Running).unwrap();
    } else {
        println!("Setup Maze - Image not loaded yet...");
    }
}


fn setup_camera(
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

fn keyboard_input_system (
    keyboard_input: Res<Input<KeyCode>>, 
//     mut dragon_query: Query<&mut DragonInput>,
    mut dragon_query: Query<&mut DragonInput, With<MyDragon>>,
    mut camera_query: Query<(&mut Transform, &mut GameCamera), With<GameCamera>>,
) {
    let mut dragon_input = dragon_query.single_mut();
    dragon_input.move_direction = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::Up) {
        dragon_input.move_direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        dragon_input.move_direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Left)  {
        dragon_input.move_direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        dragon_input.move_direction.x += 1.0;
    }

    dragon_input.fire = keyboard_input.pressed(KeyCode::Space);
    dragon_input.brake = keyboard_input.pressed(KeyCode::RShift);
    dragon_input.home = keyboard_input.pressed(KeyCode::X);
    dragon_input.ease_up = !dragon_input.brake && !dragon_input.home && dragon_input.move_direction == Vec2::ZERO;
    

    let ctrl_pressed = keyboard_input.pressed(KeyCode::LControl) || keyboard_input.pressed(KeyCode::RControl);

    if ctrl_pressed {
        let mut scale_change = 1.0;
        if keyboard_input.pressed(KeyCode::Plus) || keyboard_input.pressed(KeyCode::Equals) {
            scale_change = 0.99;
        } else if keyboard_input.pressed(KeyCode::Minus) {
            scale_change = 1.01;
        }

        let (mut camera_transform, mut game_camera) = camera_query.single_mut();
        camera_transform.scale *= Vec3::splat(scale_change);
        game_camera.scale *= scale_change;
    }
}


// fn keyboard_input_system_ice_dragon (
//     keyboard_input: Res<Input<KeyCode>>, 
//     mut dragon_query: Query<&mut DragonInput, With<IceDragon>>,
// ) {
//     let mut dragon_input = dragon_query.single_mut();
//     dragon_input.move_direction = Vec2::ZERO;

//     if keyboard_input.pressed(KeyCode::W) {
//         dragon_input.move_direction.y += 1.0;
//     }
//     if keyboard_input.pressed(KeyCode::S) {
//         dragon_input.move_direction.y -= 1.0;
//     }
//     if keyboard_input.pressed(KeyCode::A) {
//         dragon_input.move_direction.x -= 1.0;
//     }
//     if keyboard_input.pressed(KeyCode::D) {
//         dragon_input.move_direction.x += 1.0;
//     }

//     dragon_input.fire = keyboard_input.pressed(KeyCode::R);
//     dragon_input.brake = keyboard_input.pressed(KeyCode::LShift);
//     dragon_input.home = keyboard_input.pressed(KeyCode::Q);
//     dragon_input.ease_up = !dragon_input.brake && !dragon_input.home && dragon_input.move_direction == Vec2::ZERO;

// }

fn camera_follow_system(
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


fn dragon_movement_system(
    time: Res<Time>,
    mut dragon_query: Query<(&Dragon, &mut DragonAction, &DragonInput, &mut Transform)>,
    wall_query: Query<(&Wall, &Transform), Without<DragonAction>>,
    // images: Res<Assets<Image>>,
    resource_cache: Res<ResourceCache>,
) {
    for (dragon, mut dragon_action, dragon_input, mut dragon_transform) in dragon_query.iter_mut() {
        let acceleration = 0.4;

        dragon_action.velocity.x += dragon_input.move_direction.x * acceleration;
        dragon_action.velocity.y += dragon_input.move_direction.y * acceleration;

        // Brake
        if dragon_input.brake && dragon_action.motion_timer.tick(time.delta()).just_finished() {
            dragon_action.velocity *= 0.6;
        }

        // Move to home position
        if dragon_input.home {
            dragon_action.velocity = Vec3::ZERO;
            dragon_transform.translation = dragon_action.spawn_home;
        } else if dragon_input.ease_up && dragon_action.motion_timer.tick(time.delta()).just_finished() {
            dragon_action.velocity *= 0.8;
        }

        // Check for collisions
        // if let Some(dragon_image) = images.get(dragon_image_handle) {
        if let Some(dragon_image) = resource_cache.dragon_images.get(&dragon.elemental_theme) {
            // Check for wall collisions
            //let dragon_size = dragon_image.size().extend(0.0) * dragon_transform.scale.abs();
            let dragon_center_position = dragon_transform.translation;

            for (wall, wall_transform) in wall_query.iter() {
                // if let Some(wall_image) = images.get(wall_image_handle) {
                if let Some(wall_image) = resource_cache.wall_images.get(&wall.shape) {
                    // let wall_size = wall_image.size().extend(0.0) * wall_transform.scale.abs();
                    let wall_center_position = wall_transform.translation;
                     // If the collision occurs on multiple sides, the side with the deepest penetration is returned.
                     // If all sides are involved, `Inside` is returned.
                    if let Some(collision) = collide(
                        dragon_center_position,
                        dragon_image.size,
                        wall_center_position,
                        wall_image.size
                    ) {
                        dragon_action.velocity = Vec3::ZERO;
                        match collision {
                            Collision::Left => {
                                dragon_transform.translation.x = wall_center_position.x - (wall_image.size.x + dragon_image.size.x) / 2.0;
                                dragon_action.velocity.x = -0.0;
                            }
                            Collision::Right => {
                                dragon_transform.translation.x = wall_center_position.x + (wall_image.size.x + dragon_image.size.x) / 2.0;
                                dragon_action.velocity.x = 0.0;
                            }
                            Collision::Top => {
                                dragon_transform.translation.y = wall_center_position.y + (wall_image.size.y + dragon_image.size.y) / 2.0;
                                dragon_action.velocity.y = 0.0;
                            }
                            Collision::Bottom => {
                                dragon_transform.translation.y = wall_center_position.y - (wall_image.size.y + dragon_image.size.y) / 2.0;
                                dragon_action.velocity.y = -0.0;
                            }
                            Collision::Inside => {
                                // Handle inside collision as appropriate for your game
                                println!("Dragon inside wall collision!?");
                            }
                        }
                        break;
                    }
                }
            }
        }

        // Move the dragon sprite.
        if dragon_action.velocity != Vec3::ZERO {
            dragon_action.velocity = dragon_action.velocity.clamp_length_max(dragon_action.max_velocity);
            dragon_transform.translation += dragon_action.velocity;
        }

        // Flip the dragon with an animation when it changes directions between left to right.
        if dragon_action.flipping {
            if dragon_action.flip_timer.tick(time.delta()).just_finished() {
            // Finish the flipping animation.
                dragon_action.flipping = false;
                if dragon_transform.scale.x < 0.0{
                    dragon_transform.scale.x = 1.0;
                } else {
                    dragon_transform.scale.x = -1.0;
                }
            } else {
                // Continue the flipping animation.
                let progress = dragon_action.flip_timer.percent();
                dragon_transform.scale.x = dragon_transform.scale.x.signum() * (0.5 - 0.5 * (progress * std::f32::consts::PI).sin());
            }
        } else if (dragon_action.velocity.x > 0.0 && dragon_transform.scale.x < 0.0) || (dragon_action.velocity.x < 0.0 && dragon_transform.scale.x > 0.0) {
            // Start the flipping animation.
            dragon_action.flip_timer.reset();
            dragon_action.flipping = true;
        }
    }
}


fn projectile_spawn_system(
    time: Res<Time>,
    mut dragon_query: Query<(&Dragon, &mut DragonAction, &mut DragonInput, &Transform)>,
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    resource_cache: Res<ResourceCache>,
) {

    for (dragon, mut dragon_action, dragon_input, dragon_transform) in dragon_query.iter_mut() {
    
        if dragon_input.fire && dragon_action.firerate_timer.tick(time.delta()).just_finished() { 
            if let Some(projectile_image) = resource_cache.projectile_images.get(&dragon.elemental_theme) {
            
                // println!("projectile_spawn_system called");
                let mut projectile_direction = dragon_action.velocity.normalize_or_zero();
                if projectile_direction == Vec3::ZERO {
                        projectile_direction.x = 1.0 * dragon_transform.scale.x.signum();
                }

                // Calculate the speed of the projectile based on the dragon's velocity.
                let projectile_speed = projectile_direction * (250.0 + 75.0 * dragon_action.velocity.length());

                // Calculate the rotation of the projectile based on its velocity direction.
                let projectile_rotation = Quat::from_rotation_arc(Vec3::new(1.0,0.0,0.0), projectile_direction);

                
                // let texture: Handle<Image> = match dragon.elemental_theme {
                //     ElementalTheme::Fire => asset_server.load("sprites/fireball.png"),
                //     ElementalTheme::Ice => asset_server.load("sprites/iceball.png"),
                //     ElementalTheme::Water => asset_server.load("sprites/waterball.png"),
                //     ElementalTheme::Rock => asset_server.load("sprites/rockball.png"),
                // };

                // Spawn the projectile into the game.
                commands.spawn(ProjectileBundle {
                    sprite_bundle: SpriteBundle {
                        texture: projectile_image.file_handle.clone(),
                        transform: Transform {
                            translation: dragon_transform.translation + Vec3::new(110.0 * dragon_transform.scale.x.signum(), 30.0, 0.0),
                            rotation: projectile_rotation,
                            ..default()
                        },
                        ..default()
                    },
                    movement: ProjectileMovement { 
                        speed: projectile_speed,
                        despawn_timer: Timer::from_seconds(2.4, TimerMode::Once), // Set the timeout duration here
                    },
                    projectile: Projectile { elemental_theme: dragon.elemental_theme }
                });
            }
        }
    }
}


fn projectile_movement_system(
        time: Res<Time>,
        mut commands: Commands,
        mut projectile_query: Query<(Entity, &mut ProjectileMovement, &mut Transform, &Handle<Image>),With<Projectile>>,
        wall_query: Query<(&Wall, &Transform, &Handle<Image>),Without<Projectile>>,
        images: Res<Assets<Image>>,
    ) {
    let delta_time = time.delta_seconds();
    for (   projectile_entity,
            mut projectile_movement, 
            mut projectile_transform,
            projectile_image_handle,
        ) in projectile_query.iter_mut() {

        // Move the projectile sprite by distance of speed * time.
        projectile_transform.translation += projectile_movement.speed * delta_time;

        // Update the despawn timer
        projectile_movement.despawn_timer.tick(time.delta());

        // Despawn the projectile if the timer has finished
        if projectile_movement.despawn_timer.finished() {
            commands.entity(projectile_entity).despawn();
        } else {
            // Check for collisions with walls
            if let Some(projectile_image) = images.get(projectile_image_handle) {
                let projectile_size = Vec2::new(projectile_image.size().x as f32, projectile_image.size().y as f32);
                for (_wall, wall_transform, wall_image_handle) in wall_query.iter() {
                
                    if let Some(wall_image) = images.get(wall_image_handle){
                        let wall_size = Vec2::new(wall_image.size().x as f32, wall_image.size().y as f32);

                        let collision = collide(
                            projectile_transform.translation,
                            projectile_size/5.0,
                            wall_transform.translation,
                            wall_size,
                        );

                        if let Some(_) = collision {
                            commands.entity(projectile_entity).despawn();
                            break;
                        }
                    }
                }
            }
        }
    }
}
