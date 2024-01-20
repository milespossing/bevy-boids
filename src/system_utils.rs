use bevy::prelude::*;
use bevy::render::color::Color;

#[derive(Component)]
pub struct ArrowComponent {
    pub direction: Vec3,
    pub color: Color,
}

