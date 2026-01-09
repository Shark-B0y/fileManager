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
      :items="directoryInfo.items"
      :selected-item-id="selectedItemId"
      @item-click="handleItemClick"
      @item-double-click="handleItemDoubleClick"
    />

    <div v-else class="empty">
      暂无内容
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
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

// 选中的文件/文件夹 ID
const selectedItemId = ref<string | null>(null);

// 监听目录变化，清除选中状态
watch(directoryInfo, () => {
  selectedItemId.value = null;
});

// 处理文件项单击（选中）
function handleItemClick(item: FileItem) {
  // 如果点击的是已选中的项，则取消选中；否则选中该项
  if (selectedItemId.value === item.id) {
    selectedItemId.value = null;
  } else {
    selectedItemId.value = item.id;
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

