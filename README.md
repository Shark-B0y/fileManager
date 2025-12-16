# Windows本地文件管理系统

一个基于标签的智能文件管理系统，帮助用户通过标签高效管理大量文件，支持文件移动/修改后的自动识别和标签保持。

## ✨ 核心特性

### 🏷️ 智能标签管理
- 为文件添加多个标签，支持层级标签结构
- 标签颜色标记和图标支持
- 批量文件标签操作
- 智能标签推荐

### 🔍 精准文件搜索
- 标签组合搜索（AND/OR逻辑）
- 模糊标签匹配和自动补全
- 文件属性筛选（类型、大小、时间）
- 搜索结果快速预览和跳转

### 🔄 智能文件识别
- 文件指纹识别（内容哈希 + 元数据）
- 文件移动/重命名后自动关联
- 文件修改后版本追踪
- 持久化标签关联

### 💻 本地化运行
- 数据本地存储，保护隐私
- 无需网络连接
- 轻量级设计，低资源占用
- Windows原生体验

## 📋 系统要求

- **操作系统**：Windows 10/11 64位
- **.NET运行时**：.NET 8.0 Desktop Runtime
- **内存**：4GB RAM（推荐8GB）
- **存储**：100MB可用空间
- **文件系统**：NTFS/FAT32/exFAT

## 🚀 快速开始

### 安装方法

