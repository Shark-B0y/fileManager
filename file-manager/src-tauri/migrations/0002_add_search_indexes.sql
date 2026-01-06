-- 添加搜索索引
-- 优化搜索性能

-- 创建 pg_trgm 扩展（用于 trigram 索引，支持模糊搜索）
-- 如果扩展已存在，则忽略错误
CREATE EXTENSION IF NOT EXISTS pg_trgm;

-- 为files表的fingerprint_data添加GIN索引，支持JSON查询
CREATE INDEX IF NOT EXISTS idx_files_fingerprint_data_gin ON files USING GIN (fingerprint_data);

-- 为tags表添加全文搜索索引（使用 trigram）
CREATE INDEX IF NOT EXISTS idx_tags_name_trgm ON tags USING GIN (name gin_trgm_ops);

-- 为files表的current_path添加trigram索引，支持模糊路径搜索
CREATE INDEX IF NOT EXISTS idx_files_current_path_trgm ON files USING GIN (current_path gin_trgm_ops);

-- 创建复合索引，优化标签搜索
CREATE INDEX IF NOT EXISTS idx_file_tags_file_tag_composite ON file_tags(file_id, tag_id, confidence);

-- 创建视图，方便文件标签查询
CREATE OR REPLACE VIEW file_with_tags AS
SELECT
    f.id as file_id,
    f.file_hash,
    f.current_path,
    f.file_type,
    f.file_size,
    f.fingerprint_data,
    f.created_at as file_created_at,
    f.updated_at as file_updated_at,
    json_agg(
        json_build_object(
            'tag_id', t.id,
            'tag_name', t.name,
            'tag_color', t.color,
            'confidence', ft.confidence
        )
    ) as tags
FROM files f
LEFT JOIN file_tags ft ON f.id = ft.file_id
LEFT JOIN tags t ON ft.tag_id = t.id
WHERE f.deleted_at IS NULL
GROUP BY f.id, f.file_hash, f.current_path, f.file_type, f.file_size, f.fingerprint_data, f.created_at, f.updated_at;

-- 创建视图，方便标签统计
CREATE OR REPLACE VIEW tag_statistics AS
SELECT
    t.id as tag_id,
    t.name as tag_name,
    t.color as tag_color,
    t.parent_id,
    t.usage_count,
    COUNT(DISTINCT ft.file_id) as file_count,
    COUNT(DISTINCT CASE WHEN ft.confidence >= 0.8 THEN ft.file_id END) as high_confidence_count,
    MIN(ft.confidence) as min_confidence,
    MAX(ft.confidence) as max_confidence,
    AVG(ft.confidence) as avg_confidence
FROM tags t
LEFT JOIN file_tags ft ON t.id = ft.tag_id
WHERE t.deleted_at IS NULL
GROUP BY t.id, t.name, t.color, t.parent_id, t.usage_count;