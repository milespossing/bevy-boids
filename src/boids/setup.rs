use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use rand::{thread_rng, Rng};
use crate::boids::BevyStartupResource;
use crate::boids::boid::Boid;
use crate::boids::settings::SimulationSettings;
use crate::vec_utils::vec2_to_vec3;

pub struct Setup;

/*
Basic system for generating boids
 */
fn generate_boids(
    state: Res<BevyStartupResource>,
    settings: Res<SimulationSettings>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if state.debug {
        commands.spawn((
            Boid::new(vec2_to_vec3(Vec2::new(-50., 0.))),
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(5.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::RED)),
                transform: Transform::from_translation(Vec3::new(100., 10., 0.)),
                ..default()
            },
        ));
        commands.spawn((
            Boid::new(vec2_to_vec3(Vec2::new(50., 0.))),
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(5.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_translation(Vec3::new(-100., -10., 0.)),
                ..default()
            },
        ));
        return
    }
    let extents = &settings.extents;
    let mut rng = thread_rng();
    for _ in 0..state.n_boids {
        let pos2: Vec2 = Vec2::new(rng.gen_range(extents.left()..extents.right()), rng.gen_range(extents.bottom()..extents.top()));
        let pos = vec2_to_vec3(pos2);
        let vel = Vec2::new(rng.gen_range(-50.0..50.0), rng.gen_range(-50.0..50.0));

        commands.spawn((
            Boid::new(vec2_to_vec3(vel)),
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(5.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_translation(pos),
                ..default()
            },
        ));
    }
}

impl Plugin for Setup {
    fn build(&self, app: &mut App) {
        app.insert_resource(SimulationSettings::default());
        app.add_systems(Startup, generate_boids);
    }
}
