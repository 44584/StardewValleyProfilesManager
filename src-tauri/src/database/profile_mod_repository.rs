/// Profile-Mod关联数据访问层
/// 提供profile_mods表的CRUD操作和查询功能
use crate::models::profile_mod::ProfileMod;
use rusqlite::{params, Connection};
use std::path::PathBuf;

pub struct ProfileModRepository {
    connection: Connection,
}

impl ProfileModRepository {
    /// 创建新的ProfileModRepository实例
    pub fn new(connection: Connection) -> Self {
        Self { connection }
    }

    /// 添加Mod到Profile（创建关联）
    pub fn add_mod_to_profile(&self, profile_mod: &ProfileMod) -> Result<(), String> {
        // 检查关联是否已存在
        let exists = self
            .connection
            .prepare("SELECT COUNT(*) FROM profile_mods WHERE profile_id = ?1 AND mod_id = ?2")
            .map_err(|e| format!("准备检查关联存在性语句失败: {}", e))?
            .query_row(params![profile_mod.profile_id, profile_mod.mod_id], |row| {
                row.get::<_, i32>(0)
            })
            .map_err(|e| format!("检查关联存在性失败: {}", e))?
            > 0;

        if exists {
            return Err("该Mod已经添加到此Profile中".to_string());
        }

        // 插入新的关联记录
        let result = self
            .connection
            .execute(
                r#"INSERT INTO profile_mods (profile_id, mod_id, is_enabled, link_path) 
                   VALUES (?1, ?2, ?3, ?4)"#,
                params![
                    profile_mod.profile_id,
                    profile_mod.mod_id,
                    profile_mod.is_enabled,
                    &profile_mod.link_path.as_ref().map(|p| p.to_str()).flatten(),
                ],
            )
            .map_err(|e| format!("添加Mod到Profile失败: {}", e))?;

        if result == 0 {
            return Err("添加Mod到Profile时没有插入任何记录".to_string());
        }

        Ok(())
    }

    /// 从Profile中移除Mod（删除关联）
    pub fn remove_mod_from_profile(&self, profile_id: i32, mod_id: i32) -> Result<(), String> {
        let result = self
            .connection
            .execute(
                "DELETE FROM profile_mods WHERE profile_id = ?1 AND mod_id = ?2",
                params![profile_id, mod_id],
            )
            .map_err(|e| format!("从Profile移除Mod失败: {}", e))?;

        if result == 0 {
            return Err("未找到要移除的Mod关联".to_string());
        }

        Ok(())
    }

    /// 更新Mod在Profile中的启用状态
    pub fn update_mod_enabled_status(
        &self,
        profile_id: i32,
        mod_id: i32,
        is_enabled: bool,
    ) -> Result<(), String> {
        let result = self
            .connection
            .execute(
                "UPDATE profile_mods SET is_enabled = ?1 WHERE profile_id = ?2 AND mod_id = ?3",
                params![is_enabled, profile_id, mod_id],
            )
            .map_err(|e| format!("更新Mod启用状态失败: {}", e))?;

        if result == 0 {
            return Err("未找到要更新的Mod关联".to_string());
        }

        Ok(())
    }

    /// 更新Mod的符号链接路径
    pub fn update_link_path(
        &self,
        profile_id: i32,
        mod_id: i32,
        link_path: Option<String>,
    ) -> Result<(), String> {
        let result = self
            .connection
            .execute(
                "UPDATE profile_mods SET link_path = ?1 WHERE profile_id = ?2 AND mod_id = ?3",
                params![link_path, profile_id, mod_id],
            )
            .map_err(|e| format!("更新符号链接路径失败: {}", e))?;

        if result == 0 {
            return Err("未找到要更新的Mod关联".to_string());
        }

        Ok(())
    }

