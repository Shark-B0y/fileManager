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

// 路径输入框引用
const pathInputRef = ref<HTMLInputElement | null>(null);
const pathInputValue = ref<string>('');
const isPathInputFocused = ref(false);

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
  // 初始化导航历史
  if (fsCurrentPath.value) {
    const path = fsCurrentPath.value === '驱动盘' ? 'drives:' : fsCurrentPath.value;
    initNavigation(path);
  }
});

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyDown);
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

/* 错误提示消息样式 */
.error-message {
  position: absolute;
  top: 50px;
  left: 50%;
  transform: translateX(-50%);
  background-color: #f44336;
  color: #ffffff;
  padding: 12px 24px;
  border-radius: 4px;
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
</style>

