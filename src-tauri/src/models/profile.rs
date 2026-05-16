/// Profile配置模型
/// 对应profiles表，存储配置方案基本信息和状态
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    /// 配置方案ID
    pub id: Option<i32>,
    /// 配置方案名称 (如"休闲种田")
    pub name: String,
    /// 配置描述
    pub description: Option<String>,
    /// Profile目录的实际路径
    pub profile_path: PathBuf,
    // Removed is_active field as activation state is no longer needed
}

impl Profile {
    /// 创建新的Profile实例
    pub fn new(name: String, profile_path: PathBuf) -> Self {
        Self {
            id: None,
            name,
            description: None,
            profile_path,
            // is_active field removed
        }
    }
}