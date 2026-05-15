/// 应用配置管理模块
/// 负责管理Stardew Valley安装路径、Mods目录、Profiles目录等配置信息
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Stardew Valley游戏安装目录
    pub game_directory: PathBuf,
    /// Mods模组目录路径
    pub mods_directory: PathBuf,
    /// Profiles配置方案目录路径  
    pub profiles_directory: PathBuf,
    /// 数据库文件路径
    pub database_path: PathBuf,
}

impl AppConfig {
    /// 创建默认配置（基于系统标准目录）
    pub fn new() -> Self {
        let app_data_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("StardewProfilesManager");
        
        Self {
            game_directory: PathBuf::from(""),
            mods_directory: PathBuf::from(""),
            profiles_directory: app_data_dir.join("Profiles"),
            database_path: app_data_dir.join("stardew_profiles.db"),
        }
    }
}