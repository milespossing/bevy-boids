pub mod boid;
mod setup;
mod basic_motion;
mod settings;
mod flocking;
mod debugging;
mod fullscreen_plugin;
mod volume_optimization;

use bevy::prelude::*;
use debugging::BoidDebugPlugin;
use basic_motion::BasicMotion;
use crate::boids::setup::Setup;
use flocking::Flocking;
use crate::boids::fullscreen_plugin::FullscreenCanvasPlugin;
use crate::boids::volume_optimization::VolumeOptimizationPlugin;

#[derive(Resource)]
struct BevyStartupResource {
    n_boids: u32,
    debug: bool,
}

pub struct BevyBoidPlugin;

impl BevyBoidPlugin {
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for BevyBoidPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Setup);
        app.add_plugins(Flocking);
        app.add_plugins(BasicMotion);
        app.add_plugins(FullscreenCanvasPlugin);
        app.add_plugins(BoidDebugPlugin);
        // app.add_plugins(VolumeOptimizationPlugin);
    }
}