    /// 获取Profile中所有启用的Mod关联
    pub fn get_enabled_mods_for_profile(&self, profile_id: i32) -> Result<Vec<ProfileMod>, String> {
        let mut stmt = self.connection
            .prepare(
                "SELECT id, profile_id, mod_id, is_enabled, link_path FROM profile_mods WHERE profile_id = ?1 AND is_enabled = 1"
            )
            .map_err(|e| format!("准备查询启用Mod语句失败: {}", e))?;

        let profile_mods: Vec<ProfileMod> = stmt
            .query_map(params![profile_id], |row| {
                Ok(ProfileMod {
                    id: Some(row.get("id")?),
                    profile_id: row.get("profile_id")?,
                    mod_id: row.get("mod_id")?,
                    is_enabled: row.get("is_enabled")?,
                    link_path: row
                        .get::<_, Option<String>>("link_path")?
                        .map(PathBuf::from),
                })
            })
            .map_err(|e| format!("查询启用Mod失败: {}", e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("收集启用Mod结果失败: {}", e))?;

        Ok(profile_mods)
    }

    /// 获取Profile中的所有Mod关联（包括禁用的）
    pub fn get_all_mods_for_profile(&self, profile_id: i32) -> Result<Vec<ProfileMod>, String> {
        let mut stmt = self.connection
            .prepare(
                "SELECT id, profile_id, mod_id, is_enabled, link_path FROM profile_mods WHERE profile_id = ?1"
            )
            .map_err(|e| format!("准备查询所有Mod语句失败: {}", e))?;

        let profile_mods: Vec<ProfileMod> = stmt
            .query_map(params![profile_id], |row| {
                Ok(ProfileMod {
                    id: Some(row.get("id")?),
                    profile_id: row.get("profile_id")?,
                    mod_id: row.get("mod_id")?,
                    is_enabled: row.get("is_enabled")?,
                    link_path: row
                        .get::<_, Option<String>>("link_path")?
                        .map(PathBuf::from),
                })
            })
            .map_err(|e| format!("查询所有Mod失败: {}", e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("收集所有Mod结果失败: {}", e))?;

        Ok(profile_mods)
    }

    /// 检查Mod是否已添加到Profile
    pub fn is_mod_in_profile(&self, profile_id: i32, mod_id: i32) -> Result<bool, String> {
        let count = self
            .connection
            .prepare("SELECT COUNT(*) FROM profile_mods WHERE profile_id = ?1 AND mod_id = ?2")
            .map_err(|e| format!("准备检查Mod存在性语句失败: {}", e))?
            .query_row(params![profile_id, mod_id], |row| row.get::<_, i32>(0))
            .map_err(|e| format!("检查Mod存在性失败: {}", e))?;

        Ok(count > 0)
    }

    /// 获取使用指定Mod的所有Profile关联
    /// 参数:
    /// - mod_id: 模组在数据库中的id(不是uniqueId)
    /// 返回:
    /// - 成功: 相关的profile_mod记录
    /// - 失败: 错误信息
    pub fn get_profiles_using_mod(&self, mod_id: i32) -> Result<Vec<ProfileMod>, String> {
        let mut stmt = self.connection
        .prepare(
            "SELECT id, profile_id, mod_id, is_enabled, link_path FROM profile_mods WHERE mod_id = ?1"
        )
        .map_err(|e| format!("准备查询使用Mod的Profiles语句失败: {}", e))?;

        let profile_mods: Vec<ProfileMod> = stmt
            .query_map(params![mod_id], |row| {
                Ok(ProfileMod {
                    id: Some(row.get("id")?),
                    profile_id: row.get("profile_id")?,
                    mod_id: row.get("mod_id")?,
                    is_enabled: row.get("is_enabled")?,
                    link_path: row
                        .get::<_, Option<String>>("link_path")?
                        .map(PathBuf::from),
                })
            })
            .map_err(|e| format!("查询使用Mod的Profiles失败: {}", e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("收集使用Mod的Profiles结果失败: {}", e))?;

        Ok(profile_mods)
    }
}
