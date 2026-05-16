/// 游戏启动相关Tauri命令
/// 处理前端调用的游戏启动操作
use tauri::command;
use crate::database::{connection, profile_repository::ProfileRepository, profile_mod_repository::ProfileModRepository, mod_repository::ModRepository};
use crate::game_launcher::launcher::GameLauncher;
use crate::link_manager::link_manager::LinkManager;
use std::path::Path;

#[command]
pub fn launch_game_with_profile(profile_id: i32, smapi_path: String) -> Result<(), String> {
    // 验证输入参数
    if profile_id <= 0 {
        return Err("无效的Profile ID".to_string());
    }
    
    let smapi_path_buf = Path::new(&smapi_path).to_path_buf();
    if smapi_path.is_empty() {
        return Err("SMAPI路径不能为空".to_string());
    }
    
    // 获取数据库连接
    let connection = connection::init_database()?;
    
    // 获取Profile信息
    let profile_repo = ProfileRepository::new(connection);
    let profile = profile_repo.get_by_id(profile_id)?
        .ok_or("未找到指定的Profile".to_string())?;
    
    // 确保Profile目录存在
    std::fs::create_dir_all(&profile.profile_path)
        .map_err(|e| format!("创建Profile目录失败: {}", e))?;
    
    // 获取启用的模组关联
    let profile_mod_connection = connection::init_database()?;
    let profile_mod_repo = ProfileModRepository::new(profile_mod_connection);
    let enabled_profile_mods = profile_mod_repo.get_enabled_mods_for_profile(profile_id)?;
    
    if enabled_profile_mods.is_empty() {
        return Err("Profile中没有启用的模组，请先添加模组到Profile中".to_string());
    }
    
    // 获取模组实际路径
    let mod_connection = connection::init_database()?;
    let mod_repo = ModRepository::new(mod_connection);
    let mut mod_paths = Vec::new();
    
    for profile_mod in &enabled_profile_mods {
        let mod_info = mod_repo.get_by_id(profile_mod.mod_id)?
            .ok_or(format!("未找到ID为 {} 的模组", profile_mod.mod_id))?;
        mod_paths.push(mod_info.mod_path);
    }
    
    // 同步符号链接
    let link_manager = LinkManager;
    let mut valid_links = Vec::new();
    
    for (i, mod_path) in mod_paths.iter().enumerate() {
        let mod_name = mod_path.file_name()
            .ok_or_else(|| format!("无法获取模组文件夹名称: {}", mod_path.display()))?
            .to_str()
            .ok_or_else(|| "模组文件夹名称包含无效UTF-8字符".to_string())?;
        
        let link_path = profile.profile_path.join(mod_name);
        
        // 验证或创建符号链接
        if !link_manager.validate_symbolic_link(&link_path) {
            link_manager.create_symbolic_link(mod_path, &link_path)?;
            
            // 更新数据库中的link_path
            let update_conn = connection::init_database()?;
            let update_repo = ProfileModRepository::new(update_conn);
            update_repo.update_link_path(profile_id, enabled_profile_mods[i].mod_id, Some(link_path.to_str()
                .ok_or_else(|| "链接路径包含无效UTF-8字符".to_string())?
                .to_string()))?;
        }
        
        valid_links.push(link_path);
    }
    
    // 启动游戏
    let game_launcher = GameLauncher;
    game_launcher.launch_game(&smapi_path_buf, &profile.profile_path)?;
    
    Ok(())
}

#[command]
pub fn validate_smapi_installation(smapi_path: String) -> Result<bool, String> {
    if smapi_path.is_empty() {
        return Ok(false);
    }
    
    let smapi_path_buf = Path::new(&smapi_path).to_path_buf();
    let game_launcher = GameLauncher;
    Ok(game_launcher.validate_smapi_executable(&smapi_path_buf))
}