<template>
  <div class="file-list">
    <div class="list-header">
      <div class="header-cell name-cell">名称</div>
      <div class="header-cell date-cell">修改日期</div>
      <div class="header-cell type-cell">类型</div>
      <div class="header-cell size-cell">大小</div>
    </div>

    <div
      ref="listBodyRef"
      class="list-body"
      @mousedown="handleMouseDown"
      @mousemove="handleMouseMove"
      @mouseup="handleMouseUp"
      @mouseleave="handleMouseLeave"
    >
      <FileItem
        v-for="item in items"
        :key="item.id"
        :ref="(el) => setItemRef(el, item.id)"
        :item="item"
        :is-selected="selectedItemIdsSet.has(item.id)"
        :is-editing="editingItemId === item.id"
        @click="(item, event) => handleItemClick(item, event)"
        @dblclick="(item) => handleItemDoubleClick(item)"
        @rename-complete="(itemId, newName) => $emit('rename-complete', itemId, newName)"
      />

      <!-- 选择框 -->
      <div
        v-if="isSelecting"
        class="selection-box"
        :style="selectionBoxStyle"
      ></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, nextTick, onUnmounted } from 'vue';
import FileItem from './FileItem.vue';
import type { FileItem as FileItemType } from '../types/file';

const props = defineProps<{
  items: FileItemType[];
  selectedItemIds?: Set<string> | string[];
  editingItemId?: string | null;
}>();

const emit = defineEmits<{
  'item-click': [item: FileItemType, event: MouseEvent];
  'item-double-click': [item: FileItemType];
  'selection-change': [selectedIds: Set<string>];
  'rename-complete': [itemId: string, newName: string];
}>();

// 列表容器引用
const listBodyRef = ref<HTMLDivElement | null>(null);

// 文件项引用映射
const itemRefs = ref<Map<string, HTMLElement>>(new Map());

// 框选相关状态
const isSelecting = ref(false);
const selectionStart = ref<{ x: number; y: number } | null>(null);
const selectionEnd = ref<{ x: number; y: number } | null>(null);
const dragThreshold = 5; // 拖拽阈值（像素），超过此值才认为是拖拽而非点击
const isDragging = ref(false); // 是否正在拖拽（用于区分点击和拖拽）
const initialSelectionIds = ref<Set<string>>(new Set()); // 框选开始时的选中项

// 设置文件项引用
function setItemRef(el: any, id: string) {
  if (el) {
    // Vue 3 组件实例有 $el 属性，普通元素直接使用
    const element = el.$el || el;
    if (element instanceof HTMLElement) {
      itemRefs.value.set(id, element);
    }
  } else {
    // 元素被卸载时，移除引用
    itemRefs.value.delete(id);
  }
}

// 将 props 中的 selectedItemIds 转换为 Set（如果是数组的话）
const selectedItemIdsSet = computed(() => {
  if (props.selectedItemIds instanceof Set) {
    return props.selectedItemIds;
  } else if (Array.isArray(props.selectedItemIds)) {
    return new Set(props.selectedItemIds);
  }
  return new Set<string>();
});

// 当前正在编辑的文件项 ID
const editingItemId = computed(() => props.editingItemId ?? null);

function handleItemClick(item: FileItemType, event: MouseEvent) {
  emit('item-click', item, event);
}

function handleItemDoubleClick(item: FileItemType) {
  emit('item-double-click', item);
}

