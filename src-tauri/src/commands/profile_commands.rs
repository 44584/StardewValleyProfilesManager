/// Profile相关Tauri命令
/// 处理前端调用的Profile创建、删除、激活等操作
use tauri::command;

#[command]
pub fn create_profile(name: String, description: Option<String>) -> Result<i32, String> {
    // TODO: 实现Profile创建命令
    // 调用ProfileRepository和LinkManager
    Ok(1)
}

#[command]
pub fn delete_profile(profile_id: i32) -> Result<(), String> {
    // TODO: 实现Profile删除命令
    // 删除数据库记录和文件系统目录
    Ok(())
}

#[command]
pub fn activate_profile(profile_id: i32) -> Result<(), String> {
    // TODO: 实现Profile激活命令
    Ok(())
}

#[command]
pub fn get_all_profiles() -> Result<Vec<crate::models::profile::Profile>, String> {
    // TODO: 实现获取所有Profile命令
    Ok(vec![])
}
