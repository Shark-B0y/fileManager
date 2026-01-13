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
      v-else-if="directoryInfo"
      ref="fileListRef"
      :items="directoryInfo.items"
      :selected-item-ids="selectedItemIds"
      @item-click="handleItemClick"
      @item-double-click="handleItemDoubleClick"
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
import type { FileItem } from '../types/file';

const {
  directoryInfo,
  loading,
  error,
  enterDirectory,
  initialize,
  loadDirectory,
  loadDrives,
} = useFileSystem();

// 选中的文件/文件夹 ID 集合
const selectedItemIds = ref<Set<string>>(new Set());

// FileList 组件引用
const fileListRef = ref<InstanceType<typeof FileList> | null>(null);

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
      if (fileListRef.value) {
        fileListRef.value.scrollToItem(result.id);
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
    if (fileListRef.value) {
      fileListRef.value.scrollToItem(itemId);
    }
  },
  selectedItemIds,
  selectedItems,
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

