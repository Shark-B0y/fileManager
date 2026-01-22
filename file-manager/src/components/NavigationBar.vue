<template>
  <div class="navigation-bar">
    <div class="nav-controls">
      <!-- 返回按钮 -->
      <button
        class="nav-btn"
        :class="{ disabled: !canGoBack }"
        :disabled="!canGoBack"
        @click="handleGoBack"
        title="后退 (Alt+←)"
      >
        ←
      </button>

      <!-- 前进按钮 -->
      <button
        class="nav-btn"
        :class="{ disabled: !canGoForward }"
        :disabled="!canGoForward"
        @click="handleGoForward"
        title="前进 (Alt+→)"
      >
        →
      </button>

      <!-- 向上按钮（进入父级目录） -->
      <button
        class="nav-btn"
        :class="{ disabled: !canGoUp }"
        :disabled="!canGoUp"
        @click="handleGoUp"
        title="向上 (Alt+↑)"
      >
        ↑
      </button>

      <!-- 刷新按钮 -->
      <button
        class="nav-btn"
        :disabled="loading"
        @click="handleRefresh"
        title="刷新 (F5)"
      >
        ↻
      </button>
    </div>

    <!-- 路径输入框 -->
    <div class="path-display">
      <input
        ref="pathInputRef"
        v-model="pathInputValue"
        type="text"
        class="path-input"
        :placeholder="displayPathPlaceholder"
        @keydown.enter="handlePathInput"
        @keydown.esc="handlePathInputEscape"
        @blur="handlePathInputBlur"
        @focus="handlePathInputFocus"
      />
    </div>

    <!-- 搜索栏 -->
    <div class="search-container" :class="{ 'is-dropdown-open': isSearchDropdownOpen }">
      <div class="search-input-wrapper">
        <input
          ref="searchInputRef"
          v-model="searchQuery"
          type="text"
          class="search-input"
          :placeholder="getSearchPlaceholder"
          @input="handleSearchInput"
          @keydown.enter="handleSearchEnter"
          @keydown.esc="handleSearchEscape"
          @focus="handleSearchInputFocus"
        />
        <button
          class="search-dropdown-btn"
          @click.stop="toggleSearchDropdown"
          :title="getSearchTypeLabel"
        >
          <span class="dropdown-arrow">▼</span>
        </button>
      </div>

      <!-- 搜索类型页签 -->
      <transition name="dropdown-fade">
        <div v-if="isSearchDropdownOpen" class="search-tabs">
          <div
            class="search-tab"
            :class="{ active: searchType === 'file' }"
            @click="selectSearchType('file')"
          >
            文件搜索
          </div>
          <div
            class="search-tab"
            :class="{ active: searchType === 'tag' }"
            @click="selectSearchType('tag')"
          >
            标签搜索
          </div>
          <div
            class="search-tab disabled"
            @click.stop
          >
            内容搜索
            <span class="coming-soon">（即将推出）</span>
          </div>
        </div>
      </transition>

      <!-- 标签匹配列表 -->
      <transition name="dropdown-fade">
        <div v-if="searchType === 'tag' && matchedTags.length > 0 && searchQuery" class="tag-suggestions">
          <div
            v-for="tag in matchedTags"
            :key="tag.id"
            class="tag-suggestion-item"
            @click="selectTag(tag)"
          >
            <span
              class="tag-badge"
              :style="{
                backgroundColor: tag.color || '#FFFF00',
                color: tag.font_color || '#000000'
              }"
            >
              {{ tag.name }}
            </span>
          </div>
        </div>
      </transition>
    </div>

    <!-- 错误提示消息 -->
    <transition name="error-fade">
      <div v-if="errorMessage" class="error-message">
        {{ errorMessage }}
      </div>
    </transition>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted, nextTick } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useFileSystem } from '../composables/useFileSystem';
import { useNavigation } from '../composables/useNavigation';
import { useSearch } from '../composables/useSearch';
import type { Tag } from '../types/tag';

