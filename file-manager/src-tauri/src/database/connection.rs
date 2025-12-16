//! 数据库连接模块
//!
//! 提供数据库连接池管理和连接操作

use sqlx::{Pool, Postgres, Sqlite};
use sqlx::postgres::PgPoolOptions;
use sqlx::sqlite::SqlitePoolOptions;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::database::config::DatabaseConfig;
use crate::database::error::{DatabaseError, DatabaseResult};

/// 数据库连接枚举
pub enum DatabaseConnection {
    /// PostgreSQL 连接池
    Postgres(Pool<Postgres>),
    /// SQLite 连接池
    Sqlite(Pool<Sqlite>),
}

/// 数据库连接管理器
pub struct DatabaseManager {
    /// 数据库配置
    config: DatabaseConfig,
    /// 数据库连接池（使用Arc和Mutex实现线程安全）
    connection: Arc<Mutex<Option<DatabaseConnection>>>,
}

impl DatabaseManager {
    /// 创建新的数据库管理器
    pub fn new(config: DatabaseConfig) -> Self {
        Self {
            config,
            connection: Arc::new(Mutex::new(None)),
        }
    }

    /// 获取数据库配置
    pub fn config(&self) -> &DatabaseConfig {
        &self.config
    }

    /// 初始化数据库连接
    pub async fn init(&self) -> DatabaseResult<()> {
        let mut connection = self.connection.lock().await;

        if connection.is_some() {
            return Ok(());
        }

        let db_connection = match self.config.db_type {
            crate::database::config::DatabaseType::Postgres => {
                let conn_str = self.config.connection_string()
                    .map_err(|e| DatabaseError::Config(e))?;

                let pool = PgPoolOptions::new()
                    .max_connections(self.config.max_connections)
                    .acquire_timeout(std::time::Duration::from_secs(self.config.connect_timeout))
                    .connect_lazy(&conn_str)
                    .map_err(|e| DatabaseError::Connection(e.to_string()))?;

                // 测试连接
                sqlx::query("SELECT 1")
                    .execute(&pool)
                    .await
                    .map_err(|e| DatabaseError::Connection(e.to_string()))?;

                DatabaseConnection::Postgres(pool)
            }
            crate::database::config::DatabaseType::Sqlite => {
                let conn_str = self.config.connection_string()
                    .map_err(|e| DatabaseError::Config(e))?;

                // 确保SQLite文件目录存在
                if let Some(sqlite_path) = &self.config.sqlite_path {
                    let path = std::path::Path::new(sqlite_path);
                    if let Some(parent) = path.parent() {
                        println!("创建SQLite目录: {:?}", parent);
                        if let Err(e) = std::fs::create_dir_all(parent) {
                            eprintln!("创建目录失败: {:?}", e);
                        }
                    }
                    println!("SQLite文件路径: {:?}", path);
                }

                println!("SQLite连接字符串: {}", conn_str);
                let pool = SqlitePoolOptions::new()
                    .max_connections(self.config.max_connections)
                    .acquire_timeout(std::time::Duration::from_secs(self.config.connect_timeout))
                    .connect_lazy(&conn_str)
                    .map_err(|e| {
                        println!("SQLite连接失败: {}", e);
                        DatabaseError::Connection(e.to_string())
                    })?;

                // 测试连接
                sqlx::query("SELECT 1")
                    .execute(&pool)
                    .await
                    .map_err(|e| DatabaseError::Connection(e.to_string()))?;

                DatabaseConnection::Sqlite(pool)
            }
        };

        *connection = Some(db_connection);
        Ok(())
    }

    /// 获取数据库连接
    pub async fn get_connection(&self) -> DatabaseResult<DatabaseConnectionRef> {
        let connection = self.connection.lock().await;

        match connection.as_ref() {
            Some(DatabaseConnection::Postgres(pool)) => {
                Ok(DatabaseConnectionRef::Postgres(pool.clone()))
            }
            Some(DatabaseConnection::Sqlite(pool)) => {
                Ok(DatabaseConnectionRef::Sqlite(pool.clone()))
            }
            None => Err(DatabaseError::Connection("数据库未初始化".to_string())),
        }
    }

