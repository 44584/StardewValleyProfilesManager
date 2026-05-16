/// Profile相关Tauri命令
/// 处理前端调用的Profile创建、删除、激活等操作
use tauri::command;
use crate::database::{connection, profile_repository::ProfileRepository};
use crate::models::profile::Profile;

#[command]
pub fn create_profile(name: String, description: Option<String>) -> Result<i32, String> {
    // 验证输入参数
    if name.trim().is_empty() {
        return Err("Profile名称不能为空".to_string());
    }
    
    // 获取用户数据目录下的Profiles目录
    let profiles_dir = dirs::data_dir()
        .ok_or("无法获取用户数据目录".to_string())?
        .join("StardewProfilesManager")
        .join("profiles");
    
    // 创建Profiles目录（如果不存在）
    std::fs::create_dir_all(&profiles_dir)
        .map_err(|e| format!("创建Profiles目录失败: {}", e))?;
    
    // 生成Profile目录名（使用名称作为目录名，处理特殊字符）
    let safe_name = name.replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_");
    let profile_path = profiles_dir.join(&safe_name);
    
    // 如果目录已存在，添加数字后缀
    let mut counter = 1;
    let mut final_profile_path = profile_path.clone();
    while final_profile_path.exists() {
        final_profile_path = profiles_dir.join(format!("{}_{}", safe_name, counter));
        counter += 1;
    }
    
    // 创建Profile目录
    std::fs::create_dir_all(&final_profile_path)
        .map_err(|e| format!("创建Profile目录失败: {}", e))?;
    
    // 创建Profile实例
    let profile = Profile {
        id: None,
        name: name.trim().to_string(),
        description,
        profile_path: final_profile_path,
    };
    
    // 保存到数据库
    let connection = connection::init_database()?;
    let repo = ProfileRepository::new(connection);
    let profile_id = repo.create(&profile)?;
    
    Ok(profile_id)
}

#[command]
pub fn delete_profile(profile_id: i32) -> Result<(), String> {
    if profile_id <= 0 {
        return Err("无效的Profile ID".to_string());
    }
    
    // 获取数据库连接
    let connection = connection::init_database()?;
    let repo = ProfileRepository::new(connection);
    
    // 获取Profile信息以获取目录路径
    let profile = repo.get_by_id(profile_id)?
        .ok_or("未找到指定的Profile".to_string())?;
    
    // 删除数据库记录
    repo.delete(profile_id)?;
    
    // 删除Profile目录（如果存在）
    if profile.profile_path.exists() {
        std::fs::remove_dir_all(&profile.profile_path)
            .map_err(|e| format!("删除Profile目录失败: {}", e))?;
    }
    
    Ok(())
}

#[command]
pub fn update_profile(profile_id: i32, name: String, description: Option<String>) -> Result<(), String> {
    if profile_id <= 0 {
        return Err("无效的Profile ID".to_string());
    }
    
    if name.trim().is_empty() {
        return Err("Profile名称不能为空".to_string());
    }
    
    let connection = connection::init_database()?;
    let repo = ProfileRepository::new(connection);
    
    // 获取现有Profile
    let mut profile = repo.get_by_id(profile_id)?
        .ok_or("未找到指定的Profile".to_string())?;
    
    // 更新信息
    profile.name = name.trim().to_string();
    profile.description = description;
    
    // 更新数据库
    repo.update(&profile)?;
    
    Ok(())
}

#[command]
pub fn get_all_profiles() -> Result<Vec<crate::models::profile::Profile>, String> {
    let connection = connection::init_database()?;
    let repo = ProfileRepository::new(connection);
    let profiles = repo.get_all_profiles()?;
    Ok(profiles)
}

#[command]
pub fn get_profile_by_id(profile_id: i32) -> Result<Option<crate::models::profile::Profile>, String> {
    if profile_id <= 0 {
        return Err("无效的Profile ID".to_string());
    }
    
    let connection = connection::init_database()?;
    let repo = ProfileRepository::new(connection);
    let profile = repo.get_by_id(profile_id)?;
    Ok(profile)
}
