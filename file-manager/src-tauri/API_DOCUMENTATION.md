# 文件管理系统 - 数据库模块 API 文档

## 概述

本文档记录了数据库模块中的所有数据结构和方法，用于指导后续开发。新增功能时应优先使用已存在的方法。

## 模块结构

```
database/
├── mod.rs          # 模块导出
├── config.rs       # 数据库配置
├── connection.rs   # 数据库连接管理
└── error.rs        # 错误处理
```

### 模块导出 (mod.rs)
**导出项：**
- `DatabaseConfig` - 数据库配置结构体
- `DatabaseConnection` - 数据库连接枚举
- `DatabaseManager` - 数据库连接管理器
- `DatabaseConnectionRef` - 数据库连接引用枚举
- `GlobalDatabase` - 全局数据库管理器实例
- `DatabaseError` - 数据库错误枚举
- `DatabaseResult` - 数据库结果类型别名

**测试模块：**
- `tests` - 独立的测试模块（仅在测试时编译）

## 错误处理模块 (error.rs)

### 数据结构

#### `DatabaseError` 枚举
数据库错误类型，包含所有可能的数据库操作错误。

**变体：**
- `Connection(String)` - 数据库连接错误
- `Config(String)` - 数据库配置错误
- `Query(String)` - 数据库查询错误
- `Migration(String)` - 数据库迁移错误
- `Transaction(String)` - 数据库事务错误
- `Other(String)` - 其他数据库错误

**实现特性：**
- `std::fmt::Display` - 格式化显示
- `std::error::Error` - 标准错误特性
- `From<sqlx::Error>` - 从SQLx错误转换
- `From<std::io::Error>` - 从IO错误转换
- `From<serde_json::Error>` - 从JSON解析错误转换

#### `DatabaseResult<T>` 类型别名
数据库操作结果类型，`Result<T, DatabaseError>` 的别名。

## 配置模块 (config.rs)

### 数据结构

#### `DatabaseType` 枚举
数据库类型枚举。

**变体：**
- `Postgres` - PostgreSQL 数据库
- `Sqlite` - SQLite 数据库

**特性：**
- `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Serialize`, `Deserialize`

#### `DatabaseConfig` 结构体
数据库配置结构体。

**字段：**
- `db_type: DatabaseType` - 数据库类型
- `host: Option<String>` - 数据库主机地址（PostgreSQL使用）
- `port: Option<u16>` - 数据库端口（PostgreSQL使用）
- `database: String` - 数据库名称
- `username: Option<String>` - 用户名（PostgreSQL使用）
- `password: Option<String>` - 密码（PostgreSQL使用）
- `sqlite_path: Option<String>` - SQLite文件路径（SQLite使用）
- `max_connections: u32` - 连接池最大连接数（默认：10）
- `connect_timeout: u64` - 连接超时时间（秒，默认：30）

**实现特性：**
- `Default` - 默认实现

### 方法列表

#### `DatabaseConfig` 方法

| 方法名 | 功能描述 | 参数 | 返回值 |
|--------|----------|------|--------|
| `new` | 创建新的数据库配置 | `db_type`, `database`, `host`, `port`, `username`, `password`, `sqlite_path` | `DatabaseConfig` |
| `from_env` | 从环境变量加载配置 | 无 | `Result<Self, String>` |
| `from_toml_file` | 从TOML配置文件加载配置 | `path: P`（TOML文件路径） | `Result<Self, String>` |
| `from_file` | 从JSON配置文件加载配置（兼容旧版本） | `path: P`（JSON文件路径） | `Result<Self, String>` |
| `connection_string` | 生成数据库连接字符串 | 无 | `Result<String, String>` |
| `validate` | 检查配置是否有效 | 无 | `Result<(), String>` |

#### `Default` 实现方法

| 方法名 | 功能描述 | 参数 | 返回值 |
|--------|----------|------|--------|
| `default` | 创建默认配置 | 无 | `DatabaseConfig` |

**默认配置值：**
- `db_type`: `DatabaseType::Postgres`
- `host`: `Some("localhost".to_string())`
- `port`: `Some(5432)`
- `database`: `"file_manager".to_string()`
- `username`: `Some("postgres".to_string())`
- `password`: `Some("password".to_string())`
- `max_connections`: `10`
- `connect_timeout`: `30`

## 连接模块 (connection.rs)

### 数据结构

#### `DatabaseConnection` 枚举
数据库连接枚举。

**变体：**
- `Postgres(Pool<Postgres>)` - PostgreSQL 连接池
- `Sqlite(Pool<Sqlite>)` - SQLite 连接池

#### `DatabaseManager` 结构体
数据库连接管理器。

**字段：**
- `config: DatabaseConfig` - 数据库配置
- `connection: Arc<Mutex<Option<DatabaseConnection>>>` - 数据库连接池（线程安全）

#### `DatabaseConnectionRef` 枚举
数据库连接引用枚举。

**变体：**
- `Postgres(Pool<Postgres>)` - PostgreSQL 连接池引用
- `Sqlite(Pool<Sqlite>)` - SQLite 连接池引用

