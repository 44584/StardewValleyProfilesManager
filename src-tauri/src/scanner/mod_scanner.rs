/// 模组扫描和manifest解析模块
/// 负责扫描Mods目录、解析manifest.json并验证模组有效性
use crate::models::mod_info::ModInfo;
use crate::utils::file_utils::calculate_sha256_hash;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

/// Manifest.json 的两种格式共用字段
#[derive(Debug, Deserialize)]
struct ManifestBase {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Author")]
    author: String,
    #[serde(rename = "Version")]
    version: String,
    #[serde(rename = "Description")]
    description: Option<String>,
    #[serde(rename = "UniqueID")]
    unique_id: String,
    #[serde(rename = "UpdateKeys")]
    update_keys: Option<Vec<String>>,
}

/// SMAPI 模组格式 (包含 EntryDll)
#[derive(Debug, Deserialize)]
struct SmapiManifest {
    #[serde(flatten)]
    base: ManifestBase,
    #[serde(rename = "EntryDll")]
    entry_dll: String,
    #[serde(rename = "MinimumApiVersion")]
    minimum_api_version: Option<String>,
    #[serde(rename = "Dependencies")]
    dependencies: Option<Vec<Dependency>>,
}

/// 内容包格式 (包含 ContentPackFor)
#[derive(Debug, Deserialize)]
struct ContentPackManifest {
    #[serde(flatten)]
    base: ManifestBase,
    #[serde(rename = "ContentPackFor")]
    content_pack_for: ContentPackFor,
    #[serde(rename = "MinimumApiVersion")]
    minimum_api_version: Option<String>,
    #[serde(rename = "Dependencies")]
    dependencies: Option<Vec<Dependency>>,
}

/// 依赖项结构
#[derive(Debug, Deserialize, Serialize)]
struct Dependency {
    #[serde(rename = "UniqueID")]
    unique_id: String,
    #[serde(rename = "MinimumVersion")]
    minimum_version: Option<String>,
    #[serde(rename = "IsRequired")]
    is_required: Option<bool>,
}

/// 内容包宿主结构
#[derive(Debug, Deserialize, Serialize)]
struct ContentPackFor {
    #[serde(rename = "UniqueID")]
    unique_id: String,
    #[serde(rename = "MinimumVersion")]
    minimum_version: Option<String>,
}

pub struct ModScanner;

impl ModScanner {
    /// 扫描指定目录下的所有有效模组
    pub fn scan_mods_directory(&self, mods_dir: &str) -> Result<Vec<ModInfo>, String> {
        let mods_dir_path = Path::new(mods_dir);
        
        // 验证目录是否存在
        if !mods_dir_path.exists() {
            return Err(format!("Mods目录不存在: {}", mods_dir));
        }
        
        if !mods_dir_path.is_dir() {
            return Err(format!("指定路径不是目录: {}", mods_dir));
        }
        
        let mut valid_mods = Vec::new();
        
        // 使用walkdir递归遍历目录
        for entry in WalkDir::new(mods_dir_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            
            // 只处理目录（每个模组应该是一个目录）
            if path.is_dir() {
                // 检查目录中是否存在manifest.json
                let manifest_path = path.join("manifest.json");
                if manifest_path.exists() {
                    match self.parse_manifest_json(&manifest_path.to_string_lossy()) {
                        Ok(mut mod_info) => {
                            // 设置模组实际路径
                            mod_info.mod_path = path.to_path_buf();
                            
                            // 验证模组信息
                            if let Err(validation_error) = self.validate_mod_info(&mod_info) {
                                eprintln!("跳过无效模组 {}: {}", path.display(), validation_error);
                                continue;
                            }
                            
                            valid_mods.push(mod_info);
                        }
                        Err(e) => {
                            eprintln!("解析模组失败 {}: {}", path.display(), e);
                            continue;
                        }
                    }
                }
            }
        }
        
        Ok(valid_mods)
    }
    
