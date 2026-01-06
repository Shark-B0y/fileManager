mod database;
mod commands;
mod system;

use tauri::Manager;

use crate::system::init::init_database;
use crate::system::runtime::RuntimeManager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // 创建 Tokio 运行时管理器（与 Tauri 应用生命周期一致）
            // 优先从配置文件加载配置，失败则使用默认配置
            let runtime_manager = RuntimeManager::from_config_file("config/runtime.toml")
                .unwrap_or_else(|e| {
                    eprintln!("从配置文件加载运行时配置失败: {}, 使用默认配置", e);
                    RuntimeManager::new().expect("创建Tokio运行时失败")
                });

            // 初始化数据库连接（应用启动时自动初始化）
            // 使用运行时管理器执行异步初始化任务
            // 数据库失败直接终止程序
            let db = runtime_manager.block_on(async {
                init_database("config/database.toml").await
            }).unwrap();
            app.manage(db);

            // 将运行时管理器存储到应用状态，供后续使用
            // 注意：必须在数据库初始化之后存储，因为 block_on 需要运行时保持存活
            app.manage(runtime_manager);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::greet
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}