use std::fs;
use std::path::Path;

fn main() {
    let config_file_path = "pong.yml";
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let config_path = Path::new(config_file_path);
    let dest_path = Path::new(&out_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join(config_file_path);

    if config_path.exists() {
        fs::copy(config_path, dest_path).expect("Failed to copy config file");
    }
}
