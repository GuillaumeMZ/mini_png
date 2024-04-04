use std::path::Path;

use anyhow::{anyhow, Result};

use crate::{mini_png::MiniPNG, pixel::{Pixel, PixelType}};

pub fn question4(file_path: &Path) -> Result<()> {
    let mini_png = MiniPNG::from_file(file_path)?;

    let pixel_type = mini_png.get_pixel_type();
    if pixel_type != PixelType::BlackAndWhite {
        return Err(anyhow!("Unable to display the image: this only works with black and white images, but this image is using {}.", pixel_type))
    }

    for x in 0..mini_png.get_image_height() {
        for y in 0..mini_png.get_image_width() {
            let pixel = mini_png.get_pixel_at(x, y).unwrap(); //safe unwrap since we cannot go out of bounds
            
            match pixel {
                Pixel::Black => print!("X"),
                Pixel::White => print!(" "),
                _ => unreachable!()
            }
        }

        println!();
    }

    Ok(())
}