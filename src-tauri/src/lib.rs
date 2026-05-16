//! Stardew Profiles Manager - Tauri后端主库
//! 
//! 本模块整合所有功能模块并注册Tauri命令供前端调用
pub mod config;
pub mod utils;
pub mod models;
pub mod database;
pub mod scanner;
pub mod link_manager;
pub mod game_launcher;
pub mod commands;

use tauri::Builder;

/// Tauri应用构建器
/// 注册所有命令并配置应用
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化数据库
    if let Err(e) = database::initialize_database() {
        eprintln!("数据库初始化失败: {}", e);
        // 注意：这里不panic，让应用继续运行，但数据库功能可能不可用
    }
    
    Builder::default()
        .plugin(tauri_plugin_dialog::init()) // 初始化dialog插件
        .invoke_handler(tauri::generate_handler![
            // Profile相关命令
            commands::profile_commands::create_profile,
            commands::profile_commands::delete_profile,
            commands::profile_commands::get_all_profiles,
            commands::profile_commands::get_profile_by_id,
            commands::profile_commands::update_profile,
            
            // Mod相关命令
            commands::mod_commands::scan_and_register_mods,
            commands::mod_commands::add_mod_to_profile,
            commands::mod_commands::remove_mod_from_profile,
            commands::mod_commands::toggle_mod_enabled,
            commands::mod_commands::get_all_mods,
            commands::mod_commands::get_mods_for_profile,
            commands::mod_commands::is_mod_in_profile,
            
            // 游戏启动相关命令
            commands::game_commands::launch_game_with_profile,
            commands::game_commands::validate_smapi_installation,
            
            // 配置相关命令
            commands::config_commands::save_app_config,
            commands::config_commands::load_app_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}