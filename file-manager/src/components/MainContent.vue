<template>
  <div class="main-content">
    <div v-if="loading || tagSearchLoading" class="loading">
      加载中...
    </div>

    <div v-else-if="error" class="error">
      <p>错误: {{ error }}</p>
      <button @click="handleRetry">重试</button>
    </div>

    <FileList
      v-else-if="(directoryInfo || tagSearchItems.length > 0) && viewMode === 'list'"
      ref="fileListRef"
      :items="displayItems"
      :selected-item-ids="selectedItemIds"
      :editing-item-id="editingItemId"
      @item-click="handleItemClick"
      @item-double-click="handleItemDoubleClick"
      @selection-change="handleSelectionChange"
      @rename-complete="handleRenameComplete"
      @scroll-to-bottom="handleScrollToBottom"
    />

    <IconView
      v-else-if="(directoryInfo || tagSearchItems.length > 0) && viewMode === 'icon'"
      ref="iconViewRef"
      :items="displayItems"
      :selected-item-ids="selectedItemIds"
      :editing-item-id="editingItemId"
      @item-click="handleItemClick"
      @item-double-click="handleItemDoubleClick"
      @selection-change="handleSelectionChange"
      @rename-complete="handleRenameComplete"
      @scroll-to-bottom="handleScrollToBottom"
    />

    <div v-else class="empty">
      暂无内容
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted, nextTick } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useFileSystem } from '../composables/useFileSystem';
import FileList from './FileList.vue';
import IconView from './IconView.vue';
import type { FileItem, SearchResult } from '../types/file';

// 视图模式：'list' 或 'icon'
const viewMode = ref<'list' | 'icon'>('list');

// 监听视图切换事件
function handleViewModeToggle() {
  viewMode.value = viewMode.value === 'list' ? 'icon' : 'list';
  // 通知其他组件视图模式已变更
  window.dispatchEvent(new CustomEvent('view-mode-changed', {
    detail: { mode: viewMode.value }
  }));
}

onMounted(() => {
  window.addEventListener('toggle-view-mode', handleViewModeToggle);
});

onUnmounted(() => {
  window.removeEventListener('toggle-view-mode', handleViewModeToggle);
});

const {
  directoryInfo,
  loading,
  error,
  enterDirectory,
  initialize,
  loadDirectory,
  loadDrives,
  refresh,
} = useFileSystem();

// 选中的文件/文件夹 ID 集合
const selectedItemIds = ref<Set<string>>(new Set());

// 正在编辑的文件项 ID
const editingItemId = ref<string | null>(null);

// FileList 和 IconView 组件引用
const fileListRef = ref<InstanceType<typeof FileList> | null>(null);
const iconViewRef = ref<InstanceType<typeof IconView> | null>(null);

// 标签搜索相关状态
const tagSearchItems = ref<FileItem[]>([]);
const tagSearchLoading = ref(false);
const tagSearchPage = ref(1);
const tagSearchHasMore = ref(false);
const tagSearchTagId = ref<number | null>(null);
const PAGE_SIZE = 50;

// 计算显示的文件列表（优先显示标签搜索结果）
const displayItems = computed(() => {
  if (tagSearchItems.value.length > 0) {
    return tagSearchItems.value;
  }
  return directoryInfo.value?.items || [];
});

// 监听目录变化，清除选中状态和标签搜索结果
watch(directoryInfo, () => {
  selectedItemIds.value = new Set();
  // 如果切换到目录浏览，清除标签搜索结果
  if (directoryInfo.value && tagSearchItems.value.length > 0) {
    tagSearchItems.value = [];
    tagSearchPage.value = 1;
    tagSearchHasMore.value = false;
    tagSearchTagId.value = null;
  }
});

// 处理文件项单击（选中）
function handleItemClick(item: FileItem, event: MouseEvent) {
  const isCtrlPressed = event.ctrlKey || event.metaKey; // 支持 Mac 的 Cmd 键

  if (isCtrlPressed) {
    // Ctrl + 单击：多选模式
    const newSet = new Set(selectedItemIds.value);
    if (newSet.has(item.id)) {
      // 如果已选中，则取消选中
      newSet.delete(item.id);
    } else {
      // 如果未选中，则添加到选中列表
      newSet.add(item.id);
    }
    selectedItemIds.value = newSet;
  } else {
    // 普通单击：单选模式（清除其他选中项）
    selectedItemIds.value = new Set([item.id]);
  }
}

// 处理文件项双击（进入文件夹）
function handleItemDoubleClick(item: FileItem) {
  // 只有文件夹才能通过双击进入
  if (item.file_type === 'folder') {
    enterDirectory(item.path);
  }
}

// 处理框选变更事件
function handleSelectionChange(selectedIds: Set<string>) {
  selectedItemIds.value = selectedIds;
}

// 开始重命名
function startRename(item: FileItem) {
  // 确保该文件被选中
  selectedItemIds.value = new Set([item.id]);
  // 设置编辑状态
  editingItemId.value = item.id;
}

