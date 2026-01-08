//! 文件系统服务
//!
//! 提供文件系统相关的业务逻辑实现

use std::fs;
use std::path::Path;

use crate::models::file_system::{FileItem, DirectoryInfo};

/// 文件系统服务
pub struct FileSystemService;

impl FileSystemService {
    /// 获取目录内容
    ///
    /// # 参数
    /// - `path`: 目录路径
    ///
    /// # 返回
    /// - `Ok(DirectoryInfo)`: 目录信息
    /// - `Err(String)`: 错误信息
    pub fn list_directory(path: &str) -> Result<DirectoryInfo, String> {
        let dir_path = Path::new(path);

        // 检查路径是否存在
        if !dir_path.exists() {
            return Err(format!("路径不存在: {}", path));
        }

        // 检查是否为目录
        if !dir_path.is_dir() {
            return Err(format!("路径不是目录: {}", path));
        }

        // 读取目录内容
        let entries = fs::read_dir(dir_path)
            .map_err(|e| format!("读取目录失败: {}", e))?;

        let mut items = Vec::new();
        let mut total_files = 0;
        let mut total_folders = 0;

        for entry in entries {
            let entry = entry.map_err(|e| format!("读取目录项失败: {}", e))?;
            let metadata = entry.metadata()
                .map_err(|e| format!("获取文件元数据失败: {}", e))?;

            let file_path = entry.path();
            let file_name = file_path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();

            // 跳过隐藏文件（以.开头）
            if file_name.starts_with('.') {
                continue;
            }

            let is_dir = metadata.is_dir();
            let file_type = if is_dir { "folder" } else { "file" };

            if is_dir {
                total_folders += 1;
            } else {
                total_files += 1;
            }

            // 获取文件扩展名
            let extension = file_path.extension()
                .and_then(|ext| ext.to_str())
                .map(|s| s.to_string());

            // 获取修改时间和创建时间
            let modified = metadata.modified()
                .map_err(|e| format!("获取修改时间失败: {}", e))?;
            let created = metadata.created()
                .unwrap_or(modified);

            // 转换为 ISO 8601 格式
            let modified_date = Self::format_iso8601(&modified);
            let created_date = Self::format_iso8601(&created);

            let is_hidden = file_name.starts_with('.');

            let item = FileItem {
                id: file_path.to_string_lossy().to_string(),
                name: file_name,
                path: file_path.to_string_lossy().to_string(),
                file_type: file_type.to_string(),
                size: metadata.len(),
                modified_date,
                created_date,
                extension,
                is_hidden,
            };

            items.push(item);
        }

        // 排序：文件夹在前，然后按名称排序
        items.sort_by(|a, b| {
            match (a.file_type.as_str(), b.file_type.as_str()) {
                ("folder", "file") => std::cmp::Ordering::Less,
                ("file", "folder") => std::cmp::Ordering::Greater,
                _ => a.name.cmp(&b.name),
            }
        });

        // 获取父路径
        let parent_path = dir_path.parent()
            .map(|p| p.to_string_lossy().to_string());

        Ok(DirectoryInfo {
            path: path.to_string(),
            parent_path,
            items,
            total_files,
            total_folders,
        })
    }

    /// 获取用户主目录
    ///
    /// # 返回
    /// - `Ok(String)`: 用户主目录路径
    /// - `Err(String)`: 错误信息
    pub fn get_home_directory() -> Result<String, String> {
        // Windows 上使用环境变量
        #[cfg(windows)]
        {
            use std::env;
            if let Ok(home) = env::var("USERPROFILE") {
                return Ok(home);
            }
            if let Ok(home) = env::var("HOMEDRIVE") {
                if let Ok(path) = env::var("HOMEPATH") {
                    return Ok(format!("{}{}", home, path));
                }
            }
        }

        // Unix 系统
        #[cfg(unix)]
        {
            use std::env;
            if let Ok(home) = env::var("HOME") {
                return Ok(home);
            }
        }

        Err("无法获取用户主目录".to_string())
    }

    /// 格式化时间为 ISO 8601 格式
    ///
    /// # 参数
    /// - `time`: 系统时间
    ///
    /// # 返回
    /// 格式化的时间字符串（Unix 时间戳格式）
    fn format_iso8601(time: &std::time::SystemTime) -> String {
        use std::time::UNIX_EPOCH;

        let duration = time.duration_since(UNIX_EPOCH)
            .unwrap_or_default();

        let secs = duration.as_secs();
        let nanos = duration.subsec_nanos();

        // 简化的 ISO 8601 格式
        // 实际应该使用 chrono 库，但这里为了简单直接格式化
        format!("{}.{:09}Z", secs, nanos)
    }
}

