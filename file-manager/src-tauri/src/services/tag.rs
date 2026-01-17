//! 标签服务
//!
//! 提供标签相关的业务逻辑实现

use crate::database::{DatabaseConnectionRef, GlobalDatabase};
use crate::models::tag::Tag;
use sqlx::{Pool, Postgres, Sqlite, Row};

/// 标签服务
pub struct TagService;

impl TagService {
    /// 获取标签列表
    ///
    /// # 参数
    /// - `db`: 全局数据库实例
    /// - `limit`: 返回的标签数量限制，默认为 10
    /// - `mode`: 排序模式：
    ///   - "most_used"：按使用次数降序排列（默认）
    ///   - "recent_used"：按更新时间降序排列
    ///
    /// # 返回
    /// - `Ok(Vec<Tag>)`: 标签列表
    /// - `Err(String)`: 错误信息
    pub async fn get_tag_list(
        db: &GlobalDatabase,
        limit: Option<i32>,
        mode: Option<String>,
    ) -> Result<Vec<Tag>, String> {
        let connection = db
            .get_connection()
            .await
            .map_err(|e| format!("获取数据库连接失败: {}", e))?;

        let limit = limit.unwrap_or(10);
        let mode = mode.unwrap_or_else(|| "most_used".to_string());

        match connection {
            DatabaseConnectionRef::Postgres(pool) => {
                Self::get_tag_list_postgres(&pool, limit, &mode).await
            }
            DatabaseConnectionRef::Sqlite(pool) => {
                Self::get_tag_list_sqlite(&pool, limit, &mode).await
            }
        }
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
    pub async fn search_tags(
        db: &GlobalDatabase,
        keyword: String,
        limit: Option<i32>,
    ) -> Result<Vec<Tag>, String> {
        let connection = db
            .get_connection()
            .await
            .map_err(|e| format!("获取数据库连接失败: {}", e))?;

        let limit = limit.unwrap_or(10);

        match connection {
            DatabaseConnectionRef::Postgres(pool) => {
                Self::search_tags_postgres(&pool, &keyword, limit).await
            }
            DatabaseConnectionRef::Sqlite(pool) => {
                Self::search_tags_sqlite(&pool, &keyword, limit).await
            }
        }
    }

    /// 创建新标签
    ///
    /// # 参数
    /// - `db`: 全局数据库实例
    /// - `name`: 标签名称
    ///
    /// # 返回
    /// - `Ok(Tag)`: 创建成功的标签
    /// - `Err(String)`: 错误信息
    pub async fn create_tag(db: &GlobalDatabase, name: String) -> Result<Tag, String> {
        let trimmed_name = name.trim();
        if trimmed_name.is_empty() {
            return Err("标签名称不能为空".to_string());
        }

        let connection = db
            .get_connection()
            .await
            .map_err(|e| format!("获取数据库连接失败: {}", e))?;

        match connection {
            DatabaseConnectionRef::Postgres(pool) => {
                Self::create_tag_postgres(&pool, trimmed_name).await
            }
            DatabaseConnectionRef::Sqlite(pool) => {
                Self::create_tag_sqlite(&pool, trimmed_name).await
            }
        }
    }

    /// PostgreSQL 实现：获取标签列表
    async fn get_tag_list_postgres(
        pool: &Pool<Postgres>,
        limit: i32,
        mode: &str,
    ) -> Result<Vec<Tag>, String> {
        let order_clause = match mode {
            "recent_used" => "ORDER BY updated_at DESC, id ASC",
            _ => "ORDER BY usage_count DESC, id ASC",
        };

        let query = format!(
            r#"
            SELECT
                id,
                name,
                color,
                font_color,
                parent_id,
                usage_count,
                TO_CHAR(created_at, 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as created_at,
                TO_CHAR(updated_at, 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as updated_at
            FROM tags
            WHERE deleted_at IS NULL
            {order_clause}
            LIMIT $1
            "#
        );

        let rows = sqlx::query(&query)
            .bind(limit)
            .fetch_all(pool)
            .await
            .map_err(|e| format!("查询标签失败: {}", e))?;

        let mut tags = Vec::new();
        for row in rows {
            tags.push(Tag {
                id: row.get("id"),
                name: row.get("name"),
                color: row.get("color"),
                font_color: row.get("font_color"),
                parent_id: row.get("parent_id"),
                usage_count: row.get("usage_count"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }

        Ok(tags)
    }

    /// SQLite 实现：获取标签列表
    async fn get_tag_list_sqlite(
        pool: &Pool<Sqlite>,
        limit: i32,
        mode: &str,
    ) -> Result<Vec<Tag>, String> {
        let order_clause = match mode {
            "recent_used" => "ORDER BY updated_at DESC, id ASC",
            _ => "ORDER BY usage_count DESC, id ASC",
        };

        let query = format!(
            r#"
            SELECT
                id,
                name,
                color,
                font_color,
                parent_id,
                usage_count,
                datetime(created_at) as created_at,
                datetime(updated_at) as updated_at
            FROM tags
            WHERE deleted_at IS NULL
            {order_clause}
            LIMIT $1
            "#
        );

        let rows = sqlx::query(&query)
            .bind(limit)
            .fetch_all(pool)
            .await
            .map_err(|e| format!("查询标签失败: {}", e))?;

        let mut tags = Vec::new();
        for row in rows {
            tags.push(Tag {
                id: row.get("id"),
                name: row.get("name"),
                color: row.get("color"),
                font_color: row.get("font_color"),
                parent_id: row.get("parent_id"),
                usage_count: row.get("usage_count"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }

        Ok(tags)
    }

    /// PostgreSQL 实现：搜索标签
    async fn search_tags_postgres(
        pool: &Pool<Postgres>,
        keyword: &str,
        limit: i32,
    ) -> Result<Vec<Tag>, String> {
        let query = format!(
            r#"
            SELECT
                id,
                name,
                color,
                font_color,
                parent_id,
                usage_count,
                TO_CHAR(created_at, 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as created_at,
                TO_CHAR(updated_at, 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as updated_at
            FROM tags
            WHERE deleted_at IS NULL
            AND name ILIKE $1
            ORDER BY usage_count DESC, id ASC
            LIMIT $2
            "#
        );

        let search_pattern = format!("%{}%", keyword);
        let rows = sqlx::query(&query)
            .bind(&search_pattern)
            .bind(limit)
            .fetch_all(pool)
            .await
            .map_err(|e| format!("搜索标签失败: {}", e))?;

        let mut tags = Vec::new();
        for row in rows {
            tags.push(Tag {
                id: row.get("id"),
                name: row.get("name"),
                color: row.get("color"),
                font_color: row.get("font_color"),
                parent_id: row.get("parent_id"),
                usage_count: row.get("usage_count"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }

        Ok(tags)
    }

    /// SQLite 实现：搜索标签
    async fn search_tags_sqlite(
        pool: &Pool<Sqlite>,
        keyword: &str,
        limit: i32,
    ) -> Result<Vec<Tag>, String> {
        let query = format!(
            r#"
            SELECT
                id,
                name,
                color,
                font_color,
                parent_id,
                usage_count,
                datetime(created_at) as created_at,
                datetime(updated_at) as updated_at
            FROM tags
            WHERE deleted_at IS NULL
            AND name LIKE ?1
            ORDER BY usage_count DESC, id ASC
            LIMIT ?2
            "#
        );

        let search_pattern = format!("%{}%", keyword);
        let rows = sqlx::query(&query)
            .bind(&search_pattern)
            .bind(limit)
            .fetch_all(pool)
            .await
            .map_err(|e| format!("搜索标签失败: {}", e))?;

        let mut tags = Vec::new();
        for row in rows {
            tags.push(Tag {
                id: row.get("id"),
                name: row.get("name"),
                color: row.get("color"),
                font_color: row.get("font_color"),
                parent_id: row.get("parent_id"),
                usage_count: row.get("usage_count"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }

        Ok(tags)
    }

    /// PostgreSQL 实现：创建新标签
    async fn create_tag_postgres(pool: &Pool<Postgres>, name: &str) -> Result<Tag, String> {
        // 检查是否已存在同名标签
        let exists_row = sqlx::query(
            r#"
            SELECT 1
            FROM tags
            WHERE name = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(name)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("检查标签是否存在失败: {}", e))?;

        if exists_row.is_some() {
            return Err(format!("标签 \"{}\" 已存在", name));
        }

        // 使用数据库默认值插入
        let row = sqlx::query(
            r#"
            INSERT INTO tags (name)
            VALUES ($1)
            RETURNING
                id,
                name,
                color,
                font_color,
                parent_id,
                usage_count,
                TO_CHAR(created_at, 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as created_at,
                TO_CHAR(updated_at, 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as updated_at
            "#,
        )
        .bind(name)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("创建标签失败: {}", e))?;

        Ok(Tag {
            id: row.get("id"),
            name: row.get("name"),
            color: row.get("color"),
            font_color: row.get("font_color"),
            parent_id: row.get("parent_id"),
            usage_count: row.get("usage_count"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    /// SQLite 实现：创建新标签
    async fn create_tag_sqlite(pool: &Pool<Sqlite>, name: &str) -> Result<Tag, String> {
        // 检查是否已存在同名标签
        let exists_row = sqlx::query(
            r#"
            SELECT 1
            FROM tags
            WHERE name = ?1 AND deleted_at IS NULL
            "#,
        )
        .bind(name)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("检查标签是否存在失败: {}", e))?;

        if exists_row.is_some() {
            return Err(format!("标签 \"{}\" 已存在", name));
        }

        // 使用数据库默认值插入
        let row = sqlx::query(
            r#"
            INSERT INTO tags (name)
            VALUES (?1);

            SELECT
                id,
                name,
                color,
                font_color,
                parent_id,
                usage_count,
                datetime(created_at) as created_at,
                datetime(updated_at) as updated_at
            FROM tags
            WHERE name = ?1 AND deleted_at IS NULL
            ORDER BY id DESC
            LIMIT 1;
            "#,
        )
        .bind(name)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("创建标签失败: {}", e))?;

        Ok(Tag {
            id: row.get("id"),
            name: row.get("name"),
            color: row.get("color"),
            font_color: row.get("font_color"),
            parent_id: row.get("parent_id"),
            usage_count: row.get("usage_count"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
}
