<template>
  <div class="toolbar">
    <div class="toolbar-row">
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

      <div class="toolbar-group toolbar-group-right">
        <button
          class="toolbar-button tag-button"
          :class="{ active: isTagPanelExpanded }"
          @click="toggleTagPanel"
          @mouseenter="hoveredButton = 'tag'"
          @mouseleave="hoveredButton = null"
        >
          <img src="../assets/icon/tag.svg" alt="标签" class="icon" :class="{ active: isTagPanelExpanded }" />
          <span v-if="hoveredButton === 'tag'" class="tooltip">标签</span>
        </button>
      </div>
    </div>

    <!-- 标签面板展开区域 -->
    <div v-if="isTagPanelExpanded" class="tag-panel">
      <div class="tag-panel-row tag-panel-header">
        <div class="tag-panel-title">标签</div>
        <div class="tag-sort-dropdown" @click.stop="toggleTagSortDropdown">
          <span class="tag-sort-label">
            {{ tagSortMode === 'most_used' ? '最多使用' : '最近使用' }}
          </span>
          <img src="../assets/icon/pull_down.svg" alt="排序" class="tag-sort-icon" />
          <div v-if="isTagSortDropdownOpen" class="tag-sort-menu">
            <div
              class="tag-sort-option"
              :class="{ active: tagSortMode === 'most_used' }"
              @click.stop="changeTagSortMode('most_used')"
            >
              最多使用
            </div>
            <div
              class="tag-sort-option"
              :class="{ active: tagSortMode === 'recent_used' }"
              @click.stop="changeTagSortMode('recent_used')"
            >
              最近使用
            </div>
          </div>
        </div>
      </div>
      <div class="tag-panel-row">
        <div class="tag-list">
          <div
            v-for="tag in mostUsedTags"
            :key="tag.id"
            class="tag-item"
            :style="{
              backgroundColor: tag.color || '#FFFF00',
              color: tag.font_color || '#000000'
            }"
          >
            {{ tag.name }}
          </div>
          <div v-if="mostUsedTags.length === 0 && !loadingTags" class="tag-empty">
            暂无标签
          </div>
          <div v-if="loadingTags" class="tag-loading">
            加载中...
          </div>
        </div>
      </div>
      <div class="tag-panel-row">
        <!-- 第二行可以用于其他标签功能 -->
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useClipboard } from '../composables/useClipboard';
import { useFileSystem } from '../composables/useFileSystem';
import type { FileItem } from '../types/file';
import type { Tag } from '../types/tag';

const props = defineProps<{
  selectedItems: FileItem[];
}>();

const emit = defineEmits<{
  'paste-complete': [];
  'error': [message: string];
}>();

const { clipboardData, setCut, setCopy, hasData, clear } = useClipboard();
const { currentPath, refresh } = useFileSystem();

const hoveredButton = ref<'cut' | 'copy' | 'paste' | 'tag' | null>(null);
const isTagPanelExpanded = ref(false);
const mostUsedTags = ref<Tag[]>([]);
const loadingTags = ref(false);
const tagSortMode = ref<'most_used' | 'recent_used'>('most_used');
const isTagSortDropdownOpen = ref(false);

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

// 切换标签面板
async function toggleTagPanel() {
  isTagPanelExpanded.value = !isTagPanelExpanded.value;

  // 如果展开且标签列表为空，则加载标签
  if (isTagPanelExpanded.value && mostUsedTags.value.length === 0) {
    await loadTagList();
  }
}

function toggleTagSortDropdown() {
  isTagSortDropdownOpen.value = !isTagSortDropdownOpen.value;
}

async function changeTagSortMode(mode: 'most_used' | 'recent_used') {
  if (tagSortMode.value === mode) return;
  tagSortMode.value = mode;
  isTagSortDropdownOpen.value = false;
  await loadTagList();
}

// 加载标签列表
async function loadTagList() {
  loadingTags.value = true;
  try {
    const tags = await invoke<Tag[]>('get_tag_list', {
      limit: 10,
      mode: tagSortMode.value,
    });
    mostUsedTags.value = tags;
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    emit('error', `加载标签失败: ${message}`);
  } finally {
    loadingTags.value = false;
  }
}
</script>

<style scoped>
.toolbar {
  display: flex;
  flex-direction: column;
  background-color: #f5f5f5;
  border-bottom: 1px solid #e0e0e0;
}

.toolbar-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 16px;
  gap: 8px;
}

.toolbar-group {
  display: flex;
  align-items: center;
  gap: 4px;
}

.toolbar-group-right {
  margin-left: auto;
}

.tag-button.active .icon {
  filter: brightness(0.7);
  opacity: 1;
}

.tag-button .icon.active {
  filter: brightness(0.7);
  opacity: 1;
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

.tag-panel {
  padding: 8px 16px;
  background-color: #ffffff;
  border-top: 1px solid #e0e0e0;
}

.tag-panel-row {
  margin-bottom: 8px;
}

.tag-panel-row:last-child {
  margin-bottom: 0;
}

.tag-panel-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.tag-panel-title {
  font-size: 13px;
  font-weight: 500;
  color: #333;
}

.tag-sort-dropdown {
  position: relative;
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 2px 6px;
  border-radius: 4px;
  cursor: pointer;
  user-select: none;
  font-size: 12px;
  color: #555;
}

.tag-sort-dropdown:hover {
  background-color: #f0f0f0;
}

.tag-sort-label {
  line-height: 1;
}

.tag-sort-icon {
  width: 12px;
  height: 12px;
}

.tag-sort-menu {
  position: absolute;
  top: 100%;
  right: 0;
  margin-top: 4px;
  min-width: 80px;
  background-color: #ffffff;
  border: 1px solid #e0e0e0;
  border-radius: 4px;
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.08);
  z-index: 100;
}

.tag-sort-option {
  padding: 6px 8px;
  font-size: 12px;
  color: #555;
  cursor: pointer;
}

.tag-sort-option:hover {
  background-color: #f5f5f5;
}

.tag-sort-option.active {
  color: #1d4ed8;
  font-weight: 500;
  background-color: #eff6ff;
}

.tag-list {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  align-items: center;
}

.tag-item {
  padding: 4px 12px;
  border-radius: 12px;
  font-size: 12px;
  color: #ffffff;
  cursor: pointer;
  transition: opacity 0.2s;
}

.tag-item:hover {
  opacity: 0.8;
}

.tag-empty,
.tag-loading {
  padding: 4px 12px;
  font-size: 12px;
  color: #999;
}
</style>

