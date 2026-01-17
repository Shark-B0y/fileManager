<template>
  <div class="main-content">
    <div v-if="loading" class="loading">
      加载中...
    </div>

    <div v-else-if="error" class="error">
      <p>错误: {{ error }}</p>
      <button @click="handleRetry">重试</button>
    </div>

    <FileList
      v-else-if="directoryInfo && viewMode === 'list'"
      ref="fileListRef"
      :items="directoryInfo.items"
      :selected-item-ids="selectedItemIds"
      :editing-item-id="editingItemId"
      @item-click="handleItemClick"
      @item-double-click="handleItemDoubleClick"
      @selection-change="handleSelectionChange"
      @rename-complete="handleRenameComplete"
    />

    <IconView
      v-else-if="directoryInfo && viewMode === 'icon'"
      ref="iconViewRef"
      :items="directoryInfo.items"
      :selected-item-ids="selectedItemIds"
      :editing-item-id="editingItemId"
      @item-click="handleItemClick"
      @item-double-click="handleItemDoubleClick"
      @selection-change="handleSelectionChange"
      @rename-complete="handleRenameComplete"
    />

    <div v-else class="empty">
      暂无内容
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted, nextTick } from 'vue';
import { useFileSystem } from '../composables/useFileSystem';
import FileList from './FileList.vue';
import IconView from './IconView.vue';
import type { FileItem } from '../types/file';

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

// 监听目录变化，清除选中状态
watch(directoryInfo, () => {
  selectedItemIds.value = new Set();
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

// 监听搜索事件
onMounted(() => {
  window.addEventListener('file-search', handleSearchEvent as EventListener);
});

onUnmounted(() => {
  window.removeEventListener('file-search', handleSearchEvent as EventListener);
});

// 计算选中的文件项
const selectedItems = computed(() => {
  if (!directoryInfo.value) return [];
  return directoryInfo.value.items.filter(item => selectedItemIds.value.has(item.id));
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