#### `GlobalDatabase` 结构体
全局数据库管理器实例。

**字段：**
- `manager: Arc<DatabaseManager>` - 数据库管理器引用

### 方法列表

#### `DatabaseManager` 方法

| 方法名 | 功能描述 | 参数 | 返回值 |
|--------|----------|------|--------|
| `new` | 创建新的数据库管理器 | `config: DatabaseConfig` | `Self` |
| `config` | 获取数据库配置 | 无 | `&DatabaseConfig` |
| `init` | 初始化数据库连接 | 无 | `DatabaseResult<()>` |
| `get_connection` | 获取数据库连接 | 无 | `DatabaseResult<DatabaseConnectionRef>` |
| `close` | 关闭数据库连接 | 无 | `DatabaseResult<()>` |
| `check_health` | 检查数据库连接状态 | 无 | `DatabaseResult<bool>` |
| `migrate` | 执行数据库迁移 | 无 | `DatabaseResult<()>` |

#### `DatabaseConnectionRef` 方法

| 方法名 | 功能描述 | 参数 | 返回值 |
|--------|----------|------|--------|
| `as_postgres` | 获取PostgreSQL连接池 | 无 | `Option<&Pool<Postgres>>` |
| `as_sqlite` | 获取SQLite连接池 | 无 | `Option<&Pool<Sqlite>>` |

#### `GlobalDatabase` 方法

| 方法名 | 功能描述 | 参数 | 返回值 |
|--------|----------|------|--------|
| `new` | 创建全局数据库实例 | `config: DatabaseConfig` | `Self` |
| `manager` | 获取数据库管理器引用 | 无 | `&DatabaseManager` |
| `init` | 初始化全局数据库连接 | 无 | `DatabaseResult<()>` |
| `get_connection` | 获取数据库连接 | 无 | `DatabaseResult<DatabaseConnectionRef>` |
| `check_health` | 检查数据库健康状态 | 无 | `DatabaseResult<bool>` |
| `migrate` | 执行数据库迁移 | 无 | `DatabaseResult<()>` |
| `close` | 关闭数据库连接 | 无 | `DatabaseResult<()>` |

## 主模块 (main.rs)

### 函数列表

| 函数名 | 功能描述 | 参数 | 返回值 |
|--------|----------|------|--------|
| `main` | 主程序入口点 | 无 | `Result<(), Box<dyn std::error::Error>>` |

**主函数执行流程：**
1. 加载数据库配置（TOML配置文件 → 环境变量 → 默认配置）
2. 验证配置有效性
3. 创建全局数据库实例
4. 初始化数据库连接
5. 检查数据库健康状态
6. 执行数据库迁移
7. 关闭数据库连接

## 数据库迁移文件

### 迁移文件列表

| 文件名 | 功能描述 |
|--------|----------|
| `0001_initial_schema.sql` | 初始数据库架构，创建核心表结构 |
| `0002_add_search_indexes.sql` | 添加搜索索引，优化搜索性能 |
| `0003_sqlite_support.sql` | SQLite兼容性支持，提供兼容的架构 |

**迁移文件位置：** `src-tauri/migrations/`

### 核心表结构

#### `files` 表 - 文件信息
- `id` - 主键
- `file_hash` - SHA-256文件哈希（唯一标识）
- `current_path` - 当前文件路径
- `file_type` - 文件类型（video/image）
- `file_size` - 文件大小
- `fingerprint_data` - 元数据指纹（JSON格式）
- `created_at` - 创建时间
- `updated_at` - 更新时间
- `deleted_at` - 删除时间（软删除）

#### `tags` 表 - 标签定义
- `id` - 主键
- `name` - 标签名称
- `color` - 标签颜色（HEX格式）
- `parent_id` - 父标签ID（支持层级标签）
- `usage_count` - 使用次数统计
- `created_at` - 创建时间
- `updated_at` - 更新时间
- `deleted_at` - 删除时间（软删除）

#### `file_tags` 表 - 文件-标签关联
- `id` - 主键
- `file_id` - 文件ID
- `tag_id` - 标签ID
- `confidence` - 关联置信度（0.0-1.0）
- `created_at` - 创建时间
- `updated_at` - 更新时间

#### `file_changes` 表 - 文件变更历史
- `id` - 主键
- `file_hash` - 文件哈希
- `old_path` - 旧路径
- `new_path` - 新路径
- `change_type` - 变更类型（created/modified/moved/deleted）
- `detected_at` - 检测时间
- `processed_at` - 处理时间
- `status` - 处理状态（pending/processed/failed）

## 使用示例

### 初始化数据库

```rust
use crate::database::{DatabaseConfig, GlobalDatabase};

// 方法1：使用默认配置
let config = DatabaseConfig::default();

// 方法2：从环境变量加载
let config = DatabaseConfig::from_env().unwrap_or_default();

// 方法3：从TOML配置文件加载（推荐）
let config = DatabaseConfig::from_toml_file("config/database.toml").unwrap_or_default();

// 创建数据库管理器
let db = GlobalDatabase::new(config);

// 初始化连接
db.init().await?;

// 执行迁移
db.migrate().await?;
```

