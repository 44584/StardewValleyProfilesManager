一款管理星露谷模组的管理器.

# 需求分析

## 功能性需求
- 模组扫描与注册：自动检测指定目录下的有效星露谷模组，解析manifest.json并存储到本地数据库
- 配置管理：支持创建、删除、编辑多个独立的模组配置方案（Profile）
- 模组关联操作：在配置维度上实现模组的添加、移除、启用、禁用功能
- 游戏启动：通过命令行参数指定模组路径，一键启动SMAPI并加载选定配置
## 非功能性需求
- 空间效率：避免重复存储大容量模组文件，采用符号链接实现零拷贝配置
- 操作便捷性：提供图形用户界面，简化复杂的模组管理流程
- 数据一致性：确保文件系统状态与数据库记录同步
- 平台兼容性：当前支持Windows平台（需开发者模式），具备跨平台扩展潜力
- 性能优化：使用缓存机制减少频繁的磁盘I/O和数据库查询
## 约束条件
- 依赖约束：必须安装SMAPI才能正常启动游戏
- 权限约束：Windows平台需要开发者模式或管理员权限以创建符号链接


# 概要设计

## 技术架构

### 整体技术方案
采用 **Tauri (React + TypeScript + Rust)** 技术栈构建跨平台桌面应用：

- **前端**: React ^19.1.0 + TypeScript ~5.8.3 + Vite ^7.0.4
- **后端**: Rust (通过 Tauri ^2 框架集成)
- **数据存储**: SQLite (通过 rusqlite crate v0.38.0)

### 核心架构模式
**双支柱架构**：

**支柱一：命令行游戏启动机制**
- 利用 SMAPI 的 `--mods-path` 参数动态指定模组加载目录
- Rust 后端通过 `std::process::Command` 构造并执行启动命令
- 前端通过 Tauri API 调用后端命令，实现一键启动

**支柱二：符号链接零拷贝配置**
- 每个 Profile 对应独立目录（如 `Profiles/休闲种田`）
- Profile 目录内包含指向实际模组的符号链接
- 使用 `std::os::windows::fs::symlink_dir()` 创建符号链接
- 避免重复存储大容量模组文件，节省磁盘空间

### 分层架构
```
┌─────────────────┐
│    UI Layer     │ ← React + TypeScript (src/)
│  (Frontend)     │
└────────┬────────┘
         │ Tauri API Bridge
┌────────▼────────┐
│   Core Layer    │ ← Rust (src-tauri/src/)
│   (Backend)     │
└────────┬────────┘
         │ System APIs
┌────────▼────────┐
│  System Layer   │ ← Windows File System, Process Management, SQLite DB
└─────────────────┘
```

## 数据库设计

### 设计原则
- **轻量级**: SQLite 单文件数据库，无需额外服务进程
- **ACID合规**: 确保数据一致性和完整性
- **高效查询**: 为常用查询场景建立索引
- **可扩展**: 预留扩展字段支持未来功能

### 表结构设计

#### 1. mods 表 - 存储模组基本信息
```sql
CREATE TABLE mods (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    unique_id TEXT NOT NULL UNIQUE,           -- 模组唯一标识符 (YourName.YourProjectName)
    name TEXT NOT NULL,                       -- 模组名称
    author TEXT NOT NULL,                     -- 作者名称
    version TEXT NOT NULL,                    -- 版本号 (语义化版本)
    description TEXT,                         -- 描述信息
    entry_dll TEXT,                           -- DLL文件名 (SMAPI模组专用)
    content_pack_for TEXT,                    -- 内容包宿主 (内容包专用)
    minimum_api_version TEXT,                 -- 最低SMAPI版本要求
    dependencies_json TEXT,                   -- 依赖项JSON数组 (存储Dependencies字段)
    update_keys_json TEXT,                    -- 更新键JSON数组 (存储UpdateKeys字段)
    mod_path TEXT NOT NULL,                   -- 模组在文件系统中的实际路径
    manifest_hash TEXT NOT NULL,              -- manifest.json的哈希值，用于变更检测
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

**索引设计**:
- `unique_id` 字段建立唯一索引，确保模组唯一性
- `name` 字段建立普通索引，支持按名称搜索
- `author` 字段建立普通索引，支持按作者筛选
- `minimum_api_version` 字段建立索引，支持按SMAPI版本筛选兼容模组

#### 2. profiles 表 - 存储配置方案信息
```sql
CREATE TABLE profiles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,                -- 配置方案名称 (如"休闲种田")
    description TEXT,                         -- 配置描述
    profile_path TEXT NOT NULL,               -- Profile目录的实际路径
    is_active BOOLEAN DEFAULT FALSE,          -- 是否为当前激活的配置
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

