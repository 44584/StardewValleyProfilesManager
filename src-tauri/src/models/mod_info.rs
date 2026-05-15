/// Mod信息模型
/// 对应manifest.json结构，完整映射模组基本信息和可选字段
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModInfo {
    /// 模组唯一标识符 (YourName.YourProjectName)
    pub unique_id: String,
    /// 模组名称
    pub name: String,
    /// 作者名称
    pub author: String,
    /// 版本号 (语义化版本)
    pub version: String,
    /// 描述信息
    pub description: Option<String>,
    /// DLL文件名 (SMAPI模组专用)
    pub entry_dll: Option<String>,
    /// 内容包宿主 (内容包专用)
    pub content_pack_for: Option<String>,
    /// 最低SMAPI版本要求
    pub minimum_api_version: Option<String>,
    /// 依赖项JSON数组 (存储Dependencies字段)
    pub dependencies_json: Option<String>,
    /// 更新键JSON数组 (存储UpdateKeys字段)
    pub update_keys_json: Option<String>,
    /// 模组在文件系统中的实际路径
    pub mod_path: PathBuf,
    /// manifest.json的哈希值，用于变更检测
    pub manifest_hash: String,
}

impl ModInfo {
    /// 创建新的ModInfo实例
    pub fn new(
        unique_id: String,
        name: String,
        author: String,
        version: String,
        mod_path: PathBuf,
        manifest_hash: String,
    ) -> Self {
        Self {
            unique_id,
            name,
            author,
            version,
            description: None,
            entry_dll: None,
            content_pack_for: None,
            minimum_api_version: None,
            dependencies_json: None,
            update_keys_json: None,
            mod_path,
            manifest_hash,
        }
    }
}