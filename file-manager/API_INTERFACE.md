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
  - [3. list_drives - 获取驱动盘列表](#3-list_drives---获取驱动盘列表)
  - [4. check_path_exists - 检查路径是否存在](#4-check_path_exists---检查路径是否存在)
  - [5. cut_files - 剪切文件](#5-cut_files---剪切文件)
  - [6. copy_files - 复制文件](#6-copy_files---复制文件)
  - [7. rename_file - 重命名文件](#7-rename_file---重命名文件)
  - [8. delete_files - 删除文件](#8-delete_files---删除文件)
- [标签管理接口](#标签管理接口)
  - [9. get_tag_list - 获取标签列表](#9-get_tag_list---获取标签列表)
  - [10. search_tags - 搜索标签](#10-search_tags---搜索标签)
  - [11. create_tag - 创建新标签](#11-create_tag---创建新标签)
  - [12. modify_tag - 修改标签](#12-modify_tag---修改标签)
  - [13. add_tags_to_files - 批量添加标签到文件/文件夹](#13-add_tags_to_files---批量添加标签到文件文件夹)
- [示例命令](#示例命令)
  - [8. greet - 问候命令](#8-greet---问候命令)
- [数据结构定义](#数据结构定义)
  - [FileItem - 文件项](#fileitem---文件项)
  - [DirectoryInfo - 目录信息](#directoryinfo---目录信息)
  - [Tag - 标签](#tag---标签)

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

### 3. list_drives - 获取驱动盘列表

**功能描述**：获取 Windows 系统中所有可用的驱动盘列表（仅 Windows 系统支持）。用于在文件管理器中显示所有驱动盘（如 C:、D:、E: 等），方便用户在不同驱动盘之间切换。

**接口名称**：`list_drives`

**调用方式**：
```typescript
import { invoke } from '@tauri-apps/api/core';

const result = await invoke<DirectoryInfo>('list_drives');
```

#### 请求参数

**Rust 后端**：
```rust
#[tauri::command]
pub async fn list_drives() -> Result<DirectoryInfo, String>
```

**参数说明**：无参数

**TypeScript 前端**：无需传递参数

#### 返回数据

**成功返回**：`DirectoryInfo` 对象

**返回数据结构说明**：
- `path`: 固定为 `"drives:"`（用于标识这是驱动盘列表视图）
- `parent_path`: `None`（驱动盘列表是最顶层，无父路径）
- `items`: 所有可用驱动盘的列表，每个驱动盘是一个 `FileItem`，其中：
  - `id`: 驱动盘路径（如 `"C:\\"`）
  - `name`: 驱动盘名称（如 `"C:"`）
  - `path`: 驱动盘完整路径（如 `"C:\\"`）
  - `file_type`: 固定为 `"folder"`
  - `size`: 固定为 `0`（驱动盘不显示大小）
- `total_files`: 固定为 `0`
- `total_folders`: 可用驱动盘的数量

**错误返回**：`String` 错误信息

**常见错误**：
- `"此功能仅支持 Windows 系统"` - 在非 Windows 系统上调用此接口
- 其他系统错误（如无法读取驱动盘信息）

#### 数据结构

**Rust 后端** (`src-tauri/src/models/file_system.rs`)：
使用 `DirectoryInfo` 结构体，详见 [DirectoryInfo - 目录信息](#directoryinfo---目录信息)

**TypeScript 前端** (`src/types/file.ts`)：
使用 `DirectoryInfo` 接口，详见 [DirectoryInfo - 目录信息](#directoryinfo---目录信息)

#### 特殊说明

1. **平台限制**：此接口仅在 Windows 系统上可用，非 Windows 系统会返回错误
2. **驱动盘检测**：系统会遍历 A-Z 所有可能的驱动盘，只返回实际存在的驱动盘
3. **排序规则**：返回的驱动盘列表按字母顺序排序（A-Z）
4. **路径格式**：返回的驱动盘路径使用标准 Windows 格式（如 `"C:\\"`），可直接用于 `list_directory` 接口

#### 使用示例

**前端调用**：
```typescript
import { invoke } from '@tauri-apps/api/core';
import type { DirectoryInfo } from '../types/file';

async function loadDrives(): Promise<DirectoryInfo> {
  try {
    const result = await invoke<DirectoryInfo>('list_drives');
    console.log('驱动盘列表:', result);
    return result;
  } catch (error) {
    console.error('加载驱动盘列表失败:', error);
    throw error;
  }
}

// 使用示例
const drivesInfo = await loadDrives();
console.log(`共有 ${drivesInfo.total_folders} 个驱动盘`);
drivesInfo.items.forEach(drive => {
  console.log(`- ${drive.name} (${drive.path})`);
  // 点击驱动盘后，可以调用 list_directory(drive.path) 进入该驱动盘
});
```

**后端实现** (`src-tauri/src/commands.rs`)：
```rust
#[tauri::command]
pub async fn list_drives() -> Result<DirectoryInfo, String> {
    FileSystemService::list_drives()
}
```

**前端集成示例** (`src/composables/useFileSystem.ts`)：
```typescript
async function loadDrives() {
  loading.value = true;
  error.value = null;

  try {
    const result = await invoke<DirectoryInfo>('list_drives');
    directoryInfo.value = result;
    currentPath.value = '驱动盘';
    return result;
  } catch (err) {
    error.value = err instanceof Error ? err.message : String(err);
    throw err;
  } finally {
    loading.value = false;
  }
}
```

#### 与 list_directory 的配合使用

当用户在驱动盘根目录（如 `C:\`）点击返回按钮时：
1. `list_directory` 返回的 `parent_path` 为 `"drives:"`
2. 前端检测到 `parent_path === "drives:"` 时，调用 `list_drives()` 显示驱动盘列表
3. 用户点击某个驱动盘后，调用 `list_directory(drive.path)` 进入该驱动盘的根目录

---

### 4. check_path_exists - 检查路径是否存在

**功能描述**：检查指定路径是否存在且为目录。用于验证用户输入的路径是否有效，在导航栏路径输入框中使用。

**接口名称**：`check_path_exists`

**调用方式**：
```typescript
import { invoke } from '@tauri-apps/api/core';

const exists = await invoke<boolean>('check_path_exists', {
  path: 'C:\\Users\\Username'
});
```

#### 请求参数

**Rust 后端**：
```rust
#[tauri::command]
pub async fn check_path_exists(path: String) -> Result<bool, String>
```

**参数说明**：

| 参数名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| `path` | `String` | 是 | 要检查的路径（Windows 路径格式，如：`C:\\Users\\Username`） |

**TypeScript 前端**：
```typescript
interface CheckPathExistsRequest {
  path: string;
}
```

#### 返回数据

**成功返回**：`boolean`

- `true`：路径存在且为目录
- `false`：路径不存在或不是目录

**错误返回**：`String` 错误信息

**常见错误**：
- 一般情况下不会返回错误，只返回 `false` 表示路径不存在
- 可能的系统错误会被转换为字符串返回

#### 数据结构

**返回类型**：`boolean`

**示例返回值**：
- `true` - 路径存在且为目录
- `false` - 路径不存在或不是目录

#### 使用示例

**前端调用**：
```typescript
import { invoke } from '@tauri-apps/api/core';

async function validateAndNavigate(inputPath: string): Promise<void> {
  try {
    const exists = await invoke<boolean>('check_path_exists', { path: inputPath });

    if (exists) {
      // 路径存在，跳转到该目录
      await loadDirectory(inputPath);
    } else {
      // 路径不存在，弹出提示框
      alert(`路径不存在: ${inputPath}`);
    }
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : String(error);
    alert(`无法访问路径: ${errorMessage}`);
  }
}

// 使用示例
await validateAndNavigate('C:\\Users\\Username');
```

**后端实现** (`src-tauri/src/commands.rs`)：
```rust
#[tauri::command]
pub async fn check_path_exists(path: String) -> Result<bool, String> {
    FileSystemService::check_path_exists(&path)
}
```

**后端服务实现** (`src-tauri/src/services/file_system.rs`)：
```rust
pub fn check_path_exists(path: &str) -> Result<bool, String> {
    let dir_path = Path::new(path);

    // 检查路径是否存在
    if !dir_path.exists() {
        return Ok(false);
    }

    // 检查是否为目录
    if !dir_path.is_dir() {
        return Ok(false);
    }

    Ok(true)
}
```

#### 注意事项

1. **只检查目录**：此接口只返回 `true` 当路径存在且为目录时。如果路径是文件而非目录，将返回 `false`
2. **路径格式**：Windows 路径使用反斜杠（`\`），需要转义为 `\\`
3. **权限问题**：如果路径存在但无权限访问，可能会返回 `false` 或错误
4. **使用场景**：主要用于导航栏路径输入框的验证，在用户输入路径后按回车键时验证路径的有效性

---

### 5. cut_files - 剪切文件

**功能描述**：将指定的文件/文件夹移动到目标目录（剪切操作）。如果被剪切的文件原本在 files 表中有数据，则会自动更新 current_path 字段，确保标签关联不会丢失。

**接口名称**：`cut_files`

**调用方式**：
```typescript
import { invoke } from '@tauri-apps/api/core';

await invoke('cut_files', {
  paths: ['C:\\Users\\Username\\file1.txt', 'C:\\Users\\Username\\folder1'],
  targetPath: 'C:\\Users\\Username\\Documents'
});
```

#### 请求参数

**Rust 后端**：
```rust
#[tauri::command]
pub async fn cut_files(
    db: State<'_, GlobalDatabase>,
    paths: Vec<String>,
    target_path: String,
) -> Result<(), String>
```

**参数说明**：

| 参数名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| `paths` | `Vec<String>` | 是 | 要剪切的文件/文件夹路径列表 |
| `target_path` | `String` | 是 | 目标目录路径（Windows 路径格式） |

**TypeScript 前端**：
```typescript
interface CutFilesRequest {
  paths: string[];
  target_path: string;
}
```

#### 返回数据

**成功返回**：无返回值（`void`）

**错误返回**：`String` 错误信息

**常见错误**：
- `"目标路径不存在: {target_path}"` - 目标目录不存在
- `"目标路径不是目录: {target_path}"` - 目标路径不是目录
- `"源路径不存在: {path}"` - 源文件/文件夹不存在
- `"目标路径已存在: {dest_path}"` - 目标位置已存在同名文件/文件夹
- `"移动文件失败 {source} -> {dest}: {error}"` - 移动文件时发生错误

#### 使用示例

**前端调用**：
```typescript
import { invoke } from '@tauri-apps/api/core';

async function cutFiles(paths: string[], targetPath: string): Promise<void> {
  try {
    await invoke('cut_files', {
      paths,
      target_path: targetPath,
    });
    console.log('剪切成功');
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : String(error);
    console.error('剪切失败:', errorMessage);
    throw error;
  }
}

// 使用示例
await cutFiles(
  ['C:\\Users\\Username\\file1.txt', 'C:\\Users\\Username\\folder1'],
  'C:\\Users\\Username\\Documents'
);
```

**后端实现** (`src-tauri/src/commands.rs`)：
```rust
#[tauri::command]
pub async fn cut_files(
    db: State<'_, GlobalDatabase>,
    paths: Vec<String>,
    target_path: String,
) -> Result<(), String> {
    FileSystemService::cut_files(&*db, &paths, &target_path).await
}
```

#### 注意事项

1. **移动操作**：剪切操作会移动文件/文件夹，原位置的文件将被删除
2. **批量操作**：支持同时移动多个文件/文件夹
3. **递归移动**：如果移动的是文件夹，会递归移动文件夹内的所有内容
4. **目标冲突**：如果目标位置已存在同名文件/文件夹，操作会失败
5. **权限要求**：需要对源路径和目标路径都有写入权限
6. **数据库更新**：如果被剪切的文件原本在 files 表中有数据，会自动更新 current_path 字段，确保标签关联不会丢失

---

### 6. copy_files - 复制文件

**功能描述**：将指定的文件/文件夹复制到目标目录（复制操作）。如果被复制的文件原本有 tag，则新生成的文件信息需要复制一份原有的 tag；如果原来的文件没有 tag，则不需要新生成文件信息，也不需要更新 tag。

**接口名称**：`copy_files`

**调用方式**：
```typescript
import { invoke } from '@tauri-apps/api/core';

await invoke('copy_files', {
  paths: ['C:\\Users\\Username\\file1.txt', 'C:\\Users\\Username\\folder1'],
  targetPath: 'C:\\Users\\Username\\Documents'
});
```

#### 请求参数

**Rust 后端**：
```rust
#[tauri::command]
pub async fn copy_files(
    db: State<'_, GlobalDatabase>,
    paths: Vec<String>,
    target_path: String,
) -> Result<(), String>
```

**参数说明**：

| 参数名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| `paths` | `Vec<String>` | 是 | 要复制的文件/文件夹路径列表 |
| `target_path` | `String` | 是 | 目标目录路径（Windows 路径格式） |

**TypeScript 前端**：
```typescript
interface CopyFilesRequest {
  paths: string[];
  target_path: string;
}
```

#### 返回数据

**成功返回**：无返回值（`void`）

**错误返回**：`String` 错误信息

**常见错误**：
- `"目标路径不存在: {target_path}"` - 目标目录不存在
- `"目标路径不是目录: {target_path}"` - 目标路径不是目录
- `"源路径不存在: {path}"` - 源文件/文件夹不存在
- `"目标路径已存在: {dest_path}"` - 目标位置已存在同名文件/文件夹
- `"复制文件失败 {source} -> {dest}: {error}"` - 复制文件时发生错误

#### 使用示例

**前端调用**：
```typescript
import { invoke } from '@tauri-apps/api/core';

async function copyFiles(paths: string[], targetPath: string): Promise<void> {
  try {
    await invoke('copy_files', {
      paths,
      target_path: targetPath,
    });
    console.log('复制成功');
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : String(error);
    console.error('复制失败:', errorMessage);
    throw error;
  }
}

// 使用示例
await copyFiles(
  ['C:\\Users\\Username\\file1.txt', 'C:\\Users\\Username\\folder1'],
  'C:\\Users\\Username\\Documents'
);
```

**后端实现** (`src-tauri/src/commands.rs`)：
```rust
#[tauri::command]
pub async fn copy_files(
    db: State<'_, GlobalDatabase>,
    paths: Vec<String>,
    target_path: String,
) -> Result<(), String> {
    FileSystemService::copy_files(&*db, &paths, &target_path).await
}
```

#### 注意事项

1. **复制操作**：复制操作不会删除源文件/文件夹，原位置的文件保持不变
2. **批量操作**：支持同时复制多个文件/文件夹
3. **递归复制**：如果复制的是文件夹，会递归复制文件夹内的所有内容
4. **目标冲突**：如果目标位置已存在同名文件/文件夹，操作会失败
5. **权限要求**：需要对源路径有读取权限，对目标路径有写入权限
6. **隐藏文件**：复制文件夹时，会跳过隐藏文件（以 `.` 开头的文件）
7. **标签复制**：
   - 如果被复制的文件原本有 tag，则新生成的文件信息需要复制一份原有的 tag
   - 如果原来的文件没有 tag，则不需要新生成文件信息，也不需要更新 tag
   - 只有源文件在数据库中有记录且有关联标签时，才会为新文件创建记录并复制标签

---

### 7. rename_file - 重命名文件

**功能描述**：将指定的文件或文件夹重命名为新名称，并自动更新数据库中的路径记录，确保标签关联不会丢失。

**接口名称**：`rename_file`

**调用方式**：
```typescript
import { invoke } from '@tauri-apps/api/core';

await invoke('rename_file', {
  oldPath: 'C:\\Users\\Username\\file.txt',
  newName: 'newfile.txt'
});
```

#### 请求参数

**Rust 后端**：
```rust
#[tauri::command]
pub async fn rename_file(
    db: State<'_, GlobalDatabase>,
    old_path: String,
    new_name: String,
) -> Result<(), String>
```

**参数说明**：

| 参数名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| `old_path` | `String` | 是 | 原文件/文件夹路径（Windows 路径格式） |
| `new_name` | `String` | 是 | 新名称（不包含路径分隔符，仅文件名） |

**TypeScript 前端**：
```typescript
interface RenameFileRequest {
  old_path: string;
  new_name: string;
}
```

#### 返回数据

**成功返回**：无返回值（`void`）

**错误返回**：`String` 错误信息

**常见错误**：
- `"源路径不存在: {old_path}"` - 原文件/文件夹不存在
- `"新名称不能包含路径分隔符: {new_name}"` - 新名称包含了 `/` 或 `\` 字符
- `"新名称不能为空"` - 传入的新名称为空或仅空白字符
- `"目标路径已存在: {new_path}"` - 目标位置已存在同名文件/文件夹
- `"重命名失败 {old_path} -> {new_path}: {error}"` - 重命名操作失败

#### 使用示例

**前端调用**：
```typescript
import { invoke } from '@tauri-apps/api/core';

async function renameFile(oldPath: string, newName: string): Promise<void> {
  try {
    await invoke('rename_file', {
      old_path: oldPath,
      new_name: newName,
    });
    console.log('重命名成功');
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : String(error);
    console.error('重命名失败:', errorMessage);
    throw error;
  }
}

// 使用示例：重命名文件
await renameFile('C:\\Users\\Username\\file.txt', 'newfile.txt');

// 使用示例：重命名文件夹
await renameFile('C:\\Users\\Username\\folder', 'newfolder');
```

**后端实现** (`src-tauri/src/commands.rs`)：
```rust
#[tauri::command]
pub async fn rename_file(
    db: State<'_, GlobalDatabase>,
    old_path: String,
    new_name: String,
) -> Result<(), String> {
    FileSystemService::rename_file(&*db, &old_path, &new_name).await
}
```

**后端服务实现** (`src-tauri/src/services/file_system.rs`)：
```rust
pub fn rename_file(old_path: &str, new_name: &str) -> Result<(), String> {
    let source_path = Path::new(old_path);

    // 检查源路径是否存在
    if !source_path.exists() {
        return Err(format!("源路径不存在: {}", old_path));
    }

    // 验证新名称是否有效（不能包含路径分隔符）
    if new_name.contains('/') || new_name.contains('\\') {
        return Err(format!("新名称不能包含路径分隔符: {}", new_name));
    }

    // 新名称不能为空
    if new_name.trim().is_empty() {
        return Err("新名称不能为空".to_string());
    }

    // 获取父目录
    let parent_dir = source_path.parent()
        .ok_or_else(|| format!("无法获取父目录: {}", old_path))?;

    // 构建新路径
    let new_path = parent_dir.join(new_name);

    // 如果目标路径已存在，返回错误
    if new_path.exists() {
        return Err(format!("目标路径已存在: {}", new_path.display()));
    }

    // 重命名文件/文件夹
    fs::rename(source_path, &new_path)
        .map_err(|e| format!("重命名失败 {} -> {}: {}", old_path, new_path.display(), e))?;

    Ok(())
}
```

#### 注意事项

1. **新名称格式**：`new_name` 只需要提供文件名部分，不需要完整路径。系统会自动使用原文件所在目录作为新文件的位置。
2. **扩展名处理**：
   - 文件重命名时，如果用户没有提供扩展名，前端会自动保留原扩展名
   - 文件夹重命名时，直接使用提供的新名称
3. **路径限制**：新名称不能包含路径分隔符（`/` 或 `\`），只能包含文件名和扩展名
4. **目标冲突**：如果目标位置已存在同名文件/文件夹，操作会失败
5. **权限要求**：需要对源路径所在目录有写入权限
6. **使用场景**：主要用于工具栏重命名按钮，当选中单个文件或文件夹时，点击重命名按钮后，文件名显示区域会变为输入框，用户修改名称后按回车键完成重命名
7. **数据库更新**：重命名后会自动更新数据库中的 `current_path` 字段，确保文件的所有标签关联不会丢失

---

### 8. delete_files - 删除文件

**功能描述**：删除指定的文件/文件夹列表，支持递归删除文件夹。删除操作不可撤销，请谨慎使用。删除后会更新数据库记录（软删除），确保标签关联信息保留。

**接口名称**：`delete_files`

**调用方式**：
```typescript
import { invoke } from '@tauri-apps/api/core';

await invoke('delete_files', {
  paths: ['C:\\Users\\Username\\file1.txt', 'C:\\Users\\Username\\folder1']
});
```

#### 请求参数

**Rust 后端**：
```rust
#[tauri::command]
pub async fn delete_files(
    db: State<'_, GlobalDatabase>,
    paths: Vec<String>,
) -> Result<(), String>
```

**参数说明**：

| 参数名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| `paths` | `Vec<String>` | 是 | 要删除的文件/文件夹路径列表 |

**TypeScript 前端**：
```typescript
interface DeleteFilesRequest {
  paths: string[];
}
```

#### 返回数据

**成功返回**：无返回值（`void`）

**错误返回**：`String` 错误信息

**常见错误**：
- `"路径不存在: {path}"` - 指定的路径不存在
- `"删除文件失败 {path}: {error}"` - 删除文件时发生错误
- `"删除文件夹失败 {path}: {error}"` - 删除文件夹时发生错误

#### 使用示例

**前端调用**：
```typescript
import { invoke } from '@tauri-apps/api/core';

async function deleteFiles(paths: string[]): Promise<void> {
  try {
    await invoke('delete_files', { paths });
    console.log('删除成功');
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : String(error);
    console.error('删除失败:', errorMessage);
    throw error;
  }
}

// 使用示例：删除单个文件
await deleteFiles(['C:\\Users\\Username\\file.txt']);

// 使用示例：删除多个文件和文件夹
await deleteFiles([
  'C:\\Users\\Username\\file1.txt',
  'C:\\Users\\Username\\file2.txt',
  'C:\\Users\\Username\\folder1'
]);
```

**后端实现** (`src-tauri/src/commands.rs`)：
```rust
#[tauri::command]
pub async fn delete_files(
    db: State<'_, GlobalDatabase>,
    paths: Vec<String>,
) -> Result<(), String> {
    FileSystemService::delete_files(&*db, &paths).await
}
```

**后端服务实现** (`src-tauri/src/services/file_system.rs`)：
```rust
pub fn delete_files(paths: &[String]) -> Result<(), String> {
    for path in paths {
        let target_path = Path::new(path);

        // 检查路径是否存在
        if !target_path.exists() {
            return Err(format!("路径不存在: {}", path));
        }

        // 删除文件或文件夹
        if target_path.is_dir() {
            // 递归删除目录
            fs::remove_dir_all(target_path)
                .map_err(|e| format!("删除文件夹失败 {}: {}", path, e))?;
        } else {
            // 删除文件
            fs::remove_file(target_path)
                .map_err(|e| format!("删除文件失败 {}: {}", path, e))?;
        }
    }

    Ok(())
}
```

#### 注意事项

1. **不可撤销**：删除操作不可撤销，删除的文件/文件夹无法恢复，请谨慎使用
2. **批量删除**：支持同时删除多个文件/文件夹
3. **递归删除**：如果删除的是文件夹，会递归删除文件夹内的所有内容
4. **权限要求**：需要对要删除的路径有写入权限
5. **使用场景**：主要用于工具栏删除按钮，当选中文件或文件夹后，点击删除按钮会弹出确认对话框，确认后执行删除操作
6. **确认机制**：前端应在调用此接口前显示确认对话框，防止误删
7. **数据库更新**：删除后会更新数据库中的 `deleted_at` 字段（软删除），保留文件记录和标签关联信息，便于后续恢复或查询历史记录

---

## 标签管理接口

### 9. get_tag_list - 获取标签列表

**功能描述**：根据指定排序模式获取标签列表，可按使用次数或最近更新时间排序。用于在工具栏标签面板中显示常用或最近使用的标签。

**接口名称**：`get_tag_list`

**调用方式**：
```typescript
import { invoke } from '@tauri-apps/api/core';

// 获取使用次数最多的标签（默认）
const mostUsedTags = await invoke<Tag[]>('get_tag_list', {
  limit: 10,
  mode: 'most_used',
});

// 获取最近使用的标签
const recentUsedTags = await invoke<Tag[]>('get_tag_list', {
  limit: 10,
  mode: 'recent_used',
});
```

#### 请求参数

**Rust 后端**：
```rust
#[tauri::command]
pub async fn get_tag_list(
    db: State<'_, GlobalDatabase>,
    limit: Option<i32>,
    mode: Option<String>,
) -> Result<Vec<Tag>, String>
```

**参数说明**：

| 参数名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| `limit` | `Option<i32>` | 否 | 返回的标签数量限制，默认为 10 |
| `mode`  | `Option<String>` | 否 | 排序模式：`"most_used"`（默认，按 `usage_count` 降序）或 `"recent_used"`（按 `updated_at` 降序） |

**TypeScript 前端**：
```typescript
type TagListMode = 'most_used' | 'recent_used';

interface GetTagListRequest {
  limit?: number;
  mode?: TagListMode;
}
```

#### 返回数据

**成功返回**：`Tag[]` 标签数组

**错误返回**：`String` 错误信息

**常见错误**：
- `"获取数据库连接失败: {error}"` - 无法获取数据库连接
- `"查询标签失败: {error}"` - 数据库查询失败

### 10. search_tags - 搜索标签

**功能描述**：根据关键词搜索包含该文字的标签名称（模糊匹配）。用于在标签工具栏中快速查找标签。

**接口名称**：`search_tags`

**调用方式**：
```typescript
import { invoke } from '@tauri-apps/api/core';

const searchResults = await invoke<Tag[]>('search_tags', {
  keyword: '旅游',
  limit: 50,
});
```

#### 请求参数

**Rust 后端**：
```rust
#[tauri::command]
pub async fn search_tags(
    db: State<'_, GlobalDatabase>,
    keyword: String,
    limit: Option<i32>,
) -> Result<Vec<Tag>, String>
```

**参数说明**：

| 参数名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| `keyword` | `String` | 是 | 搜索关键词（标签名称中包含该文字即匹配） |
| `limit`  | `Option<i32>` | 否 | 返回的标签数量限制，默认为 50 |

**TypeScript 前端**：
```typescript
interface SearchTagsRequest {
  keyword: string;
  limit?: number;
}
```

#### 返回数据

**成功返回**：`Tag[]` 匹配的标签数组（按使用次数降序排列）

**错误返回**：`String` 错误信息

**常见错误**：
- `"获取数据库连接失败: {error}"` - 无法获取数据库连接
- `"搜索标签失败: {error}"` - 数据库查询失败

#### 数据结构

返回的数据结构与 `get_tag_list` 相同，参见 [Tag - 标签](#tag---标签) 数据结构定义。

#### 使用示例

```typescript
// 搜索包含"旅游"的标签
const tags = await invoke<Tag[]>('search_tags', {
  keyword: '旅游',
  limit: 50,
});

// 搜索结果会包含名称中包含"旅游"的所有标签
// 例如："旅游"、"旅游/日本"、"我的旅游照片" 等
```

#### 注意事项

1. **模糊匹配**：搜索使用 SQL 的 `LIKE`（SQLite）或 `ILIKE`（PostgreSQL，大小写不敏感）进行模糊匹配
2. **排序规则**：结果按 `usage_count` 降序排列，使用次数多的标签排在前面
3. **空关键词**：如果关键词为空，将返回空数组

### 11. create_tag - 创建新标签

**功能描述**：根据给定名称创建一个新的标签，其它字段使用数据库默认值。用于在标签工具栏中快速新建标签。

**接口名称**：`create_tag`

**调用方式**：
```typescript
import { invoke } from '@tauri-apps/api/core';
import type { Tag } from '../types/tag';

const newTag = await invoke<Tag>('create_tag', {
  name: '旅游/日本',
});
```

#### 请求参数

**Rust 后端**：
```rust
#[tauri::command]
pub async fn create_tag(
    db: State<'_, GlobalDatabase>,
    name: String,
) -> Result<Tag, String>
```

**参数说明**：

| 参数名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| `name` | `String` | 是 | 标签名称（不能为空，不能与现有标签重复） |

**TypeScript 前端**：
```typescript
interface CreateTagRequest {
  name: string;
}
```

#### 返回数据

**成功返回**：`Tag` 新创建的标签对象，字段与 `Tag` 数据结构一致。

**错误返回**：`String` 错误信息

**常见错误**：
- `"标签名称不能为空"` - 传入的名称为空或仅空白字符
- `"标签 \"{name}\" 已存在"` - 已存在同名且未删除的标签
- `"获取数据库连接失败: {error}"` - 无法获取数据库连接
- `"创建标签失败: {error}"` - 数据库插入或查询失败

#### 数据结构

**Rust 后端** (`src-tauri/src/models/tag.rs`)：
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    /// 标签ID
    pub id: i32,
    /// 标签名称
    pub name: String,
    /// 标签背景颜色（HEX颜色代码，如#FFFF00）
    pub color: Option<String>,
    /// 标签字体颜色（HEX颜色代码，如#000000）
    pub font_color: Option<String>,
    /// 父标签ID（用于层级标签）
    pub parent_id: Option<i32>,
    /// 使用次数统计
    pub usage_count: i32,
    /// 创建时间
    pub created_at: String,
    /// 更新时间
    pub updated_at: String,
}
```

**TypeScript 前端**：
```typescript
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
```

#### 使用示例

**前端调用**：
```typescript
import { invoke } from '@tauri-apps/api/core';
import type { Tag } from '../types/tag';

async function loadMostUsedTags(limit: number = 10): Promise<Tag[]> {
  try {
    const tags = await invoke<Tag[]>('get_most_used_tags', { limit });
    return tags;
  } catch (error) {
    console.error('加载标签失败:', error);
    throw error;
  }
}

// 使用示例
const tags = await loadMostUsedTags(10);
tags.forEach(tag => {
  console.log(`${tag.name}: ${tag.usage_count} 次使用`);
});
```

**后端实现** (`src-tauri/src/commands.rs`)：
```rust
#[tauri::command]
pub async fn get_most_used_tags(
    db: State<'_, GlobalDatabase>,
    limit: Option<i32>,
) -> Result<Vec<crate::models::tag::Tag>, String> {
    TagService::get_most_used_tags(&*db, limit).await
}
```

#### 注意事项

1. **排序规则**：标签按 `usage_count` 降序排列，如果使用次数相同，则按 `id` 升序排列
2. **软删除**：只返回未删除的标签（`deleted_at IS NULL`）
3. **默认限制**：如果不指定 `limit`，默认返回 10 个标签
4. **时间格式**：`created_at` 和 `updated_at` 使用 ISO 8601 格式字符串

### 12. modify_tag - 修改标签

**功能描述**：修改指定标签的信息，可以修改标签名称、背景颜色、字体颜色和父级标签。用于在标签管理界面中编辑标签属性。

**接口名称**：`modify_tag`

**调用方式**：
```typescript
import { invoke } from '@tauri-apps/api/core';
import type { Tag } from '../types/tag';

const modifiedTag = await invoke<Tag>('modify_tag', {
  id: 1,
  name: '新标签名',
  color: '#FF0000',
  font_color: '#FFFFFF',
  parent_id: null,
});
```

#### 请求参数

**Rust 后端**：
```rust
#[tauri::command]
pub async fn modify_tag(
    db: State<'_, GlobalDatabase>,
    id: i32,
    name: Option<String>,
    color: Option<Option<String>>,
    font_color: Option<Option<String>>,
    parent_id: Option<Option<i32>>,
) -> Result<Tag, String>
```

**参数说明**：

| 参数名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| `id` | `i32` | 是 | 要修改的标签ID |
| `name` | `Option<String>` | 否 | 新标签名称（None表示不修改，Some("")会报错） |
| `color` | `Option<Option<String>>` | 否 | 新背景颜色（None表示不修改，Some(None)表示设置为NULL，Some("#FF0000")表示设置为指定颜色） |
| `font_color` | `Option<Option<String>>` | 否 | 新字体颜色（None表示不修改，Some(None)表示设置为NULL，Some("#FFFFFF")表示设置为指定颜色） |
| `parent_id` | `Option<Option<i32>>` | 否 | 新父标签ID（None表示不修改，Some(None)表示设置为NULL，Some(123)表示设置为指定父标签） |

**TypeScript 前端**：
```typescript
interface ModifyTagRequest {
  id: number;
  name?: string;
  color?: string | null;  // null表示设置为NULL，undefined表示不修改
  font_color?: string | null;  // null表示设置为NULL，undefined表示不修改
  parent_id?: number | null;  // null表示设置为NULL，undefined表示不修改
}
```

#### 返回数据

**成功返回**：`Tag` 修改后的标签对象，字段与 `Tag` 数据结构一致。

**错误返回**：`String` 错误信息

**常见错误**：
- `"标签 ID {id} 不存在"` - 指定的标签ID不存在或已被删除
- `"标签名称不能为空"` - 传入的名称为空或仅空白字符
- `"标签 \"{name}\" 已存在"` - 新名称与其他标签重复
- `"获取数据库连接失败: {error}"` - 无法获取数据库连接
- `"修改标签失败: {error}"` - 数据库更新失败

#### 数据结构

返回的数据结构与 `create_tag` 相同，参见 [Tag - 标签](#tag---标签) 数据结构定义。

#### 使用示例

**前端调用**：
```typescript
import { invoke } from '@tauri-apps/api/core';
import type { Tag } from '../types/tag';

// 只修改标签名称
const tag1 = await invoke<Tag>('modify_tag', {
  id: 1,
  name: '新标签名',
});

// 只修改背景颜色和字体颜色
const tag2 = await invoke<Tag>('modify_tag', {
  id: 1,
  color: '#FF0000',
  font_color: '#FFFFFF',
});

// 修改多个字段
const tag3 = await invoke<Tag>('modify_tag', {
  id: 1,
  name: '新标签名',
  color: '#FF0000',
  font_color: '#FFFFFF',
  parent_id: 2,
});

// 将颜色设置为NULL（使用null值）
const tag4 = await invoke<Tag>('modify_tag', {
  id: 1,
  color: null,  // 设置为NULL
  font_color: null,  // 设置为NULL
});

// 不修改颜色（不传color字段或传undefined）
const tag5 = await invoke<Tag>('modify_tag', {
  id: 1,
  name: '新标签名',
  // color和font_color不传，表示不修改
});
```

**后端实现** (`src-tauri/src/commands.rs`)：
```rust
#[tauri::command]
pub async fn modify_tag(
    db: State<'_, GlobalDatabase>,
    id: i32,
    name: Option<String>,
    color: Option<Option<String>>,
    font_color: Option<Option<String>>,
    parent_id: Option<Option<i32>>,
) -> Result<Tag, String> {
    TagService::modify_tag(&*db, id, name, color, font_color, parent_id).await
}
```

#### 注意事项

1. **字段修改规则**：
   - 如果某个字段传入 `None`（TypeScript中为 `undefined`），表示不修改该字段
   - 如果传入 `Some(None)`（TypeScript中为 `null`），表示将该字段设置为 `NULL`
   - 如果传入 `Some(value)`（TypeScript中为具体值），表示将该字段设置为指定值

2. **名称验证**：如果提供了新名称，系统会检查名称是否为空以及是否与其他标签重复

3. **标签存在性**：修改前会检查标签是否存在，如果不存在会返回错误

4. **自动更新时间**：修改标签时，`updated_at` 字段会自动更新为当前时间

5. **颜色格式**：颜色值应使用 HEX 格式（如 `#FF0000`），但系统不会强制验证格式

### 13. add_tags_to_files - 批量添加标签到文件/文件夹

**功能描述**：为指定的文件/文件夹列表批量添加标签。如果文件记录在数据库中不存在，会自动创建文件记录。用于在文件管理器中选中文件/文件夹后，点击标签为其添加标签。

**接口名称**：`add_tags_to_files`

**调用方式**：
```typescript
import { invoke } from '@tauri-apps/api/core';

await invoke('add_tags_to_files', {
  paths: ['C:\\Users\\Username\\file1.txt', 'C:\\Users\\Username\\folder1'],
  tag_id: 1,
});
```

#### 请求参数

**Rust 后端**：
```rust
#[tauri::command]
pub async fn add_tags_to_files(
    db: State<'_, GlobalDatabase>,
    paths: Vec<String>,
    tag_id: i32,
) -> Result<(), String>
```

**参数说明**：

| 参数名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| `paths` | `Vec<String>` | 是 | 要添加标签的文件/文件夹路径列表 |
| `tag_id` | `i32` | 是 | 标签ID |

**TypeScript 前端**：
```typescript
interface AddTagsToFilesRequest {
  paths: string[];
  tag_id: number;
}
```

#### 返回数据

**成功返回**：无返回值（`void`）

**错误返回**：`String` 错误信息

**常见错误**：
- `"标签 ID {tag_id} 不存在"` - 指定的标签ID不存在或已被删除
- `"路径不存在: {path}"` - 指定的路径不存在
- `"获取文件元数据失败 {path}: {error}"` - 获取文件元数据时发生错误
- `"获取数据库连接失败: {error}"` - 无法获取数据库连接
- `"创建文件记录失败: {error}"` - 创建文件记录时发生错误
- `"添加标签关联失败: {error}"` - 添加文件-标签关联时发生错误

#### 使用示例

**前端调用**：
```typescript
import { invoke } from '@tauri-apps/api/core';

async function addTagToSelectedFiles(paths: string[], tagId: number): Promise<void> {
  try {
    await invoke('add_tags_to_files', {
      paths,
      tag_id: tagId,
    });
    console.log('添加标签成功');
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : String(error);
    console.error('添加标签失败:', errorMessage);
    throw error;
  }
}

// 使用示例：为选中的文件和文件夹添加标签
await addTagToSelectedFiles(
  ['C:\\Users\\Username\\file1.txt', 'C:\\Users\\Username\\folder1'],
  1
);
```

**后端实现** (`src-tauri/src/commands.rs`)：
```rust
#[tauri::command]
pub async fn add_tags_to_files(
    db: State<'_, GlobalDatabase>,
    paths: Vec<String>,
    tag_id: i32,
) -> Result<(), String> {
    TagService::add_tags_to_files(&*db, paths, tag_id).await
}
```

#### 注意事项

1. **自动创建文件记录**：如果文件/文件夹在数据库中不存在，系统会自动创建文件记录
2. **重复添加**：如果文件已经拥有该标签，不会重复添加（使用 `ON CONFLICT DO NOTHING` 或 `INSERT OR IGNORE`）
3. **文件夹处理**：文件夹的 `file_type` 为 `"folder"`，`file_size` 为 `0`
4. **标签使用次数**：添加标签后，会自动更新标签的 `usage_count` 字段
5. **批量操作**：支持同时为多个文件/文件夹添加标签
6. **路径验证**：系统会验证路径是否存在，如果路径不存在会返回错误

---

## 示例命令

### 11. greet - 问候命令

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
| `path` | `String` | `string` | 是 | 当前目录的完整路径。特殊值：当调用 `list_drives()` 时，此字段为 `"drives:"` |
| `parent_path` | `Option<String>` | `string \| undefined` | 否 | 父目录路径。驱动盘根目录（如 `C:\`）的 `parent_path` 为 `"drives:"`（用于返回驱动盘列表）；驱动盘列表的 `parent_path` 为 `None` |
| `items` | `Vec<FileItem>` | `FileItem[]` | 是 | 目录下的文件和文件夹列表，已排序（文件夹在前，然后按名称排序）。驱动盘列表中，`items` 包含所有可用的驱动盘 |
| `total_files` | `usize` | `number` | 是 | 目录中的文件总数（不包括文件夹）。驱动盘列表中此值为 `0` |
| `total_folders` | `usize` | `number` | 是 | 目录中的文件夹总数。驱动盘列表中此值为可用驱动盘的数量 |

#### 注意事项

1. **排序规则**：`items` 数组已排序，排序规则为：文件夹在前，文件在后，同类型按名称排序。驱动盘列表按字母顺序排序（A-Z）
2. **隐藏文件**：隐藏文件（以 `.` 开头的文件）会被过滤，不会出现在 `items` 中
3. **父路径特殊值**：
   - 普通目录：`parent_path` 为父目录的路径
   - 驱动盘根目录（如 `C:\`）：`parent_path` 为 `"drives:"`（用于返回驱动盘列表）
   - 驱动盘列表（`path === "drives:"`）：`parent_path` 为 `None`（最顶层，无父路径）
4. **统计信息**：`total_files` 和 `total_folders` 只统计非隐藏的文件和文件夹
5. **驱动盘列表**：当 `path === "drives:"` 时，表示这是驱动盘列表视图，`items` 中的每个项代表一个驱动盘

---

## 🔧 接口注册

所有接口需要在 `src-tauri/src/lib.rs` 中注册：

```rust
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::list_directory,
            commands::get_home_directory,
            commands::list_drives,
            commands::check_path_exists,
            commands::cut_files,
            commands::copy_files,
            commands::rename_file,
            commands::delete_files,
            commands::get_tag_list,
            commands::search_tags,
            commands::create_tag
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

## 数据结构定义

### Tag - 标签

**用途**：表示一个标签的信息。

#### Rust 后端定义

**位置**：`src-tauri/src/models/tag.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    /// 标签ID
    pub id: i32,
    /// 标签名称
    pub name: String,
    /// 标签背景颜色（HEX颜色代码，如#FFFF00）
    pub color: Option<String>,
    /// 标签字体颜色（HEX颜色代码，如#000000）
    pub font_color: Option<String>,
    /// 父标签ID（用于层级标签）
    pub parent_id: Option<i32>,
    /// 使用次数统计
    pub usage_count: i32,
    /// 创建时间
    pub created_at: String,
    /// 更新时间
    pub updated_at: String,
}
```

#### TypeScript 前端定义

**位置**：`src/types/tag.ts`

```typescript
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
```

#### 字段说明

| 字段名 | Rust 类型 | TypeScript 类型 | 必填 | 说明 |
|--------|-----------|-----------------|------|------|
| `id` | `i32` | `number` | 是 | 标签唯一标识符 |
| `name` | `String` | `string` | 是 | 标签名称 |
| `color` | `Option<String>` | `string \| null` | 否 | 标签背景颜色，HEX格式（如 `#FFFF00`），默认值为 `#FFFF00` |
| `font_color` | `Option<String>` | `string \| null` | 否 | 标签字体颜色，HEX格式（如 `#000000`），默认值为 `#000000` |
| `parent_id` | `Option<i32>` | `number \| null` | 否 | 父标签ID，用于层级标签结构 |
| `usage_count` | `i32` | `number` | 是 | 标签使用次数统计 |
| `created_at` | `String` | `string` | 是 | 创建时间，ISO 8601 格式 |
| `updated_at` | `String` | `string` | 是 | 更新时间，ISO 8601 格式 |

#### 注意事项

1. **颜色格式**：
   - `color` 字段用于标签背景颜色，使用 HEX 颜色代码格式（如 `#FFFF00`），默认值为 `#FFFF00`（黄色）
   - `font_color` 字段用于标签字体颜色，使用 HEX 颜色代码格式（如 `#000000`），默认值为 `#000000`（黑色）
   - 如果未设置则为 `null`，前端应使用默认值
2. **层级标签**：通过 `parent_id` 字段支持层级标签结构，根标签的 `parent_id` 为 `null`
3. **使用统计**：`usage_count` 字段记录标签被使用的次数，用于排序和推荐
4. **时间格式**：`created_at` 和 `updated_at` 使用 ISO 8601 格式字符串

---

## 📅 版本记录

### v1.6.0 (2025-12-XX)
- 添加 `delete_files` 接口，支持删除文件/文件夹（支持递归删除）
- 工具栏添加删除按钮，当选中文件/文件夹时启用
- 删除操作前显示确认对话框，防止误删
- 删除完成后自动刷新当前目录

### v1.5.0 (2025-12-XX)
- 添加 `rename_file` 接口，支持重命名文件/文件夹
- 工具栏添加重命名按钮，仅当选中单个文件/文件夹时启用
- 文件列表支持编辑模式，文件名显示区域可切换为输入框
- 优化重命名交互：按回车键完成重命名，点击其他区域或按 Esc 键取消

### v1.4.0 (2025-12-XX)
- 添加 `get_tag_list` 接口，支持获取使用数量最多的标签和最近使用的标签
- 工具栏添加标签图标、展开/收起功能和排序下拉菜单
- 添加并扩展 `Tag` 数据结构定义（支持背景色和字体颜色）

### v1.3.0 (2025-12-XX)
- 添加 `cut_files` 接口，支持剪切文件/文件夹
- 添加 `copy_files` 接口，支持复制文件/文件夹
- 工具栏组件支持剪切、复制、粘贴功能

### v1.2.0 (2025-12-XX)
- 添加 `check_path_exists` 接口，支持检查路径是否存在且为目录
- 导航栏路径显示改为可编辑输入框，支持直接输入路径跳转
- 优化路径输入体验，支持 ESC 键取消输入

### v1.1.0 (2025-12-XX)
- 添加 `list_drives` 接口，支持获取 Windows 驱动盘列表
- 优化 `list_directory` 接口，支持驱动盘根目录的特殊处理（`parent_path` 为 `"drives:"`）
- 更新 `DirectoryInfo` 数据结构说明，添加驱动盘列表相关说明

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
**文档版本**：v1.4.0

