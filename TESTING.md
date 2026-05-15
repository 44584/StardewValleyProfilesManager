# Stardew Profiles Manager 测试指南

本文档详细介绍项目的测试结构、相关文件以及如何运行和编写测试。

## 目录结构

```
src-tauri/
├── src/                    # 源代码目录
│   ├── lib.rs             # 主库入口，导出所有公共模块
│   └── ...                # 其他业务模块
└── tests/                 # 集成测试目录
    └── scanner_integration.rs  # Scanner模块集成测试
```

## 测试类型

### 1. 集成测试 (Integration Tests)

位于 `src-tauri/tests/` 目录下，用于测试模块间的集成和完整功能流程。

**当前测试文件**:
- `scanner_integration.rs` 模组扫描功能的完整测试

**特点**:
- 使用临时目录进行隔离测试
- 通过主库的公共 API 进行调用
- 测试真实的功能场景（如 manifest.json 解析）
- 不会污染真实的文件系统

### 2. 单元测试 (Unit Tests)

目前项目中尚未添加单元测试，但可以在各个模块内部添加：

```rust
// 在 src/scanner/mod_scanner.rs 中
#[cfg(test)]
mod tests {
    // 单元测试代码
}
```

## 如何运行测试

### 基本命令

```bash
# 切换到 src-tauri 目录
cd src-tauri

# 运行所有测试
cargo test

# 运行特定的集成测试
cargo test --test scanner_integration

# 运行库的单元测试（如果有）
cargo test --lib

# 运行特定测试函数
cargo test test_smapi_mod_parsing

# 显示详细输出
cargo test -- --nocapture
```

### Windows PowerShell 用户

由于 PowerShell 的语法限制，请使用以下命令：

```powershell
# 设置工作目录并运行测试
Set-Location C:\code\fun_stuff\StardewProfilesManager\src-tauri; cargo test

# 运行特定测试
Set-Location C:\code\fun_stuff\StardewProfilesManager\src-tauri; cargo test --test scanner_integration
```

## 当前测试覆盖

### Scanner 集成测试 (`scanner_integration.rs`)

包含 4 个核心测试用例：

1. **`test_smapi_mod_parsing`** - 测试 SMAPI 模组格式解析
   - 验证 PascalCase 字段名正确映射
   - 测试 EntryDll 字段处理
   - 验证依赖项和更新键的序列化

2. **`test_content_pack_parsing`** - 测试内容包格式解析
   - 验证 ContentPackFor 字段处理
   - 测试宿主模组信息提取

3. **`test_invalid_manifest_handling`** - 测试无效 manifest 处理
   - 确保无效 JSON 不会导致程序崩溃
   - 验证错误被正确跳过

4. **`test_missing_manifest_handling`** - 测试缺失 manifest 处理
   - 验证只有包含 manifest.json 的目录才会被处理
   - 确保其他目录被正确忽略

## 编写新测试

### 创建新的集成测试

1. 在 `src-tauri/tests/` 目录下创建新文件（如 `database_test.rs`）

2. 导入主库：
```rust
use stardewprofilesmanager_lib::your_module::YourStruct;
```

3. 使用 `tempfile` 创建临时资源（如数据库文件、目录等）：
```rust
use tempfile::TempDir;

#[test]
fn your_test_name() {
    let temp_dir = TempDir::new().unwrap();
    // 测试逻辑
}
```

### 添加单元测试

在源代码文件内部添加：

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_your_function() {
        // 测试逻辑
    }
}
```

## 测试依赖

项目使用以下测试相关依赖：

- **`tempfile`** - 创建临时目录和文件，确保测试隔离性
- **标准 Rust 测试框架** - 内置的 `#[test]` 宏和断言

在 `Cargo.toml` 中配置：
```toml
[dev-dependencies]
tempfile = "3.0"
```

## 最佳实践

### 1. 测试隔离性
- 始终使用 `tempfile::TempDir` 创建临时资源
- 避免在测试中使用真实路径或文件
- 确保测试完成后临时资源自动清理

### 2. 公共 API 测试
- 集成测试应通过主库导出的公共接口进行调用
- 不要直接访问内部模块结构
- 在 `lib.rs` 中使用 `pub mod` 导出需要测试的模块

### 3. 错误处理测试
- 测试正常情况和异常情况
- 验证错误消息的准确性
- 确保程序在错误情况下不会崩溃

### 4. 数据驱动测试
- 使用实际的 manifest.json 格式进行测试
- 覆盖不同的字段组合和边界情况
- 验证向后兼容性

## 调试测试

### 查看详细输出
```bash
cargo test -- --nocapture
```

### 运行单个测试
```bash
cargo test test_smapi_mod_parsing
```

### 启用调试日志
在测试代码中添加 `eprintln!` 语句，配合 `--nocapture` 参数查看。

## 常见问题

### 1. 测试失败：字段名不匹配
Stardew Valley 的 manifest.json 使用 **PascalCase** 字段名（如 `Name`, `Author`），确保 serde 结构体正确映射：

```rust
#[derive(Deserialize)]
struct ManifestBase {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Author")]
    author: String,
    // ...
}
```

### 2. 序列化错误
如果需要将结构体转换为 JSON 字符串存储，确保添加 `Serialize` trait：

```rust
#[derive(Deserialize, Serialize)]
struct Dependency {
    // ...
}
```

### 3. 路径分隔符问题
在 Windows 上使用 `\\` 而不是 `/`，但在 Rust 字符串中建议使用正斜杠或 `std::path::Path`：

```rust
// 推荐
let path = Path::new("C:/Program Files/Stardew Valley/Mods");

// 或者使用原始字符串
let path = r"C:\Program Files\Stardew Valley\Mods";
```

## 未来改进

1. **添加更多集成测试** - 为 database、link_manager、game_launcher 等模块添加测试
2. **添加单元测试** - 在关键函数内部添加细粒度测试
3. **CI/CD 集成** - 配置 GitHub Actions 自动运行测试
4. **测试覆盖率报告** - 使用 tarpaulin 等工具生成覆盖率报告

---

通过遵循本指南，您可以有效地测试和验证 Stardew Profiles Manager 的各项功能，确保代码质量和稳定性。