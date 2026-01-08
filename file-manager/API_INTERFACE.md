# 文件管理系统 - 前后端 API 接口文档

## 📋 文档说明

本文档用于规范前后端数据通信接口，包含所有 Tauri 命令接口的完整说明。

### 重要规范

1. **新增接口流程**：新增接口时，必须先在此文档中添加接口说明，然后再实现代码
2. **数据结构一致性**：前后端数据结构必须保持一致，使用下划线命名（snake_case）
3. **类型匹配**：TypeScript 类型定义必须与 Rust 结构体字段完全匹配
4. **文档更新**：接口变更时，必须同步更新本文档

### 通信方式

- **通信协议**：Tauri IPC（进程间通信）
- **数据格式**：JSON（自动序列化/反序列化）
- **调用方式**：前端使用 `invoke()` 函数调用后端命令
- **错误处理**：使用 `Result<T, String>` 返回类型

---

## 📚 目录

- [文件系统接口](#文件系统接口)
  - [1. list_directory - 获取目录内容](#1-list_directory---获取目录内容)
  - [2. get_home_directory - 获取用户主目录](#2-get_home_directory---获取用户主目录)
- [示例命令](#示例命令)
  - [3. greet - 问候命令](#3-greet---问候命令)
- [数据结构定义](#数据结构定义)
  - [FileItem - 文件项](#fileitem---文件项)
  - [DirectoryInfo - 目录信息](#directoryinfo---目录信息)

---

## 文件系统接口

### 1. list_directory - 获取目录内容

**功能描述**：列出指定目录下的所有文件和文件夹，返回目录信息和文件列表。

**接口名称**：`list_directory`

**调用方式**：
```typescript
import { invoke } from '@tauri-apps/api/core';

const result = await invoke<DirectoryInfo>('list_directory', {
  path: 'C:\\Users\\Username'
});
```

#### 请求参数

**Rust 后端**：
```rust
#[tauri::command]
pub async fn list_directory(path: String) -> Result<DirectoryInfo, String>
```

**参数说明**：

| 参数名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| `path` | `String` | 是 | 目录路径（Windows 路径格式，如：`C:\\Users\\Username`） |

**TypeScript 前端**：
```typescript
interface ListDirectoryRequest {
  path: string;
}
```

#### 返回数据

**成功返回**：`DirectoryInfo` 对象

**错误返回**：`String` 错误信息

**常见错误**：
- `"路径不存在: {path}"` - 指定的路径不存在
- `"路径不是目录: {path}"` - 指定的路径不是目录
- `"读取目录失败: {error}"` - 读取目录时发生错误
- `"获取文件元数据失败: {error}"` - 获取文件元数据时发生错误

#### 数据结构

**Rust 后端** (`src-tauri/src/models/file_system.rs`)：
```rust
pub struct DirectoryInfo {
    /// 当前路径
    pub path: String,
    /// 父路径
    pub parent_path: Option<String>,
    /// 文件列表
    pub items: Vec<FileItem>,
    /// 总文件数
    pub total_files: usize,
    /// 总文件夹数
    pub total_folders: usize,
}
```

**TypeScript 前端** (`src/types/file.ts`)：
```typescript
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
```

#### 使用示例

**前端调用**：
```typescript
import { invoke } from '@tauri-apps/api/core';
import type { DirectoryInfo } from '../types/file';

async function loadDirectory(path: string): Promise<DirectoryInfo> {
  try {
    const result = await invoke<DirectoryInfo>('list_directory', { path });
    return result;
  } catch (error) {
    console.error('加载目录失败:', error);
    throw error;
  }
}

// 使用示例
const dirInfo = await loadDirectory('C:\\Users\\Username');
console.log(`目录: ${dirInfo.path}`);
console.log(`文件数: ${dirInfo.total_files}`);
console.log(`文件夹数: ${dirInfo.total_folders}`);
dirInfo.items.forEach(item => {
  console.log(`- ${item.name} (${item.file_type})`);
});
```

**后端实现** (`src-tauri/src/commands.rs`)：
```rust
#[tauri::command]
pub async fn list_directory(path: String) -> Result<DirectoryInfo, String> {
    FileSystemService::list_directory(&path)
}
```

---

### 2. get_home_directory - 获取用户主目录

**功能描述**：获取当前用户的主目录路径。

**接口名称**：`get_home_directory`

**调用方式**：
```typescript
import { invoke } from '@tauri-apps/api/core';

const homeDir = await invoke<string>('get_home_directory');
```

#### 请求参数

**Rust 后端**：
```rust
#[tauri::command]
pub async fn get_home_directory() -> Result<String, String>
```

**参数说明**：无参数

**TypeScript 前端**：无需传递参数

#### 返回数据

**成功返回**：`String` 用户主目录路径

- **Windows**：返回 `USERPROFILE` 环境变量值（如：`C:\Users\Username`）
- **Unix/Linux**：返回 `HOME` 环境变量值（如：`/home/username`）

**错误返回**：`String` 错误信息

**常见错误**：
- `"无法获取用户主目录"` - 无法从环境变量获取主目录路径

#### 数据结构

**返回类型**：`String`

**示例返回值**：
- Windows: `"C:\\Users\\Username"`
- Linux: `"/home/username"`
- macOS: `"/Users/username"`

#### 使用示例

**前端调用**：
```typescript
import { invoke } from '@tauri-apps/api/core';

async function getHomeDirectory(): Promise<string> {
  try {
    const homeDir = await invoke<string>('get_home_directory');
    console.log('用户主目录:', homeDir);
    return homeDir;
  } catch (error) {
    console.error('获取主目录失败:', error);
    // 如果获取失败，可以使用默认路径
    return 'C:\\';
  }
}

// 使用示例
const homeDir = await getHomeDirectory();
await loadDirectory(homeDir); // 加载主目录内容
```

**后端实现** (`src-tauri/src/commands.rs`)：
```rust
#[tauri::command]
pub async fn get_home_directory() -> Result<String, String> {
    FileSystemService::get_home_directory()
}
```

---

## 示例命令

### 3. greet - 问候命令

**功能描述**：示例命令，用于测试前后端通信是否正常。

**接口名称**：`greet`

**调用方式**：
```typescript
import { invoke } from '@tauri-apps/api/core';

const message = await invoke<string>('greet', { name: 'World' });
```

#### 请求参数

**Rust 后端**：
```rust
#[tauri::command]
pub async fn greet(name: &str) -> Result<String, String>
```

**参数说明**：

| 参数名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| `name` | `&str` | 是 | 要问候的名称 |

**TypeScript 前端**：
```typescript
interface GreetRequest {
  name: string;
}
```

#### 返回数据

**成功返回**：`String` 问候消息

**返回格式**：`"Hello, {name}! You've been greeted from Rust!"`

**错误返回**：无（此命令不会返回错误）

#### 使用示例

**前端调用**：
```typescript
import { invoke } from '@tauri-apps/api/core';

async function testGreet() {
  const message = await invoke<string>('greet', { name: 'World' });
  console.log(message); // 输出: "Hello, World! You've been greeted from Rust!"
}
```

**后端实现** (`src-tauri/src/commands.rs`)：
```rust
#[tauri::command]
pub async fn greet(name: &str) -> Result<String, String> {
    Ok(format!("Hello, {}! You've been greeted from Rust!", name))
}
```

---

## 数据结构定义

### FileItem - 文件项

**用途**：表示一个文件或文件夹的信息。

#### Rust 后端定义

**位置**：`src-tauri/src/models/file_system.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileItem {
    /// 唯一标识符（文件路径）
    pub id: String,
    /// 文件名
    pub name: String,
    /// 完整路径
    pub path: String,
    /// 文件类型："file" 或 "folder"
    pub file_type: String,
    /// 文件大小（字节）
    pub size: u64,
    /// 修改日期（ISO 8601 格式）
    pub modified_date: String,
    /// 创建日期（ISO 8601 格式）
    pub created_date: String,
    /// 文件扩展名（仅文件）
    pub extension: Option<String>,
    /// 是否为隐藏文件
    pub is_hidden: bool,
}
```

#### TypeScript 前端定义

**位置**：`src/types/file.ts`

```typescript
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
```

#### 字段说明

| 字段名 | Rust 类型 | TypeScript 类型 | 必填 | 说明 |
|--------|-----------|-----------------|------|------|
| `id` | `String` | `string` | 是 | 唯一标识符，通常使用文件完整路径 |
| `name` | `String` | `string` | 是 | 文件名（不含路径） |
| `path` | `String` | `string` | 是 | 文件的完整路径 |
| `file_type` | `String` | `'file' \| 'folder'` | 是 | 文件类型，值为 `"file"` 或 `"folder"` |
| `size` | `u64` | `number` | 是 | 文件大小（字节），文件夹通常为 0 |
| `modified_date` | `String` | `string` | 是 | 修改日期，ISO 8601 格式（Unix 时间戳格式：`"{秒数}.{纳秒数}Z"`） |
| `created_date` | `String` | `string` | 是 | 创建日期，ISO 8601 格式（Unix 时间戳格式：`"{秒数}.{纳秒数}Z"`） |
| `extension` | `Option<String>` | `string \| undefined` | 否 | 文件扩展名（不含点号），仅文件有此字段 |
| `is_hidden` | `bool` | `boolean \| undefined` | 否 | 是否为隐藏文件 |

#### 注意事项

1. **时间格式**：`modified_date` 和 `created_date` 使用 Unix 时间戳格式（`"{秒数}.{纳秒数}Z"`），前端需要使用 `formatDate()` 函数进行格式化显示
2. **文件类型**：`file_type` 必须是 `"file"` 或 `"folder"` 字符串，不能是其他值
3. **扩展名**：文件夹的 `extension` 字段为 `None`（Rust）或 `undefined`（TypeScript）
4. **路径格式**：Windows 路径使用反斜杠（`\`），需要转义为 `\\`

---

### DirectoryInfo - 目录信息

**用途**：表示一个目录的信息，包含文件列表和统计信息。

#### Rust 后端定义

**位置**：`src-tauri/src/models/file_system.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryInfo {
    /// 当前路径
    pub path: String,
    /// 父路径
    pub parent_path: Option<String>,
    /// 文件列表
    pub items: Vec<FileItem>,
    /// 总文件数
    pub total_files: usize,
    /// 总文件夹数
    pub total_folders: usize,
}
```

#### TypeScript 前端定义

**位置**：`src/types/file.ts`

```typescript
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
```

#### 字段说明

| 字段名 | Rust 类型 | TypeScript 类型 | 必填 | 说明 |
|--------|-----------|-----------------|------|------|
| `path` | `String` | `string` | 是 | 当前目录的完整路径 |
| `parent_path` | `Option<String>` | `string \| undefined` | 否 | 父目录路径，根目录（如 `C:\`）为 `None` |
| `items` | `Vec<FileItem>` | `FileItem[]` | 是 | 目录下的文件和文件夹列表，已排序（文件夹在前，然后按名称排序） |
| `total_files` | `usize` | `number` | 是 | 目录中的文件总数（不包括文件夹） |
| `total_folders` | `usize` | `number` | 是 | 目录中的文件夹总数 |

#### 注意事项

1. **排序规则**：`items` 数组已排序，排序规则为：文件夹在前，文件在后，同类型按名称排序
2. **隐藏文件**：隐藏文件（以 `.` 开头的文件）会被过滤，不会出现在 `items` 中
3. **父路径**：根目录（如 `C:\`）的 `parent_path` 为 `None`（Rust）或 `undefined`（TypeScript）
4. **统计信息**：`total_files` 和 `total_folders` 只统计非隐藏的文件和文件夹

---

## 🔧 接口注册

所有接口需要在 `src-tauri/src/lib.rs` 中注册：

```rust
.invoke_handler(tauri::generate_handler![
    commands::greet,
    commands::list_directory,
    commands::get_home_directory
])
```

---

## 📝 新增接口流程

### 步骤 1：在本文档中添加接口说明

在相应的分类下添加新接口的完整说明，包括：
- 功能描述
- 请求参数（Rust 和 TypeScript）
- 返回数据（Rust 和 TypeScript）
- 数据结构定义
- 使用示例

### 步骤 2：定义数据结构

**后端**：在 `src-tauri/src/models/` 目录下创建或修改相应的模型文件，定义 Rust 结构体

**前端**：在 `src/types/` 目录下创建或修改相应的类型文件，定义 TypeScript 接口

**重要**：确保前后端字段名完全一致（使用下划线命名）

### 步骤 3：实现服务层

在 `src-tauri/src/services/` 目录下实现业务逻辑

### 步骤 4：实现命令接口

在 `src-tauri/src/commands.rs` 中添加 `#[tauri::command]` 标记的函数

### 步骤 5：注册接口

在 `src-tauri/src/lib.rs` 的 `invoke_handler` 中添加新命令

### 步骤 6：前端调用

在 `src/composables/` 或相应位置添加前端调用代码

---

## ⚠️ 注意事项

1. **命名规范**：所有字段使用下划线命名（snake_case），如 `file_type`、`modified_date`
2. **类型匹配**：确保 Rust 类型和 TypeScript 类型正确对应
   - `String` → `string`
   - `u64` / `usize` → `number`
   - `bool` → `boolean`
   - `Option<T>` → `T | undefined`
   - `Vec<T>` → `T[]`
3. **错误处理**：所有接口返回 `Result<T, String>`，前端需要处理错误情况
4. **文档同步**：接口变更时，必须同步更新本文档和代码注释

---

## 📅 版本记录

### v1.0.0 (2025-12-XX)
- 初始版本
- 添加文件系统接口文档
- 添加 `list_directory` 接口
- 添加 `get_home_directory` 接口
- 添加 `greet` 示例接口
- 添加数据结构定义说明

---

**文档维护者**：开发团队
**最后更新**：2025-12-XX
**文档版本**：v1.0.0

