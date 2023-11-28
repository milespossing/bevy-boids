use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::{thread_rng, Rng};

pub struct BevyPlugin {
    n_boids: u32,
}

impl BevyPlugin {
    pub fn new(n_boids: u32) -> Self {
        Self { n_boids }
    }
}

#[derive(Resource)]
struct BevyStartupResource {
    n_boids: u32,
}

#[derive(Component)]
pub struct Boid {
    pub velocity: Vec3,
    pub view_distance: f32,
    // the sin(theta) of the viewing angle
    pub view_angle: f32,
}

impl Boid {
    pub fn new(velocity: Vec3, view_distance: f32, view_angle: f32) -> Self {
        Boid {
            velocity,
            view_distance,
            view_angle,
        }
    }
}

impl Default for Boid {
    fn default() -> Self {
        // 30 units, 2 radians
        Self::new(rand_vec3(-5., 5.), 30., 2.)
    }
}

fn rand_vec3(lower: f32, upper: f32) -> Vec3 {
    let mut rng = thread_rng();
    Vec3::from_array([
        rng.gen_range(lower..upper),
        rng.gen_range(lower..upper),
        rng.gen_range(lower..upper),
    ])
}

enum Obstical {
    Wall { position: Vec3, normal: Vec3 },
}

#[derive(Resource)]
struct Obsticals {
    pub obsticals: Vec<Obstical>,
}

fn basic_2d_obsticals() -> Obsticals {
    let obsticals: Vec<Obstical> =
        vec![Obstical::Wall {
            position: Vec3::from_array([0., 0., 0.]),
            normal: Vec3::from_array([1., 0., 0.]),
        }];
    Obsticals { obsticals }
}

fn generate_boids(
    state: Res<BevyStartupResource>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    for _ in 0..state.n_boids {
        let pos = rand_vec3(-500., 500.);
        commands.spawn((
            Boid { ..default() },
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(5.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_translation(pos),
                ..default()
            },
        ));
    }
}

fn boid_motion(mut query: Query<(&Boid, &mut Transform)>, time: Res<Time>) {
    for (boid, mut transform) in &mut query {
        let delta_x: Vec3 = boid.velocity * time.delta_seconds();
        transform.translation += delta_x;
    }
}

fn obstical_in_view(obstical: &Obstical, boid: &Boid, transform: &Transform) -> bool {
    let closest_point: Vec3 =
        match obstical {
            Obstical::Wall { position, normal } => {
                let v: Vec3 = transform.translation - *position;
                let dist = v.dot(*normal);
                transform.translation - (*normal * dist)
            }
        };
    let distance = transform.translation.distance(closest_point);
    if distance > boid.view_distance {
        return false;
    }
    let proj_dir = closest_point.normalize();
    let face_dir = boid.velocity.normalize();
    let theta = proj_dir.angle_between(face_dir);
    return theta < boid.view_angle;
}

fn obstical_to_avoid_velocity(obstical: &Obstical) -> Vec3 {
    match obstical {
        Obstical::Wall {
            position: _,
            normal,
        } => normal.clone(),
    }
}

fn get_boid_avoid_velocity(obsticals: Vec<&Obstical>) -> Vec3 {
    // TODO: Need to add in the target velocity for obsticals
    obsticals
        .iter()
        .map(|&o| obstical_to_avoid_velocity(o))
        .sum()
}

fn boid_avoid_obsticals(obsticals: Res<Obsticals>, mut query: Query<(&mut Boid, &Transform)>) {
    for (mut boid, transform) in query.iter_mut() {
        let visible: Vec<&Obstical> = obsticals
            .obsticals
            .iter()
            .filter(|&o| obstical_in_view(o, &boid, &transform))
            .collect();
        let avoid_velocity = get_boid_avoid_velocity(visible);
        boid.velocity += avoid_velocity;
    }
}

// TODO: Clamp the boid velocity
// TODO: Set up walls correctly

impl Plugin for BevyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BevyStartupResource {
            n_boids: self.n_boids,
        });
        app.insert_resource(basic_2d_obsticals());
        app.add_systems(Startup, generate_boids);
        app.add_systems(FixedUpdate, boid_motion);
        app.add_systems(FixedUpdate, boid_avoid_obsticals);
    }
}
