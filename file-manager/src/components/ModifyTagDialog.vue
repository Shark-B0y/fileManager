<template>
  <div class="modify-tag-dialog-overlay" @click.self="handleClose">
    <div class="modify-tag-dialog">
      <div class="modify-tag-dialog-header">
        <h3>修改标签</h3>
        <button class="close-button" @click="handleClose">×</button>
      </div>

      <div class="modify-tag-dialog-content">
        <!-- 左侧表单区域 -->
        <div class="modify-tag-form">
          <!-- 标签名 -->
          <div class="form-item">
            <label class="form-label">标签名</label>
            <div class="form-input-wrapper">
              <input
                v-model="formData.name"
                class="form-input"
                type="text"
                placeholder="输入标签名称"
                @input="handleNameInput"
              />
              <div v-if="nameSuggestions.length > 0" class="suggestions-list">
                <div
                  v-for="tag in nameSuggestions"
                  :key="tag.id"
                  class="suggestion-item"
                  @click="selectTagName(tag)"
                >
                  {{ tag.name }}
                </div>
              </div>
            </div>
          </div>

          <!-- 字体颜色 -->
          <div class="form-item">
            <label class="form-label">字体颜色</label>
            <div class="form-input-wrapper">
              <input
                v-model="formData.font_color"
                class="form-input"
                type="text"
                placeholder="输入颜色代码（如#000000）"
                @focus="activeColorInput = 'font_color'"
                @blur="handleColorInputBlur"
              />
            </div>
          </div>

          <!-- 背景颜色 -->
          <div class="form-item">
            <label class="form-label">背景颜色</label>
            <div class="form-input-wrapper">
              <input
                v-model="formData.color"
                class="form-input"
                type="text"
                placeholder="输入颜色代码（如#FFFF00）"
                @focus="activeColorInput = 'color'"
                @blur="handleColorInputBlur"
              />
            </div>
          </div>

          <!-- 父级标签 -->
          <div class="form-item">
            <label class="form-label">父级标签</label>
            <div class="form-input-wrapper">
              <input
                v-model="parentTagName"
                class="form-input"
                type="text"
                placeholder="输入父级标签名称（留空表示无父标签）"
                @input="handleParentTagInput"
              />
              <div v-if="parentTagSuggestions.length > 0" class="suggestions-list">
                <div
                  v-for="tag in parentTagSuggestions"
                  :key="tag.id"
                  class="suggestion-item"
                  @click="selectParentTag(tag)"
                >
                  {{ tag.name }}
                </div>
                <div
                  class="suggestion-item"
                  @click="clearParentTag"
                  style="color: #999; font-style: italic;"
                >
                  清除父标签
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- 右侧颜色选择器 -->
        <div class="color-picker-container" @mousedown.prevent>
          <div v-if="activeColorInput" class="color-picker-wrapper">
            <canvas
              ref="colorPickerCanvas"
              class="color-picker-canvas"
              @mousedown="handleCanvasMouseDown"
              @mousemove="handleCanvasMouseMove"
              @mouseup="handleCanvasMouseUp"
              @mouseleave="handleCanvasMouseLeave"
            ></canvas>
            <div v-if="hoveredColor" class="color-preview" :style="{ backgroundColor: hoveredColor }">
              {{ hoveredColor }}
            </div>
          </div>
          <div v-else class="color-picker-placeholder">
            点击颜色输入框以选择颜色
          </div>
        </div>
      </div>

      <div class="modify-tag-dialog-footer">
        <button class="modify-button" @click="handleModify">修改</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch, nextTick } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { Tag } from '../types/tag';

const props = defineProps<{
  tag: Tag | null;
}>();

const emit = defineEmits<{
  close: [];
  modified: [];
}>();

const formData = ref({
  name: '',
  color: '',
  font_color: '',
  parent_id: null as number | null,
});

const parentTagName = ref('');
const activeColorInput = ref<'color' | 'font_color' | null>(null);
const nameSuggestions = ref<Tag[]>([]);
const parentTagSuggestions = ref<Tag[]>([]);
const colorPickerCanvas = ref<HTMLCanvasElement | null>(null);
const hoveredColor = ref<string>('');
const isDragging = ref(false);

// Canvas尺寸
const canvasSize = 200;
const centerX = canvasSize / 2;
const centerY = canvasSize / 2;
const radius = canvasSize / 2 - 10;

