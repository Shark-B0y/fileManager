//! Tauri命令模块
//!
//! ═══════════════════════════════════════════════════════════════════════════
//! ⚠️  重要：代码规范要求  ⚠️
//! ═══════════════════════════════════════════════════════════════════════════
//!
//! 本文件只保留最简单的函数调用和接口定义！
//!
//! 📋 强制要求：
//!   1. ❌ 禁止在此文件中定义数据结构（struct、enum等）
//!      ✅ 所有数据结构应定义在 models/ 模块中
//!
//!   2. ❌ 禁止在此文件中实现业务逻辑
//!      ✅ 所有业务逻辑应实现在 services/ 模块中
//!
//!   3. ✅ 本文件只应包含：
//!      - #[tauri::command] 宏标记的函数
//!      - 函数参数和返回值的类型说明
//!      - 对接口功能的简要说明
//!      - 调用 services 模块中的方法
//!
//! 📝 示例格式：
//!   ```rust
//!   /// 接口功能说明
//!   #[tauri::command]
//!   pub async fn command_name(param: Type) -> Result<ReturnType, String> {
//!       Service::method(param).await
//!   }
//!   ```
//!
//! ═══════════════════════════════════════════════════════════════════════════

use crate::config::GlobalConfigManager;
use crate::models::file_system::DirectoryInfo;
use crate::services::FileSystemService;
use tauri::State;

/// 问候命令（示例命令）
///
/// # 参数
/// - `name`: 要问候的名称
///
/// # 返回
/// 问候消息字符串
#[tauri::command]
pub async fn greet(name: &str) -> Result<String, String> {
    Ok(format!("Hello, {}! You've been greeted from Rust!", name))
}

/// 获取目录内容
///
/// 列出指定目录下的所有文件和文件夹
///
/// # 参数
/// - `path`: 目录路径
///
/// # 返回
/// - `Ok(DirectoryInfo)`: 目录信息，包含文件列表和统计信息
/// - `Err(String)`: 错误信息
#[tauri::command]
pub async fn list_directory(path: String) -> Result<DirectoryInfo, String> {
    FileSystemService::list_directory(&path)
}

/// 获取用户主目录
///
/// 获取当前用户的主目录路径
/// 优先使用全局配置中的 home_path，如果未配置则使用系统默认路径
///
/// # 参数
/// - `global_config`: 全局配置管理器状态
///
/// # 返回
/// - `Ok(String)`: 用户主目录路径
/// - `Err(String)`: 错误信息
#[tauri::command]
pub async fn get_home_directory(
    global_config: State<'_, GlobalConfigManager>,
) -> Result<String, String> {
    FileSystemService::get_home_directory(global_config)
}

/// 获取所有驱动盘列表
///
/// 获取 Windows 系统中所有可用的驱动盘列表
///
/// # 返回
/// - `Ok(DirectoryInfo)`: 包含所有驱动盘的目录信息
/// - `Err(String)`: 错误信息
#[tauri::command]
pub async fn list_drives() -> Result<DirectoryInfo, String> {
    FileSystemService::list_drives()
}

/// 检查路径是否存在且为目录
///
/// 验证指定路径是否存在并且是一个目录
///
/// # 参数
/// - `path`: 要检查的路径
///
/// # 返回
/// - `Ok(true)`: 路径存在且为目录
/// - `Ok(false)`: 路径不存在或不是目录
/// - `Err(String)`: 错误信息
#[tauri::command]
pub async fn check_path_exists(path: String) -> Result<bool, String> {
    FileSystemService::check_path_exists(&path)
}

/// 剪切文件（移动文件）
///
/// 将指定的文件/文件夹移动到目标目录
///
/// # 参数
/// - `paths`: 要剪切的文件/文件夹路径列表
/// - `target_path`: 目标目录路径
///
/// # 返回
/// - `Ok(())`: 操作成功
/// - `Err(String)`: 错误信息
#[tauri::command]
pub async fn cut_files(paths: Vec<String>, target_path: String) -> Result<(), String> {
    FileSystemService::cut_files(&paths, &target_path)
}

/// 复制文件
///
/// 将指定的文件/文件夹复制到目标目录
///
/// # 参数
/// - `paths`: 要复制的文件/文件夹路径列表
/// - `target_path`: 目标目录路径
///
/// # 返回
/// - `Ok(())`: 操作成功
/// - `Err(String)`: 错误信息
#[tauri::command]
pub async fn copy_files(paths: Vec<String>, target_path: String) -> Result<(), String> {
    FileSystemService::copy_files(&paths, &target_path)
}