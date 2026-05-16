/// 游戏启动控制模块
/// 负责构造SMAPI启动命令并执行外部进程
use std::path::Path;
use std::process::Command;

pub struct GameLauncher;

impl GameLauncher {
    /// 构造SMAPI启动命令
    pub fn build_launch_command(&self, smapi_path: &Path, mods_path: &Path) -> Result<String, String> {
        // 验证路径有效性
        if !smapi_path.exists() {
            return Err(format!("SMAPI可执行文件不存在: {}", smapi_path.display()));
        }
        
        if !mods_path.exists() {
            return Err(format!("Mods目录不存在: {}", mods_path.display()));
        }
        
        // 构造启动命令
        let command = format!("\"{}\" --mods-path \"{}\"", 
            smapi_path.to_str().ok_or("SMAPI路径包含无效UTF-8字符")?,
            mods_path.to_str().ok_or("Mods路径包含无效UTF-8字符")?
        );
        
        Ok(command)
    }
    
    /// 执行游戏启动命令
    pub fn launch_game(&self, smapi_path: &Path, mods_path: &Path) -> Result<(), String> {
        // 验证SMAPI可执行文件
        if !self.validate_smapi_executable(smapi_path) {
            return Err(format!("SMAPI可执行文件无效: {}", smapi_path.display()));
        }
        
        // 验证Mods目录
        if !mods_path.exists() {
            return Err(format!("Mods目录不存在: {}", mods_path.display()));
        }
        
        // 构造并执行命令
        let mut cmd = Command::new(smapi_path);
        cmd.arg("--mods-path")
           .arg(mods_path);
        
        // 启动游戏进程（不等待完成）
        match cmd.spawn() {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("启动游戏失败: {}", e)),
        }
    }
    
    /// 验证SMAPI可执行文件是否存在
    pub fn validate_smapi_executable(&self, smapi_path: &Path) -> bool {
        smapi_path.exists() && smapi_path.is_file()
    }
}