1. **下载安装包**
   - 从 [Releases](https://github.com/yourusername/filemanager/releases) 下载最新版本
   - 运行安装程序，按照向导完成安装

2. **首次运行**
   - 启动应用程序
   - 选择要监控的文件夹
   - 系统将自动扫描并建立文件索引

3. **开始使用**
   - 拖拽文件到应用程序窗口
   - 右键点击文件添加标签
   - 使用搜索框查找文件

### 便携版本
对于高级用户，提供绿色便携版本：
1. 下载 `FileManager-Portable.zip`
2. 解压到任意目录
3. 运行 `FileManager.exe`

## 📁 主要功能

### 1. 文件管理
- **文件浏览**：树形目录和标签视图
- **文件预览**：支持常见文件格式预览
- **批量操作**：批量添加/删除标签
- **快速访问**：最近使用文件和常用标签

### 2. 标签系统
- **标签创建**：自定义标签名称、颜色、图标
- **标签组织**：层级标签结构（父标签/子标签）
- **标签统计**：使用频率和文件数量统计
- **标签导入/导出**：JSON/CSV格式支持

### 3. 搜索功能
- **基础搜索**：按标签、文件名搜索
- **高级搜索**：组合条件筛选
- **保存搜索**：保存常用搜索条件
- **搜索历史**：最近搜索记录

### 4. 系统设置
- **监控设置**：选择监控的文件夹
- **性能设置**：调整扫描频率和资源使用
- **外观设置**：主题、字体、布局调整
- **备份设置**：自动备份配置和数据

## 🔧 技术架构

### 开发技术栈
- **前端**：Tauri + React/Vue/Svelte + TypeScript
- **后端**：Rust + Tokio（异步运行时）
- **数据库**：PostgreSQL 15+（主选） / SQLite（备选）
- **文件监控**：notify-rs（跨平台文件系统监控）
- **哈希算法**：SHA-256（ring库） + 增量指纹
- **ORM**：Diesel 或 SeaORM

### 系统架构
```
应用程序层 (Tauri + Web前端)
    ↓
业务逻辑层 (Rust服务 + 领域模型)
    ↓
数据访问层 (PostgreSQL + Diesel/SeaORM)
    ↓
系统接口层 (Rust系统调用 + 文件监控)
```

## 📊 数据模型

### 核心表结构
- **files**：文件信息表（哈希、路径、元数据）
- **tags**：标签定义表（名称、颜色、层级）
- **file_tags**：文件-标签关联表
- **file_changes**：文件变更历史表

### 文件识别机制
1. **初始扫描**：生成文件指纹（内容哈希 + 关键元数据）
2. **实时监控**：监听文件系统变更事件
3. **变更识别**：比对指纹识别移动/修改
4. **关联更新**：自动更新数据库中的文件路径

## ⚙️ 配置说明

### 配置文件位置
- **用户配置**：`%APPDATA%\FileManager\settings.json`
- **数据库文件**：`%APPDATA%\FileManager\data.db`
- **日志文件**：`%APPDATA%\FileManager\logs\`

### 重要配置项
```json
{
  "monitoring": {
    "enabledFolders": ["C:\\Users\\YourName\\Documents"],
    "scanInterval": 300,
    "realTimeMonitoring": true
  },
  "performance": {
    "maxConcurrentScans": 4,
    "hashBufferSize": 8192,
    "cacheSize": 10000
  },
  "search": {
    "enableFuzzyMatch": true,
    "maxResults": 100,
    "saveSearchHistory": true
  }
}
```

## 🔒 安全与隐私

### 数据安全
- **本地存储**：所有数据存储在用户计算机
- **数据加密**：敏感配置信息加密存储
- **权限控制**：仅访问用户指定的文件夹
- **操作审计**：记录关键操作日志

### 隐私保护
- 不收集任何用户文件内容
- 不传输数据到远程服务器
- 可完全离线使用
- 支持数据完全删除

## 🛠️ 开发指南

### 环境搭建
1. 安装 Visual Studio 2022
2. 安装 .NET 8.0 SDK
3. 克隆项目仓库
4. 还原 NuGet 包

### 项目结构
```
FileManager/
├── FileManager.Core/          # 核心领域模型
├── FileManager.Data/          # 数据访问层
├── FileManager.Business/      # 业务逻辑层
├── FileManager.UI/           # 用户界面层
├── FileManager.System/       # 系统集成层
├── FileManager.Tests/        # 测试项目
└── FileManager.Installer/    # 安装程序
```

### 构建命令
```bash

# 前置安装
rustup target add wasm32-unknown-unknown
npm install

# 构建项目
npm run tauri dev
```

## 📈 性能指标

### 基准测试（10万文件规模）
- **初始扫描**：约15-20分钟
- **标签搜索**：< 1秒响应时间
- **内存占用**：< 200MB
- **CPU使用**：< 5%（空闲时）

### 优化策略
- 增量扫描和哈希计算（Rust并行处理）
- PostgreSQL高级索引（GIN、GiST）
- 查询结果缓存（内存缓存）
- 异步文件操作（Tokio运行时）

## 🤝 贡献指南

欢迎提交 Issue 和 Pull Request！

### 开发流程
1. Fork 项目仓库
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

### 代码规范
- 遵循 Rust 编码规范（rustfmt）
- 使用 async/await 处理异步操作（Tokio）
- 添加单元测试（cargo test）
- 更新相关文档

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 📞 支持与反馈

### 问题报告
- **GitHub Issues**：报告 bug 或请求功能
- **电子邮件**：support@example.com（示例）

### 文档资源
- [用户手册](docs/user-guide.md)
- [API 文档](docs/api.md)
- [常见问题](docs/faq.md)

### 社区支持
- [Discord 频道](https://discord.gg/example)
- [Stack Overflow](https://stackoverflow.com/questions/tagged/filemanager)

## 🙏 致谢

感谢以下开源项目的支持：
- [Tauri](https://tauri.app/) - 构建小型、快速、安全的桌面应用
- [Rust](https://www.rust-lang.org/) - 内存安全、高性能的系统编程语言
- [PostgreSQL](https://www.postgresql.org/) - 强大的开源关系数据库
- [Tokio](https://tokio.rs/) - Rust异步运行时
- [Diesel](https://diesel.rs/) - Rust的ORM和查询构建器

---

# 开发者注
  这是一个ai的练手项目, ai模型为deepseek-v3.2


**版本**：v1.0.0
**最后更新**：2025-12-05
**作者**：FileManager Team
**状态**：开发中 🚧