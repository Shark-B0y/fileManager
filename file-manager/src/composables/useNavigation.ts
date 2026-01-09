// 导航管理 composable
// 用于管理文件浏览器的前进、后退历史记录

import { ref, computed } from 'vue';

interface NavigationState {
  history: string[];
  currentIndex: number;
}

// 共享状态：使用模块级别的 ref，确保所有组件实例共享同一个导航历史
const navigationState = ref<NavigationState>({
  history: [],
  currentIndex: -1,
});

/**
 * 导航管理 composable
 * 提供前进、后退、导航历史管理功能
 */
export function useNavigation() {

  /**
   * 是否可以后退
   */
  const canGoBack = computed(() => {
    return navigationState.value.currentIndex > 0;
  });

  /**
   * 是否可以前进
   */
  const canGoForward = computed(() => {
    return (
      navigationState.value.currentIndex >= 0 &&
      navigationState.value.currentIndex < navigationState.value.history.length - 1
    );
  });

  /**
   * 当前路径
   */
  const currentPath = computed(() => {
    if (
      navigationState.value.currentIndex >= 0 &&
      navigationState.value.currentIndex < navigationState.value.history.length
    ) {
      return navigationState.value.history[navigationState.value.currentIndex];
    }
    return '';
  });

  /**
   * 导航到新路径
   * 这会清除当前索引之后的历史记录
   */
  function navigateTo(path: string) {
    const state = navigationState.value;

    // 如果当前路径和新路径相同，不执行任何操作
    if (currentPath.value === path) {
      return;
    }

    // 如果当前不在历史末尾，清除之后的历史
    if (state.currentIndex < state.history.length - 1) {
      state.history = state.history.slice(0, state.currentIndex + 1);
    }

    // 添加新路径到历史
    state.history.push(path);
    state.currentIndex = state.history.length - 1;
  }

  /**
   * 后退到上一个路径
   * @returns 上一个路径，如果无法后退则返回 null
   */
  function goBack(): string | null {
    if (!canGoBack.value) {
      return null;
    }

    navigationState.value.currentIndex -= 1;
    return currentPath.value;
  }

  /**
   * 前进到下一个路径
   * @returns 下一个路径，如果无法前进则返回 null
   */
  function goForward(): string | null {
    if (!canGoForward.value) {
      return null;
    }

    navigationState.value.currentIndex += 1;
    return currentPath.value;
  }

  /**
   * 初始化导航历史
   */
  function initialize(path: string) {
    navigationState.value = {
      history: [path],
      currentIndex: 0,
    };
  }

  /**
   * 清空导航历史
   */
  function clear() {
    navigationState.value = {
      history: [],
      currentIndex: -1,
    };
  }

  return {
    canGoBack,
    canGoForward,
    currentPath,
    navigateTo,
    goBack,
    goForward,
    initialize,
    clear,
  };
}

