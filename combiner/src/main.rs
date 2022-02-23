mod args;

use args::Args;
use image::io::Reader;
use image::{DynamicImage, ImageFormat};
use std::fs::File;
use std::io::BufReader;

fn main() {
    let args = Args::new();
    println!("`println!` called\n\t{:?}", args);
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
    let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap;
    let image_format: ImageFormat = image_reader.format().unwrap();
    let image: DynamicImage = image_reader.decode().unwrap();
    (image, image_format)
}