// 初始化表单数据
onMounted(async () => {
  if (props.tag) {
    formData.value = {
      name: props.tag.name,
      color: props.tag.color || '',
      font_color: props.tag.font_color || '',
      parent_id: props.tag.parent_id,
    };
    // 如果有父标签，需要查找父标签名称
    if (props.tag.parent_id) {
      loadParentTagName(props.tag.parent_id);
    } else {
      parentTagName.value = '';
    }
  }

  // 等待DOM更新后绘制Canvas
  await nextTick();
  if (activeColorInput.value && colorPickerCanvas.value) {
    drawColorWheel();
  }
});

// 监听tag变化
watch(() => props.tag, (newTag) => {
  if (newTag) {
    formData.value = {
      name: newTag.name,
      color: newTag.color || '',
      font_color: newTag.font_color || '',
      parent_id: newTag.parent_id,
    };
    if (newTag.parent_id) {
      loadParentTagName(newTag.parent_id);
    } else {
      parentTagName.value = '';
    }
    // 清空建议列表
    nameSuggestions.value = [];
    parentTagSuggestions.value = [];
  }
}, { immediate: true });

// 监听activeColorInput变化，重新绘制Canvas
watch(activeColorInput, async (newValue) => {
  if (newValue) {
    await nextTick();
    if (colorPickerCanvas.value) {
      drawColorWheel();
    }
  } else {
    hoveredColor.value = '';
    isDragging.value = false;
  }
});

// 加载父标签名称
async function loadParentTagName(parentId: number) {
  try {
    // 这里需要根据ID获取标签，暂时使用搜索来查找
    const tags = await invoke<Tag[]>('search_tags', {
      keyword: '',
      limit: 100,
    });
    const parentTag = tags.find(t => t.id === parentId);
    if (parentTag) {
      parentTagName.value = parentTag.name;
    }
  } catch (error) {
    console.error('加载父标签名称失败:', error);
  }
}

// 处理标签名输入
async function handleNameInput() {
  const keyword = formData.value.name.trim();
  if (keyword.length === 0) {
    nameSuggestions.value = [];
    return;
  }

  try {
    const tags = await invoke<Tag[]>('search_tags', {
      keyword: keyword,
      limit: 10,
    });
    // 过滤掉当前标签
    nameSuggestions.value = tags.filter(t => !props.tag || t.id !== props.tag.id);
  } catch (error) {
    console.error('搜索标签失败:', error);
    nameSuggestions.value = [];
  }
}

// 选择标签名
function selectTagName(tag: Tag) {
  formData.value.name = tag.name;
  nameSuggestions.value = [];
}

// 处理父级标签输入
async function handleParentTagInput() {
  const keyword = parentTagName.value.trim();
  if (keyword.length === 0) {
    parentTagSuggestions.value = [];
    return;
  }

  try {
    const tags = await invoke<Tag[]>('search_tags', {
      keyword: keyword,
      limit: 10,
    });
    // 过滤掉当前标签（不能选择自己作为父标签）
    parentTagSuggestions.value = tags.filter(t => !props.tag || t.id !== props.tag.id);
  } catch (error) {
    console.error('搜索父标签失败:', error);
    parentTagSuggestions.value = [];
  }
}

// 选择父级标签
function selectParentTag(tag: Tag) {
  parentTagName.value = tag.name;
  formData.value.parent_id = tag.id;
  parentTagSuggestions.value = [];
}

// 清除父标签
function clearParentTag() {
  parentTagName.value = '';
  formData.value.parent_id = null;
  parentTagSuggestions.value = [];
}

// 绘制圆形色相环
function drawColorWheel() {
  const canvas = colorPickerCanvas.value;
  if (!canvas) return;

  const ctx = canvas.getContext('2d');
  if (!ctx) return;

  // 设置Canvas尺寸
  canvas.width = canvasSize;
  canvas.height = canvasSize;

  // 清空画布
  ctx.clearRect(0, 0, canvasSize, canvasSize);

  // 绘制圆形色相环
  const imageData = ctx.createImageData(canvasSize, canvasSize);
  const data = imageData.data;

  for (let y = 0; y < canvasSize; y++) {
    for (let x = 0; x < canvasSize; x++) {
      const dx = x - centerX;
      const dy = y - centerY;
      const distance = Math.sqrt(dx * dx + dy * dy);

      // 只绘制圆形内的像素
      if (distance <= radius) {
        const angle = Math.atan2(dy, dx);
        // 将角度转换为0-360度
        let hue = (angle * 180 / Math.PI + 180) % 360;

        // 计算饱和度：从中心到边缘，0%到100%
        const saturation = Math.min(100, (distance / radius) * 100);

        // 计算亮度：从中心到边缘，100%到50%（中心是白色，边缘是饱和色）
        const lightness = 100 - (distance / radius) * 50;

        // 将HSL转换为RGB
        const rgb = hslToRgb(hue / 360, saturation / 100, lightness / 100);

        const index = (y * canvasSize + x) * 4;
        data[index] = rgb.r;     // R
        data[index + 1] = rgb.g; // G
        data[index + 2] = rgb.b; // B
        data[index + 3] = 255;   // A
      }
    }
  }

  ctx.putImageData(imageData, 0, 0);

  // 绘制边框
  ctx.strokeStyle = '#e0e0e0';
  ctx.lineWidth = 2;
  ctx.beginPath();
  ctx.arc(centerX, centerY, radius, 0, Math.PI * 2);
  ctx.stroke();
}

