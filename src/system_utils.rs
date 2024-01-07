use bevy::prelude::*;
use bevy::render::color::Color;

#[derive(Component)]
pub struct ArrowComponent {
    pub direction: Vec3,
    pub color: Color,
}

pub struct ArrowsPlugin;

pub fn move_arrows(mut lines: ResMut<DebugLines>, arrows: Query<(&ArrowComponent, &Transform)>) {
    for (arrow, transform) in arrows.iter() {
        let start = transform.translation;
        let end = start + arrow.direction;
        lines.line_colored(start, end, 1., arrow.color);
    }
}
