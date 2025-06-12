use std::process;

use crate::hyprpaper::{hyprctl_apply_wallpaper, hyprpaper_status, start_hyprpaper};

#[derive(Debug)]
pub struct LazyWall {
    pub path: std::path::PathBuf,
}

impl LazyWall {
    pub fn parse_config(args: &[String]) -> Result<Self, &str> {
        if args.len() < 2 {
            println!("Too less arguments");
            println!("LazyWall accepts 1 argument");
            println!("1: wallpaper path");
            return Err("failed to apply wallpaper");
        }

        let path = &args[1];
        let lazy_wall = LazyWall {
            path: std::path::PathBuf::from(path),
        };
        Ok(lazy_wall)
    }

    pub fn apply_wallpaper(path: std::path::PathBuf) {
        if !hyprpaper_status() && !start_hyprpaper() {
            process::exit(1);
        }
        hyprctl_apply_wallpaper(path);
    }
}
