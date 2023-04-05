use std::collections::HashMap;
use lazy_static::lazy_static;
lazy_static! {
    pub static ref ASSET_DATA: HashMap<&'static str, Vec<u8>> = {
        let mut map = HashMap::new();
        map.insert("fonts/FiraSans-Bold.ttf", include_bytes!("c:\\workspaces\\rust\\jastery\\assets\\fonts\\FiraSans-Bold.ttf").to_vec());
        map.insert("sprites/fire-dragon.png", include_bytes!("c:\\workspaces\\rust\\jastery\\assets\\sprites\\fire-dragon.png").to_vec());
        map.insert("sprites/fire-projectile.png", include_bytes!("c:\\workspaces\\rust\\jastery\\assets\\sprites\\fire-projectile.png").to_vec());
        map.insert("sprites/ice-dragon.png", include_bytes!("c:\\workspaces\\rust\\jastery\\assets\\sprites\\ice-dragon.png").to_vec());
        map.insert("sprites/ice-projectile.png", include_bytes!("c:\\workspaces\\rust\\jastery\\assets\\sprites\\ice-projectile.png").to_vec());
        map.insert("sprites/rock-dragon.png", include_bytes!("c:\\workspaces\\rust\\jastery\\assets\\sprites\\rock-dragon.png").to_vec());
        map.insert("sprites/rock-projectile.png", include_bytes!("c:\\workspaces\\rust\\jastery\\assets\\sprites\\rock-projectile.png").to_vec());
        map.insert("sprites/wall-block.png", include_bytes!("c:\\workspaces\\rust\\jastery\\assets\\sprites\\wall-block.png").to_vec());
        map.insert("sprites/wall-straight.png", include_bytes!("c:\\workspaces\\rust\\jastery\\assets\\sprites\\wall-straight.png").to_vec());
        map.insert("sprites/water-dragon.png", include_bytes!("c:\\workspaces\\rust\\jastery\\assets\\sprites\\water-dragon.png").to_vec());
        map.insert("sprites/water-projectile.png", include_bytes!("c:\\workspaces\\rust\\jastery\\assets\\sprites\\water-projectile.png").to_vec());
        map
    };
}
