pub mod boid;
mod setup;
mod basic_motion;
mod settings;
mod flocking;
mod debugging;

use bevy::{prelude::*};
use boid::{BoidGlobalSettings};
use debugging::BoidDebugPlugin;
use basic_motion::BasicMotion;
use crate::boids::setup::Setup;
use flocking::Flocking;

#[derive(Resource)]
struct BevyStartupResource {
    n_boids: u32,
    debug: bool,
}

pub struct BevyBoidPlugin {
    n_boids: u32,
    debug: bool,
}

impl BevyBoidPlugin {
    pub fn new(n_boids: u32, debug: bool) -> Self {
        Self { n_boids, debug }
    }
}

impl Plugin for BevyBoidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BevyStartupResource {
            n_boids: self.n_boids,
            debug: false,
        });
        app.insert_resource(BoidGlobalSettings {
            turn_speed: 50.,
            view_distance: 200.,
            separation_distance: 20.,
            view_angle: 2.,
            max_speed: 100.,
        });
        app.add_plugins(Setup);
        app.add_plugins(Flocking);
        app.add_plugins(BasicMotion);
        if self.debug {
            app.add_plugins(BoidDebugPlugin { draw_radius: true });
        }
    }
}