const {
  currentPath: fsCurrentPath,
  directoryInfo,
  loading,
  goUp: fsGoUp,
  loadDirectory,
  loadDrives,
  refresh,
} = useFileSystem();

const {
  canGoBack,
  canGoForward,
  currentPath: navCurrentPath,
  navigateTo,
  goBack,
  goForward,
  initialize: initNavigation,
} = useNavigation();

// 搜索功能
const {
  searchQuery,
  searchType,
  searchResult,
  performSearch,
  clearSearch,
} = useSearch();

// 路径输入框引用
const pathInputRef = ref<HTMLInputElement | null>(null);
const pathInputValue = ref<string>('');
const isPathInputFocused = ref(false);

// 搜索输入框引用
const searchInputRef = ref<HTMLInputElement | null>(null);

// 搜索下拉菜单状态
const isSearchDropdownOpen = ref(false);

// 标签搜索相关
const matchedTags = ref<Tag[]>([]);
const selectedTagId = ref<number | null>(null);
let searchTagsTimer: number | null = null;

// 错误提示消息
const errorMessage = ref<string>('');
let errorTimer: number | null = null;

// 显示路径（如果是驱动盘列表，显示中文；否则显示实际路径）
const displayPath = computed(() => {
  const path = fsCurrentPath.value;
  if (path === '驱动盘' || directoryInfo.value?.path === 'drives:') {
    return '驱动盘';
  }
  return path || '未知路径';
});

// 路径输入框占位符（当输入框为空时显示）
const displayPathPlaceholder = computed(() => {
  return displayPath.value || '输入路径';
});

// 是否可以向上（进入父级目录）
const canGoUp = computed(() => {
  const current = fsCurrentPath.value;
  if (current === '驱动盘' || directoryInfo.value?.path === 'drives:') {
    return false;
  }
  return directoryInfo.value?.parent_path != null;
});

// 处理返回
async function handleGoBack() {
  if (!canGoBack.value) return;

  isNavigating = true;
  const path = goBack();
  if (path) {
    if (path === '驱动盘' || path === 'drives:') {
      await loadDrives();
    } else {
      await loadDirectory(path);
    }
  }
}

// 处理前进
async function handleGoForward() {
  if (!canGoForward.value) return;

  isNavigating = true;
  const path = goForward();
  if (path) {
    if (path === '驱动盘' || path === 'drives:') {
      await loadDrives();
    } else {
      await loadDirectory(path);
    }
  }
}

// 处理向上（进入父级目录）
async function handleGoUp() {
  if (!canGoUp.value) return;

  isNavigating = true;
  await fsGoUp();

  // 向上导航后，路径已经改变，手动更新导航历史
  const newPath = fsCurrentPath.value;
  const pathToRecord = newPath === '驱动盘' ? 'drives:' : newPath;
  navigateTo(pathToRecord);
}

// 处理刷新
async function handleRefresh() {
  await refresh();
}

// 处理路径输入框获得焦点
function handlePathInputFocus() {
  isPathInputFocused.value = true;
  // 获得焦点时，如果输入框为空，填入当前路径
  if (!pathInputValue.value) {
    const current = fsCurrentPath.value;
    if (current !== '驱动盘' && current) {
      pathInputValue.value = current;
      // 选中所有文本，方便用户快速替换
      nextTick(() => {
        pathInputRef.value?.select();
      });
    }
  }
}

// 处理 ESC 键（取消输入）
function handlePathInputEscape() {
  pathInputValue.value = '';
  pathInputRef.value?.blur();
}

// 处理路径输入框失去焦点
function handlePathInputBlur() {
  isPathInputFocused.value = false;
  // 失去焦点时，如果输入框内容未提交或无效，恢复为空（显示占位符）
  nextTick(() => {
    const inputPath = pathInputValue.value.trim();
    // 如果输入框内容与当前路径相同，清空（会显示占位符）
    if (inputPath === fsCurrentPath.value || !inputPath) {
      pathInputValue.value = '';
    }
  });
}

