mod args;

use args::Args;
use image::imageops::Triangle;
use image::io::Reader;
use image::{DynamicImage, GenericImageView, ImageFormat};
use std::fs::File;
use std::io::BufReader;

enum ImageDataErrors {
    DifferentImageFormats,
}

fn main() -> Result<(), ImageDataErrors> {
    let args: Args = Args::new();
    println!("'println!' called at the func 'main'\n\targs:{:?}", args);

    let (image_1, image_format_1) = find_image_from_path(args.image_1);
    let (image_2, image_format_2) = find_image_from_path(args.image_2);

    if image_format_1 != image_format_2 {
        return Err(ImageDataErrors::DifferentImageFormats);
    }

    let (image_1, image_2) = standardise_size(image_1, image_2);

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

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
    let pix_1 = dim_1.0 * dim_1.1;
    let pix_2 = dim_2.0 * dim_2.1;

    println!(
        "'println!' called at the func 'get_smallest_dimensions'\n\tpix_1: {}\n\tpix_2: {}",
        pix_1, pix_2
    );

    return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
    let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());

    println!(
        "'println!' called at the func 'standardise_size'\n\twidth: {}\n\theight: {}",
        width, height
    );

    if image_2.dimensions() == (width, height) {
        (image_1.resize_exact(width, height, Triangle), image_2)
    } else {
        (image_1, image_2.resize_exact(width, height, Triangle))
    }
}
