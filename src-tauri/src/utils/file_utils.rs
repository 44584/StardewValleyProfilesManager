use sha2::Digest;
/// 文件系统相关工具函数
/// 包括路径处理、哈希计算、文件操作等通用功能
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// 计算文件或目录的SHA256哈希值
/// 用于manifest.json变更检测和文件一致性验证
pub fn calculate_sha256_hash(path: &Path) -> Result<String, String> {
    if !path.exists() {
        return Err(format!("文件不存在: {}", path.display()));
    }

    if !path.is_file() {
        return Err(format!("路径不是文件: {}", path.display()));
    }

    let file = File::open(path).map_err(|e| format!("无法打开文件: {}", e))?;

    let mut reader = BufReader::new(file);
    let mut hasher = sha2::Sha256::new();
    let mut buffer = [0; 8192]; // 8KB buffer

    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break, // End of file
            Ok(n) => {
                hasher.update(&buffer[..n]);
            }
            Err(e) => {
                return Err(format!("读取文件失败: {}", e));
            }
        }
    }

    let hash_bytes = hasher.finalize();
    let hash_hex = format!("{:x}", hash_bytes);
    Ok(hash_hex)
}

/// 验证路径是否为有效目录
pub fn is_valid_directory(path: &Path) -> bool {
    path.exists() && path.is_dir()
}

/// 创建目录（包括父目录）
pub fn create_directory(path: &Path) -> Result<(), String> {
    std::fs::create_dir_all(path).map_err(|e| e.to_string())
}
