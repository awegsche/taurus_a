// -------------------------------------------------------------------------------------------------
// ---- Channels -----------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------
//
// ---- Trait and Constants ------------------------------------------------------------------------
pub trait IsColor: Default + Clone + Copy {
    fn mag(&self) -> f32;
}

pub trait Channel: Clone + Copy + Default {
    fn as_f32(&self) -> f32;
}

const THIRD: f32 = 1.0 / 3.0;

// ---- Channels -----------------------------------------------------------------------------------
//
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Chf32(f32);

impl Channel for Chf32 {
    fn as_f32(&self) -> f32 {
        self.0
    }
}
impl From<f32> for Chf32 {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Chu8(u8);

impl Channel for Chu8 {
    fn as_f32(&self) -> f32 {
        self.0 as f32
    }
}

impl From<u8> for Chu8 {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

// ---- RGBf32 - 3 channel float Color -------------------------------------------------------------
#[derive(Debug, Clone, Copy)]
pub struct RGB<Num: Channel> {
    pub r: Num,
    pub g: Num,
    pub b: Num,
}

impl<Num: Channel> Default for RGB<Num> {
    fn default() -> Self {
        Self {
            r: Num::default(),
            g: Num::default(),
            b: Num::default(),
        }
    }
}

impl<Num: Channel> IsColor for RGB<Num> {
    fn mag(&self) -> f32 {
        (self.r.as_f32() + self.g.as_f32() + self.b.as_f32()) * THIRD
    }
}

// ---- Frame --------------------------------------------------------------------------------------

pub struct Frame<Color: IsColor> {
    pub data: Vec<Color>,
    pub width: usize,
    pub height: usize,
}

impl<Color: IsColor> Frame<Color> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![Default::default(); width * height],
            width,
            height,
        }
    }

    pub fn from_data(data: &[Color], width: usize, height: usize) -> Self {
        Self {
            data: data.to_vec(),
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

    pub(crate) fn from_iterator(
        data: impl Iterator<Item = Color>,
        width: usize,
        height: usize,
    ) -> Self {
        Self {
            data: data.collect(),
            width,
            height,
        }
    }
}
