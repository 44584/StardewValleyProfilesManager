/// 文件系统相关工具函数
/// 包括路径处理、哈希计算、文件操作等通用功能
use std::path::Path;

/// 计算文件或目录的SHA256哈希值
/// 用于manifest.json变更检测和文件一致性验证
pub fn calculate_sha256_hash(path: &Path) -> Result<String, String> {
    // TODO: 实现SHA256哈希计算逻辑
    Ok("placeholder_hash".to_string())
}

/// 验证路径是否为有效目录
pub fn is_valid_directory(path: &Path) -> bool {
    path.exists() && path.is_dir()
}

/// 创建目录（包括父目录）
pub fn create_directory(path: &Path) -> Result<(), String> {
    std::fs::create_dir_all(path)
        .map_err(|e| e.to_string())
}