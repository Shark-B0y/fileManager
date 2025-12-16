//! Tauri命令模块
//!
//! 提供前后端通信的命令接口
//! 注意：本模块仅包含API接口定义，不包含业务逻辑实现
//! 所有业务逻辑应放在对应的服务模块中

#[tauri::command]
pub async fn greet(name: &str) -> Result<(), String>{
    println!("Hello, {}! You've been greeted from Rust!", name);
    Ok(())
}