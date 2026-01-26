// 文件系统相关类型定义

/**
 * 文件项类型
 */
export interface FileItem {
  /** 唯一标识符 */
  id: string;
  /** 文件名 */
  name: string;
  /** 完整路径 */
  path: string;
  /** 文件类型 */
  file_type: 'file' | 'folder';
  /** 文件大小（字节） */
  size: number;
  /** 修改日期 */
  modified_date: string; // ISO 8601 格式
  /** 创建日期 */
  created_date: string; // ISO 8601 格式
  /** 文件扩展名（仅文件） */
  extension?: string;
  /** 是否为隐藏文件 */
  is_hidden?: boolean;
}

/**
 * 目录信息
 */
export interface DirectoryInfo {
  /** 当前路径 */
  path: string;
  /** 父路径 */
  parent_path?: string;
  /** 文件列表 */
  items: FileItem[];
  /** 总文件数 */
  total_files: number;
  /** 总文件夹数 */
  total_folders: number;
}

/**
 * 文件系统操作结果
 */
export interface FileSystemResult<T = void> {
  /** 是否成功 */
  success: boolean;
  /** 错误信息 */
  error?: string;
  /** 返回数据 */
  data?: T;
}

/**
 * 搜索结果
 */
export interface SearchResult {
  /** 文件列表 */
  items: FileItem[];
  /** 总文件数 */
  total: number;
  /** 当前页码（从1开始） */
  page: number;
  /** 每页数量 */
  page_size: number;
  /** 是否有更多数据 */
  has_more: boolean;
}