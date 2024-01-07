mod obsticals;
mod system_utils;
mod boids;

use crate::boids::BevyPlugin;
use crate::obsticals::BasicObsticalPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BasicObsticalPlugin::new(
            [Vec3::new(-500., -300., 0.), Vec3::new(500., 300., 0.)],
        ))
        .add_plugins(BevyPlugin::new(100))
        .run();
}
