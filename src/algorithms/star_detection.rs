use crate::image::frame::{Channel, Frame};
use nalgebra::Vector2;

const THRESHOLD: f32 = 4.0;

pub struct WeightedPixel {
    pos: Vector2<f32>,
    weight: f32,
}

pub fn detect_stars<C: Channel>(frame: &Frame<C>) -> Vec<WeightedPixel> {
    let mut stars = Vec::new();

    let double: Vec<f32> = frame.data.iter().map(|c| c.mag()).collect();
    let background: f32 = double.iter().sum() / (frame.width * frame.height) as f32;

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

    let get_mut =
        |vector: &mut [f32], x: usize, y: usize, width: usize| vector.get_mut(y * width + x);

    for y in 10..frame.height - 10 {
        for x in 10..frame.width - 10 {
            if let Some(c) = get_mut(&mut double, x, y, frame.width) {
                if (*c > background) {
                    let mut adjacent_pixels = Vec::new();
                    *c = 0.0;

                    let mut stack = Vec::new();
                    adjacent_pixels.push((x, y, *c));

                    add_adj(&mut stack, x, y);

                    while let Some((ix, iy)) = stack.pop() {
                        if let Some(color) = get_mut(&mut double, ix, iy, frame.width) {
                            if *color > background {
                                adjacent_pixels.push((ix, iy, *color));
                                *color = 0.0;
                                add_adj(&mut stack, ix, iy);
                            }
                        }
                    }
                }
            }
        }
    }

    stars
}