// 处理重命名完成
async function handleRenameComplete(itemId: string, newName: string) {
  // 如果新名称为空，表示取消编辑
  if (!newName) {
    editingItemId.value = null;
    return;
  }

  // 找到要重命名的文件项
  const item = directoryInfo.value?.items.find(i => i.id === itemId);
  if (!item) {
    editingItemId.value = null;
    return;
  }

  // 如果名称没有变化，直接取消编辑
  if (newName === item.name) {
    editingItemId.value = null;
    return;
  }

  try {
    // 调用后端接口重命名
    const { invoke } = await import('@tauri-apps/api/core');
    await invoke('rename_file', {
      oldPath: item.path,
      newName: newName,
    });

    // 刷新当前目录
    await refresh();

    // 清除编辑状态
    editingItemId.value = null;
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    window.dispatchEvent(
      new CustomEvent('show-global-error', {
        detail: { message: `重命名失败: ${message}` },
      }),
    );
    // 保持编辑状态，让用户继续修改
  }
}

// 重试加载
function handleRetry() {
  if (directoryInfo.value?.path) {
    const path = directoryInfo.value.path;
    if (path === 'drives:') {
      // 如果是驱动盘列表，重新加载驱动盘
      loadDrives();
    } else {
      loadDirectory(path);
    }
  } else {
    initialize();
  }
}

// 处理搜索事件
function handleSearchEvent(event: CustomEvent) {
  const { result } = event.detail;

  if (result) {
    // 选中搜索到的文件
    selectedItemIds.value = new Set([result.id]);

    // 滚动到该文件
    nextTick(() => {
      if (viewMode.value === 'list' && fileListRef.value) {
        fileListRef.value.scrollToItem(result.id);
      } else if (viewMode.value === 'icon' && iconViewRef.value) {
        iconViewRef.value.scrollToItem(result.id);
      }
    });
  } else {
    // 如果没有搜索结果，清除选中状态
    selectedItemIds.value = new Set();
  }
}

// 处理标签搜索事件
async function handleTagSearchEvent(event: CustomEvent) {
  const { tagId, tagName } = event.detail;

  tagSearchTagId.value = tagId;
  tagSearchPage.value = 1;
  tagSearchItems.value = [];
  tagSearchLoading.value = true;
  tagSearchHasMore.value = false;

  try {
    const result = await invoke<SearchResult>('search_files_by_tag', {
      tagId: tagId,
      page: 1,
      pageSize: PAGE_SIZE,
    });

    tagSearchItems.value = result.items;
    tagSearchHasMore.value = result.has_more;
    tagSearchPage.value = result.page;

    // 清除选中状态
    selectedItemIds.value = new Set();
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    window.dispatchEvent(
      new CustomEvent('show-global-error', {
        detail: { message: `搜索文件失败: ${message}` },
      }),
    );
  } finally {
    tagSearchLoading.value = false;
  }
}

// 处理滚动到底部（瀑布流加载更多）
async function handleScrollToBottom() {
  // 只有在标签搜索模式下才加载更多
  if (tagSearchTagId.value !== null && tagSearchHasMore.value && !tagSearchLoading.value) {
    tagSearchLoading.value = true;
    const nextPage = tagSearchPage.value + 1;

      try {
        const result = await invoke<SearchResult>('search_files_by_tag', {
          tagId: tagSearchTagId.value,
          page: nextPage,
          pageSize: PAGE_SIZE,
        });

      // 追加新数据
      tagSearchItems.value = [...tagSearchItems.value, ...result.items];
      tagSearchHasMore.value = result.has_more;
      tagSearchPage.value = result.page;
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      window.dispatchEvent(
        new CustomEvent('show-global-error', {
          detail: { message: `加载更多文件失败: ${message}` },
        }),
      );
    } finally {
      tagSearchLoading.value = false;
    }
  }
}

// 监听搜索事件
onMounted(() => {
  window.addEventListener('file-search', handleSearchEvent as unknown as EventListener);
  window.addEventListener('tag-search', handleTagSearchEvent as unknown as EventListener);
});

onUnmounted(() => {
  window.removeEventListener('file-search', handleSearchEvent as unknown as EventListener);
  window.removeEventListener('tag-search', handleTagSearchEvent as unknown as EventListener);
});

// 计算选中的文件项
const selectedItems = computed(() => {
  const items = displayItems.value;
  return items.filter(item => selectedItemIds.value.has(item.id));
});

// 暴露方法和数据给父组件
defineExpose({
  scrollToItem: (itemId: string) => {
    if (viewMode.value === 'list' && fileListRef.value) {
      fileListRef.value.scrollToItem(itemId);
    } else if (viewMode.value === 'icon' && iconViewRef.value) {
      iconViewRef.value.scrollToItem(itemId);
    }
  },
  selectedItemIds,
  selectedItems,
  startRename,
  viewMode,
});

// 初始化
initialize();
</script>

<style scoped>
.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  background-color: #ffffff;
}

.loading,
.error,
.empty {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 40px;
  color: #666;
}

.error {
  color: #d32f2f;
}

.error button {
  margin-top: 16px;
  padding: 8px 16px;
  border: 1px solid #d32f2f;
  border-radius: 4px;
  background-color: #fff;
  color: #d32f2f;
  cursor: pointer;
}

.error button:hover {
  background-color: #ffebee;
}
</style>

