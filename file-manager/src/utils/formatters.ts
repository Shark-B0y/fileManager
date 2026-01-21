// 格式化工具函数

import { FILE_SIZE_UNITS, FILE_TYPE_NAME_MAP } from './constants';

/**
 * 格式化文件大小
 * @param bytes 字节数
 * @returns 格式化后的字符串
 */
export function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 B';

  const k = 1024;
  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(2))} ${FILE_SIZE_UNITS[i]}`;
}

/**
 * 格式化日期时间
 * @param dateString 时间字符串（可能是 ISO 8601 或 Unix 时间戳格式）
 * @returns 格式化后的日期字符串
 */
export function formatDate(dateString: string): string {
  let date: Date;

  // 尝试解析不同的时间格式
  // 格式可能是: "1234567890.123456789Z" (Unix 时间戳 + 纳秒)
  if (dateString.includes('.')) {
    const parts = dateString.split('.');
    const seconds = parseInt(parts[0], 10);
    date = new Date(seconds * 1000);
  } else {
    // 尝试直接解析
    date = new Date(dateString);
  }

  // 检查日期是否有效
  if (isNaN(date.getTime())) {
    return dateString; // 如果无法解析，返回原始字符串
  }

  const now = new Date();
  const diff = now.getTime() - date.getTime();
  const days = Math.floor(diff / (1000 * 60 * 60 * 24));

  // 如果是今天，显示时间
  if (days === 0) {
    return date.toLocaleTimeString('zh-CN', {
      hour: '2-digit',
      minute: '2-digit'
    });
  }

  // 如果是今年，显示月日和时间
  if (date.getFullYear() === now.getFullYear()) {
    return date.toLocaleDateString('zh-CN', {
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit'
    });
  }

  // 其他情况显示完整日期
  return date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit'
  });
}

/**
 * 获取文件类型显示名称
 * @param item 文件项
 * @returns 类型名称
 */
export function getFileTypeName(item: { file_type: 'file' | 'folder'; extension?: string }): string {
  if (item.file_type === 'folder') {
    return '文件夹';
  }

  if (!item.extension) {
    return '文件';
  }

  const ext = item.extension.toLowerCase();
  return FILE_TYPE_NAME_MAP[ext] || `${ext.toUpperCase()} 文件`;
}

