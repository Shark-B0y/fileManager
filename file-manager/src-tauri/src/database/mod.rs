//! 数据库模块
//!
//! 提供数据库连接、配置和错误处理功能

pub mod config;
pub mod connection;
pub mod error;

#[cfg(test)]
mod tests;

pub use config::DatabaseConfig;
pub use connection::{DatabaseConnection, DatabaseManager, DatabaseConnectionRef, GlobalDatabase};
pub use error::{DatabaseError, DatabaseResult};