//! 标签服务
//!
//! 提供标签相关的业务逻辑实现

use crate::database::{DatabaseConnectionRef, GlobalDatabase};
use crate::models::tag::Tag;
use sqlx::{Pool, Postgres, Sqlite, Row};

/// 标签服务
pub struct TagService;

impl TagService {
    /// 获取使用数量最多的标签
    ///
    /// # 参数
    /// - `db`: 全局数据库实例
    /// - `limit`: 返回的标签数量限制，默认为 10
    ///
    /// # 返回
    /// - `Ok(Vec<Tag>)`: 标签列表，按使用次数降序排列
    /// - `Err(String)`: 错误信息
    pub async fn get_most_used_tags(
        db: &GlobalDatabase,
        limit: Option<i32>,
    ) -> Result<Vec<Tag>, String> {
        let connection = db
            .get_connection()
            .await
            .map_err(|e| format!("获取数据库连接失败: {}", e))?;

        let limit = limit.unwrap_or(10);

        match connection {
            DatabaseConnectionRef::Postgres(pool) => {
                Self::get_most_used_tags_postgres(&pool, limit).await
            }
            DatabaseConnectionRef::Sqlite(pool) => {
                Self::get_most_used_tags_sqlite(&pool, limit).await
            }
        }
    }

    /// PostgreSQL 实现：获取使用数量最多的标签
    async fn get_most_used_tags_postgres(
        pool: &Pool<Postgres>,
        limit: i32,
    ) -> Result<Vec<Tag>, String> {
        let rows = sqlx::query(
            r#"
            SELECT
                id,
                name,
                color,
                parent_id,
                usage_count,
                TO_CHAR(created_at, 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as created_at,
                TO_CHAR(updated_at, 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as updated_at
            FROM tags
            WHERE deleted_at IS NULL
            ORDER BY usage_count DESC, id ASC
            LIMIT $1
            "#,
        )
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
                parent_id: row.get("parent_id"),
                usage_count: row.get("usage_count"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }

        Ok(tags)
    }

    /// SQLite 实现：获取使用数量最多的标签
    async fn get_most_used_tags_sqlite(
        pool: &Pool<Sqlite>,
        limit: i32,
    ) -> Result<Vec<Tag>, String> {
        let rows = sqlx::query(
            r#"
            SELECT
                id,
                name,
                color,
                parent_id,
                usage_count,
                datetime(created_at) as created_at,
                datetime(updated_at) as updated_at
            FROM tags
            WHERE deleted_at IS NULL
            ORDER BY usage_count DESC, id ASC
            LIMIT $1
            "#,
        )
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
                parent_id: row.get("parent_id"),
                usage_count: row.get("usage_count"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }

        Ok(tags)
    }
}
