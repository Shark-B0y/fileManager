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

    /// 修改标签
    ///
    /// # 参数
    /// - `db`: 全局数据库实例
    /// - `id`: 标签ID
    /// - `name`: 新标签名称（可选）
    /// - `color`: 新背景颜色（可选，None表示不修改）
    /// - `font_color`: 新字体颜色（可选，None表示不修改）
    /// - `parent_id`: 新父标签ID（可选，None表示不修改）
    ///
    /// # 返回
    /// - `Ok(Tag)`: 修改后的标签
    /// - `Err(String)`: 错误信息
    pub async fn modify_tag(
        db: &GlobalDatabase,
        id: i32,
        name: Option<String>,
        color: Option<Option<String>>,
        font_color: Option<Option<String>>,
        parent_id: Option<Option<i32>>,
    ) -> Result<Tag, String> {
        let connection = db
            .get_connection()
            .await
            .map_err(|e| format!("获取数据库连接失败: {}", e))?;

        match connection {
            DatabaseConnectionRef::Postgres(pool) => {
                Self::modify_tag_postgres(&pool, id, name, color, font_color, parent_id).await
            }
            DatabaseConnectionRef::Sqlite(pool) => {
                Self::modify_tag_sqlite(&pool, id, name, color, font_color, parent_id).await
            }
        }
    }

    /// PostgreSQL 实现：修改标签
    async fn modify_tag_postgres(
        pool: &Pool<Postgres>,
        id: i32,
        name: Option<String>,
        color: Option<Option<String>>,
        font_color: Option<Option<String>>,
        parent_id: Option<Option<i32>>,
    ) -> Result<Tag, String> {
        // 检查标签是否存在
        let exists_row = sqlx::query(
            r#"
            SELECT 1
            FROM tags
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("检查标签是否存在失败: {}", e))?;

        if exists_row.is_none() {
            return Err(format!("标签 ID {} 不存在", id));
        }

        // 如果提供了新名称，检查是否与其他标签重复
        if let Some(ref new_name) = name {
            let trimmed_name = new_name.trim();
            if trimmed_name.is_empty() {
                return Err("标签名称不能为空".to_string());
            }

            let exists_row = sqlx::query(
                r#"
                SELECT 1
                FROM tags
                WHERE name = $1 AND id != $2 AND deleted_at IS NULL
                "#,
            )
            .bind(trimmed_name)
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(|e| format!("检查标签名称是否重复失败: {}", e))?;

            if exists_row.is_some() {
                return Err(format!("标签 \"{}\" 已存在", trimmed_name));
            }
        }

        // 构建更新语句
        let mut update_fields = Vec::new();
        let mut bind_index = 1;

        if let Some(ref new_name) = name {
            update_fields.push(format!("name = ${}", bind_index));
            bind_index += 1;
        }

        if let Some(color_opt) = &color {
            update_fields.push(format!("color = ${}", bind_index));
            bind_index += 1;
        }

        if let Some(font_color_opt) = &font_color {
            update_fields.push(format!("font_color = ${}", bind_index));
            bind_index += 1;
        }

        if let Some(parent_id_opt) = &parent_id {
            update_fields.push(format!("parent_id = ${}", bind_index));
            bind_index += 1;
        }

        if update_fields.is_empty() {
            // 如果没有要更新的字段，直接返回当前标签
            return Self::get_tag_by_id_postgres(pool, id).await;
        }

        // 添加updated_at字段
        update_fields.push(format!("updated_at = CURRENT_TIMESTAMP"));

        let query = format!(
            r#"
            UPDATE tags
            SET {}
            WHERE id = ${}
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
            update_fields.join(", "),
            bind_index
        );

        let mut query_builder = sqlx::query(&query);

        if let Some(ref new_name) = name {
            query_builder = query_builder.bind(new_name.trim());
        }

        if let Some(color_opt) = &color {
            query_builder = query_builder.bind(color_opt.as_ref().map(|s| s.as_str()));
        }

        if let Some(font_color_opt) = &font_color {
            query_builder = query_builder.bind(font_color_opt.as_ref().map(|s| s.as_str()));
        }

        if let Some(parent_id_opt) = &parent_id {
            query_builder = query_builder.bind(parent_id_opt.as_ref());
        }

        query_builder = query_builder.bind(id);

        let row = query_builder
            .fetch_one(pool)
            .await
            .map_err(|e| format!("修改标签失败: {}", e))?;

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

    /// SQLite 实现：修改标签
    async fn modify_tag_sqlite(
        pool: &Pool<Sqlite>,
        id: i32,
        name: Option<String>,
        color: Option<Option<String>>,
        font_color: Option<Option<String>>,
        parent_id: Option<Option<i32>>,
    ) -> Result<Tag, String> {
        // 检查标签是否存在
        let exists_row = sqlx::query(
            r#"
            SELECT 1
            FROM tags
            WHERE id = ?1 AND deleted_at IS NULL
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("检查标签是否存在失败: {}", e))?;

        if exists_row.is_none() {
            return Err(format!("标签 ID {} 不存在", id));
        }

        // 如果提供了新名称，检查是否与其他标签重复
        if let Some(ref new_name) = name {
            let trimmed_name = new_name.trim();
            if trimmed_name.is_empty() {
                return Err("标签名称不能为空".to_string());
            }

            let exists_row = sqlx::query(
                r#"
                SELECT 1
                FROM tags
                WHERE name = ?1 AND id != ?2 AND deleted_at IS NULL
                "#,
            )
            .bind(trimmed_name)
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(|e| format!("检查标签名称是否重复失败: {}", e))?;

            if exists_row.is_some() {
                return Err(format!("标签 \"{}\" 已存在", trimmed_name));
            }
        }

        // 构建更新语句
        let mut update_fields = Vec::new();
        let mut bind_index = 1;

        if let Some(ref new_name) = name {
            update_fields.push(format!("name = ?{}", bind_index));
            bind_index += 1;
        }

        if let Some(_) = &color {
            update_fields.push(format!("color = ?{}", bind_index));
            bind_index += 1;
        }

        if let Some(_) = &font_color {
            update_fields.push(format!("font_color = ?{}", bind_index));
            bind_index += 1;
        }

        if let Some(_) = &parent_id {
            update_fields.push(format!("parent_id = ?{}", bind_index));
            bind_index += 1;
        }

        if update_fields.is_empty() {
            // 如果没有要更新的字段，直接返回当前标签
            return Self::get_tag_by_id_sqlite(pool, id).await;
        }

        // 添加updated_at字段
        update_fields.push("updated_at = CURRENT_TIMESTAMP".to_string());

        let query = format!(
            r#"
            UPDATE tags
            SET {}
            WHERE id = ?{}
            "#,
            update_fields.join(", "),
            bind_index
        );

        let mut query_builder = sqlx::query(&query);

        if let Some(ref new_name) = name {
            query_builder = query_builder.bind(new_name.trim());
        }

        if let Some(color_opt) = &color {
            query_builder = query_builder.bind(color_opt.as_ref().map(|s| s.as_str()));
        }

        if let Some(font_color_opt) = &font_color {
            query_builder = query_builder.bind(font_color_opt.as_ref().map(|s| s.as_str()));
        }

        if let Some(parent_id_opt) = &parent_id {
            query_builder = query_builder.bind(parent_id_opt.as_ref());
        }

        query_builder = query_builder.bind(id);

        query_builder
            .execute(pool)
            .await
            .map_err(|e| format!("修改标签失败: {}", e))?;

        // 返回更新后的标签
        Self::get_tag_by_id_sqlite(pool, id).await
    }

    /// PostgreSQL 实现：根据ID获取标签
    async fn get_tag_by_id_postgres(pool: &Pool<Postgres>, id: i32) -> Result<Tag, String> {
        let row = sqlx::query(
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
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("查询标签失败: {}", e))?;

        match row {
            Some(row) => Ok(Tag {
                id: row.get("id"),
                name: row.get("name"),
                color: row.get("color"),
                font_color: row.get("font_color"),
                parent_id: row.get("parent_id"),
                usage_count: row.get("usage_count"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }),
            None => Err(format!("标签 ID {} 不存在", id)),
        }
    }

    /// SQLite 实现：根据ID获取标签
    async fn get_tag_by_id_sqlite(pool: &Pool<Sqlite>, id: i32) -> Result<Tag, String> {
        let row = sqlx::query(
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
            WHERE id = ?1 AND deleted_at IS NULL
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("查询标签失败: {}", e))?;

        match row {
            Some(row) => Ok(Tag {
                id: row.get("id"),
                name: row.get("name"),
                color: row.get("color"),
                font_color: row.get("font_color"),
                parent_id: row.get("parent_id"),
                usage_count: row.get("usage_count"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }),
            None => Err(format!("标签 ID {} 不存在", id)),
        }
    }
}
