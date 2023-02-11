pub mod algorithms;
pub mod image;
pub mod utils;

use std::{io, path::Path};

use algorithms::star_detection::WeightedPixel;
use image::frame::{Chu8, Frame, RGB};
use utils::png::read_png;

use crate::algorithms::star_detection::detect_stars;

type FrameRGBu8 = Frame<RGB<Chu8>>;

struct LightFrame {
    pub frame: FrameRGBu8,
    pub detected_stars: Vec<WeightedPixel>,
}

fn main() {
    println!("Hello, world!");

    if let Ok(cwd) = std::env::current_dir() {
        println!("{}", cwd.display());
    }

    //let test_input_path: std::path::PathBuf = "K:/Astro/test/gimp".into();
    let test_input_path: std::path::PathBuf =
        "/media/awegsche/HDD1/rust/taurus_a/tests/gimp".into();

    let lightframes = get_frames_from_folder(test_input_path).expect("this should return 2 frames");

    for l in lightframes.iter() {
        println!("Frame:");
        println!(" - {} detected stars", l.detected_stars.len());
    }
}

fn get_frames_from_folder<P: AsRef<Path>>(path: P) -> io::Result<Vec<LightFrame>> {
    Ok(path
        .as_ref()
        .read_dir()?
        .filter_map(Result::ok)
        .filter_map(|file| match file.path().extension() {
            Some(ext) => {
                if ext == "png" {
                    Some(file)
                } else {
                    None
                }
            }
            None => None,
        })
        .filter_map(|file| read_png(&file.path()).ok())
        .map(|frame| {
            let detected_stars = detect_stars(&frame);
            LightFrame {
                frame,
                detected_stars,
            }
        })
        .collect())
}
