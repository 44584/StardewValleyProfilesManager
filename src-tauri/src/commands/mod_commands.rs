/// Mod相关Tauri命令
/// 处理前端调用的模组扫描、注册、关联等操作
use tauri::command;

#[command]
pub fn scan_and_register_mods(
    mods_directory: String,
) -> Result<Vec<crate::models::mod_info::ModInfo>, String> {
    // TODO: 实现模组扫描和注册命令
    // 调用ModScanner和ModRepository
    Ok(vec![])
}

#[command]
pub fn add_mod_to_profile(profile_id: i32, mod_id: i32) -> Result<(), String> {
    // TODO: 实现添加Mod到Profile命令
    // 调用ProfileModRepository和LinkManager
    Ok(())
}

#[command]
pub fn remove_mod_from_profile(profile_id: i32, mod_id: i32) -> Result<(), String> {
    // TODO: 实现从Profile移除Mod命令
    Ok(())
}

#[command]
pub fn toggle_mod_enabled(profile_id: i32, mod_id: i32, is_enabled: bool) -> Result<(), String> {
    // TODO: 实现切换Mod启用状态命令
    Ok(())
}

#[command]
pub fn get_all_mods() -> Result<Vec<crate::models::mod_info::ModInfo>, String> {
    // TODO: 实现获取所有Mod命令
    Ok(vec![])
}
