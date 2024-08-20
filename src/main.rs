use log::{info};
use std::env;
mod multicast_discovery;

fn main() {
    // 获取可执行文件的路径
    let exe_path = env::current_exe().expect("Failed to get current executable path");

    // 获取可执行文件的父目录
    let exe_dir = exe_path.parent().expect("Failed to get executable directory");

    // 设置当前工作目录为可执行文件的父目录
    env::set_current_dir(&exe_dir).expect("Failed to set current directory");
    
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("Application is starting up...");
}