// 处理路径输入（回车键时）
async function handlePathInput() {
  const inputPath = pathInputValue.value.trim();

  // 如果输入为空，恢复为空（显示占位符）
  if (!inputPath) {
    pathInputValue.value = '';
    pathInputRef.value?.blur();
    return;
  }

  // 如果输入的路径与当前路径相同，不做任何操作
  if (inputPath === fsCurrentPath.value) {
    pathInputValue.value = '';
    pathInputRef.value?.blur();
    return;
  }

  // 如果是驱动盘列表，不处理
  if (inputPath === '驱动盘' || inputPath === 'drives:') {
    pathInputValue.value = '';
    pathInputRef.value?.blur();
    return;
  }

  try {
    // 检查路径是否存在
    const exists = await invoke<boolean>('check_path_exists', { path: inputPath });

    if (exists) {
      // 路径存在，跳转到该目录
      isNavigating = true;
      await loadDirectory(inputPath);

      // 更新导航历史
      navigateTo(inputPath);

      // 清空输入框
      pathInputValue.value = '';
      pathInputRef.value?.blur();
    } else {
      // 路径不存在，显示错误提示
      showErrorMessage(`路径不存在: ${inputPath}`);
      // 清空输入框，失去焦点后会自动恢复为当前路径（占位符）
      pathInputValue.value = '';
      pathInputRef.value?.blur();
    }
  } catch (error) {
    // 发生错误，显示错误提示
    const message = error instanceof Error ? error.message : String(error);
    showErrorMessage(`无法访问路径: ${message}`);
    // 清空输入框
    pathInputValue.value = '';
    pathInputRef.value?.blur();
  }
}

// 标记是否正在执行导航操作（返回/前进/向上），避免重复记录历史
let isNavigating = false;

// 显示错误消息（2秒后自动淡出）
function showErrorMessage(message: string) {
  // 清除之前的定时器
  if (errorTimer !== null) {
    clearTimeout(errorTimer);
  }

  // 显示错误消息
  errorMessage.value = message;

  // 2秒后开始淡出（淡出动画持续0.3秒）
  errorTimer = window.setTimeout(() => {
    errorMessage.value = '';
    errorTimer = null;
  }, 2000);
}

// 监听全局错误事件（供其他组件触发）
window.addEventListener('show-global-error', (event: Event) => {
  const customEvent = event as CustomEvent<{ message: string }>;
  if (customEvent.detail?.message) {
    showErrorMessage(customEvent.detail.message);
  }
});

// 处理搜索输入
async function handleSearchInput() {
  if (searchType.value === 'file') {
    // 文件搜索模式
    if (directoryInfo.value?.items) {
      performSearch(directoryInfo.value.items);
      // 触发搜索事件，通知 MainContent 组件
      window.dispatchEvent(new CustomEvent('file-search', {
        detail: { query: searchQuery.value, result: searchResult.value }
      }));
    }
  } else if (searchType.value === 'tag') {
    // 标签搜索模式：输入时显示匹配的标签
    const query = searchQuery.value.trim();
    if (query) {
      // 防抖处理：延迟300ms后搜索
      if (searchTagsTimer !== null) {
        clearTimeout(searchTagsTimer);
      }
      searchTagsTimer = window.setTimeout(async () => {
        try {
          const tags = await invoke<Tag[]>('search_tags', {
            keyword: query,
            limit: 10,
          });
          matchedTags.value = tags;
        } catch (error) {
          console.error('搜索标签失败:', error);
          matchedTags.value = [];
        }
        searchTagsTimer = null;
      }, 300);
    } else {
      matchedTags.value = [];
      selectedTagId.value = null;
    }
  }
}

// 获取搜索类型标签
const getSearchTypeLabel = computed(() => {
  const labels: Record<string, string> = {
    file: '文件搜索',
    tag: '标签搜索',
    content: '内容搜索',
  };
  return labels[searchType.value] || '文件搜索';
});

// 获取搜索占位符
const getSearchPlaceholder = computed(() => {
  return `搜索${getSearchTypeLabel.value}...`;
});

