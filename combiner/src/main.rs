mod args;

use args::Args;
use image::io::Reader;
use image::{DynamicImage, ImageFormat};
use std::fs::File;
use std::io::BufReader;

fn main() -> Result<()> {
    let args: Args = Args::new();
    println!("'println!' called at the func 'main'\n\targs:{:?}", args);

    let (image_1, image_format_1) = find_image_from_path(args.image_1);
    let (image_2, image_format_2) = find_image_from_path(args.image_2);

    if image_format_1 != image_format_2 {
        return Err();
    }

    Ok(())
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
    println!(
        "'println!' called at the func 'find_image_from_path'\n\tpath: {}",
        path
    );

    let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap;
    let image_format: ImageFormat = image_reader.format().unwrap();
    let image: DynamicImage = image_reader.decode().unwrap();
    (image, image_format)
}
