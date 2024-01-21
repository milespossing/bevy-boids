use crate::boids::boid::Boid;
use bevy::prelude::*;
use crate::boids::settings::{Extents, SimulationSettings};
use crate::vec_utils::vec2_to_vec3;

pub enum BoundingVolume {
    SuperVolume { sub_volumes: Vec<VolumeTree> },
    Leaf { entities: Vec<Entity> },
}

pub struct AABB {
    center: Vec3, size: f32
}

impl AABB {
    fn new(center: Vec3, size: f32) -> Self {
        Self { center, size }
    }

    fn from_extents(extents: Extents) -> Self {
        let v2_center = Vec2::new(extents.left() + extents.right(), extents.top() + extents.bottom()) / 2.0;
        let extents_center = vec2_to_vec3(v2_center);
        Self::new(extents_center, (extents.left() - extents.right()).max(extents.top() - extents.bottom()))
    }
}

#[derive(Component)]
pub struct VolumeTree {
    pub volume: AABB,
    pub tree: BoundingVolume,
}

// divide the volume into 8 sub volumes
fn subdivide(volume: &AABB) -> Vec<AABB> {
    let mut sub_volumes = Vec::new();
    let center = volume.center;
    let size = volume.size;
    sub_volumes.push(AABB { center: center + Vec3::new(size / 4.0, size / 4.0, size / 4.0), size: size / 2.0 });
    sub_volumes.push(AABB { center: center + Vec3::new(size / 4.0, size / 4.0, -size / 4.0), size: size / 2.0 });
    sub_volumes.push(AABB { center: center + Vec3::new(size / 4.0, -size / 4.0, size / 4.0), size: size / 2.0 });
    sub_volumes.push(AABB { center: center + Vec3::new(size / 4.0, -size / 4.0, -size / 4.0), size: size / 2.0 });
    sub_volumes.push(AABB { center: center + Vec3::new(-size / 4.0, size / 4.0, size / 4.0), size: size / 2.0 });
    sub_volumes.push(AABB { center: center + Vec3::new(-size / 4.0, size / 4.0, -size / 4.0), size: size / 2.0 });
    sub_volumes.push(AABB { center: center + Vec3::new(-size / 4.0, -size / 4.0, size / 4.0), size: size / 2.0 });
    sub_volumes.push(AABB { center: center + Vec3::new(-size / 4.0, -size / 4.0, -size / 4.0), size: size / 2.0 });
    sub_volumes
}

pub fn contains(volume: &AABB, position: &Transform) -> bool {
    let diff = volume.center - position.translation;
    if diff.x.abs() <= volume.size / 2.0 && diff.y.abs() <= volume.size / 2.0 && diff.z.abs() <= volume.size / 2.0 {
        true
    } else {
        false
    }
}

pub fn arrange_to_cvb(max: u32, volume: &AABB, entities: Vec<(Entity, Transform)>) -> BoundingVolume {
    if entities.len() <= max as usize {
        return BoundingVolume::Leaf { entities: entities.iter().map(|(entity, _)| *entity).collect() };
    }

    // divide this volume into 8 sub volumes


    let mut sub_volumes: Vec<VolumeTree> = Vec::new();

    for sub_volume in subdivide(&volume) {
        let mut sub_entities = Vec::new();
        for (entity, transform) in entities.iter() {
            if contains(&sub_volume, transform) {
                sub_entities.push((*entity, *transform));
            }
        }
        if sub_entities.len() == 0 {
            continue;
        }
        let bounding_volume = arrange_to_cvb(max, &sub_volume, sub_entities);
        let sub_tree = VolumeTree { volume: sub_volume, tree: bounding_volume };
        sub_volumes.push(sub_tree);
    }

    BoundingVolume::SuperVolume { sub_volumes }
}

fn arrange_volumes_constant(mut commands: Commands, query: Query<(Entity, &Transform), With<Boid>>, volumes: Query<Entity, With<VolumeTree>>, simulation_settings: Query<&SimulationSettings>) {
    for entity in volumes.iter() {
        commands.entity(entity).despawn();
    }

    let mut all_boids: Vec<(Entity, Transform)> = Vec::new();

    for (entity, transform) in query.iter() {
        all_boids.push((entity, *transform));
    }

    let extents = AABB::from_extents(simulation_settings.single().extents);

    let bounding_volume = arrange_to_cvb(10, &extents, all_boids);

    commands.spawn(VolumeTree { volume: extents, tree: bounding_volume });
}

pub struct VolumeOptimizationPlugin;

impl Plugin for VolumeOptimizationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(First, arrange_volumes_constant);
    }
}

