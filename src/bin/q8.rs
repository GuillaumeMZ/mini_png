use std::path::Path;

use anyhow::{anyhow, Result};

use mini_png::mini_png::MiniPNG;

fn answer(file_path: &Path) -> Result<()> {
    let mini_png = MiniPNG::from_file(file_path)?;
    
    let palette = mini_png.get_palette().ok_or(anyhow!("Unable to display the palette: no palette found."))?;
    for (i, entry) in palette.entries().iter().enumerate() {
        println!("Entry n°{}: {:?}", i, entry);
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