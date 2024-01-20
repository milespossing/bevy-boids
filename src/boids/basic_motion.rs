use bevy::math::Vec3;
use bevy::prelude::*;
use crate::vec_utils::*;
use crate::boids::boid::{Boid, BoidGlobalSettings};
use crate::boids::settings::SimulationSettings;

pub struct BasicMotion;

impl Plugin for BasicMotion {
    fn build(&self, app: &mut App) {
        app
            .add_systems(FixedUpdate, boid_acceleration)
            .add_systems(FixedUpdate, boid_motion)
            .add_systems(FixedUpdate, boid_in_bounds)
            .add_systems(FixedUpdate, clip_velocity);
    }
}

fn boid_acceleration(boid_global_settings: Res<BoidGlobalSettings>, mut boids: Query<&mut Boid>) {
    for mut boid in &mut boids.iter_mut() {
        let intent = boid.intent;
        boid.velocity += clip(intent, boid_global_settings.turn_speed);
    }
}

fn boid_motion(mut query: Query<(&Boid, &mut Transform)>, time: Res<Time>) {
    for (boid, mut transform) in &mut query {
        let delta_x: Vec3 = boid.velocity * time.delta_seconds();
        transform.translation += delta_x;
    }
}

fn clip_velocity(settings: Res<BoidGlobalSettings>, mut boids: Query<&mut Boid>) {
    for mut boid in &mut boids.iter_mut() {
        if boid.velocity.length_squared() > settings.max_speed.powi(2) {
            boid.velocity = boid.velocity.normalize() * settings.max_speed;
        }
    }
}

fn boid_in_bounds(sim_settings: Res<SimulationSettings>, boid_global_settings: Res<BoidGlobalSettings>, time: Res<Time>, mut boid: Query<(&mut Boid, &Transform)>) {
    let extents = &sim_settings.extents;
    for (mut boid, transform) in boid.iter_mut() {
        if transform.translation.x < extents.left() + 100. {
            boid.velocity.x += boid_global_settings.turn_speed * time.delta_seconds();
        } else if transform.translation.x > extents.right() - 100. {
            boid.velocity.x -= boid_global_settings.turn_speed * time.delta_seconds();
        }
        if transform.translation.y < extents.bottom() + 100. {
            boid.velocity.y += boid_global_settings.turn_speed * time.delta_seconds();
        } else if transform.translation.y > extents.top() - 100. {
            boid.velocity.y -= boid_global_settings.turn_speed * time.delta_seconds();
        }
    }
}