// 计算选择框样式
const selectionBoxStyle = computed(() => {
  if (!selectionStart.value || !selectionEnd.value || !listBodyRef.value) {
    return {};
  }

  const containerRect = listBodyRef.value.getBoundingClientRect();
  const scrollLeft = listBodyRef.value.scrollLeft;
  const scrollTop = listBodyRef.value.scrollTop;

  // 计算相对于容器左上角的位置（考虑滚动）
  const startX = selectionStart.value.x - containerRect.left + scrollLeft;
  const startY = selectionStart.value.y - containerRect.top + scrollTop;
  const endX = selectionEnd.value.x - containerRect.left + scrollLeft;
  const endY = selectionEnd.value.y - containerRect.top + scrollTop;

  // 计算选择框的位置和尺寸
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
  if (!selectionStart.value || !selectionEnd.value || !listBodyRef.value) {
    return false;
  }

  const containerRect = listBodyRef.value.getBoundingClientRect();
  const scrollLeft = listBodyRef.value.scrollLeft;
  const scrollTop = listBodyRef.value.scrollTop;

  // 计算选择框在容器坐标系中的位置
  const startX = selectionStart.value.x - containerRect.left + scrollLeft;
  const startY = selectionStart.value.y - containerRect.top + scrollTop;
  const endX = selectionEnd.value.x - containerRect.left + scrollLeft;
  const endY = selectionEnd.value.y - containerRect.top + scrollTop;

  const selectionLeft = Math.min(startX, endX);
  const selectionTop = Math.min(startY, endY);
  const selectionRight = Math.max(startX, endX);
  const selectionBottom = Math.max(startY, endY);

  // 获取文件项在容器坐标系中的位置
  const itemRect = itemElement.getBoundingClientRect();
  const itemLeft = itemRect.left - containerRect.left + scrollLeft;
  const itemTop = itemRect.top - containerRect.top + scrollTop;
  const itemRight = itemLeft + itemRect.width;
  const itemBottom = itemTop + itemRect.height;

  // 检测矩形是否相交
  return !(
    selectionRight < itemLeft ||
    selectionLeft > itemRight ||
    selectionBottom < itemTop ||
    selectionTop > itemBottom
  );
}

// 更新选中状态（基于选择框）
function updateSelectionFromBox(event?: MouseEvent) {
  if (!isSelecting.value || !listBodyRef.value) {
    return;
  }

  const isCtrlPressed = event?.ctrlKey || event?.metaKey;
  const selectedIds = new Set<string>();

  // 如果按住Ctrl键，从初始选中项开始；否则清空
  if (isCtrlPressed) {
    initialSelectionIds.value.forEach(id => selectedIds.add(id));
  }

  // 遍历所有文件项，检测哪些与选择框相交
  props.items.forEach((item) => {
    const itemElement = itemRefs.value.get(item.id);
    if (itemElement && isItemIntersectingSelectionBox(itemElement)) {
      selectedIds.add(item.id);
    }
  });

  // 触发选择变更事件
  emit('selection-change', selectedIds);
}

// 处理鼠标按下
function handleMouseDown(event: MouseEvent) {
  // 只处理左键点击
  if (event.button !== 0) {
    return;
  }

  // 如果点击在文件项上，不要立即开始框选（避免干扰点击选择）
  const target = event.target as HTMLElement;
  if (target.closest('.file-item')) {
    return;
  }

  // 记录初始选中状态（用于Ctrl+框选）
  const isCtrlPressed = event.ctrlKey || event.metaKey;
  if (isCtrlPressed) {
    initialSelectionIds.value = new Set(selectedItemIdsSet.value);
  } else {
    initialSelectionIds.value = new Set();
  }

  // 开始框选
  isSelecting.value = true;
  isDragging.value = false;
  selectionStart.value = { x: event.clientX, y: event.clientY };
  selectionEnd.value = { x: event.clientX, y: event.clientY };

  // 阻止默认行为和事件冒泡
  event.preventDefault();
  event.stopPropagation();
}

// 处理鼠标移动
function handleMouseMove(event: MouseEvent) {
  if (!isSelecting.value || !selectionStart.value) {
    return;
  }

  // 更新选择框的结束位置
  selectionEnd.value = { x: event.clientX, y: event.clientY };

  // 检查是否达到拖拽阈值
  const dx = selectionEnd.value.x - selectionStart.value.x;
  const dy = selectionEnd.value.y - selectionStart.value.y;
  const distance = Math.sqrt(dx * dx + dy * dy);

  if (distance >= dragThreshold) {
    isDragging.value = true;
    // 更新选中状态
    updateSelectionFromBox(event);
  }

  // 阻止默认行为（仅当正在拖拽时）
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

  // 结束框选
  isSelecting.value = false;
  isDragging.value = false;

  // 如果移动距离很小且未拖拽，可能是点击空白处，清除选中
  if (selectionStart.value && selectionEnd.value && !wasDragging) {
    const dx = selectionEnd.value.x - selectionStart.value.x;
    const dy = selectionEnd.value.y - selectionStart.value.y;
    const distance = Math.sqrt(dx * dx + dy * dy);

    if (distance < dragThreshold) {
      // 点击空白处，清除选中（除非按住了Ctrl）
      const isCtrlPressed = event.ctrlKey || event.metaKey;
      if (!isCtrlPressed) {
        emit('selection-change', new Set<string>());
      }
    }
  }

  // 延迟清除选择框（让用户看到最终状态）
  nextTick(() => {
    selectionStart.value = null;
    selectionEnd.value = null;
    initialSelectionIds.value = new Set();
  });

  // 阻止默认行为（仅当进行了拖拽时）
  if (wasDragging) {
    event.preventDefault();
  }
}

// 处理鼠标离开列表区域
function handleMouseLeave(event: MouseEvent) {
  if (isSelecting.value) {
    // 鼠标离开时，保持当前选择框状态，但不再更新
    // 如果鼠标重新进入，会继续更新
    // 这里可以选择不清除选择框，让用户看到当前的选择状态
  }
}

// 全局鼠标事件处理（用于处理鼠标移出窗口后释放的情况）
function handleGlobalMouseUp(event: MouseEvent) {
  if (isSelecting.value) {
    handleMouseUp(event);
  }
}

// 组件挂载时添加全局事件监听
nextTick(() => {
  window.addEventListener('mouseup', handleGlobalMouseUp);
});

// 组件卸载时清理
onUnmounted(() => {
  window.removeEventListener('mouseup', handleGlobalMouseUp);
});

// 滚动到指定文件项
function scrollToItem(itemId: string) {
  nextTick(() => {
    const itemElement = itemRefs.value.get(itemId);
    if (itemElement && listBodyRef.value) {
      // 计算元素相对于滚动容器的位置
      const containerRect = listBodyRef.value.getBoundingClientRect();
      const itemRect = itemElement.getBoundingClientRect();

      // 计算需要滚动的距离
      const scrollTop = listBodyRef.value.scrollTop;
      const itemOffsetTop = itemRect.top - containerRect.top + scrollTop;

      // 滚动到该位置，并留出一些边距
      listBodyRef.value.scrollTo({
        top: itemOffsetTop - 10, // 向上留出10px边距
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
  position: relative;
  user-select: none;
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

