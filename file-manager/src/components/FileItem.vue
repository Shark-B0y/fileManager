<template>
  <div
    class="file-item"
    :class="{ 'is-folder': item.file_type === 'folder' }"
    @click="handleClick"
    @dblclick="handleDoubleClick"
  >
    <div class="item-cell name-cell">
      <span class="item-icon">{{ iconChar }}</span>
      <span class="item-name">{{ item.name }}</span>
    </div>
    <div class="item-cell date-cell">
      {{ formattedDate }}
    </div>
    <div class="item-cell type-cell">
      {{ typeName }}
    </div>
    <div class="item-cell size-cell">
      {{ formattedSize }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import type { FileItem } from '../types/file';
import { formatFileSize, formatDate, getFileTypeName } from '../utils/formatters';
import { getIconChar, getFileIcon } from '../utils/icons';

const props = defineProps<{
  item: FileItem;
}>();

const emit = defineEmits<{
  click: [item: FileItem];
}>();

const iconChar = computed(() => {
  return getIconChar(getFileIcon(props.item));
});

const formattedSize = computed(() => {
  if (props.item.file_type === 'folder') {
    return '-';
  }
  return formatFileSize(props.item.size);
});

const formattedDate = computed(() => {
  return formatDate(props.item.modified_date);
});

const typeName = computed(() => {
  return getFileTypeName(props.item);
});

function handleClick() {
  emit('click', props.item);
}

function handleDoubleClick() {
  // 双击事件也会触发 click，这里不需要额外处理
  emit('click', props.item);
}
</script>

<style scoped>
.file-item {
  display: flex;
  align-items: center;
  padding: 8px 16px;
  border-bottom: 1px solid #f0f0f0;
  cursor: pointer;
  transition: background-color 0.15s;
}

.file-item:hover {
  background-color: #f5f5f5;
}

.file-item.is-folder {
  font-weight: 500;
}

.item-cell {
  padding: 4px 8px;
  font-size: 14px;
  color: #212121;
}

.name-cell {
  flex: 1;
  min-width: 200px;
  display: flex;
  align-items: center;
  gap: 8px;
}

.item-icon {
  font-size: 18px;
  flex-shrink: 0;
}

.item-name {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
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
  color: #666;
}
</style>

