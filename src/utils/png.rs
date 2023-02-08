use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

use crate::image::image;

const DS_RATE: u32 = 4;
const DS_SAMPLES: u32 = DS_RATE * DS_RATE;

// how to read pngs: https://github.com/image-rs/image-png/blob/master/examples/show.rs
//
pub fn read_png<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let mut decoder = png::Decoder::new(File::open(path)?);

    let mut reader = decoder.read_info()?;
    let mut img_data = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut img_data)?;

    match info.color_type {
        png::ColorType::Grayscale => todo!(),
        png::ColorType::Rgb => {
            println!("color type: RGB");
        }
        png::ColorType::Indexed => todo!(),
        png::ColorType::GrayscaleAlpha => todo!(),
        png::ColorType::Rgba => {
            println!("color type: RGBA");
        }
    }

    let width = info.width;
    let height = info.height;
    let width_ds = info.width / DS_RATE;
    let height_ds = info.height / DS_RATE;
    let clr_size = match info.color_type {
        png::ColorType::Rgba => 4,
        _ => 3,
    };

    let img_downsampled = vec![width_ds * height_ds * 3];

    let get = |slice: &[u8], x: u32, y: u32, stride: u32, channel: usize| {
        slice[clr_size * (x + y * stride) as usize + channel]
    };

    for y in 0..height_ds {
        let y_orig = y * DS_RATE;

        for x in 0..width_ds {
            let x_orig = x * DS_RATE;

            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for dy in 0..DS_RATE {
                for dx in 0..DS_RATE {
                    red += get(&img_data, x_orig + dx, y_orig + dy, width, 0) as u32;
                    green += get(&img_data, x_orig + dx, y_orig + dy, width, 1) as u32;
                    blue += get(&img_data, x_orig + dx, y_orig + dy, width, 2) as u32;
                }
            }

            red /= DS_SAMPLES;
            green /= DS_SAMPLES;
            blue /= DS_SAMPLES;

            print!("\x1b[48;2;{};{};{}m ", red, green, blue);
        }
        println!("|");
    }

    println!("width: {}, width_ds: {}", width, width_ds);

    Ok(())
}
