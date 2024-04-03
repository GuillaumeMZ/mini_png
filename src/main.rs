mod block;
mod binary_data;
mod header_block;
mod comment_block;
mod data_block;
mod mini_png;

mod commands;

use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Not enough arguments: please provide the path of the file to parse.");
        std::process::exit(1);
    }

    let path = Path::new(&args[1]);
    match commands::question2(path) {
        Ok(_) => {},
        Err(error) => { eprintln!("Error while trying to parse the file: {}", error); }
    }
}
