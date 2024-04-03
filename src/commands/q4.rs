use std::path::Path;

use anyhow::{anyhow, Result};

use crate::{header_block::{Pixel, PixelType}, mini_png::MiniPNG};

pub fn question4(file_path: &Path) -> Result<()> {
    let mini_png = MiniPNG::from_file(file_path)?;

    let pixel_type = mini_png.get_pixel_type();
    if pixel_type != PixelType::BlackAndWhite {
        return Err(anyhow!("Unable to display the image: this only works with black and white images, but this image is using {}.", pixel_type))
    }

    for x in 0..mini_png.get_image_width() {
        for y in 0..mini_png.get_image_height() {
            let pixel = mini_png.get_pixel_at(x, y).unwrap(); //safe unwrap since we cannot go out of bounds
            let is_white = match pixel {
                Pixel::BlackAndWhite(is_white) => is_white,
                _ => unreachable!()
            };

            if !is_white {
                print!("X");
            } else {
                print!(" ");
            }
        }

        println!();
    }

    Ok(())
}