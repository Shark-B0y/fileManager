<script setup lang="ts">
import { ref, computed } from 'vue';
import NavigationBar from './components/NavigationBar.vue';
import Toolbar from './components/Toolbar.vue';
import MainContent from './components/MainContent.vue';

const mainContentRef = ref<InstanceType<typeof MainContent> | null>(null);

// 获取选中的文件项
const selectedItems = computed(() => {
  return mainContentRef.value?.selectedItems || [];
});

// 处理粘贴完成
function handlePasteComplete() {
  // 粘贴完成后可以执行一些操作，比如显示提示
  console.log('粘贴完成');
}

// 处理错误
function handleError(message: string) {
  // 显示错误提示
  alert(message);
}
</script>

<template>
  <div class="file-manager-app">
    <NavigationBar />
    <Toolbar
      :selected-items="selectedItems"
      @paste-complete="handlePasteComplete"
      @error="handleError"
    />
    <MainContent ref="mainContentRef" />
  </div>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

:root {
  font-family: 'Segoe UI', 'Microsoft YaHei', Arial, sans-serif;
  font-size: 14px;
  line-height: 1.5;
  color: #212121;
  background-color: #ffffff;
}

body {
  margin: 0;
  padding: 0;
  overflow: hidden;
}

#app {
  width: 100vw;
  height: 100vh;
  overflow: hidden;
}

.file-manager-app {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}
</style>