// HSL转RGB
function hslToRgb(h: number, s: number, l: number): { r: number; g: number; b: number } {
  let r: number, g: number, b: number;

  if (s === 0) {
    r = g = b = l; // 无色彩，灰度
  } else {
    const hue2rgb = (p: number, q: number, t: number) => {
      if (t < 0) t += 1;
      if (t > 1) t -= 1;
      if (t < 1 / 6) return p + (q - p) * 6 * t;
      if (t < 1 / 2) return q;
      if (t < 2 / 3) return p + (q - p) * (2 / 3 - t) * 6;
      return p;
    };

    const q = l < 0.5 ? l * (1 + s) : l + s - l * s;
    const p = 2 * l - q;
    r = hue2rgb(p, q, h + 1 / 3);
    g = hue2rgb(p, q, h);
    b = hue2rgb(p, q, h - 1 / 3);
  }

  return {
    r: Math.round(r * 255),
    g: Math.round(g * 255),
    b: Math.round(b * 255),
  };
}

// 从Canvas坐标获取颜色
function getColorFromCanvas(x: number, y: number): string | null {
  const canvas = colorPickerCanvas.value;
  if (!canvas) return null;

  const ctx = canvas.getContext('2d');
  if (!ctx) return null;

  // 计算相对于Canvas实际尺寸的坐标
  const rect = canvas.getBoundingClientRect();
  const scaleX = canvas.width / rect.width;
  const scaleY = canvas.height / rect.height;
  const canvasX = Math.floor(x * scaleX);
  const canvasY = Math.floor(y * scaleY);

  // 确保坐标在Canvas范围内
  if (canvasX < 0 || canvasX >= canvas.width || canvasY < 0 || canvasY >= canvas.height) {
    return null;
  }

  const imageData = ctx.getImageData(canvasX, canvasY, 1, 1);
  const [r, g, b] = imageData.data;

  // 转换为HEX格式
  const toHex = (n: number) => {
    const hex = n.toString(16);
    return hex.length === 1 ? '0' + hex : hex;
  };

  return `#${toHex(r)}${toHex(g)}${toHex(b)}`;
}

// 处理Canvas鼠标按下
function handleCanvasMouseDown(event: MouseEvent) {
  isDragging.value = true;
  const canvas = colorPickerCanvas.value;
  if (!canvas) return;

  const rect = canvas.getBoundingClientRect();
  const x = event.clientX - rect.left;
  const y = event.clientY - rect.top;

  const dx = x - centerX;
  const dy = y - centerY;
  const distance = Math.sqrt(dx * dx + dy * dy);

  // 只处理圆形内的点击
  if (distance <= radius) {
    const color = getColorFromCanvas(x, y);
    if (color) {
      selectColor(color);
    }
  }
}

// 处理Canvas鼠标移动
function handleCanvasMouseMove(event: MouseEvent) {
  const canvas = colorPickerCanvas.value;
  if (!canvas) return;

  const rect = canvas.getBoundingClientRect();
  const x = event.clientX - rect.left;
  const y = event.clientY - rect.top;

  const dx = x - centerX;
  const dy = y - centerY;
  const distance = Math.sqrt(dx * dx + dy * dy);

  // 只处理圆形内的移动
  if (distance <= radius) {
    const color = getColorFromCanvas(x, y);
    if (color) {
      hoveredColor.value = color;
    }

    // 如果正在拖拽，也选择颜色
    if (isDragging.value && color) {
      selectColor(color);
    }
  } else {
    hoveredColor.value = '';
  }
}

// 处理Canvas鼠标抬起
function handleCanvasMouseUp() {
  isDragging.value = false;
}

// 处理Canvas鼠标离开
function handleCanvasMouseLeave() {
  isDragging.value = false;
  hoveredColor.value = '';
}

// 处理颜色输入框失焦
function handleColorInputBlur() {
  // 延迟隐藏颜色选择器，以便点击颜色时不会立即隐藏
  setTimeout(() => {
    // 只有在没有点击颜色选择器时才隐藏
    if (activeColorInput.value && !isDragging.value) {
      activeColorInput.value = null;
    }
  }, 200);
}

