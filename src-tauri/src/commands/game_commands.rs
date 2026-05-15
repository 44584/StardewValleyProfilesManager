/// 游戏启动相关Tauri命令
/// 处理前端调用的游戏启动操作
use tauri::command;

#[command]
pub fn launch_game_with_profile(profile_id: i32, smapi_path: String) -> Result<(), String> {
    // TODO: 实现使用指定Profile启动游戏命令
    // 调用GameLauncher、ProfileModRepository和LinkManager
    Ok(())
}

#[command]
pub fn validate_smapi_installation(smapi_path: String) -> Result<bool, String> {
    // TODO: 实现SMAPI安装验证命令
    Ok(true)
}
