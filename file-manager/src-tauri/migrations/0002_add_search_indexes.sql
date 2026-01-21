-- 添加搜索索引
-- 优化搜索性能

-- 创建 pg_trgm 扩展（用于 trigram 索引，支持模糊搜索）
-- 如果扩展已存在，则忽略错误
CREATE EXTENSION IF NOT EXISTS pg_trgm;

-- 为tags表添加全文搜索索引（使用 trigram），支持模糊标签搜索
CREATE INDEX IF NOT EXISTS idx_tags_name_trgm ON tags USING GIN (name gin_trgm_ops);

-- 为files表的current_path添加trigram索引，支持模糊路径搜索
CREATE INDEX IF NOT EXISTS idx_files_current_path_trgm ON files USING GIN (current_path gin_trgm_ops);

-- 创建复合索引，优化标签搜索（查询某个标签关联的所有文件）
CREATE INDEX IF NOT EXISTS idx_file_tags_tag_file_composite ON file_tags(tag_id, file_id);

-- 创建视图，方便文件标签查询
CREATE OR REPLACE VIEW file_with_tags AS
SELECT
    f.id as file_id,
    f.current_path,
    f.file_type,
    f.file_size,
    f.created_at as file_created_at,
    f.updated_at as file_updated_at,
    json_agg(
        json_build_object(
            'tag_id', t.id,
            'tag_name', t.name,
            'tag_color', t.color,
            'tag_font_color', t.font_color
        )
    ) FILTER (WHERE t.id IS NOT NULL) as tags
FROM files f
LEFT JOIN file_tags ft ON f.id = ft.file_id
LEFT JOIN tags t ON ft.tag_id = t.id AND t.deleted_at IS NULL
WHERE f.deleted_at IS NULL
GROUP BY f.id, f.current_path, f.file_type, f.file_size, f.created_at, f.updated_at;

-- 创建视图，方便标签统计
CREATE OR REPLACE VIEW tag_statistics AS
SELECT
    t.id as tag_id,
    t.name as tag_name,
    t.color as tag_color,
    t.font_color as tag_font_color,
    t.parent_id,
    t.usage_count,
    COUNT(DISTINCT ft.file_id) as file_count
FROM tags t
LEFT JOIN file_tags ft ON t.id = ft.tag_id
LEFT JOIN files f ON ft.file_id = f.id AND f.deleted_at IS NULL
WHERE t.deleted_at IS NULL
GROUP BY t.id, t.name, t.color, t.font_color, t.parent_id, t.usage_count;
