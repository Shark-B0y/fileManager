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
      <span v-if="!isEditing" class="item-name">{{ item.name }}</span>
      <input
        v-else
        ref="nameInputRef"
        v-model="editingName"
        class="item-name-input"
        @keyup.enter="handleRenameComplete"
        @keyup.esc="handleRenameCancel"
        @blur="handleRenameCancel"
      />
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
import { computed, ref, watch, nextTick, onUnmounted } from 'vue';
import type { FileItem } from '../types/file';
import { formatFileSize, formatDate, getFileTypeName } from '../utils/formatters';
import { getIconChar, getFileIcon } from '../utils/icons';

const props = defineProps<{
  item: FileItem;
  isSelected?: boolean;
  isEditing?: boolean;
}>();

const emit = defineEmits<{
  click: [item: FileItem, event: MouseEvent];
  dblclick: [item: FileItem];
  'rename-complete': [itemId: string, newName: string];
}>();

// 编辑相关状态
const editingName = ref('');
const nameInputRef = ref<HTMLInputElement | null>(null);

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

// 监听编辑状态变化
watch(() => props.isEditing, (newValue) => {
  if (newValue) {
    // 进入编辑模式：设置初始值为当前文件名（不含扩展名），并聚焦输入框
    editingName.value = getFileNameWithoutExtension(props.item.name);
    nextTick(() => {
      nameInputRef.value?.focus();
      nameInputRef.value?.select(); // 选中文本，方便直接修改
    });
  } else {
    // 退出编辑模式：清空编辑内容
    editingName.value = '';
  }
});

// 获取不含扩展名的文件名
function getFileNameWithoutExtension(fileName: string): string {
  const lastDotIndex = fileName.lastIndexOf('.');
  if (lastDotIndex === -1) {
    return fileName; // 没有扩展名
  }
  // 如果是文件夹或者第一个字符就是点（隐藏文件），返回完整文件名
  if (props.item.file_type === 'folder' || fileName.startsWith('.')) {
    return fileName;
  }
  return fileName.substring(0, lastDotIndex);
}

// 获取文件扩展名
function getFileExtension(fileName: string): string {
  const lastDotIndex = fileName.lastIndexOf('.');
  if (lastDotIndex === -1 || lastDotIndex === 0) {
    return ''; // 没有扩展名或隐藏文件
  }
  return fileName.substring(lastDotIndex);
}

// 处理重命名完成
function handleRenameComplete() {
  const trimmedName = editingName.value.trim();
  
  // 如果名称为空，取消重命名
  if (!trimmedName) {
    handleRenameCancel();
    return;
  }

  // 如果是文件，需要保留扩展名
  let finalName = trimmedName;
  if (props.item.file_type === 'file' && props.item.extension) {
    // 检查用户输入是否已包含扩展名
    const extension = getFileExtension(trimmedName);
    if (!extension) {
      // 用户没有输入扩展名，添加原来的扩展名
      finalName = trimmedName + '.' + props.item.extension;
    } else {
      // 用户已经输入了扩展名，直接使用
      finalName = trimmedName;
    }
  }

  // 如果名称没有变化，不触发重命名
  if (finalName === props.item.name) {
    emit('rename-complete', props.item.id, '');
    return;
  }

  // 触发重命名完成事件
  emit('rename-complete', props.item.id, finalName);
}

// 处理重命名取消
function handleRenameCancel() {
  // 通过发送空字符串来取消编辑
  emit('rename-complete', props.item.id, '');
}

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

.item-name-input {
  flex: 1;
  padding: 2px 4px;
  border: 1px solid #2196f3;
  border-radius: 2px;
  font-size: 14px;
  font-family: inherit;
  outline: none;
  background-color: #ffffff;
  color: #212121;
}

.item-name-input:focus {
  border-color: #1976d2;
  box-shadow: 0 0 0 1px rgba(33, 150, 243, 0.2);
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

