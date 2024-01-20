mod system_utils;
mod boids;
mod vec_utils;

use crate::boids::BevyBoidPlugin;
use bevy::prelude::*;

// some basic setup here
fn basic_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_systems(Startup, basic_setup)
        .add_plugins(DefaultPlugins)
        .add_plugins(BevyBoidPlugin::new(200, false))
        .run();
}
