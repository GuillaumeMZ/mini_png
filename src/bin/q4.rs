use std::path::Path;

use anyhow::{anyhow, Result};

use mini_png::mini_png::MiniPNG;
use mini_png::pixel::{Pixel, PixelType};

fn answer(file_path: &Path) -> Result<()> {
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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Not enough arguments: please provide the path of the file to parse.");
        std::process::exit(1);
    }

    let path = Path::new(&args[1]);
    match answer(path) {
        Ok(_) => {},
        Err(error) => { 
            eprintln!("Error while trying to parse the file: {}", error); 
            std::process::exit(1);
        }
    }
}