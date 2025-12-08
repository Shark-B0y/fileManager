//! 数据库错误处理模块
//!
//! 定义数据库操作中的错误类型和错误处理

use std::fmt;

/// 数据库错误类型
#[derive(Debug)]
pub enum DatabaseError {
    /// 连接错误
    Connection(String),
    /// 配置错误
    Config(String),
    /// 查询错误
    Query(String),
    /// 迁移错误
    Migration(String),
    /// 事务错误
    Transaction(String),
    /// 其他错误
    Other(String),
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatabaseError::Connection(msg) => write!(f, "数据库连接错误: {}", msg),
            DatabaseError::Config(msg) => write!(f, "数据库配置错误: {}", msg),
            DatabaseError::Query(msg) => write!(f, "数据库查询错误: {}", msg),
            DatabaseError::Migration(msg) => write!(f, "数据库迁移错误: {}", msg),
            DatabaseError::Transaction(msg) => write!(f, "数据库事务错误: {}", msg),
            DatabaseError::Other(msg) => write!(f, "数据库错误: {}", msg),
        }
    }
}

impl std::error::Error for DatabaseError {}

impl From<sqlx::Error> for DatabaseError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::PoolClosed | sqlx::Error::PoolTimedOut => {
                DatabaseError::Connection(err.to_string())
            }
            sqlx::Error::Configuration(_) => DatabaseError::Config(err.to_string()),
            sqlx::Error::RowNotFound | sqlx::Error::ColumnNotFound(_) => {
                DatabaseError::Query(err.to_string())
            }
            sqlx::Error::TypeNotFound { type_name: _ } => DatabaseError::Query(err.to_string()),
            sqlx::Error::Protocol(_) => DatabaseError::Connection(err.to_string()),
            sqlx::Error::Io(_) => DatabaseError::Connection(err.to_string()),
            sqlx::Error::Tls(_) => DatabaseError::Connection(err.to_string()),
            sqlx::Error::Database(_) => DatabaseError::Query(err.to_string()),
            sqlx::Error::Migrate(_) => DatabaseError::Migration(err.to_string()),
            sqlx::Error::WorkerCrashed => DatabaseError::Connection(err.to_string()),
            _ => DatabaseError::Other(err.to_string()),
        }
    }
}

impl From<std::io::Error> for DatabaseError {
    fn from(err: std::io::Error) -> Self {
        DatabaseError::Other(format!("IO错误: {}", err))
    }
}

impl From<serde_json::Error> for DatabaseError {
    fn from(err: serde_json::Error) -> Self {
        DatabaseError::Config(format!("JSON解析错误: {}", err))
    }
}

/// 数据库结果类型别名
pub type DatabaseResult<T> = Result<T, DatabaseError>;