//! 工具函数模块
pub mod file_utils;

use std::path::PathBuf;
use dirs;

/// 获取应用数据目录
/// 
/// 返回系统标准的用户数据目录路径，例如：
/// - Windows: %APPDATA% (AppData\Roaming)
/// - macOS: ~/Library/Application Support  
/// - Linux: ~/.local/share
pub fn get_app_data_dir() -> Result<PathBuf, String> {
    let app_name = "StardewProfilesManager";
    
    // 使用dirs::data_dir()获取系统数据目录
    let data_dir = dirs::data_dir()
        .ok_or_else(|| "无法获取系统数据目录".to_string())?;
    
    let app_data_dir = data_dir.join(app_name);
    
    // 确保目录存在
    std::fs::create_dir_all(&app_data_dir)
        .map_err(|e| format!("创建应用数据目录失败: {}", e))?;
    
    Ok(app_data_dir)
}