// 切换搜索下拉菜单
function toggleSearchDropdown() {
  isSearchDropdownOpen.value = !isSearchDropdownOpen.value;
}

// 选择搜索类型
function selectSearchType(type: 'file' | 'tag' | 'content') {
  if (type === 'file' || type === 'tag') {
    searchType.value = type;
    isSearchDropdownOpen.value = false;
    matchedTags.value = [];
    selectedTagId.value = null;

    // 当搜索类型改变时，重新执行搜索
    if (type === 'file' && searchQuery.value && directoryInfo.value?.items) {
      performSearch(directoryInfo.value.items);
      window.dispatchEvent(new CustomEvent('file-search', {
        detail: { query: searchQuery.value, result: searchResult.value }
      }));
    } else if (type === 'tag' && searchQuery.value) {
      handleSearchInput();
    }
  }
}

// 选择标签
function selectTag(tag: Tag) {
  selectedTagId.value = tag.id;
  searchQuery.value = tag.name;
  matchedTags.value = [];
  searchInputRef.value?.blur();

  // 触发标签搜索事件
  window.dispatchEvent(new CustomEvent('tag-search', {
    detail: { tagId: tag.id, tagName: tag.name }
  }));
}

// 处理搜索输入框回车
async function handleSearchEnter() {
  if (searchType.value === 'tag') {
    const query = searchQuery.value.trim();
    if (query && matchedTags.value.length > 0) {
      // 如果有匹配的标签，使用第一个
      const firstTag = matchedTags.value[0];
      selectTag(firstTag);
    } else if (selectedTagId.value !== null) {
      // 如果已经选择了标签，触发搜索
      window.dispatchEvent(new CustomEvent('tag-search', {
        detail: { tagId: selectedTagId.value, tagName: searchQuery.value }
      }));
    }
  }
}

// 处理搜索输入框获得焦点
function handleSearchInputFocus() {
  // 输入框获得焦点时不关闭下拉菜单（由点击外部关闭）
}

// 处理搜索 ESC 键（清除搜索）
function handleSearchEscape() {
  if (isSearchDropdownOpen.value) {
    // 如果下拉菜单打开，先关闭下拉菜单
    isSearchDropdownOpen.value = false;
  } else {
    // 否则清除搜索
    clearSearch();
    searchInputRef.value?.blur();
    window.dispatchEvent(new CustomEvent('file-search', {
      detail: { query: '', result: null }
    }));
  }
}

// 点击外部关闭下拉菜单
function handleClickOutside(event: MouseEvent) {
  const target = event.target as HTMLElement;
  const searchContainer = target.closest('.search-container');
  if (!searchContainer && isSearchDropdownOpen.value) {
    isSearchDropdownOpen.value = false;
  }
}

// 监听路径变化，更新导航历史和输入框显示
// 只有当路径不是通过导航按钮改变时（如点击文件夹），才更新导航历史
watch([fsCurrentPath, directoryInfo], () => {
  // 如果正在执行导航操作，不更新历史（导航操作已经手动更新了历史）
  if (isNavigating) {
    isNavigating = false;
    // 清空输入框
    if (!isPathInputFocused.value) {
      pathInputValue.value = '';
    }
    return;
  }

  const current = fsCurrentPath.value;
  // 如果是驱动盘列表，使用特殊标识
  const pathToRecord = current === '驱动盘' || directoryInfo.value?.path === 'drives:'
    ? 'drives:'
    : current;

  // 如果当前导航历史中的路径与当前路径不同，说明是用户点击文件夹进入新目录
  if (pathToRecord && navCurrentPath.value !== pathToRecord) {
    navigateTo(pathToRecord);
  }

  // 更新输入框显示（仅在输入框未获得焦点时）
  if (!isPathInputFocused.value) {
    pathInputValue.value = '';
  }

  // 当目录变化时，如果搜索框有内容，重新执行搜索
  if (searchQuery.value && directoryInfo.value?.items) {
    performSearch(directoryInfo.value.items);
    window.dispatchEvent(new CustomEvent('file-search', {
      detail: { query: searchQuery.value, result: searchResult.value }
    }));
  }
}, { deep: true });

