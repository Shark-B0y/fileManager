-- 初始数据库架构
-- 创建核心表结构

-- files表：文件信息
CREATE TABLE IF NOT EXISTS files (
    id SERIAL PRIMARY KEY,
    current_path TEXT NOT NULL UNIQUE, -- 当前路径（唯一约束，确保同一路径只有一个文件记录）
    file_type VARCHAR(10) NOT NULL, -- 'video' 或 'image'
    file_size BIGINT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE
);

-- 为current_path创建索引（用于快速路径查找）
CREATE INDEX IF NOT EXISTS idx_files_current_path ON files(current_path);
-- 为file_type创建索引（用于按类型筛选）
CREATE INDEX IF NOT EXISTS idx_files_file_type ON files(file_type);
-- 为deleted_at创建索引（用于软删除查询）
CREATE INDEX IF NOT EXISTS idx_files_deleted_at ON files(deleted_at) WHERE deleted_at IS NULL;

-- tags表：标签定义
CREATE TABLE IF NOT EXISTS tags (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    color VARCHAR(7) DEFAULT '#FFFF00', -- HEX颜色代码，如#FFFF00（背景颜色）
    font_color VARCHAR(7) DEFAULT '#000000', -- HEX颜色代码，如#000000（字体颜色）
    parent_id INTEGER REFERENCES tags(id) ON DELETE CASCADE,
    usage_count INTEGER DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE,
    UNIQUE(name, parent_id)
);

-- 为name创建索引
CREATE INDEX IF NOT EXISTS idx_tags_name ON tags(name);
-- 为parent_id创建索引（用于层级标签查询）
CREATE INDEX IF NOT EXISTS idx_tags_parent_id ON tags(parent_id);
-- 为usage_count创建索引（用于热门标签排序）
CREATE INDEX IF NOT EXISTS idx_tags_usage_count ON tags(usage_count DESC);

-- file_tags表：文件-标签关联
CREATE TABLE IF NOT EXISTS file_tags (
    file_id INTEGER NOT NULL REFERENCES files(id) ON DELETE CASCADE,
    tag_id INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (file_id, tag_id) -- 复合主键，确保同一文件不会重复添加相同标签
);

-- 为file_id创建索引（用于查询文件的所有标签）
CREATE INDEX IF NOT EXISTS idx_file_tags_file_id ON file_tags(file_id);
-- 为tag_id创建索引（用于查询标签关联的所有文件）
CREATE INDEX IF NOT EXISTS idx_file_tags_tag_id ON file_tags(tag_id);

-- 更新触发器函数
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- 为files表创建更新触发器
DROP TRIGGER IF EXISTS update_files_updated_at ON files;
CREATE TRIGGER update_files_updated_at
    BEFORE UPDATE ON files
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- 为tags表创建更新触发器
DROP TRIGGER IF EXISTS update_tags_updated_at ON tags;
CREATE TRIGGER update_tags_updated_at
    BEFORE UPDATE ON tags
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
