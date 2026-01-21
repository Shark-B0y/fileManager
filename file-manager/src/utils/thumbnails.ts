import { readFile } from '@tauri-apps/plugin-fs';
import type { FileItem } from '../types/file';

type CacheEntry = {
  url: string;
  createdAt: number;
};

const MAX_CACHE_ENTRIES = 200;
const IMAGE_TEYP = ['jpg', 'jpeg', 'png', 'gif', 'bmp', 'webp', 'svg', 'ico'];

// path -> CacheEntry
const thumbnailCache = new Map<string, CacheEntry>();
// path -> in-flight promise (avoid duplicate reads)
const inFlight = new Map<string, Promise<string>>();

/**
 * 判断是否为图片文件。
 * 注意：这里使用扩展名判断，避免访问文件系统读取元数据造成额外开销。
 */
export function isImageFile(item: Pick<FileItem, 'file_type' | 'extension'>): boolean {
  if (item.file_type !== 'file' || !item.extension) {
    return false;
  }
  const ext = item.extension.toLowerCase();
  return IMAGE_TEYP.includes(ext);
}

/**
 * 根据扩展名推断 MIME 类型（用于 Blob 展示）。
 */
export function getImageMimeType(extension?: string): string {
  const ext = (extension || '').toLowerCase();
  switch (ext) {
    case 'jpg':
    case 'jpeg':
      return 'image/jpeg';
    case 'png':
      return 'image/png';
    case 'gif':
      return 'image/gif';
    case 'bmp':
      return 'image/bmp';
    case 'webp':
      return 'image/webp';
    case 'svg':
      return 'image/svg+xml';
    case 'ico':
      return 'image/x-icon';
    default:
      return 'application/octet-stream';
  }
}

function enforceCacheLimit() {
  if (thumbnailCache.size <= MAX_CACHE_ENTRIES) {
    return;
  }

  // 按创建时间淘汰最旧的条目
  const entries = Array.from(thumbnailCache.entries()).sort((a, b) => a[1].createdAt - b[1].createdAt);
  const toRemove = entries.slice(0, Math.max(0, thumbnailCache.size - MAX_CACHE_ENTRIES));
  toRemove.forEach(([path, entry]) => {
    URL.revokeObjectURL(entry.url);
    thumbnailCache.delete(path);
  });
}

/**
 * 读取本地图片并生成可用于 <img> 的 blob URL。
 *
 * - 不使用 `file:///`，避免本地安全策略限制
 * - 使用 `@tauri-apps/plugin-fs` 直接读取文件字节
 */
export async function getThumbnailUrl(path: string, extension?: string): Promise<string> {
  const cached = thumbnailCache.get(path);
  if (cached) {
    return cached.url;
  }

  const existing = inFlight.get(path);
  if (existing) {
    return existing;
  }

  const task = (async () => {
    try {
      console.log('[thumbnails] 开始读取文件:', path);
      const bytes = await readFile(path);
      console.log('[thumbnails] 文件读取成功，大小:', bytes.length, 'bytes');
      const mimeType = getImageMimeType(extension);
      const blob = new Blob([bytes], { type: mimeType });
      const url = URL.createObjectURL(blob);
      console.log('[thumbnails] 生成 blob URL:', url);
      thumbnailCache.set(path, { url, createdAt: Date.now() });
      enforceCacheLimit();
      return url;
    } catch (error) {
      console.error('[thumbnails] 读取文件失败:', path, error);
      throw error;
    } finally {
      inFlight.delete(path);
    }
  })();

  inFlight.set(path, task);
  return task;
}

/**
 * 清理缓存（释放 blob URL），可在窗口切换/批量刷新时调用。
 */
export function clearThumbnailCache() {
  thumbnailCache.forEach((entry) => URL.revokeObjectURL(entry.url));
  thumbnailCache.clear();
  inFlight.clear();
}

