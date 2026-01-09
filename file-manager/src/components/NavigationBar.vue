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

    <!-- 路径显示 -->
    <div class="path-display">
      <span class="current-path">{{ displayPath }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, watch, onMounted, onUnmounted } from 'vue';
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

// 显示路径（如果是驱动盘列表，显示中文；否则显示实际路径）
const displayPath = computed(() => {
  const path = fsCurrentPath.value;
  if (path === '驱动盘' || directoryInfo.value?.path === 'drives:') {
    return '驱动盘';
  }
  return path || '未知路径';
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

// 标记是否正在执行导航操作（返回/前进/向上），避免重复记录历史
let isNavigating = false;

// 监听路径变化，更新导航历史
// 只有当路径不是通过导航按钮改变时（如点击文件夹），才更新导航历史
watch([fsCurrentPath, directoryInfo], () => {
  // 如果正在执行导航操作，不更新历史（导航操作已经手动更新了历史）
  if (isNavigating) {
    isNavigating = false;
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
});
</script>

<style scoped>
.navigation-bar {
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
  padding: 4px 8px;
  background-color: #fff;
  border: 1px solid #ccc;
  border-radius: 3px;
  height: 32px;
  overflow: hidden;
}

.current-path {
  flex: 1;
  font-size: 14px;
  color: #212121;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  line-height: 24px;
}
</style>

