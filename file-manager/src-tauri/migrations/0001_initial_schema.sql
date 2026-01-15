-- 初始数据库架构
-- 创建核心表结构

-- files表：文件信息
CREATE TABLE IF NOT EXISTS files (
    id SERIAL PRIMARY KEY,
    file_hash VARCHAR(64) NOT NULL UNIQUE, -- SHA-256哈希值
    current_path TEXT NOT NULL,
    file_type VARCHAR(10) NOT NULL, -- 'video' 或 'image'
    file_size BIGINT NOT NULL,
    fingerprint_data JSONB, -- 存储视频时长、分辨率、图片尺寸等元数据
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE
);

-- 为file_hash创建索引
CREATE INDEX IF NOT EXISTS idx_files_file_hash ON files(file_hash);
-- 为current_path创建索引
CREATE INDEX IF NOT EXISTS idx_files_current_path ON files(current_path);
-- 为file_type创建索引
CREATE INDEX IF NOT EXISTS idx_files_file_type ON files(file_type);

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
-- 为parent_id创建索引
CREATE INDEX IF NOT EXISTS idx_tags_parent_id ON tags(parent_id);

-- file_tags表：文件-标签关联
CREATE TABLE IF NOT EXISTS file_tags (
    id SERIAL PRIMARY KEY,
    file_id INTEGER NOT NULL REFERENCES files(id) ON DELETE CASCADE,
    tag_id INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    confidence FLOAT DEFAULT 1.0, -- 关联置信度(0.0-1.0)
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(file_id, tag_id)
);

-- 为file_id创建索引
CREATE INDEX IF NOT EXISTS idx_file_tags_file_id ON file_tags(file_id);
-- 为tag_id创建索引
CREATE INDEX IF NOT EXISTS idx_file_tags_tag_id ON file_tags(tag_id);
-- 为confidence创建索引
CREATE INDEX IF NOT EXISTS idx_file_tags_confidence ON file_tags(confidence);

-- file_changes表：文件变更历史
CREATE TABLE IF NOT EXISTS file_changes (
    id SERIAL PRIMARY KEY,
    file_hash VARCHAR(64) NOT NULL,
    old_path TEXT,
    new_path TEXT,
    change_type VARCHAR(20) NOT NULL, -- 'created', 'modified', 'moved', 'deleted'
    detected_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    processed_at TIMESTAMP WITH TIME ZONE,
    status VARCHAR(20) DEFAULT 'pending' -- 'pending', 'processed', 'failed'
);

-- 为file_hash创建索引
CREATE INDEX IF NOT EXISTS idx_file_changes_file_hash ON file_changes(file_hash);
-- 为change_type创建索引
CREATE INDEX IF NOT EXISTS idx_file_changes_change_type ON file_changes(change_type);
-- 为status创建索引
CREATE INDEX IF NOT EXISTS idx_file_changes_status ON file_changes(status);
-- 为detected_at创建索引
CREATE INDEX IF NOT EXISTS idx_file_changes_detected_at ON file_changes(detected_at);

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

-- 为file_tags表创建更新触发器
DROP TRIGGER IF EXISTS update_file_tags_updated_at ON file_tags;
CREATE TRIGGER update_file_tags_updated_at
    BEFORE UPDATE ON file_tags
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();