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

        <button
          class="toolbar-button"
          :class="{ disabled: !canRename }"
          :disabled="!canRename"
          @click="handleRename"
          @mouseenter="hoveredButton = 'rename'"
          @mouseleave="hoveredButton = null"
        >
          <img src="../assets/icon/rename.svg" alt="重命名" class="icon" />
          <span v-if="hoveredButton === 'rename'" class="tooltip">重命名</span>
        </button>

        <button
          class="toolbar-button"
          :class="{ disabled: !canDelete }"
          :disabled="!canDelete"
          @click="handleDelete"
          @mouseenter="hoveredButton = 'delete'"
          @mouseleave="hoveredButton = null"
        >
          <img src="../assets/icon/delete.svg" alt="删除" class="icon" />
          <span v-if="hoveredButton === 'delete'" class="tooltip">删除</span>
        </button>
      </div>

      <div class="toolbar-group toolbar-group-right">
        <button
          class="toolbar-button view-toggle-button"
          @click="toggleViewMode"
          @mouseenter="hoveredButton = 'viewToggle'"
          @mouseleave="hoveredButton = null"
        >
          <img
            v-if="viewMode === 'icon'"
            src="../assets/icon/list.svg"
            alt="切换到列表视图"
            class="icon"
          />
          <img
            v-else
            src="../assets/icon/symbol.svg"
            alt="切换到图标视图"
            class="icon"
          />
          <span v-if="hoveredButton === 'viewToggle'" class="tooltip">
            {{ viewMode === 'list' ? '切换到图标视图' : '切换到列表视图' }}
          </span>
        </button>

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

    <!-- 修改标签弹窗 -->
    <ModifyTagDialog
      v-if="showModifyTagDialog"
      :tag="selectedTag"
      @close="closeModifyTagDialog"
      @modified="handleTagModified"
    />

    <!-- 标签面板展开区域 -->
    <div v-if="isTagPanelExpanded" class="tag-panel">
      <div class="tag-panel-row tags-row">
        <div class="tag-add-wrapper">
          <button class="tag-add-button" @click.stop="toggleAddTagInput">
            <img src="../assets/icon/add.svg" alt="新增标签" class="tag-add-icon" />
          </button>
          <div v-if="showAddTagInput" class="tag-add-input-wrapper">
            <input
              ref="addTagInputRef"
              v-model="newTagName"
              class="tag-add-input"
              type="text"
              placeholder="输入标签名称后回车"
              @keyup.enter="handleCreateTag"
              @blur="hideAddTagInput"
            />
          </div>
        </div>
        <div class="tag-search-wrapper">
          <button class="tag-search-button" @click.stop="toggleSearchTagInput">
            <img src="../assets/icon/find.svg" alt="搜索标签" class="tag-search-icon" />
          </button>
          <div v-if="showSearchTagInput" class="tag-search-input-wrapper">
            <input
              ref="searchTagInputRef"
              v-model="searchTagKeyword"
              class="tag-search-input"
              type="text"
              placeholder="输入搜索关键词后回车"
              @keyup.enter="handleSearchTags"
              @blur="hideSearchTagInput"
            />
          </div>
        </div>
        <div class="tag-modify-wrapper">
          <button class="tag-modify-button" @click.stop="openModifyTagDialog">
            <img src="../assets/icon/modify.svg" alt="修改标签" class="tag-modify-icon" />
          </button>
        </div>
        <div
          ref="tagListContainer"
          class="tag-list-scroll"
          :class="{ dragging: isDraggingTagList }"
          @mousedown.prevent="onTagListMouseDown"
          @mousemove.prevent="onTagListMouseMove"
          @mouseup="onTagListMouseUp"
          @mouseleave="onTagListMouseUp"
        >
          <div class="tag-list">
            <div
              v-for="tag in mostUsedTags"
              :key="tag.id"
              class="tag-item"
              :class="{ selected: selectedTag?.id === tag.id }"
              :style="{
                backgroundColor: tag.color || '#FFFF00',
                color: tag.font_color || '#000000'
              }"
              @click="selectTagForModify(tag)"
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
        <div class="tag-sort-dropdown" @click.stop="toggleTagSortDropdown">
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
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useClipboard } from '../composables/useClipboard';
import { useFileSystem } from '../composables/useFileSystem';
import ModifyTagDialog from './ModifyTagDialog.vue';
import type { FileItem } from '../types/file';
import type { Tag } from '../types/tag';

const props = defineProps<{
  selectedItems: FileItem[];
}>();

const emit = defineEmits<{
  'paste-complete': [];
  'error': [message: string];
  'rename': [item: FileItem];
  'delete-complete': [];
}>();

const { clipboardData, setCut, setCopy, hasData, clear } = useClipboard();
const { currentPath, refresh } = useFileSystem();

