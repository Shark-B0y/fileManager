mod database;
mod commands;

use crate::database::GlobalDatabase;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // .setup(|app|{
        //     // 初始化数据库连接（应用启动时自动初始化）
        //     let app_handle = app.handle();

        //     // 尝试从配置文件初始化数据库，失败则使用默认配置
        //     let db_result = tokio::runtime::Runtime::new()
        //         .expect("创建Tokio运行时失败")
        //         .block_on(async {
        //             // 优先尝试从配置文件初始化
        //             let config_path = "config/database.toml";
        //             if std::path::Path::new(config_path).exists() {
        //                 match GlobalDatabase::init_from_config_file(config_path).await {
        //                     Ok(db) => Ok(db),
        //                     Err(e) => {
        //                         eprintln!("从配置文件初始化数据库失败: {}, 使用默认配置", e);
        //                         GlobalDatabase::init_from_default_config().await
        //                     }
        //                 }
        //             } else {
        //                 // 使用默认配置
        //                 GlobalDatabase::init_from_default_config().await
        //             }
        //         });

        //     match db_result {
        //         Ok(db) => {
        //             // 将数据库实例存储到应用状态
        //             app_handle.manage(db);
        //             println!("数据库初始化成功");
        //         }
        //         Err(e) => {
        //             eprintln!("数据库初始化失败: {}", e);
        //             // 即使数据库初始化失败，应用仍然可以启动
        //             // 用户可以在前端手动初始化数据库
        //         }
        //     }

        //     Ok(())
        // })
        .invoke_handler(tauri::generate_handler![
            commands::greet
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}