use std::path::Path;

use anyhow::{anyhow, Result};

use crate::mini_png::MiniPNG;

pub fn question8(file_path: &Path) -> Result<()> {
    let mini_png = MiniPNG::from_file(file_path)?;
    
    let palette = mini_png.get_palette().ok_or(anyhow!("Unable to display the palette: no palette found."))?;
    for (i, entry) in palette.entries().iter().enumerate() {
        println!("Entry n°{}: {:?}", i, entry);
    }

    Ok(())
}