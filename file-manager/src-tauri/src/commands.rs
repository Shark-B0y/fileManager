//! Tauri命令模块
//!
//! 提供前后端通信的命令接口

use serde::{Deserialize, Serialize};
use tauri::{Manager, State};

use crate::database::{DatabaseConfig, GlobalDatabase};
use crate::database::config::DatabaseType;

#[tauri::command]
pub async fn greet(name: &str) -> Result<(), String>{
    println!("Hello, {}! You've been greeted from Rust!", name);
    Ok(())
}


/// 数据库初始化命令参数
#[derive(Debug, Serialize, Deserialize)]
pub struct InitDatabaseParams {
    /// 数据库类型：postgres 或 sqlite
    pub db_type: String,
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
}

/// 数据库初始化命令响应
#[derive(Debug, Serialize)]
pub struct InitDatabaseResponse {
    /// 是否成功
    pub success: bool,
    /// 消息
    pub message: String,
    /// 错误信息（如果有）
    pub error: Option<String>,
}

/// 初始化数据库连接
///
/// # 参数
/// - `app_handle`: Tauri应用句柄
/// - `params`: 数据库初始化参数
///
/// # 返回值
/// - `Result<InitDatabaseResponse, String>`: 初始化结果
#[tauri::command]
pub async fn init_database(
    app_handle: tauri::AppHandle,
    params: InitDatabaseParams,
) -> Result<InitDatabaseResponse, String> {
    // 解析数据库类型
    let db_type = match params.db_type.as_str() {
        "postgres" => DatabaseType::Postgres,
        "sqlite" => DatabaseType::Sqlite,
        other => {
            return Ok(InitDatabaseResponse {
                success: false,
                message: format!("未知的数据库类型: {}", other),
                error: Some(format!("未知的数据库类型: {}", other)),
            });
        }
    };

    // 创建数据库配置
    let config = DatabaseConfig::new(
        db_type,
        params.database,
        params.host,
        params.port,
        params.username,
        params.password,
        params.sqlite_path,
    );

    // 验证配置
    if let Err(e) = config.validate() {
        return Ok(InitDatabaseResponse {
            success: false,
            message: format!("数据库配置验证失败: {}", e),
            error: Some(e),
        });
    }

    // 创建全局数据库实例
    let db = GlobalDatabase::new(config);

    // 初始化数据库连接
    match db.init().await {
        Ok(_) => {
            // 将数据库实例存储到应用状态
            app_handle.manage(db);

            Ok(InitDatabaseResponse {
                success: true,
                message: "数据库连接初始化成功".to_string(),
                error: None,
            })
        }
        Err(e) => Ok(InitDatabaseResponse {
            success: false,
            message: format!("数据库连接初始化失败: {}", e),
            error: Some(e.to_string()),
        }),
    }
}

/// 检查数据库健康状态
///
/// # 参数
/// - `db_state`: 数据库状态
///
/// # 返回值
/// - `Result<InitDatabaseResponse, String>`: 健康检查结果
#[tauri::command]
pub async fn check_database_health(
    db_state: State<'_, GlobalDatabase>,
) -> Result<InitDatabaseResponse, String> {
    match db_state.check_health().await {
        Ok(healthy) => {
            if healthy {
                Ok(InitDatabaseResponse {
                    success: true,
                    message: "数据库健康检查通过".to_string(),
                    error: None,
                })
            } else {
                Ok(InitDatabaseResponse {
                    success: false,
                    message: "数据库健康检查失败".to_string(),
                    error: Some("数据库连接异常".to_string()),
                })
            }
        }
        Err(e) => Ok(InitDatabaseResponse {
            success: false,
            message: format!("数据库健康检查错误: {}", e),
            error: Some(e.to_string()),
        }),
    }
}

/// 执行数据库迁移
///
/// # 参数
/// - `db_state`: 数据库状态
///
/// # 返回值
/// - `Result<InitDatabaseResponse, String>`: 迁移结果
#[tauri::command]
pub async fn migrate_database(
    db_state: State<'_, GlobalDatabase>,
) -> Result<InitDatabaseResponse, String> {
    match db_state.migrate().await {
        Ok(_) => Ok(InitDatabaseResponse {
            success: true,
            message: "数据库迁移执行成功".to_string(),
            error: None,
        }),
        Err(e) => Ok(InitDatabaseResponse {
            success: false,
            message: format!("数据库迁移执行失败: {}", e),
            error: Some(e.to_string()),
        }),
    }
}

/// 关闭数据库连接
///
/// # 参数
/// - `db_state`: 数据库状态
///
/// # 返回值
/// - `Result<InitDatabaseResponse, String>`: 关闭结果
#[tauri::command]
pub async fn close_database(
    db_state: State<'_, GlobalDatabase>,
) -> Result<InitDatabaseResponse, String> {
    match db_state.close().await {
        Ok(_) => Ok(InitDatabaseResponse {
            success: true,
            message: "数据库连接关闭成功".to_string(),
            error: None,
        }),
        Err(e) => Ok(InitDatabaseResponse {
            success: false,
            message: format!("数据库连接关闭失败: {}", e),
            error: Some(e.to_string()),
        }),
    }
}

/// 从配置文件加载数据库配置
///
/// # 参数
/// - `config_path`: 配置文件路径
///
/// # 返回值
/// - `Result<InitDatabaseResponse, String>`: 加载结果
#[tauri::command]
pub async fn load_database_config(
    app_handle: tauri::AppHandle,
    config_path: String,
) -> Result<InitDatabaseResponse, String> {
    match DatabaseConfig::from_toml_file(&config_path) {
        Ok(config) => {
            // 验证配置
            if let Err(e) = config.validate() {
                return Ok(InitDatabaseResponse {
                    success: false,
                    message: format!("数据库配置验证失败: {}", e),
                    error: Some(e),
                });
            }

            // 创建全局数据库实例
            let db = GlobalDatabase::new(config);

            // 初始化数据库连接
            match db.init().await {
                Ok(_) => {
                    // 将数据库实例存储到应用状态
                    app_handle.manage(db);

                    Ok(InitDatabaseResponse {
                        success: true,
                        message: format!("从配置文件 {} 加载数据库配置成功", config_path),
                        error: None,
                    })
                }
                Err(e) => Ok(InitDatabaseResponse {
                    success: false,
                    message: format!("数据库连接初始化失败: {}", e),
                    error: Some(e.to_string()),
                }),
            }
        }
        Err(e) => Ok(InitDatabaseResponse {
            success: false,
            message: format!("加载配置文件失败: {}", e),
            error: Some(e),
        }),
    }
}

/// 获取数据库配置信息
///
/// # 参数
/// - `db_state`: 数据库状态
///
/// # 返回值
/// - `Result<DatabaseConfig, String>`: 数据库配置
#[tauri::command]
pub async fn get_database_config(
    db_state: State<'_, GlobalDatabase>,
) -> Result<DatabaseConfig, String> {
    Ok(db_state.manager().config().clone())
}