// 键盘快捷键支持
function handleKeyDown(event: KeyboardEvent) {
  // Alt + ← 后退
  if (event.altKey && event.key === 'ArrowLeft') {
    event.preventDefault();
    handleGoBack();
  }
  // Alt + → 前进
  else if (event.altKey && event.key === 'ArrowRight') {
    event.preventDefault();
    handleGoForward();
  }
  // Alt + ↑ 向上
  else if (event.altKey && event.key === 'ArrowUp') {
    event.preventDefault();
    handleGoUp();
  }
  // F5 刷新
  else if (event.key === 'F5') {
    event.preventDefault();
    handleRefresh();
  }
}

onMounted(() => {
  window.addEventListener('keydown', handleKeyDown);
  document.addEventListener('click', handleClickOutside);
  // 初始化导航历史
  if (fsCurrentPath.value) {
    const path = fsCurrentPath.value === '驱动盘' ? 'drives:' : fsCurrentPath.value;
    initNavigation(path);
  }
});

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyDown);
  document.removeEventListener('click', handleClickOutside);
  // 清理错误提示定时器
  if (errorTimer !== null) {
    clearTimeout(errorTimer);
  }
});
</script>

<style scoped>
.navigation-bar {
  position: relative;
  display: flex;
  align-items: center;
  height: 40px;
  padding: 0 8px;
  background-color: #f5f5f5;
  border-bottom: 1px solid #e0e0e0;
  gap: 8px;
}

.nav-controls {
  display: flex;
  align-items: center;
  gap: 2px;
  flex-shrink: 0;
}

.nav-btn {
  width: 32px;
  height: 32px;
  padding: 0;
  border: 1px solid #ccc;
  border-radius: 3px;
  background-color: #fff;
  cursor: pointer;
  font-size: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background-color 0.15s, border-color 0.15s;
  color: #212121;
}

.nav-btn:hover:not(:disabled) {
  background-color: #e8e8e8;
  border-color: #999;
}

.nav-btn:active:not(:disabled) {
  background-color: #d0d0d0;
}

.nav-btn:disabled,
.nav-btn.disabled {
  opacity: 0.5;
  cursor: not-allowed;
  background-color: #f5f5f5;
}

.path-display {
  flex: 1;
  min-width: 0;
  display: flex;
  align-items: center;
  padding: 0;
  background-color: #fff;
  border: 1px solid #ccc;
  border-radius: 3px;
  height: 32px;
  overflow: hidden;
}

.path-input {
  flex: 1;
  width: 100%;
  height: 100%;
  padding: 4px 8px;
  border: none;
  outline: none;
  background-color: transparent;
  font-size: 14px;
  color: #212121;
  font-family: inherit;
  box-sizing: border-box;
}

.path-input::placeholder {
  color: #999;
  opacity: 0.7;
}

.path-display:focus-within {
  border-color: #2196f3;
  box-shadow: 0 0 0 2px rgba(33, 150, 243, 0.1);
}

.path-input:focus {
  outline: none;
}

/* 搜索栏样式 */
.search-container {
  position: relative;
  flex-shrink: 0;
  min-width: 250px;
}

.search-input-wrapper {
  position: relative;
  display: flex;
  align-items: center;
  height: 32px;
  border: 1px solid #ccc;
  border-radius: 3px;
  background-color: #fff;
  transition: border-color 0.15s, box-shadow 0.15s;
}

.search-container:focus-within .search-input-wrapper,
.search-container.is-dropdown-open .search-input-wrapper {
  border-color: #2196f3;
  box-shadow: 0 0 0 2px rgba(33, 150, 243, 0.1);
}

.search-input {
  flex: 1;
  min-width: 0;
  height: 100%;
  padding: 4px 32px 4px 8px;
  border: none;
  background-color: transparent;
  font-size: 14px;
  color: #212121;
  font-family: inherit;
  outline: none;
}

