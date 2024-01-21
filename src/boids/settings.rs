use bevy::prelude::*;

#[derive(Debug, Clone, Copy)]
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

    pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self((left, top, right, bottom))
    }
}

impl PartialEq for Extents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

pub fn extents_from_window(window: &Window) -> Extents {
    // convert the window width and height to f32 tuple
    let (width, height) = (window.physical_width() as f32, window.physical_height() as f32);
    let left = -width / 2.0;
    let right = width / 2.0;
    let top = height / 2.0;
    let bottom = -height / 2.0;
    return Extents::new(left, top, right, bottom);
}


#[derive(Component)]
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
