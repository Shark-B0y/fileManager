//! 全局配置模块
//!
//! 管理应用的全局配置，包括用户主目录等设置

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::sync::{Arc, RwLock};

/// 全局配置结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
    /// 用户主目录路径（可选）
    /// 如果设置，get_home_directory 将优先使用此路径
    pub home_path: Option<String>,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            home_path: None,
        }
    }
}

impl GlobalConfig {
    /// 创建新的全局配置
    pub fn new(home_path: Option<String>) -> Self {
        Self { home_path }
    }

    /// 从 TOML 文件加载配置
    ///
    /// # 参数
    /// - `path`: 配置文件路径
    ///
    /// # 返回
    /// - `Ok(GlobalConfig)`: 配置对象
    /// - `Err(String)`: 错误信息
    pub fn from_toml_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let path = path.as_ref();

        // 如果文件不存在，返回默认配置
        if !path.exists() {
            eprintln!("全局配置文件不存在: {}, 使用默认配置", path.display());
            return Ok(Self::default());
        }

        let content = fs::read_to_string(path)
            .map_err(|e| format!("读取全局配置文件失败 {}: {}", path.display(), e))?;

        let mut config: GlobalConfig = toml::from_str(&content)
            .map_err(|e| format!("解析全局配置文件失败 {}: {}", path.display(), e))?;

        // 如果 home_path 是空字符串，则转换为 None
        if let Some(ref home_path) = config.home_path {
            if home_path.is_empty() {
                config.home_path = None;
            }
        }

        Ok(config)
    }

    /// 从环境变量加载配置
    pub fn from_env() -> Self {
        use std::env;

        let home_path = env::var("GLOBAL_HOME_PATH").ok();

        Self::new(home_path)
    }
}

/// 全局配置管理器
///
/// 提供线程安全的全局配置访问
#[derive(Debug, Clone)]
pub struct GlobalConfigManager {
    config: Arc<RwLock<GlobalConfig>>,
}

impl GlobalConfigManager {
    /// 创建新的配置管理器
    pub fn new(config: GlobalConfig) -> Self {
        Self {
            config: Arc::new(RwLock::new(config)),
        }
    }

    /// 从 TOML 文件创建配置管理器
    ///
    /// # 参数
    /// - `path`: 配置文件路径
    ///
    /// # 返回
    /// - `Ok(GlobalConfigManager)`: 配置管理器
    /// - `Err(String)`: 错误信息
    pub fn from_toml_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let config = GlobalConfig::from_toml_file(path)?;
        Ok(Self::new(config))
    }

    /// 从环境变量创建配置管理器
    pub fn from_env() -> Self {
        let config = GlobalConfig::from_env();
        Self::new(config)
    }

    /// 使用默认配置创建配置管理器
    pub fn from_default() -> Self {
        Self::new(GlobalConfig::default())
    }

    /// 获取用户主目录路径
    ///
    /// # 返回
    /// - `Some(String)`: 如果配置中设置了主目录路径
    /// - `None`: 如果配置中未设置主目录路径
    pub fn get_home_path(&self) -> Option<String> {
        let config = self.config.read().unwrap();
        config.home_path.clone()
    }

    /// 设置用户主目录路径
    ///
    /// # 参数
    /// - `path`: 主目录路径
    pub fn set_home_path(&self, path: Option<String>) {
        let mut config = self.config.write().unwrap();
        config.home_path = path;
    }

    /// 获取完整的配置对象（克隆）
    pub fn get_config(&self) -> GlobalConfig {
        let config = self.config.read().unwrap();
        config.clone()
    }

    /// 更新配置
    ///
    /// # 参数
    /// - `new_config`: 新的配置对象
    pub fn update_config(&self, new_config: GlobalConfig) {
        let mut config = self.config.write().unwrap();
        *config = new_config;
    }
}

