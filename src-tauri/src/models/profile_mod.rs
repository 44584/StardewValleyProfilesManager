/// Profile-Mod关联模型
/// 对应profile_mods表，维护模组与配置的多对多关系并包含业务状态
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileMod {
    /// 关联记录ID
    #[serde(rename = "id")]
    pub id: Option<i32>,
    /// 关联的Profile ID
    #[serde(rename = "profileId")]
    pub profile_id: i32,
    /// 关联的Mod ID, 而不是ModInfo中的unique_id
    #[serde(rename = "modId")]
    pub mod_id: i32,
    /// 在该配置中是否启用
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
    /// 符号链接路径 (可为空，表示未创建链接)
    #[serde(rename = "linkPath")]
    pub link_path: Option<PathBuf>,
}

impl ProfileMod {
    /// 创建新的ProfileMod关联实例
    pub fn new(profile_id: i32, mod_id: i32) -> Self {
        Self {
            id: None,
            profile_id,
            mod_id,
            is_enabled: true,
            link_path: None,
        }
    }
}
