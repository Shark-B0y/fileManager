// 搜索功能 composable

import { ref, computed } from 'vue';
import type { FileItem } from '../types/file';

// 搜索类型枚举
export type SearchType = 'file' | 'tag' | 'content';

// 共享搜索状态
const searchQuery = ref<string>('');
const searchType = ref<SearchType>('file');
const searchResult = ref<FileItem | null>(null);

export function useSearch() {
  /**
   * 在当前目录的文件列表中搜索匹配的文件或文件夹
   * @param items 文件列表
   * @param query 搜索关键词
   * @returns 匹配的第一个文件或文件夹，如果没有匹配则返回 null
   */
  function searchInFiles(items: FileItem[], query: string): FileItem | null {
    if (!query || !query.trim()) {
      return null;
    }

    const trimmedQuery = query.trim().toLowerCase();

    // 优先匹配文件名开头
    for (const item of items) {
      const itemName = item.name.toLowerCase();
      if (itemName.startsWith(trimmedQuery)) {
        return item;
      }
    }

    // 如果没有开头匹配，则匹配包含
    for (const item of items) {
      const itemName = item.name.toLowerCase();
      if (itemName.includes(trimmedQuery)) {
        return item;
      }
    }

    return null;
  }

  /**
   * 执行搜索
   * @param items 文件列表
   */
  function performSearch(items: FileItem[]) {
    if (searchType.value === 'file') {
      searchResult.value = searchInFiles(items, searchQuery.value);
    } else {
      // 其他搜索类型待实现
      searchResult.value = null;
    }
  }

  /**
   * 清除搜索
   */
  function clearSearch() {
    searchQuery.value = '';
    searchResult.value = null;
  }

  return {
    searchQuery,
    searchType,
    searchResult,
    searchInFiles,
    performSearch,
    clearSearch,
  };
}

