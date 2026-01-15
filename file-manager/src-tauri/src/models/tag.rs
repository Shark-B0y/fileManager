//! 标签数据模型
//!
//! 定义标签相关的数据结构

use serde::{Deserialize, Serialize};

/// 标签信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    /// 标签ID
    pub id: i32,
    /// 标签名称
    pub name: String,
    /// 标签背景颜色（HEX颜色代码，如#FFFF00）
    pub color: Option<String>,
    /// 标签字体颜色（HEX颜色代码，如#000000）
    pub font_color: Option<String>,
    /// 父标签ID（用于层级标签）
    pub parent_id: Option<i32>,
    /// 使用次数统计
    pub usage_count: i32,
    /// 创建时间
    pub created_at: String,
    /// 更新时间
    pub updated_at: String,
}
