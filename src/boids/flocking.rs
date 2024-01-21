use bevy::prelude::*;

use crate::boids::boid::{Boid, BoidGlobalSettings};


type BoidWithTrans<'a> = (Entity, &'a Boid, &'a Transform);

#[derive(Component)]
pub struct FlockingSettings {
    pub separation: f32,
    pub alignment: f32,
    pub cohesion: f32,
    pub flocking_strength: f32,
}

impl Default for FlockingSettings {
    fn default() -> Self {
        Self {
            separation: 0.3,
            alignment: 0.8,
            cohesion: 0.2,
            flocking_strength: 20.0,
        }
    }
}

pub struct Flocking;

impl Plugin for Flocking {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_flocking);
        app.add_systems(FixedUpdate, flock);
    }
}

fn initialize_flocking(mut commands: Commands) {
    commands.spawn(FlockingSettings {
        ..default()
    });
}

fn in_flock(this_boid: &BoidWithTrans, other_boid: &Vec3, distance: f32) -> bool {
    let diff = this_boid.2.translation.distance(*other_boid);
    if diff < distance {
        true
    } else {
        false
    }
}

fn separation(this_boid: &BoidWithTrans, neighbours: &Vec<(Vec3, Vec3)>, seperation_distance: f32) -> Vec3 {
    let mut separation = Vec3::ZERO;
    for neighbour in neighbours {
        let distance = neighbour.1.distance(this_boid.2.translation);
        if distance < seperation_distance {
            separation += (seperation_distance - distance) * (this_boid.2.translation - neighbour.1) / seperation_distance;
        }
    }
    separation
}

fn alignment(neighbours: &Vec<(Vec3, Vec3)>) -> Vec3 {
    let mut alignment = Vec3::ZERO;
    for neighbour in neighbours {
        alignment += neighbour.0;
    }
    alignment.normalize()
}

fn cohesion(this_boid: &BoidWithTrans, neighbours: &Vec<(Vec3, Vec3)>) -> Vec3 {
    let mut cohesion = Vec3::ZERO;
    for &neighbour in neighbours {
        cohesion += neighbour.1;
    }
    let direction = (cohesion / neighbours.len() as f32) - this_boid.2.translation;
    direction.normalize()
}

pub fn flock(settings_query: Query<&FlockingSettings>, boid_settings: Res<BoidGlobalSettings>, mut query: Query<(Entity, &mut Boid, &Transform)>) {
    let all_boids: Vec<(Entity, Vec3, Vec3)> = query.iter().map(|(e, boid, transform)| (e.clone(), boid.velocity, transform.translation)).collect();
    let settings = settings_query.single();

    for (entity, mut boid, transform) in query.iter_mut() {
        let flock: Vec<(Vec3, Vec3)> = all_boids.iter()
            .filter(|(e, _, _)| *e != entity)
            .filter(|(_, _, t)| in_flock(&(entity, &boid, transform), t, boid_settings.view_distance))
            .map(|&(_, v, t)| (v, t))
            .collect();

        if flock.len() == 0 {
            continue;
        }

        let separation_force = separation(&(entity, &boid, transform), &flock, boid_settings.separation_distance);
        let alignment_force = alignment(&flock);
        let cohesion_force = cohesion(&(entity, &boid, transform) ,&flock);
        boid.intent += settings.flocking_strength * (separation_force * settings.separation + alignment_force * settings.alignment + cohesion_force * settings.cohesion);
    }
}
