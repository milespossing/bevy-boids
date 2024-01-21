use bevy::prelude::*;
use crate::boids::settings::{extents_from_window, SimulationSettings};

pub struct FullscreenCanvasPlugin;

impl Plugin for FullscreenCanvasPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, update_sim_extents);
    }
}

fn update_sim_extents(query: Query<&Window>, mut sim_settings: Query<&mut SimulationSettings>) {
    let window = query.single();
    let mut sim_settings = sim_settings.single_mut();

    let current_extents = &sim_settings.extents;
    let new_extents = extents_from_window(window);

    if &new_extents == current_extents {
        return;
    }

    sim_settings.extents = new_extents;
}