<template>
  <transition name="message-fade">
    <div
      v-if="message.text"
      class="global-message"
      :class="message.type"
    >
      {{ message.text }}
    </div>
  </transition>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';

interface Message {
  text: string;
  type: 'error' | 'normal';
}

const message = ref<Message>({
  text: '',
  type: 'error',
});

let messageTimer: number | null = null;

/**
 * 显示错误消息
 * @param text - 错误消息文本
 */
function showErrorMessage(text: string) {
  // 清除之前的定时器
  if (messageTimer !== null) {
    clearTimeout(messageTimer);
  }

  // 显示错误消息
  message.value = {
    text,
    type: 'error',
  };

  // 2秒后开始淡出（淡出动画持续0.3秒）
  messageTimer = window.setTimeout(() => {
    message.value.text = '';
    messageTimer = null;
  }, 2000);
}

/**
 * 显示普通消息（成功消息等）
 * @param text - 消息文本
 */
function showNormalMessage(text: string) {
  // 清除之前的定时器
  if (messageTimer !== null) {
    clearTimeout(messageTimer);
  }

  // 显示普通消息
  message.value = {
    text,
    type: 'normal',
  };

  // 2秒后开始淡出（淡出动画持续0.3秒）
  messageTimer = window.setTimeout(() => {
    message.value.text = '';
    messageTimer = null;
  }, 2000);
}

// 处理全局错误事件
function handleGlobalError(event: Event) {
  const customEvent = event as CustomEvent<{ message: string }>;
  if (customEvent.detail?.message) {
    showErrorMessage(customEvent.detail.message);
  }
}

// 处理全局普通消息事件
function handleGlobalNormalMessage(event: Event) {
  const customEvent = event as CustomEvent<{ message: string }>;
  if (customEvent.detail?.message) {
    showNormalMessage(customEvent.detail.message);
  }
}

onMounted(() => {
  window.addEventListener('show-global-error', handleGlobalError);
  window.addEventListener('show-global-normal-message', handleGlobalNormalMessage);
});

onUnmounted(() => {
  window.removeEventListener('show-global-error', handleGlobalError);
  window.removeEventListener('show-global-normal-message', handleGlobalNormalMessage);
  // 清理定时器
  if (messageTimer !== null) {
    clearTimeout(messageTimer);
    messageTimer = null;
  }
});
</script>

<style scoped>
.global-message {
  position: fixed;
  top: 50px;
  left: 50%;
  transform: translateX(-50%);
  padding: 10px 20px;
  border-radius: 10px;
  font-size: 14px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  z-index: 10000;
  white-space: nowrap;
  max-width: 80%;
  overflow: hidden;
  text-overflow: ellipsis;
  pointer-events: none;
}

.global-message.error {
  background-color: #f44336;
  color: #ffffff;
}

.global-message.normal {
  background-color: #e3f2fd;
  color: #1976d2;
}

/* 淡入淡出动画 */
.message-fade-enter-active {
  transition: opacity 0.3s ease-in;
}

.message-fade-leave-active {
  transition: opacity 0.3s ease-out;
}

.message-fade-enter-from,
.message-fade-leave-to {
  opacity: 0;
}

.message-fade-enter-to,
.message-fade-leave-from {
  opacity: 1;
}
</style>
