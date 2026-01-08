<script setup lang="ts">
import MainContent from './components/MainContent.vue';
</script>

<template>
  <div class="file-manager-app">
    <MainContent />
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
