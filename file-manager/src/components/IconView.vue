<template>
  <div
    ref="iconViewRef"
    class="icon-view"
    @mousedown="handleMouseDown"
    @mousemove="handleMouseMove"
    @mouseup="handleMouseUp"
    @mouseleave="handleMouseLeave"
  >
    <div class="icon-grid">
      <div
        v-for="item in items"
        :key="item.id"
        :ref="(el) => setItemRef(el, item.id)"
        class="icon-item"
        :class="{
          'is-folder': item.file_type === 'folder',
          'is-selected': selectedItemIdsSet.has(item.id)
        }"
        @click="(e) => handleItemClick(item, e)"
        @dblclick="() => handleItemDoubleClick(item)"
      >
        <div class="icon-item-icon">
          <img
            v-if="getThumbnailForItem(item)"
            :src="getThumbnailForItem(item)"
            class="icon-item-thumbnail"
            :alt="item.name"
            draggable="false"
          />
          <span v-else>{{ iconChar(item) }}</span>
        </div>
        <div v-if="!isEditingItem(item.id)" class="icon-item-name">{{ item.name }}</div>
        <input
          v-else
          :ref="(el) => setInputRef(el, item.id)"
          :value="editingName"
          class="icon-item-name-input"
          @input="(e) => handleNameInput(e, item)"
          @keyup.enter="handleRenameComplete(item)"
          @keyup.esc="handleRenameCancel(item)"
          @blur="handleRenameCancel(item)"
        />
      </div>
    </div>

    <!-- 选择框 -->
    <div
      v-if="isSelecting"
      class="selection-box"
      :style="selectionBoxStyle"
    ></div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, nextTick, onUnmounted, watch } from 'vue';
import type { FileItem } from '../types/file';
import { getIconChar, getFileIcon } from '../utils/icons';
import { getThumbnailUrl, isImageFile } from '../utils/thumbnails';

const props = defineProps<{
  items: FileItem[];
  selectedItemIds?: Set<string> | string[];
  editingItemId?: string | null;
}>();

const emit = defineEmits<{
  'item-click': [item: FileItem, event: MouseEvent];
  'item-double-click': [item: FileItem];
  'selection-change': [selectedIds: Set<string>];
  'rename-complete': [itemId: string, newName: string];
}>();

// 容器引用
const iconViewRef = ref<HTMLDivElement | null>(null);

// 文件项引用映射
const itemRefs = ref<Map<string, HTMLElement>>(new Map());

// 缩略图 URL（item.id -> blob url）
const thumbnailUrls = ref<Map<string, string>>(new Map());

// 输入框引用映射
const inputRefs = ref<Map<string, HTMLInputElement>>(new Map());

// 编辑相关状态
const editingName = ref('');

// 判断项目是否正在编辑
function isEditingItem(itemId: string): boolean {
  return props.editingItemId === itemId;
}

// 设置输入框引用
function setInputRef(el: any, id: string) {
  if (el) {
    const element = el.$el || el;
    if (element instanceof HTMLInputElement) {
      inputRefs.value.set(id, element);
    }
  } else {
    inputRefs.value.delete(id);
  }
}

// 监听编辑状态变化
watch(() => props.editingItemId, (newEditingId) => {
  if (newEditingId) {
    const item = props.items.find(i => i.id === newEditingId);
    if (item) {
      editingName.value = getFileNameWithoutExtension(item.name, item);
      nextTick(() => {
        const input = inputRefs.value.get(newEditingId);
        if (input) {
          input.focus();
          input.select();
        }
      });
    }
  } else {
    editingName.value = '';
  }
});

// 获取不含扩展名的文件名
function getFileNameWithoutExtension(fileName: string, item: FileItem): string {
  const lastDotIndex = fileName.lastIndexOf('.');
  if (lastDotIndex === -1) {
    return fileName;
  }
  // 如果是文件夹或者第一个字符就是点（隐藏文件），返回完整文件名
  if (item.file_type === 'folder' || fileName.startsWith('.')) {
    return fileName;
  }
  return fileName.substring(0, lastDotIndex);
}

