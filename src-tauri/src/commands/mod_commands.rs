/// Mod相关Tauri命令
/// 处理前端调用的模组扫描、注册、关联等操作
use crate::database::{
    connection, mod_repository::ModRepository, profile_mod_repository::ProfileModRepository,
    profile_repository::ProfileRepository,
};
use crate::link_manager::link_manager::LinkManager;
use crate::models::profile_mod::ProfileMod;
use crate::scanner::mod_scanner::ModScanner;
use tauri::command;

fn build_profile_mod_link_path(
    profile_path: &std::path::Path,
    mod_path: &std::path::Path,
) -> Result<std::path::PathBuf, String> {
    let mod_name = mod_path
        .file_name()
        .ok_or_else(|| format!("无法获取模组文件夹名称: {}", mod_path.display()))?
        .to_str()
        .ok_or_else(|| "模组文件夹名称包含无效UTF-8字符".to_string())?;

    Ok(profile_path.join(mod_name))
}

#[command]
pub fn scan_and_register_mods(
    mods_directory: String,
) -> Result<Vec<crate::models::mod_info::ModInfo>, String> {
    // 初始化数据库连接
    let conn = connection::init_database()?;

    // 创建扫描器和仓库
    let scanner = ModScanner;
    let repo = ModRepository::new(conn);

    // 扫描目录获取所有有效模组
    let scanned_mods = scanner.scan_mods_directory(&mods_directory)?;

    // 对每个扫描到的模组进行 upsert 操作
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

    // 获取profile和mod信息
    let profile_conn = connection::init_database()?;
    let profile_repo = ProfileRepository::new(profile_conn);
    let profile = profile_repo
        .get_by_id(profile_id)?
        .ok_or("未找到指定的Profile".to_string())?;

    let mod_conn = connection::init_database()?;
    let mod_repo = ModRepository::new(mod_conn);
    let mod_info = mod_repo
        .find_by_unique_id(&unique_id)?
        .ok_or(format!("未找到Unique ID为 {} 的模组", unique_id))?;

    let mod_id = mod_info.id.ok_or("模组ID不能为空".to_string())?;

    // 添加关联
    let profile_mod_conn = connection::init_database()?;
    let profile_mod_repo = ProfileModRepository::new(profile_mod_conn);
    let profile_mod = ProfileMod::new(profile_id, mod_id);
    profile_mod_repo.add_mod_to_profile(&profile_mod)?;

    // 创建符号链接
    let link_manager = LinkManager;
    let link_path = build_profile_mod_link_path(&profile.profile_path, &mod_info.mod_path)?;
    if let Err(err) = link_manager.create_symbolic_link(&mod_info.mod_path, &link_path) {
        // 如果创建链接失败，尽量回滚数据库关联，避免数据库与文件系统不一致
        let rollback_conn = connection::init_database()?;
        let rollback_repo = ProfileModRepository::new(rollback_conn);
        let _ = rollback_repo.remove_mod_from_profile(profile_id, mod_id);
        return Err(err);
    }

    Ok(())
}

#[command]
pub fn remove_mod_from_profile(profile_id: i32, unique_id: String) -> Result<(), String> {
    if profile_id <= 0 || unique_id.trim().is_empty() {
        return Err("无效的Profile ID或Mod Unique ID".to_string());
    }

    // 获取profile和mod信息
    let profile_conn = connection::init_database()?;
    let profile_repo = ProfileRepository::new(profile_conn);
    let profile = profile_repo
        .get_by_id(profile_id)?
        .ok_or("未找到指定的Profile".to_string())?;

    let mod_conn = connection::init_database()?;
    let mod_repo = ModRepository::new(mod_conn);
    let mod_info = mod_repo
        .find_by_unique_id(&unique_id)?
        .ok_or(format!("未找到Unique ID为 {} 的模组", unique_id))?;

    let mod_id = mod_info.id.ok_or("模组ID不能为空".to_string())?;

    // 移除关联
    let profile_mod_conn = connection::init_database()?;
    let profile_mod_repo = ProfileModRepository::new(profile_mod_conn);
    profile_mod_repo.remove_mod_from_profile(profile_id, mod_id)?;

    // 移除符号链接
    let link_manager = LinkManager;
    let link_path = build_profile_mod_link_path(&profile.profile_path, &mod_info.mod_path)?;
    link_manager.remove_symbolic_link(&link_path)?;

    Ok(())
}

#[command]
pub fn toggle_mod_enabled(
    profile_id: i32,
    unique_id: String,
    is_enabled: bool,
) -> Result<(), String> {
    if profile_id <= 0 || unique_id.trim().is_empty() {
        return Err("无效的Profile ID或Mod Unique ID".to_string());
    }

    // 获取profile和mod信息
    let profile_conn = connection::init_database()?;
    let profile_repo = ProfileRepository::new(profile_conn);
    let profile = profile_repo
        .get_by_id(profile_id)?
        .ok_or("未找到指定的Profile".to_string())?;

    let mod_conn = connection::init_database()?;
    let mod_repo = ModRepository::new(mod_conn);
    let mod_info = mod_repo
        .find_by_unique_id(&unique_id)?
        .ok_or(format!("未找到Unique ID为 {} 的模组", unique_id))?;

    let mod_id = mod_info.id.ok_or("模组ID不能为空".to_string())?;

    // 更新启用状态
    let profile_mod_conn = connection::init_database()?;
    let profile_mod_repo = ProfileModRepository::new(profile_mod_conn);
    profile_mod_repo.update_mod_enabled_status(profile_id, mod_id, is_enabled)?;

    // 根据启用状态同步符号链接
    let link_manager = LinkManager;
    let link_path = build_profile_mod_link_path(&profile.profile_path, &mod_info.mod_path)?;
    if is_enabled {
        link_manager.create_symbolic_link(&mod_info.mod_path, &link_path)?;
    } else {
        link_manager.remove_symbolic_link(&link_path)?;
    }

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
    let mod_info = mod_repo
        .find_by_unique_id(&unique_id)?
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
pub fn get_mods_for_profile(
    profile_id: i32,
) -> Result<Vec<crate::models::profile_mod::ProfileMod>, String> {
    if profile_id <= 0 {
        return Err("无效的Profile ID".to_string());
    }

    let conn = connection::init_database()?;
    let repo = ProfileModRepository::new(conn);
    repo.get_all_mods_for_profile(profile_id)
}
