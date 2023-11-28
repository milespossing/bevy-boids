mod boids;

use crate::boids::BevyPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BevyPlugin::new(100))
        .run();
}
