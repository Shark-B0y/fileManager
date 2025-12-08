# Windows本地文件管理系统 - API接口文档

## 📋 概述

本系统采用 **Tauri IPC（进程间通信）** 作为前后端通信机制，而非传统的RESTful API。Tauri IPC提供安全的、类型安全的进程间通信，前端通过调用Rust后端暴露的命令（commands）来实现功能。

**通信架构**：
- **前端**：TypeScript/JavaScript调用`invoke()`函数
- **后端**：Rust函数通过`#[tauri::command]`宏暴露为命令
- **通信协议**：JSON序列化/反序列化
- **安全性**：Tauri自动处理权限和沙箱

## 🏷️ 标签管理接口

### 1. 标签查询接口
**接口名**: `get_tags`
**接口功能**: 获取标签列表，支持分页、搜索和层级过滤
**通信方式**: Tauri IPC命令
**调用示例**:
```typescript
const tags = await invoke('get_tags', {
  page: 1,
  pageSize: 20,
  search: '项目',
  parentId: null
});
```

**请求参数**:
```typescript
interface GetTagsRequest {
  page?: number;        // 页码，默认1
  pageSize?: number;    // 每页数量，默认20
  search?: string;      // 搜索关键词
  parentId?: number | null; // 父标签ID，null表示根标签
}
```

**响应数据**:
```typescript
interface GetTagsResponse {
  tags: Tag[];
  total: number;
  page: number;
  pageSize: number;
}

interface Tag {
  id: number;
  name: string;
  color: string;
  icon: string | null;
  parentId: number | null;
  description: string | null;
  createdTime: string; // ISO 8601格式
  usageCount: number;
  children?: Tag[]; // 子标签（可选展开）
}
```

### 2. 创建标签接口
**接口名**: `create_tag`
**接口功能**: 创建新标签
**调用示例**:
```typescript
const newTag = await invoke('create_tag', {
  name: '项目A',
  color: '#FF6B6B',
  icon: '📁',
  parentId: null,
  description: '项目相关文件'
});
```

**请求参数**:
```typescript
interface CreateTagRequest {
  name: string;
  color: string;
  icon?: string;
  parentId?: number | null;
  description?: string;
}
```

### 3. 更新标签接口
**接口名**: `update_tag`
**接口功能**: 更新标签信息
**调用示例**:
```typescript
await invoke('update_tag', {
  id: 1,
  name: '项目A-修改',
  color: '#4ECDC4'
});
```

**请求参数**:
```typescript
interface UpdateTagRequest {
  id: number;
  name?: string;
  color?: string;
  icon?: string | null;
  parentId?: number | null;
  description?: string | null;
}
```

### 4. 删除标签接口
**接口名**: `delete_tag`
**接口功能**: 删除标签（支持级联删除或保留关联）
**调用示例**:
```typescript
await invoke('delete_tag', {
  id: 1,
  cascade: false // 是否级联删除关联
});
```

**请求参数**:
```typescript
interface DeleteTagRequest {
  id: number;
  cascade?: boolean; // 是否删除关联的文件标签
}
```

## 📁 文件管理接口

### 5. 文件扫描接口
**接口名**: `scan_files`
**接口功能**: 扫描指定目录的文件，生成文件指纹并建立索引
**调用示例**:
```typescript
const result = await invoke('scan_files', {
  paths: ['C:/Users/User/Documents', 'D:/Photos'],
  recursive: true,
  forceRescan: false
});
```

**请求参数**:
```typescript
interface ScanFilesRequest {
  paths: string[];      // 要扫描的目录路径数组
  recursive?: boolean;  // 是否递归扫描子目录
  forceRescan?: boolean; // 是否强制重新扫描（忽略缓存）
}
```

**响应数据**:
```typescript
interface ScanFilesResponse {
  scannedCount: number;
  newFiles: number;
  updatedFiles: number;
  skippedFiles: number;
  errors: ScanError[];
  duration: number; // 扫描耗时（毫秒）
}

interface ScanError {
  path: string;
  error: string;
}
```