// 获取文件扩展名
function getFileExtension(fileName: string): string {
  const lastDotIndex = fileName.lastIndexOf('.');
  if (lastDotIndex === -1 || lastDotIndex === 0) {
    return '';
  }
  return fileName.substring(lastDotIndex);
}

// 处理名称输入
function handleNameInput(event: Event, item: FileItem) {
  const target = event.target as HTMLInputElement;
  editingName.value = target.value;
}

// 处理重命名完成
function handleRenameComplete(item: FileItem) {
  const trimmedName = editingName.value.trim();

  if (!trimmedName) {
    handleRenameCancel(item);
    return;
  }

  let finalName = trimmedName;
  if (item.file_type === 'file' && item.extension) {
    const extension = getFileExtension(trimmedName);
    if (!extension) {
      finalName = trimmedName + '.' + item.extension;
    } else {
      finalName = trimmedName;
    }
  }

  if (finalName === item.name) {
    emit('rename-complete', item.id, '');
    return;
  }

  emit('rename-complete', item.id, finalName);
}

// 处理重命名取消
function handleRenameCancel(item: FileItem) {
  emit('rename-complete', item.id, '');
}

// 将 props 中的 selectedItemIds 转换为 Set
const selectedItemIdsSet = computed(() => {
  if (props.selectedItemIds instanceof Set) {
    return props.selectedItemIds;
  } else if (Array.isArray(props.selectedItemIds)) {
    return new Set(props.selectedItemIds);
  }
  return new Set<string>();
});

// 框选相关状态
const isSelecting = ref(false);
const selectionStart = ref<{ x: number; y: number } | null>(null);
const selectionEnd = ref<{ x: number; y: number } | null>(null);
const dragThreshold = 5; // 拖拽阈值（像素）
const isDragging = ref(false);
const initialSelectionIds = ref<Set<string>>(new Set());

// 设置文件项引用
function setItemRef(el: any, id: string) {
  if (el) {
    const element = el.$el || el;
    if (element instanceof HTMLElement) {
      itemRefs.value.set(id, element);
    }
  } else {
    itemRefs.value.delete(id);
  }
}

// 获取图标字符
function iconChar(item: FileItem): string {
  return getIconChar(getFileIcon(item));
}

async function ensureThumbnailLoaded(item: FileItem) {
  if (!isImageFile(item)) {
    thumbnailUrls.value.delete(item.id);
    return;
  }

  if (thumbnailUrls.value.has(item.id)) {
    return;
  }

  try {
    console.log('[IconView] 加载缩略图:', item.path, item.extension);
    const url = await getThumbnailUrl(item.path, item.extension);
    // console.log('[IconView] 缩略图加载成功:', item.id, url);
    thumbnailUrls.value.set(item.id, url);
  } catch (error) {
    // 读取失败则回退到默认图标
    console.error('[IconView] 缩略图加载失败:', item.path, error);
    thumbnailUrls.value.delete(item.id);
  }
}

watch(
  () => props.items,
  (items) => {
    // 删除已不存在的 item 缓存（避免 Map 无限增长）
    const ids = new Set(items.map(i => i.id));
    Array.from(thumbnailUrls.value.keys()).forEach((id) => {
      if (!ids.has(id)) {
        thumbnailUrls.value.delete(id);
      }
    });

    // 预加载图片缩略图（异步，不阻塞渲染）
    items.forEach((item) => {
      void ensureThumbnailLoaded(item);
    });
  },
  { immediate: true }
);

function getThumbnailForItem(item: FileItem): string | undefined {
  const val = thumbnailUrls.value.get(item.id);
  if (!val && isImageFile(item)) {
    console.log('[IconView] 缩略图未加载:', item.id, item.path);
  }
  return val;
}

onUnmounted(() => {
  thumbnailUrls.value.clear();
});

// 处理文件项单击
function handleItemClick(item: FileItem, event: MouseEvent) {
  emit('item-click', item, event);
}

// 处理文件项双击
function handleItemDoubleClick(item: FileItem) {
  emit('item-double-click', item);
}

