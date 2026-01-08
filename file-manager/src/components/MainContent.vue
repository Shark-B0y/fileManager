<template>
  <div class="main-content">
    <div class="content-header">
      <div class="path-display">
        <span class="current-path">{{ currentPath }}</span>
        <button
          v-if="canGoUp"
          @click="handleGoUp"
          class="go-up-btn"
          title="返回上级目录"
        >
          ↑
        </button>
      </div>
    </div>

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
      @item-click="handleItemClick"
    />

    <div v-else class="empty">
      暂无内容
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useFileSystem } from '../composables/useFileSystem';
import FileList from './FileList.vue';
import type { FileItem } from '../types/file';

const {
  currentPath,
  directoryInfo,
  loading,
  error,
  enterDirectory,
  goUp,
  initialize,
  loadDirectory,
} = useFileSystem();

// 是否可以返回上级目录
const canGoUp = computed(() => {
  return directoryInfo.value?.parent_path != null;
});

// 处理文件项点击
function handleItemClick(item: FileItem) {
  if (item.type === 'folder') {
    enterDirectory(item.path);
  } else {
    // 文件点击处理（后续实现）
    console.log('点击文件:', item.name);
  }
}

// 返回上级目录
function handleGoUp() {
  goUp();
}

// 重试加载
function handleRetry() {
  if (currentPath.value) {
    loadDirectory(currentPath.value);
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

.content-header {
  padding: 8px 16px;
  border-bottom: 1px solid #e0e0e0;
  background-color: #f5f5f5;
}

.path-display {
  display: flex;
  align-items: center;
  gap: 8px;
}

.current-path {
  flex: 1;
  font-size: 14px;
  color: #212121;
  word-break: break-all;
}

.go-up-btn {
  padding: 4px 12px;
  border: 1px solid #ccc;
  border-radius: 4px;
  background-color: #fff;
  cursor: pointer;
  font-size: 16px;
  transition: background-color 0.2s;
}

.go-up-btn:hover {
  background-color: #f0f0f0;
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