### 6. 获取文件列表接口
**接口名**: `get_files`
**接口功能**: 获取文件列表，支持标签过滤、分页和排序
**调用示例**:
```typescript
const files = await invoke('get_files', {
  page: 1,
  pageSize: 50,
  tagIds: [1, 2, 3],
  search: '报告',
  fileType: 'pdf',
  sortBy: 'modifiedTime',
  sortOrder: 'desc'
});
```

**请求参数**:
```typescript
interface GetFilesRequest {
  page?: number;
  pageSize?: number;
  tagIds?: number[];    // 标签ID数组（AND逻辑）
  search?: string;      // 文件名搜索
  fileType?: string;    // 文件类型过滤
  minSize?: number;     // 最小文件大小（字节）
  maxSize?: number;     // 最大文件大小（字节）
  startDate?: string;   // 开始日期（ISO格式）
  endDate?: string;     // 结束日期（ISO格式）
  sortBy?: 'name' | 'size' | 'modifiedTime' | 'createdTime';
  sortOrder?: 'asc' | 'desc';
}
```

**响应数据**:
```typescript
interface GetFilesResponse {
  files: FileInfo[];
  total: number;
  page: number;
  pageSize: number;
}

interface FileInfo {
  id: number;
  fileHash: string;
  currentPath: string;
  fileName: string;
  fileSize: number;
  fileType: string;
  createdTime: string;
  modifiedTime: string;
  lastSeenTime: string;
  isActive: boolean;
  tags: Tag[]; // 关联的标签
}
```

### 7. 文件详情接口
**接口名**: `get_file_detail`
**接口功能**: 获取文件的详细信息，包括变更历史
**调用示例**:
```typescript
const detail = await invoke('get_file_detail', {
  fileId: 123
});
```

**请求参数**:
```typescript
interface GetFileDetailRequest {
  fileId: number;
}
```

**响应数据**:
```typescript
interface FileDetail {
  fileInfo: FileInfo;
  changeHistory: FileChange[];
  relatedFiles: RelatedFile[]; // 相似或相关文件
}

interface FileChange {
  id: number;
  changeType: 'MOVE' | 'RENAME' | 'MODIFY' | 'DELETE';
  oldPath: string;
  newPath: string;
  oldHash: string | null;
  newHash: string | null;
  changeTime: string;
  detectedTime: string;
}

interface RelatedFile {
  id: number;
  fileName: string;
  similarity: number; // 相似度0-1
  relationType: 'SAME_CONTENT' | 'SIMILAR_NAME' | 'SAME_TAGS';
}
```

## 🔗 文件-标签关联接口

### 8. 添加文件标签接口
**接口名**: `add_file_tags`
**接口功能**: 为文件添加一个或多个标签
**调用示例**:
```typescript
await invoke('add_file_tags', {
  fileId: 123,
  tagIds: [1, 2, 3],
  confidence: 1.0
});
```

**请求参数**:
```typescript
interface AddFileTagsRequest {
  fileId: number;
  tagIds: number[];
  confidence?: number; // 关联置信度，默认1.0
}
```

### 9. 移除文件标签接口
**接口名**: `remove_file_tags`
**接口功能**: 从文件移除一个或多个标签
**调用示例**:
```typescript
await invoke('remove_file_tags', {
  fileId: 123,
  tagIds: [2, 3]
});
```

**请求参数**:
```typescript
interface RemoveFileTagsRequest {
  fileId: number;
  tagIds: number[];
}
```

### 10. 批量文件标签操作接口
**接口名**: `batch_file_tags`
**接口功能**: 批量操作多个文件的标签
**调用示例**:
```typescript
await invoke('batch_file_tags', {
  fileIds: [123, 124, 125],
  addTagIds: [1, 2],
  removeTagIds: [3]
});
```

**请求参数**:
```typescript
interface BatchFileTagsRequest {
  fileIds: number[];
  addTagIds?: number[];
  removeTagIds?: number[];
}
```

## 🔍 搜索接口

### 11. 标签搜索接口
**接口名**: `search_by_tags`
**接口功能**: 根据标签组合进行搜索（支持AND/OR逻辑）
**调用示例**:
```typescript
const results = await invoke('search_by_tags', {
  tagGroups: [
    { tagIds: [1, 2], logic: 'AND' }, // 标签1 AND 标签2
    { tagIds: [3, 4], logic: 'OR' }   // 标签3 OR 标签4
  ],
  groupLogic: 'AND' // 组间逻辑
});
```