// 计算选择框样式
const selectionBoxStyle = computed(() => {
  if (!selectionStart.value || !selectionEnd.value || !iconViewRef.value) {
    return {};
  }

  const containerRect = iconViewRef.value.getBoundingClientRect();
  const scrollLeft = iconViewRef.value.scrollLeft;
  const scrollTop = iconViewRef.value.scrollTop;

  const startX = selectionStart.value.x - containerRect.left + scrollLeft;
  const startY = selectionStart.value.y - containerRect.top + scrollTop;
  const endX = selectionEnd.value.x - containerRect.left + scrollLeft;
  const endY = selectionEnd.value.y - containerRect.top + scrollTop;

  const left = Math.min(startX, endX);
  const top = Math.min(startY, endY);
  const width = Math.abs(endX - startX);
  const height = Math.abs(endY - startY);

  return {
    left: `${left}px`,
    top: `${top}px`,
    width: `${width}px`,
    height: `${height}px`,
  };
});

// 检测文件项是否与选择框相交
function isItemIntersectingSelectionBox(itemElement: HTMLElement): boolean {
  if (!selectionStart.value || !selectionEnd.value || !iconViewRef.value) {
    return false;
  }

  const containerRect = iconViewRef.value.getBoundingClientRect();
  const scrollLeft = iconViewRef.value.scrollLeft;
  const scrollTop = iconViewRef.value.scrollTop;

  const startX = selectionStart.value.x - containerRect.left + scrollLeft;
  const startY = selectionStart.value.y - containerRect.top + scrollTop;
  const endX = selectionEnd.value.x - containerRect.left + scrollLeft;
  const endY = selectionEnd.value.y - containerRect.top + scrollTop;

  const selectionLeft = Math.min(startX, endX);
  const selectionTop = Math.min(startY, endY);
  const selectionRight = Math.max(startX, endX);
  const selectionBottom = Math.max(startY, endY);

  const itemRect = itemElement.getBoundingClientRect();
  const itemLeft = itemRect.left - containerRect.left + scrollLeft;
  const itemTop = itemRect.top - containerRect.top + scrollTop;
  const itemRight = itemLeft + itemRect.width;
  const itemBottom = itemTop + itemRect.height;

  return !(
    selectionRight < itemLeft ||
    selectionLeft > itemRight ||
    selectionBottom < itemTop ||
    selectionTop > itemBottom
  );
}

// 更新选中状态（基于选择框）
function updateSelectionFromBox(event?: MouseEvent) {
  if (!isSelecting.value || !iconViewRef.value) {
    return;
  }

  const isCtrlPressed = event?.ctrlKey || event?.metaKey;
  const selectedIds = new Set<string>();

  if (isCtrlPressed) {
    initialSelectionIds.value.forEach(id => selectedIds.add(id));
  }

  props.items.forEach((item) => {
    const itemElement = itemRefs.value.get(item.id);
    if (itemElement && isItemIntersectingSelectionBox(itemElement)) {
      selectedIds.add(item.id);
    }
  });

  emit('selection-change', selectedIds);
}

// 处理鼠标按下
function handleMouseDown(event: MouseEvent) {
  if (event.button !== 0) {
    return;
  }

  const target = event.target as HTMLElement;
  if (target.closest('.icon-item')) {
    return;
  }

  const isCtrlPressed = event.ctrlKey || event.metaKey;
  if (isCtrlPressed) {
    initialSelectionIds.value = new Set(selectedItemIdsSet.value);
  } else {
    initialSelectionIds.value = new Set();
  }

  isSelecting.value = true;
  isDragging.value = false;
  selectionStart.value = { x: event.clientX, y: event.clientY };
  selectionEnd.value = { x: event.clientX, y: event.clientY };

  event.preventDefault();
  event.stopPropagation();
}

