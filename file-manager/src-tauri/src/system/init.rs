//! 系统初始化模块
//!
//! 负责应用启动时的系统级初始化操作，包括数据库初始化

use crate::database::{DatabaseResult, GlobalDatabase};
use std::path::Path;

/// 初始化数据库
///
/// 优先尝试从配置文件初始化数据库，如果失败则使用默认配置
///
/// # 参数
/// - `config_path`: 配置文件路径，默认为 "config/database.toml"
///
/// # 返回
/// - `Ok(GlobalDatabase)`: 初始化成功的数据库实例
/// - `Err(DatabaseError)`: 初始化失败的错误信息
pub async fn init_database<P: AsRef<Path>>(config_path: P) -> DatabaseResult<GlobalDatabase> {
    // 优先尝试从配置文件初始化
    if Path::new(config_path.as_ref()).exists() {
        match GlobalDatabase::init_from_config_file(config_path).await {
            Ok(db) => {
                println!("从配置文件初始化数据库成功");
                // 执行数据库迁移
                db.migrate().await?;
                println!("数据库迁移完成");
                Ok(db)
            }
            Err(e) => {
                eprintln!("从配置文件初始化数据库失败: {}, 使用默认配置", e);
                init_database_with_default().await
            }
        }
    } else {
        println!("配置文件不存在，使用默认配置");
        init_database_with_default().await
    }
}

/// 使用默认配置初始化数据库
///
/// # 返回
/// - `Ok(GlobalDatabase)`: 初始化成功的数据库实例
/// - `Err(DatabaseError)`: 初始化失败的错误信息
pub async fn init_database_with_default() -> DatabaseResult<GlobalDatabase> {
    let db = GlobalDatabase::init_from_default_config().await?;
    println!("使用默认配置初始化数据库成功");
    // 执行数据库迁移
    db.migrate().await?;
    println!("数据库迁移完成");
    Ok(db)
}