### 执行查询

```rust
// 获取数据库连接
let connection = db.get_connection().await?;

match connection {
    DatabaseConnectionRef::Postgres(pool) => {
        // PostgreSQL查询
        let files = sqlx::query!("SELECT * FROM files LIMIT 10")
            .fetch_all(&pool)
            .await?;
    }
    DatabaseConnectionRef::Sqlite(pool) => {
        // SQLite查询
        let files = sqlx::query!("SELECT * FROM files LIMIT 10")
            .fetch_all(&pool)
            .await?;
    }
}
```

### 错误处理

```rust
use crate::database::error::DatabaseError;

match db.init().await {
    Ok(_) => println!("成功"),
    Err(DatabaseError::Connection(msg)) => eprintln!("连接错误: {}", msg),
    Err(DatabaseError::Config(msg)) => eprintln!("配置错误: {}", msg),
    Err(DatabaseError::Query(msg)) => eprintln!("查询错误: {}", msg),
    Err(DatabaseError::Migration(msg)) => eprintln!("迁移错误: {}", msg),
    Err(DatabaseError::Transaction(msg)) => eprintln!("事务错误: {}", msg),
    Err(DatabaseError::Other(msg)) => eprintln!("其他错误: {}", msg),
}
```

## 开发指南

### 新增功能原则

1. **优先使用现有方法**：在添加新功能前，检查是否已有类似方法
2. **遵循错误处理模式**：使用 `DatabaseError` 和 `DatabaseResult`
3. **保持配置一致性**：使用 `DatabaseConfig` 管理所有配置
4. **支持双数据库**：确保功能同时支持 PostgreSQL 和 SQLite

### 添加新表

1. 在迁移文件中添加 `CREATE TABLE` 语句
2. 更新 API 文档
3. 添加相应的 Rust 模型（后续开发）

### 添加新查询

1. 在适当的模块中添加查询函数
2. 使用 `sqlx::query!` 或 `sqlx::query_as!` 宏
3. 添加错误处理
4. 更新 API 文档

### 测试新功能

1. 编写单元测试
2. 测试 PostgreSQL 和 SQLite 兼容性
3. 验证错误处理

## 注意事项

1. **线程安全**：所有共享资源使用 `Arc<Mutex<T>>` 包装
2. **连接池管理**：合理设置 `max_connections` 和 `connect_timeout`
3. **错误传播**：使用 `?` 操作符传播错误，在顶层处理
4. **资源清理**：确保数据库连接正确关闭
5. **迁移兼容性**：保持迁移文件向后兼容

## 测试模块

### 单元测试
数据库模块包含以下单元测试：

#### `DatabaseManager` 测试
- `test_database_manager_init()` - 测试数据库管理器初始化（无实际数据库时失败）
- `test_sqlite_connection()` - 测试SQLite连接创建、健康检查和关闭

#### `GlobalDatabase` 测试
- `test_global_database()` - 测试全局数据库实例的完整生命周期
  - 初始化数据库连接
  - 健康状态检查
  - 数据库迁移执行
  - 连接关闭

#### `DatabaseConfig` 测试
- `test_default_config()` - 测试默认配置值
- `test_postgres_connection_string()` - 测试PostgreSQL连接字符串生成
- `test_sqlite_connection_string()` - 测试SQLite连接字符串生成
- `test_validate_postgres()` - 测试PostgreSQL配置验证
- `test_validate_sqlite()` - 测试SQLite配置验证

### 测试依赖
- `tempfile` - 创建临时目录和文件用于SQLite测试
- `tokio` - 异步运行时支持

## 版本记录

### v1.0.0 (初始版本)
- 基础数据库配置管理
- PostgreSQL 和 SQLite 双数据库支持
- 连接池管理
- 数据库迁移系统
- 完整的错误处理
- 健康检查功能

### v1.0.1 (2025-12-08)
- 修复模块导出问题：`GlobalDatabase` 未在 `mod.rs` 中导出
- 修复连接池创建：使用 `connect_lazy()` 替代 `connect().await`
- 添加 `GlobalDatabase` 单元测试
- 更新API文档，添加模块导出说明和测试模块文档

### v1.1.0 (2025-12-08)
- 添加TOML配置文件支持：`config/database.toml`
- 新增 `from_toml_file()` 方法从TOML文件加载配置
- 修改主函数配置加载优先级：TOML文件 → 环境变量 → 默认配置
- 创建配置文件模板 `config/database.toml`
- 更新API文档，添加TOML配置说明

### v1.2.0 (2025-12-08)
- 重构测试代码：将所有测试代码移到独立的 `tests.rs` 模块
- 移除 `config.rs` 和 `connection.rs` 中的 `#[cfg(test)]` 模块
- 更新模块结构，添加 `#[cfg(test)] mod tests;` 声明
- 保持测试覆盖率不变，提高代码组织性

---

**文档维护指南：**
- 新增方法时，立即更新本文档
- 修改现有方法时，更新对应的文档条目
- 删除方法时，标记为已弃用并说明替代方案
- 定期检查文档与实际代码的一致性

**最后更新：** 2025-12-08