const hoveredButton = ref<'cut' | 'copy' | 'paste' | 'rename' | 'delete' | 'tag' | 'viewToggle' | null>(null);
const viewMode = ref<'list' | 'icon'>('list');

// 切换视图模式
function toggleViewMode() {
  viewMode.value = viewMode.value === 'list' ? 'icon' : 'list';
  window.dispatchEvent(new CustomEvent('toggle-view-mode'));
}

// 监听视图模式变更（从MainContent同步）
function handleViewModeChange(event: CustomEvent) {
  viewMode.value = event.detail.mode;
}

onMounted(() => {
  window.addEventListener('view-mode-changed', handleViewModeChange as EventListener);
});

onUnmounted(() => {
  window.removeEventListener('view-mode-changed', handleViewModeChange as EventListener);
});
const isTagPanelExpanded = ref(false);
const mostUsedTags = ref<Tag[]>([]);
const loadingTags = ref(false);
const tagSortMode = ref<'most_used' | 'recent_used'>('most_used');
const isTagSortDropdownOpen = ref(false);
const tagListContainer = ref<HTMLElement | null>(null);
const isDraggingTagList = ref(false);
const dragStartX = ref(0);
const dragStartScrollLeft = ref(0);
const showAddTagInput = ref(false);
const newTagName = ref('');
const addTagInputRef = ref<HTMLInputElement | null>(null);
const showSearchTagInput = ref(false);
const searchTagKeyword = ref('');
const searchTagInputRef = ref<HTMLInputElement | null>(null);
const showModifyTagDialog = ref(false);
const selectedTag = ref<Tag | null>(null);

// 是否可以剪切或复制（有选中项时可用）
const canCutOrCopy = computed(() => {
  return props.selectedItems.length > 0;
});

// 是否可以粘贴（剪贴板有数据时可用）
const canPaste = computed(() => {
  return hasData() && currentPath.value !== '';
});

// 是否可以重命名（选中单个文件或文件夹时可用）
const canRename = computed(() => {
  return props.selectedItems.length === 1;
});

