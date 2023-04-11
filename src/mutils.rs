use bevy::prelude::*;

// helper functions
pub fn val_to_f32(val: Val) -> f32 {
    match val {
        Val::Px(value) => value,
        _ => 0.0,
    }
}
pub fn size_to_vec2(size: Size) -> Vec2 {
    Vec2::new(val_to_f32(size.width), val_to_f32(size.height))
}


// pub fn vec3_round(v: Vec3) -> Vec3 {
//     Vec3::new(v.x.round(), v.y.round(), v.z.round())
// }
