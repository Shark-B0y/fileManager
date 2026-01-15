// 标签相关类型定义

/**
 * 标签信息
 */
export interface Tag {
  /** 标签ID */
  id: number;
  /** 标签名称 */
  name: string;
  /** 标签背景颜色（HEX颜色代码，如#FFFF00） */
  color: string | null;
  /** 标签字体颜色（HEX颜色代码，如#000000） */
  font_color: string | null;
  /** 父标签ID（用于层级标签） */
  parent_id: number | null;
  /** 使用次数统计 */
  usage_count: number;
  /** 创建时间 */
  created_at: string;
  /** 更新时间 */
  updated_at: string;
}

