// 文件图标映射

import {
  IMAGE_EXTENSIONS,
  VIDEO_EXTENSIONS,
  AUDIO_EXTENSIONS,
  DOCUMENT_EXTENSIONS,
  CODE_EXTENSIONS,
  ARCHIVE_EXTENSIONS,
  ICON_CHAR_MAP,
  isExtensionInList,
} from './constants';

/**
 * 根据文件类型和扩展名获取图标类名或路径
 * @param item 文件项
 * @returns 图标标识
 */
export function getFileIcon(item: { file_type: 'file' | 'folder'; extension?: string }): string {
  if (item.file_type === 'folder') {
    return 'folder';
  }

  if (!item.extension) {
    return 'file';
  }

  const ext = item.extension.toLowerCase();

  // 图片类型
  if (isExtensionInList(ext, IMAGE_EXTENSIONS)) {
    return 'image';
  }

  // 视频类型
  if (isExtensionInList(ext, VIDEO_EXTENSIONS)) {
    return 'video';
  }

  // 音频类型
  if (isExtensionInList(ext, AUDIO_EXTENSIONS)) {
    return 'audio';
  }

  // 文档类型
  if (isExtensionInList(ext, DOCUMENT_EXTENSIONS)) {
    return 'document';
  }

  // 代码类型
  if (isExtensionInList(ext, CODE_EXTENSIONS)) {
    return 'code';
  }

  // 压缩文件
  if (isExtensionInList(ext, ARCHIVE_EXTENSIONS)) {
    return 'archive';
  }

  // 默认文件图标
  return 'file';
}

/**
 * 获取图标 Unicode 字符（简单实现）
 * @param iconType 图标类型
 * @returns Unicode 字符
 */
export function getIconChar(iconType: string): string {
  return ICON_CHAR_MAP[iconType] || '📄';
}