**索引设计**:
- `name` 字段建立唯一索引，确保配置名称唯一性
- `is_active` 字段建立索引，快速查找当前激活配置

#### 3. profile_mods 表 - 存储模组与配置的关联关系
```sql
CREATE TABLE profile_mods (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    profile_id INTEGER NOT NULL,              -- 关联的Profile ID
    mod_id INTEGER NOT NULL,                  -- 关联的Mod ID
    is_enabled BOOLEAN DEFAULT TRUE,          -- 在该配置中是否启用
    link_path TEXT,                           -- 符号链接路径 (可为空，表示未创建链接)
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (profile_id) REFERENCES profiles(id) ON DELETE CASCADE,
    FOREIGN KEY (mod_id) REFERENCES mods(id) ON DELETE CASCADE,
    UNIQUE(profile_id, mod_id)                -- 确保同一模组在同一配置中只关联一次
);
```

**索引设计**:
- `(profile_id, mod_id)` 复合唯一索引，防止重复关联
- `profile_id` 字段建立外键索引，加速Profile相关查询
- `mod_id` 字段建立外键索引，加速Mod相关查询
- `is_enabled` 字段建立索引，快速筛选启用/禁用状态

### 数据库操作场景

#### 1. 模组注册场景
- 扫描Mods目录，解析每个manifest.json
- 计算manifest.json的哈希值作为变更检测依据
- 插入或更新mods表记录
- 如果是新模组，自动关联到所有现有Profile（默认启用）

#### 2. Profile管理场景
- 创建Profile：插入profiles表，创建对应目录
- 删除Profile：删除profiles表记录，级联删除profile_mods关联，清理文件系统
- 激活Profile：更新profiles表的is_active字段，确保只有一个激活配置

#### 3. 模组关联场景
- 添加模组到Profile：插入profile_mods记录，创建符号链接
- 移除模组：删除profile_mods记录，删除符号链接
- 启用/禁用模组：更新profile_mods的is_enabled字段，符号链接保持不变但启动时过滤

#### 4. 游戏启动场景
- 查询激活Profile的所有启用模组
- 验证符号链接状态，必要时重建
- 构造SMAPI启动命令

### 缓存策略
- **内存缓存**: 常用查询结果缓存（如活跃Profile的模组列表）
- **文件系统缓存**: manifest.json哈希值缓存，避免重复解析
- **数据库连接池**: 单线程应用，使用单个数据库连接复用

## 功能模块划分

### 前端模块 (React + TypeScript)
- **Profile 管理界面**: 创建、删除、编辑配置方案
- **Mod 浏览器**: 显示已注册的模组列表，支持搜索和筛选
- **Mod 关联操作**: 在配置维度上添加、移除、启用、禁用模组
- **游戏启动控制**: 一键启动SMAPI并加载选定配置
- **状态同步**: 实时显示文件系统与数据库的一致性状态

### 后端模块 (Rust)
- **Mod Scanner**: 使用 `walkdir` 扫描Mods目录，解析 `manifest.json`
- **Database Manager**: 使用 `rusqlite` 管理SQLite数据库操作
- **Link Manager**: 管理符号链接的创建、删除和维护
- **Game Launcher**: 构造并执行SMAPI启动命令
- **File System Service**: 处理文件系统操作和权限管理
- **Cache Manager**: 实现多层缓存策略

## 数据流设计

1. **模组注册流程**:
   ```
   Mods目录扫描 → manifest.json解析 → 计算哈希值 → 数据库存储 → 自动关联Profile → 前端展示
   ```

2. **配置创建流程**:
   ```
   用户输入配置名称 → 创建Profile目录 → 数据库存储 → 前端展示空配置
   ```

3. **模组关联流程**:
   ```
   用户选择模组 → 创建符号链接 → 更新profile_mods关联 → 前端刷新状态
   ```

4. **游戏启动流程**:
   ```
   选择Profile → 查询启用模组列表 → 验证符号链接 → 构造SMAPI命令 → 执行启动
   ```

## 关键技术组件