// 是否可以删除（有选中项时可用）
const canDelete = computed(() => {
  return props.selectedItems.length > 0;
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

// 处理重命名
function handleRename() {
  if (!canRename.value) return;

  const item = props.selectedItems[0];
  emit('rename', item);
}

// 处理删除
async function handleDelete() {
  if (!canDelete.value) return;

  // 确认删除
  const itemCount = props.selectedItems.length;
  const confirmMessage = itemCount === 1
    ? `确定要删除 "${props.selectedItems[0].name}" 吗？此操作不可撤销。`
    : `确定要删除选中的 ${itemCount} 个项目吗？此操作不可撤销。`;

  if (!confirm(confirmMessage)) {
    return;
  }

  try {
    const paths = props.selectedItems.map(item => item.path);
    await invoke('delete_files', { paths });

    // 刷新当前目录
    await refresh();

    emit('delete-complete');
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    emit('error', `删除失败: ${message}`);
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

function toggleAddTagInput() {
  showAddTagInput.value = !showAddTagInput.value;
  if (showAddTagInput.value) {
    newTagName.value = '';
    // 下一帧聚焦输入框
    requestAnimationFrame(() => {
      addTagInputRef.value?.focus();
    });
  }
}

function hideAddTagInput() {
  // 失焦时如果没有内容就直接关闭
  if (!newTagName.value.trim()) {
    showAddTagInput.value = false;
  }
}

function toggleSearchTagInput() {
  showSearchTagInput.value = !showSearchTagInput.value;
  if (showSearchTagInput.value) {
    searchTagKeyword.value = '';
    // 下一帧聚焦输入框
    requestAnimationFrame(() => {
      searchTagInputRef.value?.focus();
    });
  }
}

function hideSearchTagInput() {
  // 失焦时如果没有内容就直接关闭
  if (!searchTagKeyword.value.trim()) {
    showSearchTagInput.value = false;
  }
}

async function handleSearchTags() {
  const keyword = searchTagKeyword.value.trim();
  if (!keyword) {
    window.dispatchEvent(
      new CustomEvent('show-global-error', {
        detail: { message: '搜索关键词不能为空' },
      }),
    );
    return;
  }

  try {
    loadingTags.value = true;
    const tags = await invoke<Tag[]>('search_tags', {
      keyword: keyword,
      limit: 50,
    });
    mostUsedTags.value = tags;
    // 搜索成功后关闭输入框
    showSearchTagInput.value = false;
    searchTagKeyword.value = '';
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    emit('error', `搜索标签失败: ${message}`);
  } finally {
    loadingTags.value = false;
  }
}

async function handleCreateTag() {
  const name = newTagName.value.trim();
  if (!name) {
    window.dispatchEvent(
      new CustomEvent('show-global-error', {
        detail: { message: '标签名称不能为空' },
      }),
    );
    return;
  }

  try {
    await invoke<Tag>('create_tag', { name });
    showAddTagInput.value = false;
    newTagName.value = '';
    // 重新加载当前排序模式下的标签列表
    await loadTagList();
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    window.dispatchEvent(
      new CustomEvent('show-global-error', {
        detail: { message },
      }),
    );
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

function onTagListMouseDown(event: MouseEvent) {
  if (!tagListContainer.value) return;
  isDraggingTagList.value = true;
  dragStartX.value = event.clientX;
  dragStartScrollLeft.value = tagListContainer.value.scrollLeft;
}

function onTagListMouseMove(event: MouseEvent) {
  if (!isDraggingTagList.value || !tagListContainer.value) return;
  const deltaX = event.clientX - dragStartX.value;
  tagListContainer.value.scrollLeft = dragStartScrollLeft.value - deltaX;
}

function onTagListMouseUp() {
  isDraggingTagList.value = false;
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

// 选择要修改的标签
function selectTagForModify(tag: Tag) {
  selectedTag.value = tag;
  showModifyTagDialog.value = true;
}

// 打开修改标签弹窗
function openModifyTagDialog() {
  // 如果没有选中的标签，提示用户先选择标签
  if (!selectedTag.value) {
    if (mostUsedTags.value.length === 0) {
      window.dispatchEvent(
        new CustomEvent('show-global-error', {
          detail: { message: '没有可修改的标签' },
        }),
      );
      return;
    }
    // 如果没有选中标签，默认选择第一个标签
    selectedTag.value = mostUsedTags.value[0];
  }
  showModifyTagDialog.value = true;
}

// 关闭修改标签弹窗
function closeModifyTagDialog() {
  showModifyTagDialog.value = false;
  selectedTag.value = null;
}

// 处理标签修改完成
async function handleTagModified() {
  // 重新加载标签列表
  await loadTagList();
  closeModifyTagDialog();
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

.tags-row {
  display: flex;
  align-items: center;
  gap: 8px;
}

.tag-add-wrapper {
  position: relative;
  display: flex;
  align-items: center;
  margin-right: 4px;
}

.tag-add-button {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  border-radius: 4px;
  border: 1px solid transparent;
  background-color: transparent;
  cursor: pointer;
  padding: 0;
}

.tag-add-button:hover {
  background-color: #f0f0f0;
}

.tag-add-icon {
  width: 14px;
  height: 14px;
}

.tag-add-input-wrapper {
  position: absolute;
  bottom: 100%;
  left: 0;
  margin-bottom: 4px;
  padding: 4px 6px;
  background-color: #ffffff;
  border-radius: 4px;
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.15);
  z-index: 110;
}

.tag-add-input {
  width: 140px;
  padding: 4px 6px;
  border: 1px solid #d1d5db;
  border-radius: 4px;
  font-size: 12px;
  outline: none;
}

.tag-add-input:focus {
  border-color: #2563eb;
  box-shadow: 0 0 0 1px rgba(37, 99, 235, 0.2);
}

.tag-search-wrapper {
  position: relative;
  display: flex;
  align-items: center;
  margin-right: 4px;
}

.tag-search-button {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  border-radius: 4px;
  border: 1px solid transparent;
  background-color: transparent;
  cursor: pointer;
  padding: 0;
}

.tag-search-button:hover {
  background-color: #f0f0f0;
}

.tag-search-icon {
  width: 14px;
  height: 14px;
}

.tag-search-input-wrapper {
  position: absolute;
  bottom: 100%;
  left: 0;
  margin-bottom: 4px;
  padding: 4px 6px;
  background-color: #ffffff;
  border-radius: 4px;
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.15);
  z-index: 110;
}

.tag-search-input {
  width: 140px;
  padding: 4px 6px;
  border: 1px solid #d1d5db;
  border-radius: 4px;
  font-size: 12px;
  outline: none;
}

.tag-search-input:focus {
  border-color: #2563eb;
  box-shadow: 0 0 0 1px rgba(37, 99, 235, 0.2);
}

.tag-modify-wrapper {
  position: relative;
  display: flex;
  align-items: center;
  margin-right: 4px;
}

.tag-modify-button {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  border-radius: 4px;
  border: 1px solid transparent;
  background-color: transparent;
  cursor: pointer;
  padding: 0;
}

.tag-modify-button:hover {
  background-color: #f0f0f0;
}

.tag-modify-icon {
  width: 14px;
  height: 14px;
}

.tag-list-scroll {
  flex: 1;
  overflow-x: auto;
  overflow-y: hidden;
  cursor: grab;
}

.tag-list-scroll.dragging {
  cursor: grabbing;
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
  flex-wrap: nowrap;
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

.tag-item.selected {
  border: 2px solid #2563eb;
  box-shadow: 0 0 0 2px rgba(37, 99, 235, 0.2);
}

.tag-empty,
.tag-loading {
  padding: 4px 12px;
  font-size: 12px;
  color: #999;
}
</style>

