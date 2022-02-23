mod args;
mod floating_image;
mod helper_functions;
mod image_data_errors;

use crate::floating_image::FloatingImage;
use crate::helper_functions::{combine_images, find_image_from_path, standardise_size};
use args::Args;
use image_data_errors::ImageDataErrors;

fn main() -> Result<(), ImageDataErrors> {
    let args: Args = Args::new();
    println!("'println!' called at the func 'main'\n\targs:{:?}", args);

    let (image_1, image_format_1) = find_image_from_path(args.image_1);
    let (image_2, image_format_2) = find_image_from_path(args.image_2);

    if image_format_1 != image_format_2 {
        return Err(ImageDataErrors::DifferentImageFormats);
    }

    let (image_1, image_2) = standardise_size(image_1, image_2);
    let mut output = FloatingImage::new(image_1.width(), image_1.height(), args.output);
    let combined_data = combine_images(image_1, image_2);
    output.set_data(combined_data)?;

    image::save_buffer_with_format(
        output.name,
        &output.data,
        output.width,
        output.height,
        image::ColorType::Rgb8,
        image_format_1,
    );

    Ok(())
}
