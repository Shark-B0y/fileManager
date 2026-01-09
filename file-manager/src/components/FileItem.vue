<template>
  <div
    class="file-item"
    :class="{
      'is-folder': item.file_type === 'folder',
      'is-selected': isSelected
    }"
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
import { computed, onUnmounted } from 'vue';
import type { FileItem } from '../types/file';
import { formatFileSize, formatDate, getFileTypeName } from '../utils/formatters';
import { getIconChar, getFileIcon } from '../utils/icons';

const props = defineProps<{
  item: FileItem;
  isSelected?: boolean;
}>();

const emit = defineEmits<{
  click: [item: FileItem, event: MouseEvent];
  dblclick: [item: FileItem];
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

// 用于防止双击时触发选中操作
let lastClickTime = 0;
let lastClickItem: FileItem | null = null;

function handleClick(event: MouseEvent) {
  const isCtrlPressed = event.ctrlKey || event.metaKey; // 支持 Mac 的 Cmd 键
  const now = Date.now();
  const isSameItem = lastClickItem === props.item;
  const timeSinceLastClick = now - lastClickTime;

  // Ctrl+单击：立即执行，不受双击检测影响（多选模式）
  if (isCtrlPressed) {
    emit('click', props.item, event);
  } else {
    // 普通单击：如果是300ms内对同一个item的第二次点击，可能是双击，不执行单击
    // 否则立即执行单击（立即更新选中状态）
    if (!(isSameItem && timeSinceLastClick < 300)) {
      emit('click', props.item, event);
    }
  }

  // 记录点击时间和item（仅记录非Ctrl的点击，用于双击检测）
  if (!isCtrlPressed) {
    lastClickTime = now;
    lastClickItem = props.item;
  }
}

function handleDoubleClick() {
  // 双击时立即触发双击事件（如果是文件夹，会进入新目录，自动清除选中状态）
  emit('dblclick', props.item);
}

// 组件卸载时清理
onUnmounted(() => {
  lastClickItem = null;
});
</script>

<style scoped>
.file-item {
  display: flex;
  align-items: center;
  padding: 8px 16px;
  border-bottom: 1px solid #f0f0f0;
  cursor: pointer;
  transition: background-color 0.1s, border-color 0.1s;
  user-select: none;
}

.file-item:hover {
  background-color: #f5f5f5;
}

.file-item.is-folder {
  font-weight: 500;
}

.file-item.is-selected {
  background-color: #e3f2fd;
  border-left: 3px solid #2196f3;
  padding-left: 13px; /* 减去 3px 边框，保持对齐 */
}

.file-item.is-selected:hover {
  background-color: #bbdefb;
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

