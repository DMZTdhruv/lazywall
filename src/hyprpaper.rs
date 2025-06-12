use std::process;

pub fn hyprpaper_status() -> bool {
    let output = process::Command::new("pgrep")
        .args(["-x", "hyprpaper"])
        .output()
        .expect("hyprpaper failed to execute");

    output.status.success()
}

pub fn start_hyprpaper() -> bool {
    let output = process::Command::new("hyprpaper")
        .args(["&"])
        .output()
        .expect("hyprpaper failed to start");

    output.status.success()
}

pub fn hyprctl_apply_wallpaper(path: std::path::PathBuf) {
    let wallpaper_path = path.to_str().unwrap_or_else(|| {
        process::exit(1);
    });

    if !path.exists() {
        println!("File doesn't exist");
        process::exit(1);
    }

    process::Command::new("hyprctl")
        .args(["hyprpaper", "unload", "all"])
        .output()
        .expect("failed to unload previous wallpapers");

    process::Command::new("hyprctl")
        .args(["hyprpaper", "preload", wallpaper_path])
        .output()
        .expect("failed to unload previous wallpapers");

    let screen_wallpaper_path = format!("eDP-1,{}", wallpaper_path);
    let command_arg = screen_wallpaper_path.as_str();
    process::Command::new("hyprctl")
        .args(["hyprpaper", "wallpaper", command_arg])
        .output()
        .expect("failed to unload previous wallpapers");
}
