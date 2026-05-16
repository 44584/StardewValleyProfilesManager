//! 配置管理命令
//! 
//! 提供应用配置的保存和加载功能

use serde::{Deserialize, Serialize};
use tauri::command;

use crate::utils::get_app_data_dir;

/// 应用配置结构
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub smapi_path: Option<String>,
    pub mods_directory: Option<String>,
}

/// 保存应用配置
#[command]
pub fn save_app_config(smapi_path: Option<String>, mods_directory: Option<String>) -> Result<(), String> {
    let config = AppConfig {
        smapi_path,
        mods_directory,
    };
    
    let app_data_dir = get_app_data_dir()
        .map_err(|e| format!("获取应用数据目录失败: {}", e))?;
    
    let config_path = app_data_dir.join("config.json");
    
    let config_json = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("序列化配置失败: {}", e))?;
    
    std::fs::write(config_path, config_json)
        .map_err(|e| format!("写入配置文件失败: {}", e))?;
    
    Ok(())
}

/// 加载应用配置
#[command]
pub fn load_app_config() -> Result<AppConfig, String> {
    let app_data_dir = get_app_data_dir()
        .map_err(|e| format!("获取应用数据目录失败: {}", e))?;
    
    let config_path = app_data_dir.join("config.json");
    
    if !config_path.exists() {
        // 如果配置文件不存在，返回空配置
        return Ok(AppConfig {
            smapi_path: None,
            mods_directory: None,
        });
    }
    
    let config_json = std::fs::read_to_string(config_path)
        .map_err(|e| format!("读取配置文件失败: {}", e))?;
    
    let config: AppConfig = serde_json::from_str(&config_json)
        .map_err(|e| format!("解析配置文件失败: {}", e))?;
    
    Ok(config)
}