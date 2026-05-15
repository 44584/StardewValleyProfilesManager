//! Stardew Profiles Manager - Tauri后端主库
//! 
//! 本模块整合所有功能模块并注册Tauri命令供前端调用
mod config;
mod utils;
mod models;
mod database;
mod scanner;
mod link_manager;
mod game_launcher;
mod commands;

use tauri::Builder;

/// Tauri应用构建器
/// 注册所有命令并配置应用
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .invoke_handler(tauri::generate_handler![
            // Profile相关命令
            commands::profile_commands::create_profile,
            commands::profile_commands::delete_profile,
            commands::profile_commands::activate_profile,
            commands::profile_commands::get_all_profiles,
            
            // Mod相关命令
            commands::mod_commands::scan_and_register_mods,
            commands::mod_commands::add_mod_to_profile,
            commands::mod_commands::remove_mod_from_profile,
            commands::mod_commands::toggle_mod_enabled,
            commands::mod_commands::get_all_mods,
            
            // 游戏启动相关命令
            commands::game_commands::launch_game_with_profile,
            commands::game_commands::validate_smapi_installation,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}