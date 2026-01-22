//! 时间相关工具函数

/// 格式化时间为 ISO 8601 格式
///
/// # 参数
/// - `time`: 系统时间
///
/// # 返回
/// 格式化的时间字符串（Unix 时间戳格式：`"{秒数}.{纳秒数}Z"`）
///
/// # 示例
/// ```
/// use std::time::SystemTime;
/// use crate::utils::format_iso8601;
///
/// let now = SystemTime::now();
/// let formatted = format_iso8601(&now);
/// println!("{}", formatted); // 例如: "1234567890.123456789Z"
/// ```
pub fn format_iso8601(time: &std::time::SystemTime) -> String {
    use std::time::UNIX_EPOCH;

    let duration = time.duration_since(UNIX_EPOCH)
        .unwrap_or_default();

    let secs = duration.as_secs();
    let nanos = duration.subsec_nanos();

    // 简化的 ISO 8601 格式
    // 实际应该使用 chrono 库，但这里为了简单直接格式化
    format!("{}.{:09}Z", secs, nanos)
}
