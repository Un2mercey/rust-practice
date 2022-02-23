use image::imageops::Triangle;
use image::io::Reader;
use image::{DynamicImage, GenericImageView, ImageFormat};
use std::fs::File;
use std::io::BufReader;

pub fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
    println!(
        "'println!' called at the func 'find_image_from_path'\n\tpath: {}",
        path
    );

    let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
    let image_format: ImageFormat = image_reader.format().unwrap();
    let image: DynamicImage = image_reader.decode().unwrap();
    (image, image_format)
}

pub fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
    let pix_1 = dim_1.0 * dim_1.1;
    let pix_2 = dim_2.0 * dim_2.1;

    println!(
        "'println!' called at the func 'get_smallest_dimensions'\n\tpix_1: {}\n\tpix_2: {}",
        pix_1, pix_2
    );

    return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

pub fn standardise_size(
    image_1: DynamicImage,
    image_2: DynamicImage,
) -> (DynamicImage, DynamicImage) {
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

pub fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
    println!("'println!' called at the func 'combine_images'");

    let vec_1 = image_1.to_rgba8().into_vec();
    let vec_2 = image_2.to_rgba8().into_vec();

    alternate_pixels(vec_1, vec_2)
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
    println!("'println!' called at the func 'alternate_pixels'");

    // if vec_1.len() == 5, vec! macro will create the same [0,0,0,0,0]
    let mut combine_data = vec![0u8; vec_1.len()];

    let mut i = 0;
    while i < vec_1.len() {
        if i % 8 == 0 {
            combine_data.splice(i..=i + 3, set_rgba(&vec_1, i, i + 3))
        } else {
            combine_data.splice(i..=i + 3, set_rgba(&vec_2, i, i + 3))
        }
        i += 4;
    }
    combine_data
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
    println!("'println!' called at the func 'set_rgba'");

    let mut rgba: Vec<u8> = Vec::new();
    for i in start..=end {
        let val: u8 = match vec.get(i) {
            Some(d) => *d,
            None => panic!("index out of bounds"),
        };
        rgba.push(val);
    }
    rgba
}
