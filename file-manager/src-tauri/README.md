# 文件管理系统 - 数据库模块

## 概述

这是文件管理系统的数据库模块，基于Rust和SQLx构建，支持PostgreSQL和SQLite数据库。

## 功能特性

- ✅ 支持PostgreSQL和SQLite双数据库
- ✅ 数据库连接池管理
- ✅ 配置管理（环境变量、配置文件、默认配置）
- ✅ 错误处理
- ✅ 数据库迁移
- ✅ 健康检查
- ✅ 线程安全

## 项目结构

```
src-tauri/
├── src/
│   ├── database/          # 数据库访问层
│   │   ├── mod.rs          # 模块导出
│   │   ├── config.rs       # 数据库配置
│   │   ├── connection.rs   # 数据库连接管理
│   │   └── error.rs        # 错误处理
│   ├── system/            # 系统集成层
│   │   ├── mod.rs          # 模块导出
│   │   └── init.rs         # 系统初始化（数据库初始化）
│   ├── lib.rs             # 库入口（Tauri应用启动）
│   └── main.rs            # 主程序入口
├── migrations/            # 数据库迁移文件
│   ├── 0001_initial_schema.sql
│   ├── 0002_add_search_indexes.sql
│   └── 0003_sqlite_support.sql
├── config/                # 配置文件目录
│   └── database.toml      # 数据库配置文件
├── Cargo.toml            # 依赖配置
└── README.md             # 本文档
```

## 快速开始

### 环境要求

- Rust 1.70+
- PostgreSQL 15+ 或 SQLite 3.35+

### 安装依赖

```bash
cd src-tauri
cargo build
```

### 配置数据库

#### 方式1：环境变量

```bash
# PostgreSQL配置
export DATABASE_TYPE=postgres
export DATABASE_HOST=localhost
export DATABASE_PORT=5432
export DATABASE_NAME=file_manager
export DATABASE_USERNAME=postgres
export DATABASE_PASSWORD=password

# SQLite配置
export DATABASE_TYPE=sqlite
export DATABASE_SQLITE_PATH=/path/to/database.db
```

#### 方式2：使用默认配置

默认使用PostgreSQL配置：
- 主机：localhost
- 端口：5432
- 数据库：file_manager
- 用户名：postgres
- 密码：password

### 运行测试

```bash
cargo run
```

## 数据库架构

### 核心表

1. **files表** - 文件信息
   - `file_hash`: SHA-256文件哈希（唯一标识）
   - `current_path`: 当前文件路径
   - `file_type`: 文件类型（video/image）
   - `file_size`: 文件大小
   - `fingerprint_data`: 元数据指纹（JSON格式）

2. **tags表** - 标签定义
   - `name`: 标签名称
   - `color`: 标签颜色
   - `parent_id`: 父标签ID（支持层级标签）
   - `usage_count`: 使用次数统计

3. **file_tags表** - 文件-标签关联
   - `file_id`: 文件ID
   - `tag_id`: 标签ID
   - `confidence`: 关联置信度（0.0-1.0）

4. **file_changes表** - 文件变更历史
   - `file_hash`: 文件哈希
   - `old_path`/`new_path`: 路径变更
   - `change_type`: 变更类型
   - `status`: 处理状态

### 索引优化

- 文件哈希索引：快速文件查找
- 路径索引：支持路径搜索
- 标签名称索引：支持标签搜索
- JSON GIN索引：支持元数据查询
- 复合索引：优化关联查询

## API使用示例

### 应用启动时自动初始化数据库

应用启动时会自动初始化数据库，优先从配置文件 `config/database.toml` 加载配置，如果配置文件不存在或加载失败，则使用默认配置。

初始化逻辑位于 `system/init.rs` 模块中：

```rust
use crate::system::init::init_database;

// 在应用启动时调用（lib.rs 中已实现）
let db = init_database("config/database.toml").await?;
```

### 手动初始化数据库

如果需要手动初始化数据库，可以使用以下方式：

```rust
use crate::database::{DatabaseConfig, GlobalDatabase};

// 方式1：从配置文件初始化
let db = GlobalDatabase::init_from_config_file("config/database.toml").await?;
db.migrate().await?;

// 方式2：使用默认配置初始化
let db = GlobalDatabase::init_from_default_config().await?;
db.migrate().await?;

// 方式3：自定义配置
let config = DatabaseConfig::from_env().unwrap_or_default();
let db = GlobalDatabase::new(config);
db.init().await?;
db.migrate().await?;
```

### 获取数据库连接

```rust
// 获取连接
let connection = db.get_connection().await?;

match connection {
    DatabaseConnectionRef::Postgres(pool) => {
        // 执行PostgreSQL查询
        let result = sqlx::query("SELECT * FROM files LIMIT 10")
            .fetch_all(&pool)
            .await?;
    }
    DatabaseConnectionRef::Sqlite(pool) => {
        // 执行SQLite查询
        let result = sqlx::query("SELECT * FROM files LIMIT 10")
            .fetch_all(&pool)
            .await?;
    }
}
```

### 错误处理

```rust
use crate::database::error::DatabaseError;

match db.init().await {
    Ok(_) => println!("数据库初始化成功"),
    Err(DatabaseError::Connection(msg)) => eprintln!("连接错误: {}", msg),
    Err(DatabaseError::Config(msg)) => eprintln!("配置错误: {}", msg),
    Err(e) => eprintln!("其他错误: {}", e),
}
```

## 开发指南

### 添加新的数据库表

1. 在迁移文件中添加CREATE TABLE语句
2. 更新对应的Rust模型（后续添加）
3. 添加必要的索引

### 添加新的查询功能

1. 在适当的模块中添加查询函数
2. 使用SQLx的query!宏或query_as!宏
3. 添加错误处理

### 测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_database_manager_init
```

## 注意事项

1. **数据库选择**：
   - PostgreSQL：功能完整，性能好，适合生产环境
   - SQLite：轻量级，适合开发和测试环境

2. **连接池**：
   - 默认最大连接数：10
   - 连接超时：30秒
   - 根据实际负载调整

3. **迁移文件**：
   - 按顺序执行（0001, 0002, 0003...）
   - 支持回滚
   - 兼容PostgreSQL和SQLite

4. **错误处理**：
   - 使用DatabaseError统一错误类型
   - 提供详细的错误信息
   - 支持错误转换

## 故障排除

### 连接失败

1. 检查数据库服务是否运行
2. 验证配置信息（主机、端口、用户名、密码）
3. 检查网络连接
4. 查看防火墙设置

### 迁移失败

1. 检查迁移文件语法
2. 确认数据库用户有足够的权限
3. 查看错误日志

### 性能问题

1. 调整连接池大小
2. 添加合适的索引
3. 优化查询语句

## 许可证

MIT