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
        :is-selected="selectedItemIdsSet.has(item.id)"
        @click="(item, event) => handleItemClick(item, event)"
        @dblclick="(item) => handleItemDoubleClick(item)"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import FileItem from './FileItem.vue';
import type { FileItem as FileItemType } from '../types/file';

const props = defineProps<{
  items: FileItemType[];
  selectedItemIds?: Set<string> | string[];
}>();

const emit = defineEmits<{
  'item-click': [item: FileItemType, event: MouseEvent];
  'item-double-click': [item: FileItemType];
}>();

// 将 props 中的 selectedItemIds 转换为 Set（如果是数组的话）
const selectedItemIdsSet = computed(() => {
  if (props.selectedItemIds instanceof Set) {
    return props.selectedItemIds;
  } else if (Array.isArray(props.selectedItemIds)) {
    return new Set(props.selectedItemIds);
  }
  return new Set<string>();
});

function handleItemClick(item: FileItemType, event: MouseEvent) {
  emit('item-click', item, event);
}

function handleItemDoubleClick(item: FileItemType) {
  emit('item-double-click', item);
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

