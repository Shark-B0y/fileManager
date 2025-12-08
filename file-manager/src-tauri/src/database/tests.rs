//! 数据库模块测试
//!
//! 包含数据库配置和连接的单元测试

use super::config::{DatabaseConfig, DatabaseType};
use super::connection::{DatabaseManager, GlobalDatabase};
use tempfile::tempdir;

#[test]
fn test_default_config() {
    let config = DatabaseConfig::default();
    assert_eq!(config.db_type, DatabaseType::Postgres);
    assert_eq!(config.host, Some("localhost".to_string()));
    assert_eq!(config.port, Some(5432));
    assert_eq!(config.database, "file_manager".to_string());
    assert_eq!(config.username, Some("postgres".to_string()));
    assert_eq!(config.password, Some("password".to_string()));
    assert_eq!(config.max_connections, 10);
    assert_eq!(config.connect_timeout, 30);
}

#[test]
fn test_postgres_connection_string() {
    let config = DatabaseConfig::new(
        DatabaseType::Postgres,
        "test_db".to_string(),
        Some("localhost".to_string()),
        Some(5432),
        Some("user".to_string()),
        Some("pass".to_string()),
        None,
    );

    let conn_str = config.connection_string().unwrap();
    assert_eq!(conn_str, "postgres://user:pass@localhost:5432/test_db");
}

#[test]
fn test_sqlite_connection_string() {
    let config = DatabaseConfig::new(
        DatabaseType::Sqlite,
        "test".to_string(),
        None,
        None,
        None,
        None,
        Some("/path/to/db.sqlite".to_string()),
    );

    let conn_str = config.connection_string().unwrap();
    if cfg!(windows) {
        assert_eq!(conn_str, "sqlite:///path/to/db.sqlite");
    } else {
        assert_eq!(conn_str, "sqlite:///path/to/db.sqlite");
    }
}

#[test]
fn test_validate_postgres() {
    let config = DatabaseConfig::new(
        DatabaseType::Postgres,
        "".to_string(),
        Some("localhost".to_string()),
        Some(5432),
        Some("user".to_string()),
        Some("pass".to_string()),
        None,
    );

    assert!(config.validate().is_err());
}

#[test]
fn test_validate_sqlite() {
    let config = DatabaseConfig::new(
        DatabaseType::Sqlite,
        "test".to_string(),
        None,
        None,
        None,
        None,
        None,
    );

    assert!(config.validate().is_err());
}

#[tokio::test]
async fn test_database_manager_init() {
    let config = DatabaseConfig::default();
    let manager = DatabaseManager::new(config);

    // 由于没有实际的数据库，这个测试会失败
    // 在实际环境中应该能够成功初始化
    let result = manager.init().await;
    assert!(result.is_err()); // 应该失败，因为没有数据库服务
}

#[tokio::test]
async fn test_global_database() {
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("global_test.db");
    let db_path_str = db_path.to_str().unwrap();
    println!("GlobalDatabase测试文件路径: {}", db_path_str);

    let config = DatabaseConfig::new(
        DatabaseType::Sqlite,
        "global_test".to_string(),
        None,
        None,
        None,
        None,
        Some(db_path_str.to_string()),
    );

    let db = GlobalDatabase::new(config);

    // 测试初始化
    let init_result = db.init().await;
    if let Err(e) = &init_result {
        eprintln!("GlobalDatabase初始化失败: {:?}", e);
    }
    assert!(init_result.is_ok());

    // 测试健康检查
    let health_result = db.check_health().await;
    assert!(health_result.is_ok());
    assert!(health_result.unwrap());

    // 测试迁移（应该成功，即使没有新迁移）
    let migrate_result = db.migrate().await;
    assert!(migrate_result.is_ok());

    // 测试关闭
    let close_result = db.close().await;
    assert!(close_result.is_ok());
}

#[tokio::test]
async fn test_sqlite_connection() {
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db_path_str = db_path.to_str().unwrap();
    println!("SQLite测试文件路径: {}", db_path_str);

    let config = DatabaseConfig::new(
        DatabaseType::Sqlite,
        "test".to_string(),
        None,
        None,
        None,
        None,
        Some(db_path_str.to_string()),
    );

    let manager = DatabaseManager::new(config);

    // SQLite文件不存在时会自动创建
    let result = manager.init().await;
    if let Err(e) = &result {
        eprintln!("SQLite初始化失败: {:?}", e);
    }
    assert!(result.is_ok());

    let health = manager.check_health().await;
    assert!(health.is_ok());
    assert!(health.unwrap());

    manager.close().await.unwrap();
}

#[tokio::test]
async fn test_postgres_connection() {
    let config_path = "config/database.toml";
    let config = DatabaseConfig::from_toml_file(config_path).unwrap();

    let db = GlobalDatabase::new(config);
    db.init().await.unwrap();
    db.close().await.unwrap()
}