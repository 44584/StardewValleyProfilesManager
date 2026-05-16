一款管理星露谷模组的管理器.

# 需求分析

## 功能性需求
- 模组扫描与注册：自动检测指定目录下的有效星露谷模组，解析manifest.json并存储到本地数据库
- 配置管理：支持创建、删除、编辑多个独立的模组配置方案（Profile）
- 模组关联操作：在配置维度上实现模组的添加、移除、启用、禁用功能
- 游戏启动：通过命令行参数指定模组路径，一键启动SMAPI并加载选定配置
- **配置持久化**：自动保存和加载SMAPI可执行文件路径及Mods目录路径，提升用户体验
- **系统文件选择器**：集成操作系统原生文件/目录选择对话框，简化路径输入

## 非功能性需求
- 空间效率：避免重复存储大容量模组文件，采用符号链接实现零拷贝配置
- 操作便捷性：提供图形用户界面，简化复杂的模组管理流程
- 数据一致性：确保文件系统状态与数据库记录同步
- 平台兼容性：当前支持Windows平台（需开发者模式），具备跨平台扩展潜力
- 性能优化：使用缓存机制减少频繁的磁盘I/O和数据库查询
- **用户体验**：记住用户配置，避免重复输入相同路径信息

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

**支柱三：配置持久化管理**
- 应用配置（SMAPI路径、Mods目录）保存在JSON配置文件中
- 配置文件位置：`%APPDATA%\StardewProfilesManager\config.json`
- 应用启动时自动加载配置，状态变化时自动保存
- 提供系统原生文件/目录选择对话框，简化路径输入

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

## 数据存储设计

### 数据库文件位置
- **Windows**: `%APPDATA%\StardewProfilesManager\profiles.db` (`C:\Users\{username}\AppData\Roaming\StardewProfilesManager\profiles.db`)
- **macOS**: `~/Library/Application Support/StardewProfilesManager/profiles.db`
- **Linux**: `~/.local/share/StardewProfilesManager/profiles.db`

### 配置文件位置
- **配置文件**: `%APPDATA%\StardewProfilesManager\config.json`
- **Profile目录**: `%APPDATA%\StardewProfilesManager\Profiles\`
- **单个Profile**: `%APPDATA%\StardewProfilesManager\Profiles\{profile_name}`

### 配置文件结构
```json
{
  "smapi_path": "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Stardew Valley\\StardewModdingAPI.exe",
  "mods_directory": "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Stardew Valley\\Mods"
}
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
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

**索引设计**:
- `name` 字段建立唯一索引，确保配置名称唯一性

## 业务流程设计

#### 1. 应用初始化场景
- 应用启动时检查配置文件是否存在
- 如果存在，加载SMAPI路径和Mods目录到UI
- 如果不存在，显示空输入框等待用户配置

#### 2. 配置保存场景
- 用户修改SMAPI路径或Mods目录时
- 自动保存配置到config.json文件
- 配置保存失败时记录警告但不影响主要功能

#### 3. 文件选择场景
- 用户点击"浏览"按钮选择SMAPI可执行文件
- 调用系统文件选择对话框，过滤exe文件
- 用户点击"浏览"按钮选择Mods目录
- 调用系统目录选择对话框

#### 4. 模组扫描场景
- 扫描指定目录下的所有文件夹
- 解析每个文件夹下的manifest.json
- 验证manifest.json内容
- 插入mods表

#### 5. Profile管理场景
- 创建Profile：插入profiles表，创建对应目录
- 删除Profile：删除profiles表记录，级联删除profile_mods关联，清理文件系统

#### 6. 模组关联场景
- 添加模组到Profile：插入profile_mods记录，创建符号链接
- 移除模组：删除profile_mods记录，删除符号链接
- 启用/禁用模组：更新profile_mods的is_enabled字段，符号链接保持不变但启动时过滤

#### 7. 游戏启动场景
- 直接通过Profile ID获取对应Profile目录路径
- 查询指定Profile的所有启用模组
- 验证符号链接状态，必要时重建
- 构造SMAPI启动命令：`StardewModdingAPI.exe --mods-path "profile_directory_path"`

#### 8. **游戏启动流程**:
```  
选择Profile → 直接使用Profile ID启动 → 查询启用模组列表 → 验证/同步符号链接 → 构造SMAPI命令 → 执行启动
```

## Profile管理接口

### 创建Profile
- **命令**: `create_profile`
- **参数**: 
  - `name: string` - Profile名称（必填）
  - `description: string | null` - Profile描述（可选）
- **返回**: `Profile` - 创建的Profile对象
- **错误处理**: 
  - 名称为空时返回"Profile名称不能为空"
  - 名称重复时返回"Profile名称已存在"

### 删除Profile
- **命令**: `delete_profile`
- **参数**: `profile_id: number` - Profile ID（必填）
- **返回**: `void`
- **错误处理**: 
  - 无效ID时返回"无效的Profile ID"
  - Profile不存在时返回"未找到指定的Profile"

### 更新Profile
- **命令**: `update_profile`
- **参数**: 
  - `profile_id: number` - Profile ID（必填）
  - `name: string` - 新的Profile名称（必填）
  - `description: string | null` - 新的Profile描述（可选）
- **返回**: `void`
- **错误处理**: 
  - 无效ID时返回"无效的Profile ID"
  - 名称为空时返回"Profile名称不能为空"
  - Profile不存在时返回"未找到指定的Profile"

### 获取所有Profile
- **命令**: `get_all_profiles`
- **参数**: 无
- **返回**: `Profile[]` - Profile对象数组
- **错误处理**: 数据库操作失败时返回错误信息

### 根据ID获取Profile
- **命令**: `get_profile_by_id`
- **参数**: `profile_id: number` - Profile ID（必填）
- **返回**: `Profile | null` - Profile对象或null
- **错误处理**: 无效ID时返回"无效的Profile ID"

## 应用配置管理接口

### 保存应用配置
- **命令**: `save_app_config`
- **参数**: 
  - `smapi_path: string | null` - SMAPI可执行文件路径
  - `mods_directory: string | null` - Mods目录路径
- **返回**: `void`
- **错误处理**: 文件写入失败时返回错误信息

### 加载应用配置
- **命令**: `load_app_config`
- **参数**: 无
- **返回**: `AppConfig` - 包含smapi_path和mods_directory的对象
- **错误处理**: 文件读取或解析失败时返回空配置

## Mod管理接口