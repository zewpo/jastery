// src/shared/systems/mod.rs

pub mod projectile;
pub mod dragon;
pub mod resource_cache;
pub mod game;

pub use game::GamePlugin;
pub use resource_cache::ResourceCachePlugin;