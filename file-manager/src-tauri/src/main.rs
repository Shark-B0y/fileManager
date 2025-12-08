mod database;

use crate::database::{DatabaseConfig, GlobalDatabase};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("文件管理系统 - 数据库模块测试");

    // 加载数据库配置
    let config_path = "config/database.toml";
    let config = match DatabaseConfig::from_toml_file(config_path) {
        Ok(config) => {
            println!("从配置文件 {} 加载数据库配置成功", config_path);
            config
        }
        Err(e) => {
            eprintln!("从配置文件加载失败: {}, 尝试从环境变量加载", e);
            match DatabaseConfig::from_env() {
                Ok(config) => {
                    println!("从环境变量加载数据库配置成功");
                    config
                }
                Err(_) => {
                    println!("使用默认数据库配置");
                    DatabaseConfig::default()
                }
            }
        }
    };

    // 验证配置
    if let Err(e) = config.validate() {
        eprintln!("数据库配置验证失败: {}", e);
        return Err(e.into());
    }

    println!("数据库配置: {:?}", config);

    // 创建全局数据库实例
    let db = GlobalDatabase::new(config);

    // 初始化数据库连接
    match db.init().await {
        Ok(_) => println!("数据库连接初始化成功"),
        Err(e) => {
            eprintln!("数据库连接初始化失败: {}", e);
            return Err(e.into());
        }
    }

    // 检查数据库健康状态
    match db.check_health().await {
        Ok(healthy) => {
            if healthy {
                println!("数据库健康检查通过");
            } else {
                eprintln!("数据库健康检查失败");
                return Err("数据库健康检查失败".into());
            }
        }
        Err(e) => {
            eprintln!("数据库健康检查错误: {}", e);
            return Err(e.into());
        }
    }

    // 执行数据库迁移
    match db.migrate().await {
        Ok(_) => println!("数据库迁移执行成功"),
        Err(e) => {
            eprintln!("数据库迁移执行失败: {}", e);
            // 迁移失败不一定需要退出，可以继续运行
        }
    }

    println!("数据库模块测试完成");

    // 关闭数据库连接
    match db.close().await {
        Ok(_) => println!("数据库连接关闭成功"),
        Err(e) => eprintln!("数据库连接关闭失败: {}", e),
    }

    Ok(())
}
