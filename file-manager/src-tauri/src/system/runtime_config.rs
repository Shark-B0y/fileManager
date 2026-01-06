//! Tokio 运行时配置模块
//!
//! 定义 Tokio 运行时的配置参数和配置加载功能

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Tokio 运行时类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuntimeType {
    /// 多线程运行时（推荐，适合大多数场景）
    #[serde(rename = "multi_thread")]
    MultiThread,
    /// 单线程运行时（轻量级，适合简单场景）
    #[serde(rename = "current_thread")]
    CurrentThread,
}

impl Default for RuntimeType {
    fn default() -> Self {
        RuntimeType::MultiThread
    }
}

/// Tokio 运行时配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    /// 运行时类型
    #[serde(default)]
    pub runtime_type: RuntimeType,
    /// 工作线程数量（仅多线程运行时有效）
    /// 如果为 None，则使用默认值（CPU 核心数）
    pub worker_threads: Option<usize>,
    /// 线程名称前缀
    pub thread_name_prefix: Option<String>,
    /// 线程栈大小（字节）
    pub thread_stack_size: Option<usize>,
    /// 是否启用 I/O 驱动
    #[serde(default = "default_true")]
    pub enable_io: bool,
    /// 是否启用时间驱动
    #[serde(default = "default_true")]
    pub enable_time: bool,
    /// 是否启用信号处理
    #[serde(default = "default_false")]
    pub enable_signal: bool,
    /// 全局并发限制
    pub global_concurrency_limit: Option<usize>,
}

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            runtime_type: RuntimeType::MultiThread,
            worker_threads: None, // 使用默认值（CPU 核心数）
            thread_name_prefix: Some("tokio-worker".to_string()),
            thread_stack_size: None, // 使用默认值
            enable_io: true,
            enable_time: true,
            enable_signal: false,
            global_concurrency_limit: None,
        }
    }
}

impl RuntimeConfig {
    /// 从 TOML 配置文件加载配置
    ///
    /// # 参数
    /// - `path`: 配置文件路径
    ///
    /// # 返回
    /// - `Ok(Self)`: 成功加载配置
    /// - `Err(String)`: 加载配置失败
    pub fn from_toml_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        if !path.as_ref().exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(path)
            .map_err(|e| format!("读取配置文件失败: {}", e))?;

        let config: RuntimeConfig = toml::from_str(&content)
            .map_err(|e| format!("解析TOML配置文件失败: {}", e))?;

        Ok(config)
    }

    /// 从环境变量加载配置
    pub fn from_env() -> Self {
        let worker_threads = std::env::var("TOKIO_WORKER_THREADS")
            .ok()
            .and_then(|v| v.parse().ok());

        let runtime_type = match std::env::var("TOKIO_RUNTIME_TYPE")
            .unwrap_or_else(|_| "multi_thread".to_string())
            .as_str()
        {
            "current_thread" => RuntimeType::CurrentThread,
            _ => RuntimeType::MultiThread,
        };

        Self {
            runtime_type,
            worker_threads,
            ..Default::default()
        }
    }
}

