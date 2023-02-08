pub mod algorithms;
pub mod image;
pub mod utils;

use utils::png::read_png;

fn main() {
    println!("Hello, world!");

    if let Ok(cwd) = std::env::current_dir() {
        println!("{}", cwd.display());
    }

    let test_input_path: std::path::PathBuf = "K:/Astro/test/gimp".into();

    println!("{}, entries:", test_input_path.display());

    match test_input_path.read_dir() {
        Ok(entries) => {
            for file in entries.filter_map(Result::ok) {
                println!("{}", file.path().display());
                if file.path().extension().unwrap() == "png" {
                    println!("is a png file");
                    read_png(&file.path()).expect("this should work");
                }
            }
        }
        Err(err) => {
            println!("err: {}", err);
        }
    }
}