    /// 关闭数据库连接
    pub async fn close(&self) -> DatabaseResult<()> {
        let mut connection = self.connection.lock().await;

        if let Some(conn) = connection.take() {
            match conn {
                DatabaseConnection::Postgres(pool) => {
                    pool.close().await;
                }
                DatabaseConnection::Sqlite(pool) => {
                    pool.close().await;
                }
            }
        }

        Ok(())
    }

    /// 检查数据库连接状态
    pub async fn check_health(&self) -> DatabaseResult<bool> {
        let connection = self.get_connection().await?;

        match connection {
            DatabaseConnectionRef::Postgres(pool) => {
                sqlx::query("SELECT 1")
                    .execute(&pool)
                    .await
                    .map(|_| true)
                    .map_err(|e| DatabaseError::Connection(e.to_string()))
            }
            DatabaseConnectionRef::Sqlite(pool) => {
                sqlx::query("SELECT 1")
                    .execute(&pool)
                    .await
                    .map(|_| true)
                    .map_err(|e| DatabaseError::Connection(e.to_string()))
            }
        }
    }

    /// 执行数据库迁移
    pub async fn migrate(&self) -> DatabaseResult<()> {
        let connection = self.get_connection().await?;

        match connection {
            DatabaseConnectionRef::Postgres(pool) => {
                sqlx::migrate!("./migrations")
                    .run(&pool)
                    .await
                    .map_err(|e| DatabaseError::Migration(e.to_string()))?;
            }
            DatabaseConnectionRef::Sqlite(pool) => {
                sqlx::migrate!("./migrations")
                    .run(&pool)
                    .await
                    .map_err(|e| DatabaseError::Migration(e.to_string()))?;
            }
        }

        Ok(())
    }
}

/// 数据库连接引用枚举
pub enum DatabaseConnectionRef {
    /// PostgreSQL 连接池引用
    Postgres(Pool<Postgres>),
    /// SQLite 连接池引用
    Sqlite(Pool<Sqlite>),
}

impl DatabaseConnectionRef {
    /// 获取PostgreSQL连接池（如果是PostgreSQL类型）
    pub fn as_postgres(&self) -> Option<&Pool<Postgres>> {
        match self {
            DatabaseConnectionRef::Postgres(pool) => Some(pool),
            _ => None,
        }
    }

    /// 获取SQLite连接池（如果是SQLite类型）
    pub fn as_sqlite(&self) -> Option<&Pool<Sqlite>> {
        match self {
            DatabaseConnectionRef::Sqlite(pool) => Some(pool),
            _ => None,
        }
    }
}

/// 全局数据库管理器实例
pub struct GlobalDatabase {
    manager: Arc<DatabaseManager>,
}

impl GlobalDatabase {
    /// 创建全局数据库实例
    pub fn new(config: DatabaseConfig) -> Self {
        Self {
            manager: Arc::new(DatabaseManager::new(config)),
        }
    }

    /// 获取数据库管理器引用
    pub fn manager(&self) -> &DatabaseManager {
        &self.manager
    }

    /// 初始化全局数据库连接
    pub async fn init(&self) -> DatabaseResult<()> {
        self.manager.init().await
    }

    /// 获取数据库连接
    pub async fn get_connection(&self) -> DatabaseResult<DatabaseConnectionRef> {
        self.manager.get_connection().await
    }

    /// 检查数据库健康状态
    pub async fn check_health(&self) -> DatabaseResult<bool> {
        self.manager.check_health().await
    }

    /// 执行数据库迁移
    pub async fn migrate(&self) -> DatabaseResult<()> {
        self.manager.migrate().await
    }

    /// 关闭数据库连接
    pub async fn close(&self) -> DatabaseResult<()> {
        self.manager.close().await
    }

    /// 从默认配置初始化数据库（应用启动时调用）
    pub async fn init_from_default_config() -> DatabaseResult<Self> {
        let config = DatabaseConfig::default();
        let db = Self::new(config);
        db.init().await?;
        Ok(db)
    }

    /// 从配置文件初始化数据库（应用启动时调用）
    pub async fn init_from_config_file<P: AsRef<std::path::Path>>(config_path: P) -> DatabaseResult<Self> {
        let config = DatabaseConfig::from_toml_file(config_path)?;
        let db = Self::new(config);
        db.init().await?;
        Ok(db)
    }
}

