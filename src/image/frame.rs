// -------------------------------------------------------------------------------------------------
// ---- Channels -----------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------
//
// ---- Trait and Constants ------------------------------------------------------------------------
pub trait Channel: Default + Clone {
    fn mag(&self) -> f32;
}

const THIRD: f32 = 1.0 / 3.0;

// ---- RGBf32 - 3 channel float Color -------------------------------------------------------------
#[derive(Debug, Clone, Copy)]
pub struct RGBf32 {
    r: f32,
    g: f32,
    b: f32,
}

impl Default for RGBf32 {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }
}

impl Channel for RGBf32 {
    fn mag(&self) -> f32 {
        (self.r + self.g + self.b) * THIRD
    }
}

pub struct Frame<Color: Channel> {
    pub data: Vec<Color>,
    pub width: usize,
    pub height: usize,
}

impl<Color: Channel> Frame<Color> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![Default::default(); width * height],
            width,
            height,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Color {
        self.data[y * self.width + x]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut Color {
        &mut self.data[y * self.width + x]
    }
}
