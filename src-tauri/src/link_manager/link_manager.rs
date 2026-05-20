use std::fs;
/// 符号链接管理模块
/// 负责创建、删除、验证符号链接，支持Windows平台的符号链接操作
use std::path::Path;

pub struct LinkManager;

impl LinkManager {
    /// 创建符号链接（Windows平台）
    /// 如果链接已存在，先删除
    pub fn create_symbolic_link(&self, target: &Path, link: &Path) -> Result<(), String> {
        // 确保目标路径存在
        if !target.exists() {
            return Err(format!("目标路径不存在: {}", target.display()));
        }

        // 如果链接已存在，先删除
        if link.exists() {
            fs::remove_dir_all(link).map_err(|e| format!("删除现有链接失败: {}", e))?;
        }

        // 创建父目录（如果不存在）
        if let Some(parent) = link.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("创建父目录失败: {}", e))?;
        }

        // 创建符号链接（Windows平台）
        #[cfg(windows)]
        {
            use std::os::windows::fs::symlink_dir;
            symlink_dir(target, link).map_err(|e| format!("创建符号链接失败: {}", e))
        }

        #[cfg(not(windows))]
        {
            use std::os::unix::fs::symlink;
            symlink(target, link).map_err(|e| format!("创建符号链接失败: {}", e))
        }
    }

    /// 删除符号链接
    pub fn remove_symbolic_link(&self, link: &Path) -> Result<(), String> {
        if link.exists() {
            fs::remove_dir_all(link).map_err(|e| format!("删除符号链接失败: {}", e))?;
        }
        Ok(())
    }

    /// 验证符号链接是否有效
    pub fn validate_symbolic_link(&self, link: &Path) -> bool {
        if !link.exists() {
            return false;
        }

        // 尝试读取链接指向的内容
        match fs::read_dir(link) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    /// 为Profile批量创建符号链接
    pub fn create_links_for_profile(
        &self,
        profile_path: &Path,
        mod_paths: &[&Path],
    ) -> Result<Vec<String>, String> {
        // 确保Profile目录存在
        fs::create_dir_all(profile_path).map_err(|e| format!("创建Profile目录失败: {}", e))?;

        let mut created_links = Vec::new();

        for mod_path in mod_paths {
            // 获取模组文件夹名称
            let mod_name = mod_path
                .file_name()
                .ok_or_else(|| format!("无法获取模组文件夹名称: {}", mod_path.display()))?
                .to_str()
                .ok_or_else(|| "模组文件夹名称包含无效UTF-8字符".to_string())?;

            // 构造链接路径
            let link_path = profile_path.join(mod_name);

            // 创建符号链接
            self.create_symbolic_link(mod_path, &link_path)?;
            created_links.push(
                link_path
                    .to_str()
                    .ok_or_else(|| "链接路径包含无效UTF-8字符".to_string())?
                    .to_string(),
            );
        }

        Ok(created_links)
    }
}
