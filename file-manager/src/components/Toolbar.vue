<template>
  <div class="toolbar">
    <div class="toolbar-group">
      <button
        class="toolbar-button"
        :class="{ disabled: !canCutOrCopy }"
        :disabled="!canCutOrCopy"
        @click="handleCut"
        @mouseenter="hoveredButton = 'cut'"
        @mouseleave="hoveredButton = null"
      >
        <img src="../assets/icon/cut.svg" alt="剪切" class="icon" />
        <span v-if="hoveredButton === 'cut'" class="tooltip">剪切</span>
      </button>

      <button
        class="toolbar-button"
        :class="{ disabled: !canCutOrCopy }"
        :disabled="!canCutOrCopy"
        @click="handleCopy"
        @mouseenter="hoveredButton = 'copy'"
        @mouseleave="hoveredButton = null"
      >
        <img src="../assets/icon/copy.svg" alt="复制" class="icon" />
        <span v-if="hoveredButton === 'copy'" class="tooltip">复制</span>
      </button>

      <button
        class="toolbar-button"
        :class="{ disabled: !canPaste }"
        :disabled="!canPaste"
        @click="handlePaste"
        @mouseenter="hoveredButton = 'paste'"
        @mouseleave="hoveredButton = null"
      >
        <img src="../assets/icon/paste.svg" alt="粘贴" class="icon" />
        <span v-if="hoveredButton === 'paste'" class="tooltip">粘贴</span>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useClipboard } from '../composables/useClipboard';
import { useFileSystem } from '../composables/useFileSystem';
import type { FileItem } from '../types/file';

const props = defineProps<{
  selectedItems: FileItem[];
}>();

const emit = defineEmits<{
  'paste-complete': [];
  'error': [message: string];
}>();

const { clipboardData, setCut, setCopy, hasData, clear } = useClipboard();
const { currentPath, refresh } = useFileSystem();

const hoveredButton = ref<'cut' | 'copy' | 'paste' | null>(null);

// 是否可以剪切或复制（有选中项时可用）
const canCutOrCopy = computed(() => {
  return props.selectedItems.length > 0;
});

// 是否可以粘贴（剪贴板有数据时可用）
const canPaste = computed(() => {
  return hasData() && currentPath.value !== '';
});

// 处理剪切
async function handleCut() {
  if (!canCutOrCopy.value) return;

  try {
    const paths = props.selectedItems.map(item => item.path);
    setCut(paths);
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    emit('error', `剪切失败: ${message}`);
  }
}

// 处理复制
async function handleCopy() {
  if (!canCutOrCopy.value) return;

  try {
    const paths = props.selectedItems.map(item => item.path);
    setCopy(paths);
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    emit('error', `复制失败: ${message}`);
  }
}

// 处理粘贴
async function handlePaste() {
  if (!canPaste.value) return;

  try {
    const paths = clipboardData.value.paths;
    const operation = clipboardData.value.operation;
    const targetPath = currentPath.value;

    if (!targetPath || targetPath === 'drives:') {
      emit('error', '无法粘贴到当前位置');
      return;
    }

    if (operation === 'cut') {
      // 剪切：移动文件
      await invoke('cut_files', {
        paths: paths,
        targetPath: targetPath,
      });
    } else if (operation === 'copy') {
      // 复制：复制文件
      await invoke('copy_files', {
        paths: paths,
        targetPath: targetPath,
      });
    }

    // 清除剪贴板（如果是剪切操作）
    if (operation === 'cut') {
      clear();
    }

    // 刷新当前目录
    await refresh();

    emit('paste-complete');
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    emit('error', `粘贴失败: ${message}`);
  }
}
</script>

<style scoped>
.toolbar {
  display: flex;
  align-items: center;
  padding: 8px 16px;
  background-color: #f5f5f5;
  border-bottom: 1px solid #e0e0e0;
  gap: 8px;
}

.toolbar-group {
  display: flex;
  align-items: center;
  gap: 4px;
}

.toolbar-button {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  padding: 0;
  border: 1px solid transparent;
  border-radius: 4px;
  background-color: transparent;
  cursor: pointer;
  transition: background-color 0.2s;
}

.toolbar-button:hover:not(.disabled) {
  background-color: #e0e0e0;
}

.toolbar-button.disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.toolbar-button .icon {
  width: 20px;
  height: 20px;
  pointer-events: none;
}

.tooltip {
  position: absolute;
  bottom: 100%;
  left: 50%;
  transform: translateX(-50%);
  margin-bottom: 4px;
  padding: 4px 8px;
  background-color: #333;
  color: #fff;
  font-size: 12px;
  white-space: nowrap;
  border-radius: 4px;
  pointer-events: none;
  z-index: 1000;
}

.tooltip::after {
  content: '';
  position: absolute;
  top: 100%;
  left: 50%;
  transform: translateX(-50%);
  border: 4px solid transparent;
  border-top-color: #333;
}
</style>

