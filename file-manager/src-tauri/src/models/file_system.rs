//! 文件系统数据模型

use serde::{Deserialize, Serialize};

/// 文件项数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileItem {
    /// 唯一标识符（文件路径）
    pub id: String,
    /// 文件名
    pub name: String,
    /// 完整路径
    pub path: String,
    /// 文件类型："file" 或 "folder"
    pub file_type: String,
    /// 文件大小（字节）
    pub size: u64,
    /// 修改日期（ISO 8601 格式）
    pub modified_date: String,
    /// 创建日期（ISO 8601 格式）
    pub created_date: String,
    /// 文件扩展名（仅文件）
    pub extension: Option<String>,
    /// 是否为隐藏文件
    pub is_hidden: bool,
}

/// 目录信息数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryInfo {
    /// 当前路径
    pub path: String,
    /// 父路径
    pub parent_path: Option<String>,
    /// 文件列表
    pub items: Vec<FileItem>,
    /// 总文件数
    pub total_files: usize,
    /// 总文件夹数
    pub total_folders: usize,
}

/// 搜索结果数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// 文件列表
    pub items: Vec<FileItem>,
    /// 总文件数
    pub total: usize,
    /// 当前页码（从1开始）
    pub page: usize,
    /// 每页数量
    pub page_size: usize,
    /// 是否有更多数据
    pub has_more: bool,
}