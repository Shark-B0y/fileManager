<template>
  <div class="file-list">
    <div class="list-header">
      <div class="header-cell name-cell">名称</div>
      <div class="header-cell date-cell">修改日期</div>
      <div class="header-cell type-cell">类型</div>
      <div class="header-cell size-cell">大小</div>
    </div>

    <div class="list-body">
      <FileItem
        v-for="item in items"
        :key="item.id"
        :item="item"
        @click="handleItemClick(item)"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import FileItem from './FileItem.vue';
import type { FileItem as FileItemType } from '../types/file';

defineProps<{
  items: FileItemType[];
}>();

const emit = defineEmits<{
  'item-click': [item: FileItemType];
}>();

function handleItemClick(item: FileItemType) {
  emit('item-click', item);
}
</script>

<style scoped>
.file-list {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.list-header {
  display: flex;
  padding: 8px 16px;
  background-color: #f5f5f5;
  border-bottom: 1px solid #e0e0e0;
  font-weight: 500;
  font-size: 13px;
  color: #666;
}

.header-cell {
  padding: 4px 8px;
}

.name-cell {
  flex: 1;
  min-width: 200px;
}

.date-cell {
  width: 180px;
}

.type-cell {
  width: 120px;
}

.size-cell {
  width: 100px;
  text-align: right;
}

.list-body {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
}
</style>

