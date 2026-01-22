//! 文件系统服务
//!
//! 提供文件系统相关的业务逻辑实现

use std::fs;
use std::path::Path;

use crate::models::file_system::{FileItem, DirectoryInfo};
use crate::config::GlobalConfigManager;
use crate::database::{DatabaseConnectionRef, GlobalDatabase};
use sqlx::{Pool, Postgres, Sqlite};

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

        // 规范化当前路径（统一驱动盘格式为 X:\）
        let normalized_path = if Self::is_drive_root(path) {
            #[cfg(windows)]
            {
                let path_trimmed = path.trim();
                let normalized = path_trimmed.replace('/', "\\").to_uppercase();
                if normalized.len() == 2 && normalized.ends_with(':') {
                    format!("{}\\", normalized)
                } else {
                    normalized
                }
            }
            #[cfg(not(windows))]
            {
                path.to_string()
            }
        } else {
            path.to_string()
        };

        // 获取父路径
        let parent_path = if Self::is_drive_root(path) {
            // 如果是驱动盘根目录，父路径设为 "drives:"，用于显示驱动盘列表
            Some("drives:".to_string())
        } else {
            dir_path.parent()
                .map(|p| {
                    let parent = p.to_string_lossy().to_string();
                    // 规范化父路径（如果是驱动盘根目录）
                    if Self::is_drive_root(&parent) {
                        #[cfg(windows)]
                        {
                            let parent_trimmed = parent.trim();
                            let parent_normalized = parent_trimmed.replace('/', "\\").to_uppercase();
                            if parent_normalized.len() == 2 && parent_normalized.ends_with(':') {
                                format!("{}\\", parent_normalized)
                            } else {
                                parent_normalized
                            }
                        }
                        #[cfg(not(windows))]
                        {
                            parent
                        }
                    } else {
                        parent
                    }
                })
        };

        Ok(DirectoryInfo {
            path: normalized_path,
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
    pub fn get_home_directory(global_config: &GlobalConfigManager) -> Result<String, String> {
        // Windows 上使用环境变量
        #[cfg(windows)]
        {
            use std::env;
                // 尝试从全局配置获取主目录路径
            if let Some(home_path) = global_config.get_home_path() {
                // 如果配置中的路径不为空，则使用配置的路径
                if !home_path.is_empty() {
                    return Ok(home_path);
                }
            }
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

    /// 检查路径是否为 Windows 驱动盘根目录
    ///
    /// # 参数
    /// - `path`: 路径字符串
    ///
    /// # 返回
    /// 如果路径是驱动盘根目录（如 "C:\"、"C:/" 或 "C:"），返回 true
    fn is_drive_root(path: &str) -> bool {
        #[cfg(windows)]
        {
            let path_trimmed = path.trim();
            // Windows 驱动盘格式：C:\、C:/ 或 C:
            // 规范化路径：将斜杠统一为反斜杠
            let normalized = path_trimmed.replace('/', "\\").to_uppercase();

            // 匹配格式：X:\ 或 X:（长度为2或3）
            if normalized.len() == 3 && normalized.ends_with(":\\") {
                let drive_letter = normalized.chars().next().unwrap();
                return drive_letter.is_ascii_alphabetic();
            } else if normalized.len() == 2 && normalized.ends_with(':') {
                let drive_letter = normalized.chars().next().unwrap();
                return drive_letter.is_ascii_alphabetic();
            }
        }
        false
    }

    /// 获取所有 Windows 驱动盘列表
    ///
    /// # 返回
    /// - `Ok(DirectoryInfo)`: 包含所有驱动盘的目录信息
    /// - `Err(String)`: 错误信息
    pub fn list_drives() -> Result<DirectoryInfo, String> {
        #[cfg(windows)]
        {
            let mut items = Vec::new();

            // 遍历 A-Z 驱动盘
            for drive_letter in b'A'..=b'Z' {
                let drive = format!("{}:\\", drive_letter as char);
                let drive_path = Path::new(&drive);

                // 检查驱动盘是否存在
                if drive_path.exists() {
                    // 获取驱动盘的元数据
                    let metadata = match fs::metadata(drive_path) {
                        Ok(m) => m,
                        Err(_) => continue,
                    };

                    // 获取修改时间和创建时间
                    let modified = metadata.modified()
                        .unwrap_or_else(|_| std::time::SystemTime::UNIX_EPOCH);
                    let created = metadata.created()
                        .unwrap_or(modified);

                    let modified_date = Self::format_iso8601(&modified);
                    let created_date = Self::format_iso8601(&created);

                    let item = FileItem {
                        id: drive.clone(),
                        name: format!("{}:", drive_letter as char),
                        path: drive.clone(),
                        file_type: "folder".to_string(),
                        size: 0,
                        modified_date,
                        created_date,
                        extension: None,
                        is_hidden: false,
                    };

                    items.push(item);
                }
            }

            // 按驱动盘字母排序
            items.sort_by(|a, b| a.name.cmp(&b.name));

            let total_folders = items.len();

            Ok(DirectoryInfo {
                path: "drives:".to_string(),
                parent_path: None,
                items,
                total_files: 0,
                total_folders,
            })
        }

        #[cfg(not(windows))]
        {
            // 非 Windows 系统返回根目录
            Err("此功能仅支持 Windows 系统".to_string())
        }
    }

    /// 检查路径是否存在且为目录
    ///
    /// # 参数
    /// - `path`: 路径字符串
    ///
    /// # 返回
    /// - `Ok(true)`: 路径存在且为目录
    /// - `Ok(false)`: 路径不存在或不是目录
    /// - `Err(String)`: 错误信息
    pub fn check_path_exists(path: &str) -> Result<bool, String> {
        let dir_path = Path::new(path);

        // 检查路径是否存在
        if !dir_path.exists() {
            return Ok(false);
        }

        // 检查是否为目录
        if !dir_path.is_dir() {
            return Ok(false);
        }

        Ok(true)
    }

    /// 剪切文件（移动文件）
    ///
    /// # 参数
    /// - `paths`: 要剪切的文件/文件夹路径列表
    /// - `target_path`: 目标目录路径
    ///
    /// # 返回
    /// - `Ok(())`: 操作成功
    /// - `Err(String)`: 错误信息
    pub fn cut_files(paths: &[String], target_path: &str) -> Result<(), String> {
        let target_dir = Path::new(target_path);

        // 检查目标路径是否存在且为目录
        if !target_dir.exists() {
            return Err(format!("目标路径不存在: {}", target_path));
        }

        if !target_dir.is_dir() {
            return Err(format!("目标路径不是目录: {}", target_path));
        }

        // 移动每个文件/文件夹
        for path in paths {
            let source_path = Path::new(path);

            if !source_path.exists() {
                return Err(format!("源路径不存在: {}", path));
            }

            // 获取文件名
            let file_name = source_path.file_name()
                .and_then(|n| n.to_str())
                .ok_or_else(|| format!("无法获取文件名: {}", path))?;

            // 构建目标路径
            let dest_path = target_dir.join(file_name);

            // 如果目标路径已存在，返回错误
            if dest_path.exists() {
                return Err(format!("目标路径已存在: {}", dest_path.display()));
            }

            // 移动文件/文件夹
            fs::rename(source_path, &dest_path)
                .map_err(|e| format!("移动文件失败 {} -> {}: {}", path, dest_path.display(), e))?;
        }

        Ok(())
    }

    /// 复制文件
    ///
    /// # 参数
    /// - `paths`: 要复制的文件/文件夹路径列表
    /// - `target_path`: 目标目录路径
    ///
    /// # 返回
    /// - `Ok(())`: 操作成功
    /// - `Err(String)`: 错误信息
    pub fn copy_files(paths: &[String], target_path: &str) -> Result<(), String> {
        let target_dir = Path::new(target_path);

        // 检查目标路径是否存在且为目录
        if !target_dir.exists() {
            return Err(format!("目标路径不存在: {}", target_path));
        }

        if !target_dir.is_dir() {
            return Err(format!("目标路径不是目录: {}", target_path));
        }

        // 复制每个文件/文件夹
        for path in paths {
            let source_path = Path::new(path);

            if !source_path.exists() {
                return Err(format!("源路径不存在: {}", path));
            }

            // 获取文件名
            let file_name = source_path.file_name()
                .and_then(|n| n.to_str())
                .ok_or_else(|| format!("无法获取文件名: {}", path))?;

            // 构建目标路径
            let dest_path = target_dir.join(file_name);

            // 如果目标路径已存在，返回错误
            if dest_path.exists() {
                return Err(format!("目标路径已存在: {}", dest_path.display()));
            }

            // 复制文件/文件夹
            if source_path.is_dir() {
                // 递归复制目录
                Self::copy_directory(source_path, &dest_path)?;
            } else {
                // 复制文件
                fs::copy(source_path, &dest_path)
                    .map_err(|e| format!("复制文件失败 {} -> {}: {}", path, dest_path.display(), e))?;
            }
        }

        Ok(())
    }

    /// 递归复制目录
    ///
    /// # 参数
    /// - `source`: 源目录路径
    /// - `dest`: 目标目录路径
    ///
    /// # 返回
    /// - `Ok(())`: 操作成功
    /// - `Err(String)`: 错误信息
    fn copy_directory(source: &Path, dest: &Path) -> Result<(), String> {
        // 创建目标目录
        fs::create_dir_all(dest)
            .map_err(|e| format!("创建目标目录失败 {}: {}", dest.display(), e))?;

        // 读取源目录内容
        let entries = fs::read_dir(source)
            .map_err(|e| format!("读取目录失败 {}: {}", source.display(), e))?;

        // 复制每个条目
        for entry in entries {
            let entry = entry.map_err(|e| format!("读取目录项失败: {}", e))?;
            let entry_path = entry.path();
            let entry_name = entry_path.file_name()
                .and_then(|n| n.to_str())
                .ok_or_else(|| format!("无法获取文件名: {}", entry_path.display()))?;

            // 跳过隐藏文件
            if entry_name.starts_with('.') {
                continue;
            }

            let dest_entry_path = dest.join(entry_name);

            if entry_path.is_dir() {
                // 递归复制子目录
                Self::copy_directory(&entry_path, &dest_entry_path)?;
            } else {
                // 复制文件
                fs::copy(&entry_path, &dest_entry_path)
                    .map_err(|e| format!("复制文件失败 {} -> {}: {}", entry_path.display(), dest_entry_path.display(), e))?;
            }
        }

        Ok(())
    }

    /// 重命名文件或文件夹
    ///
    /// # 参数
    /// - `db`: 全局数据库实例
    /// - `old_path`: 原文件/文件夹路径
    /// - `new_name`: 新名称
    ///
    /// # 返回
    /// - `Ok(())`: 操作成功
    /// - `Err(String)`: 错误信息
    pub async fn rename_file(
        db: &GlobalDatabase,
        old_path: &str,
        new_name: &str,
    ) -> Result<(), String> {
        let source_path = Path::new(old_path);

        // 检查源路径是否存在
        if !source_path.exists() {
            return Err(format!("源路径不存在: {}", old_path));
        }

        // 验证新名称是否有效（不能包含路径分隔符）
        if new_name.contains('/') || new_name.contains('\\') {
            return Err(format!("新名称不能包含路径分隔符: {}", new_name));
        }

        // 新名称不能为空
        if new_name.trim().is_empty() {
            return Err("新名称不能为空".to_string());
        }

        // 获取父目录
        let parent_dir = source_path.parent()
            .ok_or_else(|| format!("无法获取父目录: {}", old_path))?;

        // 构建新路径
        let new_path = parent_dir.join(new_name);
        let new_path_str = new_path.to_string_lossy().to_string();

        // 如果目标路径已存在，返回错误
        if new_path.exists() {
            return Err(format!("目标路径已存在: {}", new_path.display()));
        }

        // 重命名文件/文件夹
        fs::rename(source_path, &new_path)
            .map_err(|e| format!("重命名失败 {} -> {}: {}", old_path, new_path.display(), e))?;

        // 更新数据库中的路径
        let connection = db
            .get_connection()
            .await
            .map_err(|e| format!("获取数据库连接失败: {}", e))?;
        match connection {
            DatabaseConnectionRef::Postgres(pool) => {
                Self::update_file_path_postgres(&pool, old_path, &new_path_str).await
            }
            DatabaseConnectionRef::Sqlite(pool) => {
                Self::update_file_path_sqlite(&pool, old_path, &new_path_str).await
            }
        }
    }

    /// 删除文件或文件夹
    ///
    /// 删除指定的文件/文件夹列表，支持递归删除文件夹
    ///
    /// # 参数
    /// - `db`: 全局数据库实例
    /// - `paths`: 要删除的文件/文件夹路径列表
    ///
    /// # 返回
    /// - `Ok(())`: 操作成功
    /// - `Err(String)`: 错误信息
    pub async fn delete_files(db: &GlobalDatabase, paths: &[String]) -> Result<(), String> {
        // 先删除文件系统中的文件
        for path in paths {
            let target_path = Path::new(path);

            // 检查路径是否存在
            if !target_path.exists() {
                return Err(format!("路径不存在: {}", path));
            }

            // 删除文件或文件夹
            if target_path.is_dir() {
                // 递归删除目录
                fs::remove_dir_all(target_path)
                    .map_err(|e| format!("删除文件夹失败 {}: {}", path, e))?;
            } else {
                // 删除文件
                fs::remove_file(target_path)
                    .map_err(|e| format!("删除文件失败 {}: {}", path, e))?;
            }
        }

        // 更新数据库：软删除文件记录（设置 deleted_at）
        let connection = db
            .get_connection()
            .await
            .map_err(|e| format!("获取数据库连接失败: {}", e))?;

        match connection {
            DatabaseConnectionRef::Postgres(pool) => {
                Self::soft_delete_files_postgres(&pool, paths).await
            }
            DatabaseConnectionRef::Sqlite(pool) => {
                Self::soft_delete_files_sqlite(&pool, paths).await
            }
        }
    }

    /// PostgreSQL 实现：更新文件路径
    async fn update_file_path_postgres(
        pool: &Pool<Postgres>,
        old_path: &str,
        new_path: &str,
    ) -> Result<(), String> {
        println!("update_file_path_postgres: old_path = {}, new_path = {}", old_path, new_path);
        sqlx::query(
            r#"
            UPDATE files
            SET current_path = $1, updated_at = CURRENT_TIMESTAMP
            WHERE current_path = $2 AND deleted_at IS NULL
            "#,
        )
        .bind(new_path)
        .bind(old_path)
        .execute(pool)
        .await
        .map_err(|e| format!("更新文件路径失败: {}", e))?;

        Ok(())
    }

    /// SQLite 实现：更新文件路径
    async fn update_file_path_sqlite(
        pool: &Pool<Sqlite>,
        old_path: &str,
        new_path: &str,
    ) -> Result<(), String> {
        sqlx::query(
            r#"
            UPDATE files
            SET current_path = ?1, updated_at = CURRENT_TIMESTAMP
            WHERE current_path = ?2 AND deleted_at IS NULL
            "#,
        )
        .bind(new_path)
        .bind(old_path)
        .execute(pool)
        .await
        .map_err(|e| format!("更新文件路径失败: {}", e))?;

        Ok(())
    }

    /// PostgreSQL 实现：软删除文件记录
    async fn soft_delete_files_postgres(
        pool: &Pool<Postgres>,
        paths: &[String],
    ) -> Result<(), String> {
        for path in paths {
            sqlx::query(
                r#"
                UPDATE files
                SET deleted_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP
                WHERE current_path = $1 AND deleted_at IS NULL
                "#,
            )
            .bind(path)
            .execute(pool)
            .await
            .map_err(|e| format!("软删除文件记录失败: {}", e))?;
        }

        Ok(())
    }

    /// SQLite 实现：软删除文件记录
    async fn soft_delete_files_sqlite(
        pool: &Pool<Sqlite>,
        paths: &[String],
    ) -> Result<(), String> {
        for path in paths {
            sqlx::query(
                r#"
                UPDATE files
                SET deleted_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP
                WHERE current_path = ?1 AND deleted_at IS NULL
                "#,
            )
            .bind(path)
            .execute(pool)
            .await
            .map_err(|e| format!("软删除文件记录失败: {}", e))?;
        }

        Ok(())
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

