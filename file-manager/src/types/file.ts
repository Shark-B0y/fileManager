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
  type: 'file' | 'folder';
  /** 文件大小（字节） */
  size: number;
  /** 修改日期 */
  modifiedDate: string; // ISO 8601 格式
  /** 创建日期 */
  createdDate: string; // ISO 8601 格式
  /** 文件扩展名（仅文件） */
  extension?: string;
  /** 是否为隐藏文件 */
  isHidden?: boolean;
}

/**
 * 目录信息
 */
export interface DirectoryInfo {
  /** 当前路径 */
  path: string;
  /** 父路径 */
  parentPath?: string;
  /** 文件列表 */
  items: FileItem[];
  /** 总文件数 */
  totalFiles: number;
  /** 总文件夹数 */
  totalFolders: number;
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

