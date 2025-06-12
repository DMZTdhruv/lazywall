mod cli;
mod hyprpaper;

use cli::LazyWall;
use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let output = LazyWall::parse_config(&args).unwrap_or_else(|err| {
        println!("Error: {}", err);
        process::exit(1);
    });

    LazyWall::apply_wallpaper(output.path);
}