    /// 解析单个manifest.json文件
    pub fn parse_manifest_json(&self, manifest_path: &str) -> Result<ModInfo, String> {
        let manifest_file = fs::File::open(manifest_path)
            .map_err(|e| format!("无法打开manifest.json文件: {}", e))?;
        
        let manifest_content: serde_json::Value = serde_json::from_reader(manifest_file)
            .map_err(|e| format!("JSON解析失败: {}", e))?;
        
        // 计算manifest.json的哈希值
        let manifest_hash = calculate_sha256_hash(Path::new(manifest_path))
            .unwrap_or_else(|_| "unknown_hash".to_string());
        
        // 尝试解析为SMAPI模组格式
        if let Ok(smapi_manifest) = serde_json::from_value::<SmapiManifest>(manifest_content.clone()) {
            let base = smapi_manifest.base;
            let dependencies_json = smapi_manifest.dependencies.map(|deps| {
                serde_json::to_string(&deps).unwrap_or_default()
            });
            let update_keys_json = base.update_keys.map(|keys| {
                serde_json::to_string(&keys).unwrap_or_default()
            });
            
            let mod_info = ModInfo::new(
                base.unique_id,
                base.name,
                base.author,
                base.version,
                Path::new("").to_path_buf(), // 路径在scan_mods_directory中设置
                manifest_hash,
            );
            
            Ok(ModInfo {
                description: base.description,
                entry_dll: Some(smapi_manifest.entry_dll),
                content_pack_for: None,
                minimum_api_version: smapi_manifest.minimum_api_version,
                dependencies_json,
                update_keys_json,
                ..mod_info
            })
        }
        // 尝试解析为内容包格式
        else if let Ok(content_pack_manifest) = serde_json::from_value::<ContentPackManifest>(manifest_content) {
            let base = content_pack_manifest.base;
            let dependencies_json = content_pack_manifest.dependencies.map(|deps| {
                serde_json::to_string(&deps).unwrap_or_default()
            });
            let update_keys_json = base.update_keys.map(|keys| {
                serde_json::to_string(&keys).unwrap_or_default()
            });
            
            let mod_info = ModInfo::new(
                base.unique_id,
                base.name,
                base.author,
                base.version,
                Path::new("").to_path_buf(), // 路径在scan_mods_directory中设置
                manifest_hash,
            );
            
            Ok(ModInfo {
                description: base.description,
                entry_dll: None,
                content_pack_for: Some(content_pack_manifest.content_pack_for.unique_id),
                minimum_api_version: content_pack_manifest.minimum_api_version,
                dependencies_json,
                update_keys_json,
                ..mod_info
            })
        }
        else {
            Err("manifest.json格式无效：既不是SMAPI模组也不是内容包格式".to_string())
        }
    }
    
    /// 验证模组是否有效（检查必备字段）
    pub fn validate_mod_info(&self, mod_info: &ModInfo) -> Result<(), String> {
        // 检查必备字段
        if mod_info.unique_id.trim().is_empty() {
            return Err("unique_id不能为空".to_string());
        }
        
        if mod_info.name.trim().is_empty() {
            return Err("name不能为空".to_string());
        }
        
        if mod_info.author.trim().is_empty() {
            return Err("author不能为空".to_string());
        }
        
        if mod_info.version.trim().is_empty() {
            return Err("version不能为空".to_string());
        }
        
        // 检查互斥性：EntryDll 和 ContentPackFor 不能同时存在
        if mod_info.entry_dll.is_some() && mod_info.content_pack_for.is_some() {
            return Err("EntryDll 和 ContentPackFor 不能同时存在".to_string());
        }
        
        // 检查至少有一个必须存在
        if mod_info.entry_dll.is_none() && mod_info.content_pack_for.is_none() {
            return Err("必须指定 EntryDll 或 ContentPackFor".to_string());
        }
        
        Ok(())
    }
}