use bevy::math::{Vec2, Vec3};

pub fn vec2_to_vec3(v: Vec2) -> Vec3 {
    Vec3::new(v.x, v.y, 0.)
}


pub fn clip(v: Vec3, max_speed: f32) -> Vec3 {
    if v.length_squared() > max_speed.powi(2) {
        v.normalize() * max_speed
    } else {
        v
    }
}
