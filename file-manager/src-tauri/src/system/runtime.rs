//! Tokio 运行时管理模块
//!
//! 提供与 Tauri 应用生命周期一致的 Tokio 运行时
//!
//! ## Arc 使用说明
//!
//! 当前实现中，`RuntimeManager` 通过 `app.manage()` 存储到 Tauri 应用状态中，
//! Tauri 会管理其生命周期。虽然理论上可以不用 `Arc`，但使用 `Arc` 有以下好处：
//!
//! 1. **线程安全**：`Arc` 提供线程安全的引用计数，确保在多线程环境下安全共享
//! 2. **灵活性**：为将来可能的并发访问（如多个命令处理器同时访问）提供支持
//! 3. **生命周期管理**：确保 `Runtime` 在整个应用生命周期内保持存活
//!
//! 注意：Tauri 本身**没有内置 Tokio 运行时**，需要手动创建和管理。

use std::sync::Arc;
use tokio::runtime::{Handle, Runtime};
use crate::system::runtime_config::RuntimeConfig;

pub struct RuntimeManager {
    /// Tokio 运行时实例（使用 Arc 确保线程安全和生命周期管理）
    runtime: Arc<Runtime>,
}

impl RuntimeManager {
    /// 使用默认配置创建新的运行时管理器
    ///
    /// # 返回
    /// - `Ok(Self)`: 成功创建运行时管理器
    /// - `Err(String)`: 创建运行时失败
    pub fn new() -> Result<Self, String> {
        Self::with_config(RuntimeConfig::default())
    }

    /// 使用指定配置创建新的运行时管理器
    ///
    /// # 参数
    /// - `config`: 运行时配置
    ///
    /// # 返回
    /// - `Ok(Self)`: 成功创建运行时管理器
    /// - `Err(String)`: 创建运行时失败
    pub fn with_config(config: RuntimeConfig) -> Result<Self, String> {
        let runtime = match config.runtime_type {
            crate::system::runtime_config::RuntimeType::MultiThread => {
                let mut builder = tokio::runtime::Builder::new_multi_thread();

                // 配置工作线程数量
                if let Some(worker_threads) = config.worker_threads {
                    builder.worker_threads(worker_threads);
                }

                // 配置线程名称前缀
                if let Some(prefix) = &config.thread_name_prefix {
                    builder.thread_name(prefix.clone());
                }

                // 配置线程栈大小
                if let Some(stack_size) = config.thread_stack_size {
                    builder.thread_stack_size(stack_size);
                }

                // 配置 I/O 驱动
                if config.enable_io {
                    builder.enable_io();
                }

                // 配置时间驱动
                if config.enable_time {
                    builder.enable_time();
                }

                // 配置信号处理
                #[cfg(unix)]
                if config.enable_signal {
                    builder.enable_signal();
                }

                // 配置全局并发限制
                if let Some(limit) = config.global_concurrency_limit {
                    builder.max_blocking_threads(limit);
                }

                builder.build()
                    .map_err(|e| format!("创建多线程Tokio运行时失败: {}", e))?
            }
            crate::system::runtime_config::RuntimeType::CurrentThread => {
                let mut builder = tokio::runtime::Builder::new_current_thread();

                // 配置线程名称前缀
                if let Some(prefix) = &config.thread_name_prefix {
                    builder.thread_name(prefix.clone());
                }

                // 配置线程栈大小
                if let Some(stack_size) = config.thread_stack_size {
                    builder.thread_stack_size(stack_size);
                }

                // 配置 I/O 驱动
                if config.enable_io {
                    builder.enable_io();
                }

                // 配置时间驱动
                if config.enable_time {
                    builder.enable_time();
                }

                // 配置信号处理
                #[cfg(unix)]
                if config.enable_signal {
                    builder.enable_signal();
                }

                builder.build()
                    .map_err(|e| format!("创建单线程Tokio运行时失败: {}", e))?
            }
        };

        Ok(Self {
            runtime: Arc::new(runtime),
        })
    }

    /// 从配置文件创建运行时管理器
    ///
    /// # 参数
    /// - `config_path`: 配置文件路径，默认为 "config/runtime.toml"
    ///
    /// # 返回
    /// - `Ok(Self)`: 成功创建运行时管理器
    /// - `Err(String)`: 创建运行时失败
    pub fn from_config_file<P: AsRef<std::path::Path>>(config_path: P) -> Result<Self, String> {
        let config = RuntimeConfig::from_toml_file(config_path)?;
        Self::with_config(config)
    }

    /// 获取运行时句柄
    ///
    /// 用于在异步上下文中执行异步任务
    pub fn handle(&self) -> Handle {
        self.runtime.handle().clone()
    }

    /// 在运行时中执行异步任务（阻塞当前线程直到完成）
    ///
    /// # 参数
    /// - `f`: 要执行的异步函数
    ///
    /// # 返回
    /// - `T`: 任务执行结果
    pub fn block_on<F, T>(&self, f: F) -> T
    where
        F: std::future::Future<Output = T>,
    {
        // 在 setup 函数中，我们需要阻塞等待初始化完成
        // 使用 Runtime::block_on 是安全的，因为这是在应用启动时执行的
        self.runtime.block_on(f)
    }

    /// 在运行时中执行异步任务（不阻塞）
    ///
    /// # 参数
    /// - `f`: 要执行的异步函数
    ///
    /// # 返回
    /// - `tokio::task::JoinHandle`: 任务句柄
    pub fn spawn<F>(&self, f: F) -> tokio::task::JoinHandle<F::Output>
    where
        F: std::future::Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.handle().spawn(f)
    }
}

impl Default for RuntimeManager {
    fn default() -> Self {
        Self::new().expect("无法创建默认的 Tokio 运行时")
    }
}

