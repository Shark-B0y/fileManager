// 剪贴板状态管理 composable

import { ref } from 'vue';

export type ClipboardOperation = 'cut' | 'copy' | null;

interface ClipboardData {
  operation: ClipboardOperation;
  paths: string[];
}

// 共享状态：使用模块级别的 ref，确保所有组件实例共享同一个状态
const clipboardData = ref<ClipboardData>({
  operation: null,
  paths: [],
});

export function useClipboard() {
  /**
   * 设置剪贴板数据（剪切）
   */
  function setCut(paths: string[]) {
    clipboardData.value = {
      operation: 'cut',
      paths: [...paths],
    };
  }

  /**
   * 设置剪贴板数据（复制）
   */
  function setCopy(paths: string[]) {
    clipboardData.value = {
      operation: 'copy',
      paths: [...paths],
    };
  }

  /**
   * 清除剪贴板数据
   */
  function clear() {
    clipboardData.value = {
      operation: null,
      paths: [],
    };
  }

  /**
   * 检查剪贴板是否有数据
   */
  function hasData(): boolean {
    return clipboardData.value.operation !== null && clipboardData.value.paths.length > 0;
  }

  /**
   * 获取剪贴板操作类型
   */
  function getOperation(): ClipboardOperation {
    return clipboardData.value.operation;
  }

  /**
   * 获取剪贴板路径列表
   */
  function getPaths(): string[] {
    return [...clipboardData.value.paths];
  }

  return {
    clipboardData,
    setCut,
    setCopy,
    clear,
    hasData,
    getOperation,
    getPaths,
  };
}