// 处理鼠标移动
function handleMouseMove(event: MouseEvent) {
  if (!isSelecting.value || !selectionStart.value) {
    return;
  }

  selectionEnd.value = { x: event.clientX, y: event.clientY };

  const dx = selectionEnd.value.x - selectionStart.value.x;
  const dy = selectionEnd.value.y - selectionStart.value.y;
  const distance = Math.sqrt(dx * dx + dy * dy);

  if (distance >= dragThreshold) {
    isDragging.value = true;
    updateSelectionFromBox(event);
  }

  if (isDragging.value) {
    event.preventDefault();
  }
}

// 处理鼠标释放
function handleMouseUp(event: MouseEvent) {
  if (!isSelecting.value) {
    return;
  }

  const wasDragging = isDragging.value;

  isSelecting.value = false;
  isDragging.value = false;

  if (selectionStart.value && selectionEnd.value && !wasDragging) {
    const dx = selectionEnd.value.x - selectionStart.value.x;
    const dy = selectionEnd.value.y - selectionStart.value.y;
    const distance = Math.sqrt(dx * dx + dy * dy);

    if (distance < dragThreshold) {
      const isCtrlPressed = event.ctrlKey || event.metaKey;
      if (!isCtrlPressed) {
        emit('selection-change', new Set<string>());
      }
    }
  }

  nextTick(() => {
    selectionStart.value = null;
    selectionEnd.value = null;
    initialSelectionIds.value = new Set();
  });

  if (wasDragging) {
    event.preventDefault();
  }
}

// 处理鼠标离开
function handleMouseLeave(event: MouseEvent) {
  // 保持选择框状态
}

// 滚动到指定文件项
function scrollToItem(itemId: string) {
  nextTick(() => {
    const itemElement = itemRefs.value.get(itemId);
    if (itemElement && iconViewRef.value) {
      const containerRect = iconViewRef.value.getBoundingClientRect();
      const itemRect = itemElement.getBoundingClientRect();

      const scrollTop = iconViewRef.value.scrollTop;
      const itemOffsetTop = itemRect.top - containerRect.top + scrollTop;

      iconViewRef.value.scrollTo({
        top: itemOffsetTop - 10,
        behavior: 'smooth',
      });
    }
  });
}

// 暴露方法给父组件
defineExpose({
  scrollToItem,
});
</script>

<style scoped>
.icon-view {
  flex: 1;
  overflow: auto;
  position: relative;
  user-select: none;
  padding: 16px;
}

.icon-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
  gap: 16px;
  align-items: start;
}

.icon-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 12px 8px;
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.2s;
  position: relative;
}

.icon-item:hover {
  background-color: #f5f5f5;
}

.icon-item.is-selected {
  background-color: #e3f2fd;
  border: 2px solid #2196f3;
  padding: 10px 6px;
}

.icon-item.is-selected:hover {
  background-color: #bbdefb;
}

.icon-item-icon {
  width: 64px;
  height: 64px;
  font-size: 48px;
  margin-bottom: 8px;
  user-select: none;
  display: flex;
  align-items: center;
  justify-content: center;
}

.icon-item-thumbnail {
  width: 64px;
  height: 64px;
  object-fit: cover;
  border-radius: 6px;
  user-select: none;
}

.icon-item-name {
  font-size: 12px;
  text-align: center;
  word-break: break-word;
  max-width: 100%;
  overflow: hidden;
  text-overflow: ellipsis;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  line-height: 1.4;
  color: #212121;
}

.icon-item.is-folder .icon-item-name {
  font-weight: 500;
}

.icon-item-name-input {
  font-size: 12px;
  text-align: center;
  width: 100%;
  padding: 2px 4px;
  border: 1px solid #2196f3;
  border-radius: 2px;
  font-family: inherit;
  outline: none;
  background-color: #ffffff;
  color: #212121;
  max-width: 100%;
}

.icon-item-name-input:focus {
  border-color: #1976d2;
  box-shadow: 0 0 0 1px rgba(33, 150, 243, 0.2);
}

/* 选择框样式 */
.selection-box {
  position: absolute;
  border: 1px solid #2196f3;
  background-color: rgba(33, 150, 243, 0.1);
  pointer-events: none;
  z-index: 10;
}
</style>
