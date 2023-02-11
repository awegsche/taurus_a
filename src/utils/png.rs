use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

use crate::image::frame::{Chu8, Frame, RGB};

const DS_RATE: u32 = 4;
const DS_SAMPLES: u32 = DS_RATE * DS_RATE;

// how to read pngs: https://github.com/image-rs/image-png/blob/master/examples/show.rs
//
pub fn read_png<P: AsRef<Path>>(path: P) -> io::Result<Frame<RGB<Chu8>>> {
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
    let clr_size = match info.color_type {
        png::ColorType::Rgba => 4,
        _ => 3,
    };

    let get = |slice: &[u8], x: u32, y: u32, stride: u32, channel: usize| {
        slice[clr_size * (x + y * stride) as usize + channel]
    };

    Ok(Frame::from_iterator(
        img_data.chunks(clr_size).map(|ch| -> RGB<Chu8> {
            RGB {
                r: ch[0].into(),
                g: ch[1].into(),
                b: ch[2].into(),
            }
        }),
        width as usize,
        height as usize,
    ))
}
