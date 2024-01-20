use bevy::prelude::*;

pub struct Extents(pub (f32, f32, f32, f32));

impl Extents {
    pub fn left(&self) -> f32 {
        self.0 .0
    }

    pub fn right(&self) -> f32 {
        self.0 .2
    }

    pub fn top(&self) -> f32 {
        self.0 .1
    }

    pub fn bottom(&self) -> f32 {
        self.0 .3
    }
}

#[derive(Resource)]
pub struct SimulationSettings {
    // Clock-wise, starting with left, in pixels from (0,0) along dimension
    // so (-10, 10, 10, -10) would be a square with sides of 20 from (-10,-10) to (10,10)
    pub extents: Extents,
    // distance from simulation extents boids start turning
    pub margin: f32,
}

impl Default for SimulationSettings {
    fn default() -> Self {
        Self {
            extents: Extents((-500., 500., 500., -500.)),
            margin: 10.,
        }
    }
}
