mod system_utils;
mod boids;
mod vec_utils;

use crate::boids::BevyBoidPlugin;
use bevy::prelude::*;
use bevy::window::Window;

// some basic setup here
fn basic_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_systems(Startup, basic_setup)
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }
        ))
        .add_plugins(BevyBoidPlugin::new())
        // .add_plugins(fullscreen_plugin::FullscreenCanvasPlugin)
        .run();
}
