//! 数据库配置模块
//!
//! 定义数据库连接配置和配置加载功能

use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;
use toml;

/// 数据库类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DatabaseType {
    /// PostgreSQL 数据库
    Postgres,
    /// SQLite 数据库
    Sqlite,
}

/// 数据库配置结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// 数据库类型
    pub db_type: DatabaseType,
    /// 数据库主机地址（PostgreSQL使用）
    pub host: Option<String>,
    /// 数据库端口（PostgreSQL使用）
    pub port: Option<u16>,
    /// 数据库名称
    pub database: String,
    /// 用户名（PostgreSQL使用）
    pub username: Option<String>,
    /// 密码（PostgreSQL使用）
    pub password: Option<String>,
    /// SQLite文件路径（SQLite使用）
    pub sqlite_path: Option<String>,
    /// 连接池最大连接数
    pub max_connections: u32,
    /// 连接超时时间（秒）
    pub connect_timeout: u64,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            db_type: DatabaseType::Postgres,
            host: Some("localhost".to_string()),
            port: Some(5432),
            database: "file_manager".to_string(),
            username: Some("postgres".to_string()),
            password: Some("password".to_string()),
            sqlite_path: None,
            max_connections: 10,
            connect_timeout: 30,
        }
    }
}

impl DatabaseConfig {
    /// 创建新的数据库配置
    pub fn new(
        db_type: DatabaseType,
        database: String,
        host: Option<String>,
        port: Option<u16>,
        username: Option<String>,
        password: Option<String>,
        sqlite_path: Option<String>,
    ) -> Self {
        Self {
            db_type,
            host,
            port,
            database,
            username,
            password,
            sqlite_path,
            max_connections: 10,
            connect_timeout: 30,
        }
    }

    /// 从环境变量加载配置
    pub fn from_env() -> Result<Self, String> {
        let db_type = match env::var("DATABASE_TYPE").unwrap_or_else(|_| "postgres".to_string()).as_str() {
            "postgres" => DatabaseType::Postgres,
            "sqlite" => DatabaseType::Sqlite,
            other => return Err(format!("未知的数据库类型: {}", other)),
        };

        let host = env::var("DATABASE_HOST").ok();
        let port = env::var("DATABASE_PORT").ok().and_then(|p| p.parse().ok());
        let database = env::var("DATABASE_NAME").unwrap_or_else(|_| "file_manager".to_string());
        let username = env::var("DATABASE_USERNAME").ok();
        let password = env::var("DATABASE_PASSWORD").ok();
        let sqlite_path = env::var("DATABASE_SQLITE_PATH").ok();

        Ok(Self::new(
            db_type,
            database,
            host,
            port,
            username,
            password,
            sqlite_path,
        ))
    }

