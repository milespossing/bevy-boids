use bevy::math::Vec3;
use bevy::prelude::{Component, Resource};

#[derive(Resource, Component)]
pub struct BoidGlobalSettings {
    // more like acceleration max
    pub turn_speed: f32,
    pub view_distance: f32,
    // the sin(theta) of the viewing angle
    pub view_angle: f32,
    pub max_speed: f32,
    pub separation_distance: f32,
}

#[derive(Component)]
pub struct Boid {
    pub velocity: Vec3,
    pub intent: Vec3,
}

impl Boid {
    pub fn new(velocity: Vec3) -> Self {
        Boid {
            velocity,
            intent: velocity,
        }
    }
}

