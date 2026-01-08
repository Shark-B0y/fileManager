// 文件图标映射

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
  if (['jpg', 'jpeg', 'png', 'gif', 'bmp', 'svg', 'webp'].includes(ext)) {
    return 'image';
  }

  // 视频类型
  if (['mp4', 'avi', 'mkv', 'mov', 'wmv', 'flv', 'webm'].includes(ext)) {
    return 'video';
  }

  // 音频类型
  if (['mp3', 'wav', 'flac', 'aac', 'ogg', 'wma'].includes(ext)) {
    return 'audio';
  }

  // 文档类型
  if (['txt', 'md', 'doc', 'docx', 'pdf'].includes(ext)) {
    return 'document';
  }

  // 代码类型
  if (['js', 'ts', 'jsx', 'tsx', 'vue', 'html', 'css', 'json', 'xml'].includes(ext)) {
    return 'code';
  }

  // 压缩文件
  if (['zip', 'rar', '7z', 'tar', 'gz'].includes(ext)) {
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
  const iconMap: Record<string, string> = {
    'folder': '📁',
    'file': '📄',
    'image': '🖼️',
    'video': '🎬',
    'audio': '🎵',
    'document': '📝',
    'code': '💻',
    'archive': '📦',
  };

  return iconMap[iconType] || '📄';
}

