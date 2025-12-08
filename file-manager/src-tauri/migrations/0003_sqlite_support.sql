-- SQLite兼容性支持
-- 为SQLite数据库提供兼容的架构

-- 注意：SQLite不支持某些PostgreSQL特性，这里提供兼容版本

-- 如果使用SQLite，创建兼容的表结构
-- 这个迁移文件在SQLite环境下会被执行，在PostgreSQL环境下会被跳过或使用兼容语法

-- 创建files表（SQLite版本）
CREATE TABLE IF NOT EXISTS files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    file_hash TEXT NOT NULL UNIQUE,
    current_path TEXT NOT NULL,
    file_type TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    fingerprint_data TEXT, -- SQLite不支持JSONB，使用TEXT存储JSON
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

-- 创建索引
CREATE INDEX IF NOT EXISTS idx_files_file_hash ON files(file_hash);
CREATE INDEX IF NOT EXISTS idx_files_current_path ON files(current_path);
CREATE INDEX IF NOT EXISTS idx_files_file_type ON files(file_type);

-- tags表（SQLite版本）
CREATE TABLE IF NOT EXISTS tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    color TEXT,
    parent_id INTEGER,
    usage_count INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP,
    FOREIGN KEY (parent_id) REFERENCES tags(id) ON DELETE CASCADE,
    UNIQUE(name, parent_id)
);

CREATE INDEX IF NOT EXISTS idx_tags_name ON tags(name);
CREATE INDEX IF NOT EXISTS idx_tags_parent_id ON tags(parent_id);

-- file_tags表（SQLite版本）
CREATE TABLE IF NOT EXISTS file_tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    file_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    confidence REAL DEFAULT 1.0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE,
    UNIQUE(file_id, tag_id)
);

CREATE INDEX IF NOT EXISTS idx_file_tags_file_id ON file_tags(file_id);
CREATE INDEX IF NOT EXISTS idx_file_tags_tag_id ON file_tags(tag_id);
CREATE INDEX IF NOT EXISTS idx_file_tags_confidence ON file_tags(confidence);

-- file_changes表（SQLite版本）
CREATE TABLE IF NOT EXISTS file_changes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    file_hash TEXT NOT NULL,
    old_path TEXT,
    new_path TEXT,
    change_type TEXT NOT NULL,
    detected_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    processed_at TIMESTAMP,
    status TEXT DEFAULT 'pending'
);

CREATE INDEX IF NOT EXISTS idx_file_changes_file_hash ON file_changes(file_hash);
CREATE INDEX IF NOT EXISTS idx_file_changes_change_type ON file_changes(change_type);
CREATE INDEX IF NOT EXISTS idx_file_changes_status ON file_changes(status);
CREATE INDEX IF NOT EXISTS idx_file_changes_detected_at ON file_changes(detected_at);

-- SQLite不支持GIN索引和trigram扩展，跳过这些索引创建
-- SQLite不支持视图中的JSON函数，跳过视图创建

-- 创建触发器更新updated_at（SQLite版本）
CREATE TRIGGER IF NOT EXISTS update_files_updated_at
AFTER UPDATE ON files
FOR EACH ROW
BEGIN
    UPDATE files SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER IF NOT EXISTS update_tags_updated_at
AFTER UPDATE ON tags
FOR EACH ROW
BEGIN
    UPDATE tags SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER IF NOT EXISTS update_file_tags_updated_at
AFTER UPDATE ON file_tags
FOR EACH ROW
BEGIN
    UPDATE file_tags SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;