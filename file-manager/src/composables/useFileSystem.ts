// 文件系统操作 composable

import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { FileItem, DirectoryInfo } from '../types/file';

export function useFileSystem() {
  const currentPath = ref<string>('');
  const directoryInfo = ref<DirectoryInfo | null>(null);
  const loading = ref(false);
  const error = ref<string | null>(null);

  /**
   * 加载目录内容
   */
  async function loadDirectory(path: string) {
    loading.value = true;
    error.value = null;

    try {
      const result = await invoke<DirectoryInfo>('list_directory', { path });
      directoryInfo.value = result;
      currentPath.value = result.path;
      return result;
    } catch (err) {
      error.value = err instanceof Error ? err.message : String(err);
      throw err;
    } finally {
      loading.value = false;
    }
  }

  /**
   * 获取用户主目录
   */
  async function getHomeDirectory(): Promise<string> {
    try {
      return await invoke<string>('get_home_directory');
    } catch (err) {
      throw new Error(err instanceof Error ? err.message : String(err));
    }
  }

  /**
   * 进入目录
   */
  async function enterDirectory(path: string) {
    await loadDirectory(path);
  }

  /**
   * 返回上级目录
   */
  async function goUp() {
    if (directoryInfo.value?.parent_path) {
      const parentPath = directoryInfo.value.parent_path;
      // 如果父路径是 "drives:"，则加载驱动盘列表
      if (parentPath === 'drives:') {
        await loadDrives();
      } else {
        await loadDirectory(parentPath);
      }
    }
  }

  /**
   * 加载驱动盘列表
   */
  async function loadDrives() {
    loading.value = true;
    error.value = null;

    try {
      const result = await invoke<DirectoryInfo>('list_drives');
      directoryInfo.value = result;
      currentPath.value = '驱动盘';
      return result;
    } catch (err) {
      error.value = err instanceof Error ? err.message : String(err);
      throw err;
    } finally {
      loading.value = false;
    }
  }

  /**
   * 初始化：加载用户主目录
   */
  async function initialize() {
    try {
      const homeDir = await getHomeDirectory();
      await loadDirectory(homeDir);
    } catch (err) {
      error.value = err instanceof Error ? err.message : String(err);
      // 如果获取主目录失败，尝试使用 C:\
      await loadDirectory('C:\\');
    }
  }

  return {
    currentPath,
    directoryInfo,
    loading,
    error,
    loadDirectory,
    getHomeDirectory,
    enterDirectory,
    goUp,
    initialize,
    loadDrives,
  };
}


