use bevy::prelude::*;
use crate::boids::boid::*;

pub struct BoidDebugPlugin {
    pub draw_radius: bool,
}

impl Plugin for BoidDebugPlugin {
    fn build(&self, app: &mut App) {
        println!("Adding debug plugin");
        if self.draw_radius {
            app.add_systems(Last, (draw_radius, draw_intent));
        }
    }
}

fn draw_radius(settings: Res<BoidGlobalSettings>, mut gizmos: Gizmos, boids: Query<&Transform, With<Boid>>) {
    for transform in boids.iter() {
        let outer = settings.view_distance;
        let inner = settings.separation_distance;

        gizmos.circle(transform.translation, Vec3::Z, outer, Color::RED);
        gizmos.circle(transform.translation, Vec3::Z, inner, Color::GREEN);
    }
}

fn draw_intent(mut gizmos: Gizmos, boids: Query<(&Transform, &Boid)>) {
    for (transform, boid) in boids.iter() {
        let intent = transform.translation + boid.velocity;
        gizmos.line(transform.translation, intent, Color::BLUE);
    }
}

