use crate::image::frame::{Channel, Frame, IsColor};
use nalgebra::Vector2;

const THRESHOLD: f32 = 4.0;

#[derive(Debug, Copy, Clone)]
pub struct WeightedPixel {
    pos: Vector2<f32>,
    weight: f32,
}

impl WeightedPixel {
    pub fn new(x: usize, y: usize, weight: f32) -> Self {
        Self {
            pos: Vector2::new(x as f32, y as f32),
            weight,
        }
    }
    pub fn average(mut self, count: usize) -> Self {
        let fcount = count as f32;
        self.pos /= fcount;
        self.weight /= fcount;
        self
    }
}

pub fn detect_stars<C: IsColor>(frame: &Frame<C>) -> Vec<WeightedPixel> {
    let mut magnitudes: Vec<f32> = frame.data.iter().map(|c| c.mag()).collect();
    let background = magnitudes.iter().sum::<f32>() / (frame.width * frame.height) as f32;

    println!("background: {}", background);

    let add_adj = |stack: &mut Vec<(usize, usize)>, x: usize, y: usize| {
        stack.push((x - 1, y - 1));
        stack.push((x, y - 1));
        stack.push((x + 1, y - 1));

        stack.push((x - 1, y));
        stack.push((x + 1, y));

        stack.push((x - 1, y + 1));
        stack.push((x, y + 1));
        stack.push((x + 1, y + 1));
    };

    let extract_lumi = |pixels: &mut Vec<f32>,
                        x: usize,
                        y: usize,
                        width: usize,
                        threshold: f32,
                        adjacent_pixels: &mut Vec<WeightedPixel>| {
        if let Some(lumi) = pixels.get_mut(y * width + x) {
            if *lumi > threshold {
                adjacent_pixels.push(WeightedPixel::new(x, y, *lumi));
                *lumi = 0.0;
                return true;
            }
        }
        false
    };

    let mut stars = Vec::new();

    let mut adjacent_pixels = Vec::new(); // re-use
    let mut stack = Vec::new();

    println!("detecting stars");
    for y in 10..frame.height - 10 {
        for x in 10..frame.width - 10 {
            adjacent_pixels.clear();
            stack.clear();
            stack.push((x, y));

            while let Some((ix, iy)) = stack.pop() {
                if extract_lumi(
                    &mut magnitudes,
                    ix,
                    iy,
                    frame.width,
                    background,
                    &mut adjacent_pixels,
                ) {
                    add_adj(&mut stack, ix, iy);
                }
            }

            if !adjacent_pixels.is_empty() {
                println!("detected a star, {} adjacent pixels", adjacent_pixels.len());
                let star = adjacent_pixels
                    .iter()
                    .copied()
                    .reduce(|a, b| WeightedPixel {
                        pos: a.pos + b.pos,
                        weight: a.weight + b.weight,
                    })
                    .unwrap()
                    .average(adjacent_pixels.len());

                println!("  {} [{}]", star.pos, star.weight);
                stars.push(star);
            }
        }
    }

    stars
}
