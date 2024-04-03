use std::path::Path;

use anyhow::Result;

use crate::mini_png::MiniPNG;

pub fn question1(file_path: &Path) -> Result<()> {
    let mini_png = MiniPNG::from_file(file_path)?;
    
    println!("Width: {}", mini_png.get_image_width());
    println!("Height: {}", mini_png.get_image_height());
    println!("Pixel type: {}", mini_png.get_pixel_type());

    Ok(())
}