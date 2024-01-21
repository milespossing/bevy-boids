use bevy::prelude::*;
use crate::boids::boid::*;

#[derive(Component)]
pub struct DebugSettings {
    pub draw_radius: bool,
    pub draw_intent: bool,
    pub draw_velocity: bool,
}

impl Default for DebugSettings {
    fn default() -> Self {
        Self {
            draw_radius: false,
            draw_intent: false,
            draw_velocity: false,
        }
    }
}

pub struct BoidDebugPlugin;


impl Plugin for BoidDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Last, draw_debug);
    }
}

fn draw_debug(settings: Res<BoidGlobalSettings>, mut gizmos: Gizmos, boids: Query<(&Transform, &Boid)>, debug_settings: Query<&DebugSettings>) {
    let debug = debug_settings.single();
    for (transform, boid) in boids.iter() {
        if debug.draw_radius {
            let outer = settings.view_distance;
            let inner = settings.separation_distance;

            gizmos.circle(transform.translation, Vec3::Z, outer, Color::RED);
            gizmos.circle(transform.translation, Vec3::Z, inner, Color::GREEN);
        }

        if debug.draw_intent {
            gizmos.line(transform.translation, transform.translation + boid.intent, Color::BLUE);
        }

        if debug.draw_velocity {
            gizmos.line(transform.translation, transform.translation + boid.velocity, Color::YELLOW);
        }
    }
}