// 选择颜色
function selectColor(color: string) {
  if (activeColorInput.value === 'color') {
    formData.value.color = color;
  } else if (activeColorInput.value === 'font_color') {
    formData.value.font_color = color;
  }
  hoveredColor.value = color;

  // 延迟清除，确保颜色已设置
  setTimeout(() => {
    if (!isDragging.value) {
      activeColorInput.value = null;
    }
  }, 100);
}

// 处理修改
async function handleModify() {
  if (!props.tag) {
    return;
  }

  try {
    // 构建修改参数
    const modifyParams: any = {
      id: props.tag.id,
    };

    // 如果名称有变化，添加名称参数
    if (formData.value.name !== props.tag.name) {
      modifyParams.name = formData.value.name.trim();
    }

    // 如果颜色有变化，添加颜色参数
    if (formData.value.color !== (props.tag.color || '')) {
      modifyParams.color = formData.value.color.trim() || null;
    }

    // 如果字体颜色有变化，添加字体颜色参数
    if (formData.value.font_color !== (props.tag.font_color || '')) {
      modifyParams.font_color = formData.value.font_color.trim() || null;
    }

    // 如果父标签有变化，添加父标签参数
    // 如果输入框为空，表示要清除父标签
    if (parentTagName.value.trim() === '' && props.tag.parent_id !== null) {
      modifyParams.parent_id = null;
    } else if (formData.value.parent_id !== props.tag.parent_id) {
      modifyParams.parent_id = formData.value.parent_id;
    }

    // 调用修改接口
    await invoke<Tag>('modify_tag', modifyParams);

    emit('modified');
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    window.dispatchEvent(
      new CustomEvent('show-global-error', {
        detail: { message: `修改标签失败: ${message}` },
      }),
    );
  }
}

// 处理关闭
function handleClose() {
  emit('close');
}
</script>

<style scoped>
.modify-tag-dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modify-tag-dialog {
  background-color: #ffffff;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  width: 600px;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
}

.modify-tag-dialog-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  border-bottom: 1px solid #e0e0e0;
}

.modify-tag-dialog-header h3 {
  margin: 0;
  font-size: 18px;
  font-weight: 500;
  color: #333;
}

.close-button {
  background: none;
  border: none;
  font-size: 24px;
  color: #999;
  cursor: pointer;
  padding: 0;
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  line-height: 1;
}

.close-button:hover {
  color: #333;
}

.modify-tag-dialog-content {
  display: flex;
  padding: 20px;
  gap: 20px;
  flex: 1;
  overflow-y: auto;
}

.modify-tag-form {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.form-item {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.form-label {
  font-size: 14px;
  font-weight: 500;
  color: #333;
}

.form-input-wrapper {
  position: relative;
}

.form-input {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid #d1d5db;
  border-radius: 4px;
  font-size: 14px;
  outline: none;
  box-sizing: border-box;
}

.form-input:focus {
  border-color: #2563eb;
  box-shadow: 0 0 0 1px rgba(37, 99, 235, 0.2);
}

.suggestions-list {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  margin-top: 4px;
  background-color: #ffffff;
  border: 1px solid #e0e0e0;
  border-radius: 4px;
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.1);
  max-height: 200px;
  overflow-y: auto;
  z-index: 100;
}

.suggestion-item {
  padding: 8px 12px;
  font-size: 14px;
  color: #333;
  cursor: pointer;
  transition: background-color 0.2s;
}

.suggestion-item:hover {
  background-color: #f5f5f5;
}

.color-picker-container {
  width: 200px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.color-picker-wrapper {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
}

.color-picker-canvas {
  width: 200px;
  height: 200px;
  border-radius: 50%;
  cursor: crosshair;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
}

.color-preview {
  padding: 6px 12px;
  background-color: #f5f5f5;
  border: 1px solid #e0e0e0;
  border-radius: 4px;
  font-size: 12px;
  font-family: monospace;
  color: #333;
  min-width: 80px;
  text-align: center;
}

.color-picker-placeholder {
  text-align: center;
  color: #999;
  font-size: 14px;
  padding: 20px;
}

.modify-tag-dialog-footer {
  padding: 16px 20px;
  border-top: 1px solid #e0e0e0;
  display: flex;
  justify-content: center;
}

.modify-button {
  padding: 8px 24px;
  background-color: #2563eb;
  color: #ffffff;
  border: none;
  border-radius: 4px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.2s;
}

.modify-button:hover {
  background-color: #1d4ed8;
}

.modify-button:active {
  background-color: #1e40af;
}
</style>