### Rust 依赖库
- **`walkdir` v2.5.0**: 高效目录遍历
- **`dirs` v6.0.0**: 系统目录路径获取
- **`rusqlite` v0.38.0**: SQLite数据库操作
- **`serde` + `serde_json`**: JSON序列化/反序列化
- **[toml](file://c:\code\fun_stuff\StardewProfilesManager\src-tauri\Cargo.toml) v0.9.10**: 配置文件支持

### Tauri 插件
- **@tauri-apps/api**: 核心API调用
- **@tauri-apps/plugin-opener**: 外部链接处理

## 平台兼容性考虑

- **当前重点**: Windows平台（需要开发者模式或管理员权限）
- **未来扩展**: 
  - Linux/macOS: 使用标准符号链接 (`ln -s`)
  - 跨平台抽象层设计，便于后续扩展

## 性能与安全设计

- **缓存机制**: 减少频繁的磁盘I/O和数据库查询
- **数据一致性**: 文件系统状态与数据库记录同步，支持自动修复
- **权限管理**: 明确的capabilities配置，最小权限原则
- **错误处理**: 完善的错误边界和用户友好的错误提示
- **事务支持**: 数据库操作使用事务确保原子性

# 背景介绍

## 核心支柱

本模组管理器基于两大核心技术支柱构建：

### 支柱一：命令行方式启动游戏并指定模组路径

SMAPI（Stardew Modding API）支持通过命令行参数 `--mods-path` 指定模组加载目录。启动命令格式如下：

```bash
"C:\Program Files (x86)\Steam\steamapps\common\Stardew Valley\StardewModdingAPI.exe" --mods-path "目标模组目录"
```

**关键特性：**
- SMAPI 会从指定目录加载所有符合要求的模组
- 目录路径可以是任意位置，不限于默认的 `Mods` 文件夹
- 这为动态切换模组配置提供了技术基础

**Rust 实现工具：**
- **`std::process::Command`**：Rust 标准库中的进程管理模块，用于构造和执行外部命令

### 支柱二：符号链接实现零拷贝模组配置

符号链接（Symbolic Links）是一种文件系统功能，允许创建指向其他文件或目录的"快捷方式"，但对应用程序透明。

**在本项目中的应用：**
- 每个模组配置（Profile）对应一个独立目录（如 `Profiles/休闲种田`）
- 配置目录内包含多个符号链接，每个链接指向实际的模组文件夹
- SMAPI 启动时无法区分符号链接和真实目录，能正常加载所有模组

**优势：**
1. **节省磁盘空间**：避免重复存储大容量模组文件（特别是高清纹理包）
2. **快速切换**：创建/删除链接比复制文件快得多
3. **保持一致性**：所有配置共享同一份模组文件，更新一次即可影响所有配置

**Rust 实现工具：**
- **`std::os::windows::fs::symlink_dir()`**：Windows 平台专用的符号链接创建函数
- **`walkdir` crate (v2.5.0)**：高效遍历目录结构，用于模组扫描
- **`dirs` crate (v6.0.0)**：获取系统标准目录路径（如用户数据目录）
- **实现位置**：`src/link_manager.rs` 中的 `create_link()` 和 `create_links()` 方法
- **平台限制**：需要 Windows 开发者模式或管理员权限

## 辅助技术栈

除了两大核心支柱外，项目还依赖以下 Rust 工具链：

### 数据持久化
- **`rusqlite` crate (v0.38.0)**：SQLite 数据库绑定，用于存储模组信息和配置关系
- **`serde` + `serde_json`**：序列化/反序列化框架，用于解析 `manifest.json`
- **`toml` crate (v0.9.10)**：配置文件格式支持

### 文件系统操作
- **`std::fs`**：标准库文件操作

## 工作流程

1. **模组注册**：使用 `walkdir` 扫描 `Mods` 目录，通过 `serde_json` 解析 `manifest.json`，将信息存入 `rusqlite` 数据库
2. **配置创建**：用户通过 `eframe` GUI 选择模组组合，系统调用 `symlink_dir()` 在 `Profiles` 目录下创建符号链接
3. **游戏启动**：使用 `std::process::Command` 构造启动命令，指定 `--mods-path` 为对应配置的符号链接目录
4. **SMAPI 加载**：SMAPI 读取配置目录中的符号链接，正常加载所有关联模组

---

# 模组 manifest.json 的说明

《星露谷物语》模组的 `manifest.json` 文件是每个 SMAPI 模组或内容包（Content Pack）必须包含的清单文件，用于向 SMAPI 提供模组的基本信息，以便识别、加载模组以及检查更新等。

---

## 基本格式

根据模组类型不同，`manifest.json` 有两种基本格式：

### 1. SMAPI 模组（含代码的 DLL 模组）

```json
{
  "Name": "Your Project Name",
  "Author": "your name",
  "Version": "1.0.0",
  "Description": "One or two sentences about the mod.",
  "UniqueID": "YourName.YourProjectName",
  "EntryDll": "YourDllFileName.dll",
  "UpdateKeys": []
}
```

### 2. 内容包（Content Pack，通常基于 Content Patcher）

```json
{
  "Name": "Your Project Name",
  "Author": "your name",
  "Version": "1.0.0",
  "Description": "One or two sentences about the mod.",
  "UniqueID": "YourName.YourProjectName",
  "UpdateKeys": [],
  "ContentPackFor": {
    "UniqueID": "Pathoschild.ContentPatcher"
  }
}
```

---

## 字段规范

### 必备字段

所有模组都必须指定以下字段：

| 字段 | 描述 | 示例 |
|------|------|------|
| `Name` | 模组名称，SMAPI 会在玩家消息、日志和错误中使用。 | `"Name": "Lookup Anything"` |
| `Author` | 模组作者的名字，建议使用发布模组时使用的用户名。 | `"Author": "Pathoschild"` |
| `Version` | 模组的语义化版本号，用于更新检查、模组依赖和兼容性黑名单。每次发布都必须更新。 | `"Version": "1.0.0"` 或 `"Version": "1.0.1-beta.2"` |
| `Description` | 对模组功能的简短描述（一两句话），会显示在 SMAPI 日志中。 | `"Description": "View metadata about anything by pressing a button."` |
| `UniqueID` | 模组的唯一标识符。推荐格式为 `<作者名>.<模组名>`，不能包含空格或特殊字符。发布后不应更改。 | `"UniqueID": "Pathoschild.LookupAnything"` |
| `EntryDll` 或 `ContentPackFor` | 二选一。`EntryDll` 用于 SMAPI 模组，指定模组文件夹中已编译的 DLL 文件名。`ContentPackFor` 用于内容包，指定可以读取该内容包的模组。 | `"EntryDll": "LookupAnything.dll"` 或 `"ContentPackFor": {"UniqueID": "Pathoschild.ContentPatcher"}` |

### 可选字段

| 字段 | 描述 | 示例 |
|------|------|------|
| `MinimumApiVersion` | 指定运行此模组所需的最低 SMAPI 版本。如果玩家使用更旧的 SMAPI 版本，会收到需要更新的友好提示。这也间接限定了最低游戏版本。 | `"MinimumApiVersion": "3.8.0"` |
| `Dependencies` | 定义此模组所依赖的其他模组（即"前置"）。如果未安装依赖项，模组将不会加载。 | `"Dependencies": [{"UniqueID": "SMAPI.ConsoleCommands", "MinimumVersion": "3.8.0"}]`<br>可将依赖标记为可选：`"IsRequired": false` |
| `UpdateKeys` | SMAPI 根据此字段检查更新。常见的更新键包括 Nexus Mods、GitHub、ModDrop 等平台的标识符。 | `"UpdateKeys": ["Nexus:1234", "Github:user/repo"]"` |
| `ContentPackFor.MinimumVersion` | 在 `ContentPackFor` 对象中可选，指定所需宿主模组的最低版本。 | `"ContentPackFor": {"UniqueID": "Pathoschild.ContentPatcher", "MinimumVersion": "1.0.0"}` |

---

## 重要说明

1. **互斥性**：`EntryDll` 和 `ContentPackFor` 是互斥的，不能同时指定。
2. **版本号**：建议使用语义化版本（如 `主版本.次版本.修订号`），并确保每次发布都更新版本号。
3. **唯一标识符**：`UniqueID` 是模组与外界交互的"名片"，其他模组需要通过它来引用你的模组。一旦发布，不应更改。
4. **额外字段**：模组作者可以自行添加其他属性，这些属性会存储在 `IManifest.ExtraFields` 字典中，但 SMAPI 不会识别它们。
5. **大小写敏感**：JSON 键名区分大小写，请严格按照上述格式书写。

> 以上规范主要基于 Stardew Valley Wiki 的官方文档。在编写 `manifest.json` 时，请务必填写所有必备字段，并根据需要添加可选字段，以确保模组能被 SMAPI 正确识别和加载。