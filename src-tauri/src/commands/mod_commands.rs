/// Mod相关Tauri命令
/// 处理前端调用的模组扫描、注册、关联等操作
use crate::database::{connection, mod_repository::ModRepository, profile_mod_repository::ProfileModRepository};
use crate::scanner::mod_scanner::ModScanner;
use crate::models::profile_mod::ProfileMod;
use tauri::command;

#[command]
pub fn scan_and_register_mods(mods_directory: String) -> Result<Vec<crate::models::mod_info::ModInfo>, String> {
    // 初始化数据库连接
    let conn = connection::init_database()?;

    // 创建扫描器和仓库
    let scanner = ModScanner;
    let repo = ModRepository::new(conn);

    // 扫描目录获取所有有效模组
    let scanned_mods = scanner.scan_mods_directory(&mods_directory)?;

    // 对每个扫描到的模组进行upsert操作
    for mod_info in &scanned_mods {
        repo.save_or_update(mod_info)?;
    }

    // 重新从数据库查询，返回带有正确 id 的完整 ModInfo 列表
    let conn = connection::init_database()?;
    let repo = ModRepository::new(conn);
    repo.get_all_mods()
}

#[command]
pub fn add_mod_to_profile(profile_id: i32, unique_id: String) -> Result<(), String> {
    if profile_id <= 0 || unique_id.trim().is_empty() {
        return Err("无效的Profile ID或Mod Unique ID".to_string());
    }
    
    // 获取mod信息
    let mod_conn = connection::init_database()?;
    let mod_repo = ModRepository::new(mod_conn);
    let mod_info = mod_repo.find_by_unique_id(&unique_id)?
        .ok_or(format!("未找到Unique ID为 {} 的模组", unique_id))?;
    
    let mod_id = mod_info.id.ok_or("模组ID不能为空".to_string())?;
    
    // 添加关联
    let profile_mod_conn = connection::init_database()?;
    let profile_mod_repo = ProfileModRepository::new(profile_mod_conn);
    let profile_mod = ProfileMod::new(profile_id, mod_id);
    profile_mod_repo.add_mod_to_profile(&profile_mod)?;
    
    Ok(())
}

#[command]
pub fn remove_mod_from_profile(profile_id: i32, unique_id: String) -> Result<(), String> {
    if profile_id <= 0 || unique_id.trim().is_empty() {
        return Err("无效的Profile ID或Mod Unique ID".to_string());
    }
    
    // 获取mod信息
    let mod_conn = connection::init_database()?;
    let mod_repo = ModRepository::new(mod_conn);
    let mod_info = mod_repo.find_by_unique_id(&unique_id)?
        .ok_or(format!("未找到Unique ID为 {} 的模组", unique_id))?;
    
    let mod_id = mod_info.id.ok_or("模组ID不能为空".to_string())?;
    
    // 移除关联
    let profile_mod_conn = connection::init_database()?;
    let profile_mod_repo = ProfileModRepository::new(profile_mod_conn);
    profile_mod_repo.remove_mod_from_profile(profile_id, mod_id)?;
    
    Ok(())
}

#[command]
pub fn toggle_mod_enabled(profile_id: i32, unique_id: String, is_enabled: bool) -> Result<(), String> {
    if profile_id <= 0 || unique_id.trim().is_empty() {
        return Err("无效的Profile ID或Mod Unique ID".to_string());
    }
    
    // 获取mod信息
    let mod_conn = connection::init_database()?;
    let mod_repo = ModRepository::new(mod_conn);
    let mod_info = mod_repo.find_by_unique_id(&unique_id)?
        .ok_or(format!("未找到Unique ID为 {} 的模组", unique_id))?;
    
    let mod_id = mod_info.id.ok_or("模组ID不能为空".to_string())?;
    
    // 更新启用状态
    let profile_mod_conn = connection::init_database()?;
    let profile_mod_repo = ProfileModRepository::new(profile_mod_conn);
    profile_mod_repo.update_mod_enabled_status(profile_id, mod_id, is_enabled)?;
    
    Ok(())
}

#[command]
pub fn is_mod_in_profile(profile_id: i32, unique_id: String) -> Result<bool, String> {
    if profile_id <= 0 || unique_id.trim().is_empty() {
        return Err("无效的Profile ID或Mod Unique ID".to_string());
    }
    
    // 获取mod信息
    let mod_conn = connection::init_database()?;
    let mod_repo = ModRepository::new(mod_conn);
    let mod_info = mod_repo.find_by_unique_id(&unique_id)?
        .ok_or(format!("未找到Unique ID为 {} 的模组", unique_id))?;
    
    let mod_id = mod_info.id.ok_or("模组ID不能为空".to_string())?;
    
    // 检查关联
    let profile_mod_conn = connection::init_database()?;
    let profile_mod_repo = ProfileModRepository::new(profile_mod_conn);
    let exists = profile_mod_repo.is_mod_in_profile(profile_id, mod_id)?;
    Ok(exists)
}

#[command]
pub fn get_all_mods() -> Result<Vec<crate::models::mod_info::ModInfo>, String> {
    let conn = connection::init_database()?;
    let repo = ModRepository::new(conn);
    repo.get_all_mods()
}

#[command]
pub fn get_mods_for_profile(profile_id: i32) -> Result<Vec<crate::models::profile_mod::ProfileMod>, String> {
    if profile_id <= 0 {
        return Err("无效的Profile ID".to_string());
    }
    
    let conn = connection::init_database()?;
    let repo = ProfileModRepository::new(conn);
    repo.get_all_mods_for_profile(profile_id)
}
