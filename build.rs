use std::fs;
use std::path::Path;

/// 该函数将配置文件从源位置复制到目标目录中
/// 主要用于构建过程中确保配置文件被正确地包含在输出目录里
///
/// # Panics
/// - 当环境变量"OUT_DIR"不存在时会panic
/// - 当路径操作失败时会panic
/// - 当文件复制失败时会panic
fn main() {
    // 定义源配置文件路径和获取输出目录路径
    let config_file_name = concat!(env!("CARGO_PKG_NAME"), ".yml");
    let config_file_path = Path::new(config_file_name);
    let out_dir = std::env::var("OUT_DIR").unwrap();

    // 构造目标文件路径，通过向上回溯OUT_DIR的父级目录来定位
    let dest_path = Path::new(&out_dir)
        .ancestors()
        .nth(3)
        .unwrap()
        .join(config_file_name);

    // 如果源配置文件存在，则执行复制操作
    if config_file_path.exists() {
        fs::copy(config_file_path, dest_path).expect("Failed to copy config file");
    }
}