.search-input::placeholder {
  color: #999;
  opacity: 0.7;
}

.search-dropdown-btn {
  position: absolute;
  right: 0;
  top: 0;
  width: 24px;
  height: 100%;
  padding: 0;
  border: none;
  background-color: transparent;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  opacity: 0;
  transition: opacity 0.2s, background-color 0.15s;
  outline: none;
  z-index: 1;
}

/* 鼠标悬停搜索框右侧时显示下拉按钮 */
.search-input-wrapper:hover .search-dropdown-btn,
.search-container.is-dropdown-open .search-dropdown-btn {
  opacity: 1;
}

.search-dropdown-btn:hover {
  background-color: rgba(0, 0, 0, 0.05);
}

.search-dropdown-btn:active {
  background-color: rgba(0, 0, 0, 0.1);
}

.dropdown-arrow {
  font-size: 10px;
  color: #666;
  transition: transform 0.2s;
}

.search-container.is-dropdown-open .dropdown-arrow {
  transform: rotate(180deg);
}

/* 搜索类型页签样式 */
.search-tabs {
  position: absolute;
  top: calc(100% + 4px);
  left: 0;
  right: 0;
  display: flex;
  gap: 4px;
  padding: 4px;
  background-color: #fff;
  border: 1px solid #ccc;
  border-radius: 3px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  z-index: 1000;
}

.search-tab {
  flex: 1;
  padding: 6px 12px;
  font-size: 13px;
  color: #212121;
  cursor: pointer;
  text-align: center;
  border-radius: 2px;
  transition: background-color 0.15s, color 0.15s;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 4px;
  white-space: nowrap;
}

.search-tab:hover:not(.disabled) {
  background-color: #f5f5f5;
}

.search-tab.active {
  background-color: #e3f2fd;
  color: #2196f3;
  font-weight: 500;
}

.search-tab.disabled {
  color: #999;
  cursor: not-allowed;
  font-style: italic;
  opacity: 0.6;
}

.search-tab.disabled:hover {
  background-color: transparent;
}

.coming-soon {
  font-size: 11px;
  color: #999;
  margin-left: 4px;
}

/* 页签动画 */
.dropdown-fade-enter-active {
  transition: opacity 0.2s ease-in, transform 0.2s ease-in;
}

.dropdown-fade-leave-active {
  transition: opacity 0.15s ease-out, transform 0.15s ease-out;
}

.dropdown-fade-enter-from {
  opacity: 0;
  transform: translateY(-8px);
}

.dropdown-fade-leave-to {
  opacity: 0;
  transform: translateY(-8px);
}

/* 错误提示消息样式 */
.error-message {
  position: absolute;
  top: 50px;
  left: 50%;
  transform: translateX(-50%);
  background-color: #f44336;
  color: #ffffff;
  padding: 10px 20px;
  border-radius: 10px;
  font-size: 14px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  z-index: 1000;
  white-space: nowrap;
  max-width: 80%;
  overflow: hidden;
  text-overflow: ellipsis;
}

/* 淡入淡出动画 */
.error-fade-enter-active {
  transition: opacity 0.3s ease-in;
}

.error-fade-leave-active {
  transition: opacity 0.3s ease-out;
}

.error-fade-enter-from,
.error-fade-leave-to {
  opacity: 0;
}

.error-fade-enter-to,
.error-fade-leave-from {
  opacity: 1;
}

/* 标签建议列表样式 */
.tag-suggestions {
  position: absolute;
  top: calc(100% + 4px);
  left: 0;
  right: 0;
  max-height: 200px;
  overflow-y: auto;
  background-color: #fff;
  border: 1px solid #ccc;
  border-radius: 3px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  z-index: 1000;
  padding: 4px;
}

.tag-suggestion-item {
  padding: 6px 8px;
  cursor: pointer;
  border-radius: 2px;
  transition: background-color 0.15s;
}

.tag-suggestion-item:hover {
  background-color: #f5f5f5;
}

.tag-badge {
  display: inline-block;
  padding: 2px 8px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 500;
}
</style>

