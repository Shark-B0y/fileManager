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
use crate::database::GlobalDatabase;
use crate::models::file_system::DirectoryInfo;
use crate::services::{FileSystemService, TagService};
use crate::models::tag::Tag;
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
    FileSystemService::get_home_directory(&*global_config)
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
/// 如果被剪切的文件原本在 files 表中有数据，则会更新 current_path 字段
///
/// # 参数
/// - `db`: 全局数据库实例
/// - `paths`: 要剪切的文件/文件夹路径列表
/// - `target_path`: 目标目录路径
///
/// # 返回
/// - `Ok(())`: 操作成功
/// - `Err(String)`: 错误信息
#[tauri::command]
pub async fn cut_files(
    db: State<'_, GlobalDatabase>,
    paths: Vec<String>,
    target_path: String,
) -> Result<(), String> {
    FileSystemService::cut_files(&*db, &paths, &target_path).await
}

/// 复制文件
///
/// 将指定的文件/文件夹复制到目标目录
/// 如果被复制的文件原本有 tag，则新生成的文件信息需要复制一份原有的 tag
/// 如果原来的文件没有 tag，则不需要新生成文件信息，也不需要更新 tag
///
/// # 参数
/// - `db`: 全局数据库实例
/// - `paths`: 要复制的文件/文件夹路径列表
/// - `target_path`: 目标目录路径
///
/// # 返回
/// - `Ok(())`: 操作成功
/// - `Err(String)`: 错误信息
#[tauri::command]
pub async fn copy_files(
    db: State<'_, GlobalDatabase>,
    paths: Vec<String>,
    target_path: String,
) -> Result<(), String> {
    FileSystemService::copy_files(&*db, &paths, &target_path).await
}

/// 获取标签列表
///
/// 根据指定模式获取标签列表：
/// - "most_used"：按使用次数降序排列（默认）
/// - "recent_used"：按更新时间降序排列
///
/// # 参数
/// - `db`: 全局数据库实例
/// - `limit`: 返回的标签数量限制，默认为 10
/// - `mode`: 排序模式，"most_used" 或 "recent_used"
///
/// # 返回
/// - `Ok(Vec<Tag>)`: 标签列表
/// - `Err(String)`: 错误信息
#[tauri::command]
pub async fn get_tag_list(
    db: State<'_, GlobalDatabase>,
    limit: Option<i32>,
    mode: Option<String>,
) -> Result<Vec<Tag>, String> {
    TagService::get_tag_list(&*db, limit, mode).await
}

/// 搜索标签
///
/// 根据关键词搜索包含该文字的标签名称（模糊匹配）
///
/// # 参数
/// - `db`: 全局数据库实例
/// - `keyword`: 搜索关键词
/// - `limit`: 返回的标签数量限制，默认为 50
///
/// # 返回
/// - `Ok(Vec<Tag>)`: 匹配的标签列表
/// - `Err(String)`: 错误信息
#[tauri::command]
pub async fn search_tags(
    db: State<'_, GlobalDatabase>,
    keyword: String,
    limit: Option<i32>,
) -> Result<Vec<Tag>, String> {
    TagService::search_tags(&*db, keyword, limit).await
}

/// 创建新标签
///
/// 使用指定名称创建一个新标签，其它字段使用数据库默认值：
/// - color: '#FFFF00'
/// - font_color: '#000000'
/// - usage_count: 0
/// - parent_id: NULL
///
/// # 参数
/// - `db`: 全局数据库实例
/// - `name`: 标签名称
///
/// # 返回
/// - `Ok(Tag)`: 创建成功的标签
/// - `Err(String)`: 错误信息（名称为空或重复等）
#[tauri::command]
pub async fn create_tag(
    db: State<'_, GlobalDatabase>,
    name: String,
) -> Result<Tag, String> {
    TagService::create_tag(&*db, name).await
}

/// 修改标签
///
/// 修改指定标签的信息，可以修改标签名称、背景颜色、字体颜色和父级标签。
/// 如果某个字段传入None，表示不修改该字段；如果传入Some(None)，表示将该字段设置为NULL。
///
/// # 参数
/// - `db`: 全局数据库实例
/// - `id`: 标签ID
/// - `name`: 新标签名称（可选，None表示不修改）
/// - `color`: 新背景颜色（可选，None表示不修改，Some(None)表示设置为NULL）
/// - `font_color`: 新字体颜色（可选，None表示不修改，Some(None)表示设置为NULL）
/// - `parent_id`: 新父标签ID（可选，None表示不修改，Some(None)表示设置为NULL）
///
/// # 返回
/// - `Ok(Tag)`: 修改后的标签
/// - `Err(String)`: 错误信息（标签不存在、名称重复等）
#[tauri::command]
pub async fn modify_tag(
    db: State<'_, GlobalDatabase>,
    id: i32,
    name: Option<String>,
    color: Option<Option<String>>,
    font_color: Option<Option<String>>,
    parent_id: Option<Option<i32>>,
) -> Result<Tag, String> {
    TagService::modify_tag(&*db, id, name, color, font_color, parent_id).await
}

/// 重命名文件或文件夹
///
/// 将指定路径的文件或文件夹重命名为新名称，并更新数据库中的路径记录
///
/// # 参数
/// - `db`: 全局数据库实例
/// - `old_path`: 原文件/文件夹路径
/// - `new_name`: 新名称
///
/// # 返回
/// - `Ok(())`: 操作成功
/// - `Err(String)`: 错误信息
#[tauri::command]
pub async fn rename_file(
    db: State<'_, GlobalDatabase>,
    old_path: String,
    new_name: String,
) -> Result<(), String> {
    FileSystemService::rename_file(&*db, &old_path, &new_name).await
}

/// 删除文件或文件夹
///
/// 删除指定的文件/文件夹列表，支持递归删除文件夹
///
/// # 参数
/// - `paths`: 要删除的文件/文件夹路径列表
///
/// # 返回
/// - `Ok(())`: 操作成功
/// - `Err(String)`: 错误信息
#[tauri::command]
pub async fn delete_files(
    db: State<'_, GlobalDatabase>,
    paths: Vec<String>,
) -> Result<(), String> {
    FileSystemService::delete_files(&*db, &paths).await
}

/// 批量添加标签到文件/文件夹
///
/// 为指定的文件/文件夹列表添加标签。如果文件记录不存在，会自动创建。
///
/// # 参数
/// - `db`: 全局数据库实例
/// - `paths`: 要添加标签的文件/文件夹路径列表
/// - `tag_id`: 标签ID
///
/// # 返回
/// - `Ok(())`: 操作成功
/// - `Err(String)`: 错误信息
#[tauri::command]
pub async fn add_tags_to_files(
    db: State<'_, GlobalDatabase>,
    paths: Vec<String>,
    tag_id: i32,
) -> Result<(), String> {
    TagService::add_tags_to_files(&*db, paths, tag_id).await
}