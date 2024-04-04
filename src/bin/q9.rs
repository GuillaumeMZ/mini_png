use std::path::Path;

use anyhow::{anyhow, Result};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use mini_png::mini_png::MiniPNG;

pub fn display(image: &MiniPNG) -> Result<()> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().map_err(|err| anyhow!(err))?;

    let window = video_subsystem.window("MiniPNG Viewer", image.get_image_width(), image.get_image_height())
        .position_centered()
        .build()
        .map_err(|err| anyhow!(err))?;

    let mut canvas = window.into_canvas().build().map_err(|err| anyhow!(err))?;

    let mut event_pump = sdl_context.event_pump().map_err(|err| anyhow!(err))?;

    'running: loop {
        for x in 0..image.get_image_height() {
            for y in 0..image.get_image_width() {
                let pixel = image.get_rgb_at(x, y).unwrap(); //safe unwrap since we cannot go out of bounds
                
                canvas.set_draw_color(pixel);
                canvas.draw_point((y as i32, x as i32)).map_err(|err| anyhow!(err))?;
            }
        }
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Not enough arguments: please provide the path of the file to parse.");
        std::process::exit(1);
    }

    let path = Path::new(&args[1]);
    let image = MiniPNG::from_file(path)?;

    match display(&image) {
        Ok(_) => Ok(()),
        Err(error) => { 
            eprintln!("Error while trying to parse the file: {}", error); 
            std::process::exit(1);
        }
    }
}