/**
 * 工具函数常量定义
 *
 * 集中管理所有工具函数中使用的常量，包括文件类型、图标映射等
 */

// ==================== 文件类型扩展名 ====================

/** 图片文件扩展名列表 */
export const IMAGE_EXTENSIONS = ['jpg', 'jpeg', 'png', 'gif', 'bmp', 'svg', 'webp', 'ico'] as const;

/** 视频文件扩展名列表 */
export const VIDEO_EXTENSIONS = ['mp4', 'avi', 'mkv', 'mov', 'wmv', 'flv', 'webm'] as const;

/** 音频文件扩展名列表 */
export const AUDIO_EXTENSIONS = ['mp3', 'wav', 'flac', 'aac', 'ogg', 'wma'] as const;

/** 文档文件扩展名列表 */
export const DOCUMENT_EXTENSIONS = ['txt', 'md', 'doc', 'docx', 'pdf'] as const;

/** 代码文件扩展名列表 */
export const CODE_EXTENSIONS = ['js', 'ts', 'jsx', 'tsx', 'vue', 'html', 'css', 'json', 'xml'] as const;

/** 压缩文件扩展名列表 */
export const ARCHIVE_EXTENSIONS = ['zip', 'rar', '7z', 'tar', 'gz'] as const;

// ==================== 图标映射 ====================

/** 图标类型到 Unicode 字符的映射 */
export const ICON_CHAR_MAP: Record<string, string> = {
  'folder': '📁',
  'file': '📄',
  'image': '🖼️',
  'video': '🎬',
  'audio': '🎵',
  'document': '📝',
  'code': '💻',
  'archive': '📦',
} as const;

// ==================== 文件类型显示名称映射 ====================

/** 文件扩展名到显示名称的映射 */
export const FILE_TYPE_NAME_MAP: Record<string, string> = {
  'txt': '文本文档',
  'doc': 'Word 文档',
  'docx': 'Word 文档',
  'xls': 'Excel 表格',
  'xlsx': 'Excel 表格',
  'ppt': 'PowerPoint 演示文稿',
  'pptx': 'PowerPoint 演示文稿',
  'pdf': 'PDF 文档',
  'jpg': 'JPEG 图像',
  'jpeg': 'JPEG 图像',
  'png': 'PNG 图像',
  'gif': 'GIF 图像',
  'mp4': 'MP4 视频',
  'avi': 'AVI 视频',
  'mp3': 'MP3 音频',
  'zip': 'ZIP 压缩文件',
  'rar': 'RAR 压缩文件',
} as const;

// ==================== MIME 类型映射 ====================

/** 文件扩展名到 MIME 类型的映射 */
export const MIME_TYPE_MAP: Record<string, string> = {
  'jpg': 'image/jpeg',
  'jpeg': 'image/jpeg',
  'png': 'image/png',
  'gif': 'image/gif',
  'bmp': 'image/bmp',
  'webp': 'image/webp',
  'svg': 'image/svg+xml',
  'ico': 'image/x-icon',
} as const;

// ==================== 文件大小单位 ====================

/** 文件大小单位数组 */
export const FILE_SIZE_UNITS = ['B', 'KB', 'MB', 'GB', 'TB'] as const;

// ==================== 辅助函数 ====================

/**
 * 检查扩展名是否在指定的扩展名列表中
 * @param ext 扩展名（不含点号）
 * @param extensions 扩展名列表
 * @returns 是否匹配
 */
export function isExtensionInList(ext: string, extensions: readonly string[]): boolean {
  return extensions.includes(ext.toLowerCase());
}