**请求参数**:
```typescript
interface SearchByTagsRequest {
  tagGroups: TagGroup[];
  groupLogic?: 'AND' | 'OR';
  page?: number;
  pageSize?: number;
}

interface TagGroup {
  tagIds: number[];
  logic: 'AND' | 'OR';
}
```

### 12. 高级搜索接口
**接口名**: `advanced_search`
**接口功能**: 综合搜索，支持标签、文件名、文件属性组合
**调用示例**:
```typescript
const results = await invoke('advanced_search', {
  query: '项目 报告',
  tagIds: [1, 2],
  fileTypes: ['pdf', 'docx'],
  minSize: 1024,
  maxSize: 10485760,
  dateRange: {
    start: '2024-01-01',
    end: '2024-12-31'
  }
});
```

**请求参数**:
```typescript
interface AdvancedSearchRequest {
  query?: string; // 搜索关键词（文件名）
  tagIds?: number[];
  fileTypes?: string[];
  minSize?: number;
  maxSize?: number;
  dateRange?: {
    start: string;
    end: string;
  };
  page?: number;
  pageSize?: number;
  sortBy?: string;
  sortOrder?: 'asc' | 'desc';
}
```

### 13. 智能搜索建议接口
**接口名**: `get_search_suggestions`
**接口功能**: 根据输入提供搜索建议（标签、文件名）
**调用示例**:
```typescript
const suggestions = await invoke('get_search_suggestions', {
  input: '项',
  limit: 10
});
```

**请求参数**:
```typescript
interface GetSearchSuggestionsRequest {
  input: string;
  limit?: number;
}
```

**响应数据**:
```typescript
interface SearchSuggestions {
  tags: TagSuggestion[];
  files: FileSuggestion[];
  popularSearches: string[];
}

interface TagSuggestion {
  id: number;
  name: string;
  matchType: 'PREFIX' | 'CONTAINS' | 'FUZZY';
}

interface FileSuggestion {
  id: number;
  fileName: string;
  path: string;
}
```

## 🔄 文件监控接口

### 14. 启动文件监控接口
**接口名**: `start_file_monitoring`
**接口功能**: 启动文件系统监控，监听文件变更
**调用示例**:
```typescript
await invoke('start_file_monitoring', {
  paths: ['C:/Users/User/Documents'],
  pollInterval: 300 // 轮询间隔（秒）
});
```

**请求参数**:
```typescript
interface StartFileMonitoringRequest {
  paths: string[];
  pollInterval?: number; // 轮询间隔（秒），默认300
  realTime?: boolean;    // 是否启用实时监控
}
```

### 15. 停止文件监控接口
**接口名**: `stop_file_monitoring`
**接口功能**: 停止文件系统监控
**调用示例**:
```typescript
await invoke('stop_file_monitoring');
```

### 16. 获取监控状态接口
**接口名**: `get_monitoring_status`
**接口功能**: 获取文件监控的当前状态
**调用示例**:
```typescript
const status = await invoke('get_monitoring_status');
```

**响应数据**:
```typescript
interface MonitoringStatus {
  isActive: boolean;
  monitoredPaths: string[];
  lastScanTime: string | null;
  totalFilesMonitored: number;
  recentChanges: FileChange[];
}
```

## ⚙️ 系统管理接口

### 17. 系统状态接口
**接口名**: `get_system_status`
**接口功能**: 获取系统运行状态和统计信息
**调用示例**:
```typescript
const status = await invoke('get_system_status');
```

**响应数据**:
```typescript
interface SystemStatus {
  database: {
    totalFiles: number;
    totalTags: number;
    totalAssociations: number;
    size: number; // 数据库文件大小（字节）
  };
  performance: {
    memoryUsage: number; // 内存使用（MB）
    cpuUsage: number;    // CPU使用率（%）
    diskUsage: number;   // 磁盘使用（MB）
  };
  lastBackup: string | null;
  uptime: number; // 运行时间（秒）
}
```

### 18. 数据库备份接口
**接口名**: `backup_database`
**接口功能**: 备份数据库到指定位置
**调用示例**:
```typescript
await invoke('backup_database', {
  backupPath: 'C:/Backups/filemanager_backup.db'
});
```