    /// 从TOML配置文件加载配置
    pub fn from_toml_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("读取配置文件失败: {}", e))?;

        let config_value: toml::Value = toml::from_str(&content)
            .map_err(|e| format!("解析TOML配置文件失败: {}", e))?;

        let db_type_str = config_value.get("db_type")
            .and_then(|v| v.as_str())
            .ok_or("配置文件中缺少 db_type 字段")?;

        let db_type = match db_type_str {
            "postgres" => DatabaseType::Postgres,
            "sqlite" => DatabaseType::Sqlite,
            other => return Err(format!("未知的数据库类型: {}", other)),
        };

        match db_type {
            DatabaseType::Postgres => {
                let postgres_section = config_value.get("postgres")
                    .ok_or("配置文件中缺少 postgres 配置节")?;

                let host = postgres_section.get("host")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let port = postgres_section.get("port")
                    .and_then(|v| v.as_integer())
                    .map(|p| p as u16);
                let database = postgres_section.get("database")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
                    .ok_or("postgres配置中缺少 database 字段")?;
                let username = postgres_section.get("username")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let password = postgres_section.get("password")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let max_connections = postgres_section.get("max_connections")
                    .and_then(|v| v.as_integer())
                    .map(|m| m as u32)
                    .unwrap_or(10);
                let connect_timeout = postgres_section.get("connect_timeout")
                    .and_then(|v| v.as_integer())
                    .map(|t| t as u64)
                    .unwrap_or(30);

                Ok(Self {
                    db_type,
                    host,
                    port,
                    database,
                    username,
                    password,
                    sqlite_path: None,
                    max_connections,
                    connect_timeout,
                })
            }
            DatabaseType::Sqlite => {
                let sqlite_section = config_value.get("sqlite")
                    .ok_or("配置文件中缺少 sqlite 配置节")?;

                let database = sqlite_section.get("database")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
                    .ok_or("sqlite配置中缺少 database 字段")?;
                let sqlite_path = sqlite_section.get("sqlite_path")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let max_connections = sqlite_section.get("max_connections")
                    .and_then(|v| v.as_integer())
                    .map(|m| m as u32)
                    .unwrap_or(10);
                let connect_timeout = sqlite_section.get("connect_timeout")
                    .and_then(|v| v.as_integer())
                    .map(|t| t as u64)
                    .unwrap_or(30);

                Ok(Self {
                    db_type,
                    host: None,
                    port: None,
                    database,
                    username: None,
                    password: None,
                    sqlite_path,
                    max_connections,
                    connect_timeout,
                })
            }
        }
    }

    /// 从配置文件加载配置（兼容旧版本，使用JSON格式）
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("读取配置文件失败: {}", e))?;

        serde_json::from_str(&content)
            .map_err(|e| format!("解析配置文件失败: {}", e))
    }

    /// 生成数据库连接字符串
    pub fn connection_string(&self) -> Result<String, String> {
        match self.db_type {
            DatabaseType::Postgres => {
                let host = self.host.as_ref().ok_or("PostgreSQL配置需要host字段")?;
                let port = self.port.ok_or("PostgreSQL配置需要port字段")?;
                let username = self.username.as_ref().ok_or("PostgreSQL配置需要username字段")?;
                let password = self.password.as_ref().ok_or("PostgreSQL配置需要password字段")?;

                Ok(format!(
                    "postgres://{}:{}@{}:{}/{}",
                    username, password, host, port, self.database
                ))
            }
            DatabaseType::Sqlite => {
                let path = self.sqlite_path.as_ref()
                    .ok_or("SQLite配置需要sqlite_path字段")?;

                // Windows路径需要特殊处理
                let normalized_path = if cfg!(windows) {
                    // 将反斜杠替换为正斜杠，并确保路径格式正确
                    // 对于绝对路径，需要添加额外的斜杠
                    let replaced = path.replace('\\', "/");
                    if replaced.starts_with("C:/") || replaced.starts_with("D:/") || replaced.starts_with("E:/") {
                        format!("sqlite:///{}", replaced)
                    } else {
                        format!("sqlite://{}", replaced)
                    }
                } else {
                    format!("sqlite://{}", path)
                };

                Ok(normalized_path)
            }
        }
    }

    /// 检查配置是否有效
    pub fn validate(&self) -> Result<(), String> {
        match self.db_type {
            DatabaseType::Postgres => {
                if self.host.is_none() {
                    return Err("PostgreSQL配置需要host字段".to_string());
                }
                if self.port.is_none() {
                    return Err("PostgreSQL配置需要port字段".to_string());
                }
                if self.username.is_none() {
                    return Err("PostgreSQL配置需要username字段".to_string());
                }
                if self.password.is_none() {
                    return Err("PostgreSQL配置需要password字段".to_string());
                }
            }
            DatabaseType::Sqlite => {
                if self.sqlite_path.is_none() {
                    return Err("SQLite配置需要sqlite_path字段".to_string());
                }
            }
        }

        if self.database.is_empty() {
            return Err("数据库名称不能为空".to_string());
        }

        if self.max_connections == 0 {
            return Err("连接池最大连接数必须大于0".to_string());
        }

        Ok(())
    }
}

