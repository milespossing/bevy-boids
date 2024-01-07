use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

#[derive(Component)]
pub enum Obstical {
    Wall { normal: Vec3 },
}

impl Obstical {
    pub fn build_wall(normal: Vec3) -> Obstical {
        Obstical::Wall { normal }
    }
}

pub struct BasicObsticalPlugin {
    // one for each corner
    extents: [Vec3; 2],
}

impl BasicObsticalPlugin {
    pub fn new(extents: [Vec3; 2]) -> Self {
        Self { extents }
    }
}

fn add_simple_obsticals(
    state: Res<WallStartupResource>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // left wall
    commands.spawn((
        Obstical::Wall {
            normal: LEFT_WALL_NORMAL,
        },
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Quad::new(Vec2::new(10., state.top - state.bottom)).into()).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_translation(Vec3::new(
                state.left,
                (state.top + state.bottom) / 2.,
                0.0,
            )),
            ..default()
        },
    ));
    // top wall
    commands.spawn((
        Obstical::Wall {
            normal: TOP_WALL_NORMAL,
        },
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Quad::new(Vec2::new(state.right - state.left, 10.)).into()).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_translation(Vec3::new(
                (state.right + state.left) / 2.,
                state.top,
                0.0,
            )),
            ..default()
        },
    ));
    // right wall
    commands.spawn((
        Obstical::Wall {
            normal: RIGHT_WALL_NORMAL,
        },
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Quad::new(Vec2::new(10., state.top - state.bottom)).into()).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_translation(Vec3::new(
                state.right,
                (state.top + state.bottom) / 2.,
                0.0,
            )),
            ..default()
        },
    ));
    // bottom wall
    commands.spawn((
        Obstical::Wall {
            normal: BOTTOM_WALL_NORMAL,
        },
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Quad::new(Vec2::new(state.right - state.left, 10.)).into()).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_translation(Vec3::new(
                (state.right + state.left) / 2.,
                state.bottom,
                0.0,
            )),
            ..default()
        },
    ));

}

const LEFT_WALL_NORMAL: Vec3 = Vec3::new(1.0, 0.0, 0.0);
const RIGHT_WALL_NORMAL: Vec3 = Vec3::new(-1.0, 0.0, 0.0);
const TOP_WALL_NORMAL: Vec3 = Vec3::new(0.0, -1.0, 0.0);
const BOTTOM_WALL_NORMAL: Vec3 = Vec3::new(0.0, 1.0, 0.0);

#[derive(Resource)]
pub struct WallStartupResource {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Plugin for BasicObsticalPlugin {
    fn build(&self, app: &mut App) {
        let left = self
            .extents
            .iter()
            .map(|e| e.x)
            .fold(f32::INFINITY, |a, b| a.min(b));
        let right = self
            .extents
            .iter()
            .map(|e| e.x)
            .fold(f32::NEG_INFINITY, |a, b| a.max(b));
        let bottom = self
            .extents
            .iter()
            .map(|e| e.y)
            .fold(f32::INFINITY, |a, b| a.min(b));
        let top = self
            .extents
            .iter()
            .map(|e| e.y)
            .fold(f32::NEG_INFINITY, |a, b| a.max(b));

        app.insert_resource(WallStartupResource {
            left,
            right,
            top,
            bottom,
        });

        app.add_systems(Startup, add_simple_obsticals);
    }
}