**请求参数**:
```typescript
interface BackupDatabaseRequest {
  backupPath: string;
  compress?: boolean; // 是否压缩
}
```

### 19. 数据库恢复接口
**接口名**: `restore_database`
**接口功能**: 从备份文件恢复数据库
**调用示例**:
```typescript
await invoke('restore_database', {
  backupPath: 'C:/Backups/filemanager_backup.db'
});
```

**请求参数**:
```typescript
interface RestoreDatabaseRequest {
  backupPath: string;
}
```

### 20. 导出数据接口
**接口名**: `export_data`
**接口功能**: 导出标签和关联数据
**调用示例**:
```typescript
const exportData = await invoke('export_data', {
  format: 'json',
  includeFiles: true
});
```

**请求参数**:
```typescript
interface ExportDataRequest {
  format: 'json' | 'csv';
  includeFiles?: boolean; // 是否包含文件信息
  includeTags?: boolean;  // 是否包含标签信息
  includeAssociations?: boolean; // 是否包含关联信息
}
```

**响应数据**:
```typescript
interface ExportDataResponse {
  data: string; // 导出的数据字符串
  format: string;
  size: number;
}
```

## 🚀 Tauri IPC实现示例

### Rust后端命令定义示例
```rust
use tauri::command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct GetTagsRequest {
    page: Option<u32>,
    page_size: Option<u32>,
    search: Option<String>,
    parent_id: Option<i32>,
}

#[derive(Debug, Serialize)]
struct Tag {
    id: i32,
    name: String,
    color: String,
    icon: Option<String>,
    parent_id: Option<i32>,
    description: Option<String>,
    created_time: String,
    usage_count: i32,
}

#[derive(Debug, Serialize)]
struct GetTagsResponse {
    tags: Vec<Tag>,
    total: u32,
    page: u32,
    page_size: u32,
}

#[command]
async fn get_tags(
    request: GetTagsRequest,
    app_handle: tauri::AppHandle,
) -> Result<GetTagsResponse, String> {
    // 数据库查询逻辑
    // 返回标签列表
    Ok(GetTagsResponse {
        tags: vec![],
        total: 0,
        page: request.page.unwrap_or(1),
        page_size: request.page_size.unwrap_or(20),
    })
}

// 在main.rs中注册命令
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_tags,
            create_tag,
            update_tag,
            // ... 其他命令
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 前端调用示例
```typescript
import { invoke } from '@tauri-apps/api/tauri';

// 调用标签查询
async function fetchTags() {
  try {
    const response = await invoke<GetTagsResponse>('get_tags', {
      page: 1,
      pageSize: 20,
      search: '项目'
    });
    console.log('标签列表:', response.tags);
  } catch (error) {
    console.error('获取标签失败:', error);
  }
}

// 调用文件扫描
async function scanDirectory(path: string) {
  try {
    const result = await invoke<ScanFilesResponse>('scan_files', {
      paths: [path],
      recursive: true
    });
    console.log(`扫描完成: ${result.scannedCount} 个文件`);
  } catch (error) {
    console.error('扫描失败:', error);
  }
}
```

## 🔒 错误处理

### 错误响应格式
```typescript
interface ApiError {
  code: string;    // 错误代码
  message: string; // 错误信息
  details?: any;   // 错误详情
}
```

### 常见错误代码
- `DATABASE_ERROR`: 数据库操作错误
- `FILE_NOT_FOUND`: 文件不存在
- `TAG_NOT_FOUND`: 标签不存在
- `PERMISSION_DENIED`: 权限不足
- `INVALID_PARAMETER`: 参数无效
- `SYSTEM_ERROR`: 系统错误

## 📊 性能优化建议

1. **批量操作**：使用批量接口减少IPC调用次数
2. **分页查询**：大数据集使用分页，避免一次性加载
3. **缓存策略**：前端缓存常用数据（如标签列表）
4. **增量更新**：文件监控使用增量更新而非全量扫描
5. **异步处理**：耗时操作使用异步接口，提供进度反馈

---

**文档版本**: v1.0
**最后更新**: 2025-12-05
**通信方式**: Tauri IPC（非RESTful）
**安全性**: Tauri沙箱 + 权限控制
**数据类型**: JSON序列化