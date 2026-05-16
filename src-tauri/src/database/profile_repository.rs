/// Profile数据访问层
/// 提供profiles表的CRUD操作和查询功能
use crate::models::profile::Profile;
use rusqlite::{Connection, params, OptionalExtension};

pub struct ProfileRepository {
    connection: Connection,
}

impl ProfileRepository {
    /// 创建新的ProfileRepository实例
    pub fn new(connection: Connection) -> Self {
        Self { connection }
    }
    
    /// 创建新的Profile
    pub fn create(&self, profile: &Profile) -> Result<i32, String> {
        // 插入新Profile记录
        let result = self.connection
            .execute(
                r#"INSERT INTO profiles (name, description, profile_path) 
                   VALUES (?1, ?2, ?3)"#,
                params![
                    &profile.name,
                    &profile.description,
                    profile.profile_path.to_str().unwrap_or(""),
                ],
            )
            .map_err(|e| format!("创建Profile失败: {}", e))?;
        
        if result == 0 {
            return Err("创建Profile时没有插入任何记录".to_string());
        }
        
        // 获取最后插入的ID
        let id = self.connection
            .last_insert_rowid();
            
        Ok(id as i32)
    }
    
    /// 获取所有Profile
    pub fn get_all_profiles(&self) -> Result<Vec<Profile>, String> {
        let mut stmt = self.connection
            .prepare(
                "SELECT id, name, description, profile_path FROM profiles ORDER BY name"
            )
            .map_err(|e| format!("准备查询语句失败: {}", e))?;
        
        let profiles: Vec<Profile> = stmt
            .query_map([], |row| {
                Ok(Profile {
                    id: Some(row.get("id")?),
                    name: row.get("name")?,
                    description: row.get("description").ok(),
                    profile_path: std::path::PathBuf::from(row.get::<_, String>("profile_path")?),
                })
            })
            .map_err(|e| format!("查询所有profiles失败: {}", e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("收集profiles结果失败: {}", e))?;
        
        Ok(profiles)
    }
    
    /// 根据ID获取Profile
    pub fn get_by_id(&self, id: i32) -> Result<Option<Profile>, String> {
        let mut stmt = self.connection
            .prepare(
                "SELECT id, name, description, profile_path FROM profiles WHERE id = ?1"
            )
            .map_err(|e| format!("准备查询语句失败: {}", e))?;
        
        let result = stmt
            .query_row(params![id], |row| {
                Ok(Profile {
                    id: Some(row.get("id")?),
                    name: row.get("name")?,
                    description: row.get("description").ok(),
                    profile_path: std::path::PathBuf::from(row.get::<_, String>("profile_path")?),
                })
            })
            .optional()
            .map_err(|e| format!("查询profile失败: {}", e))?;
        
        Ok(result)
    }
    
    /// 更新Profile信息
    pub fn update(&self, profile: &Profile) -> Result<(), String> {
        let id = profile.id.ok_or("Profile ID不能为空")?;
        
        let result = self.connection
            .execute(
                r#"UPDATE profiles SET name = ?1, description = ?2, profile_path = ?3 
                   WHERE id = ?4"#,
                params![
                    &profile.name,
                    &profile.description,
                    profile.profile_path.to_str().unwrap_or(""),
                    id,
                ],
            )
            .map_err(|e| format!("更新Profile失败: {}", e))?;
        
        if result == 0 {
            return Err("未找到要更新的Profile".to_string());
        }
        
        Ok(())
    }
    
    /// 删除Profile
    pub fn delete(&self, id: i32) -> Result<(), String> {
        // 先级联删除关联记录，确保外键约束不会阻止删除
        self.connection
            .execute("DELETE FROM profile_mods WHERE profile_id = ?1", params![id])
            .map_err(|e| format!("删除Profile关联模组失败: {}", e))?;

        let result = self.connection
            .execute("DELETE FROM profiles WHERE id = ?1", params![id])
            .map_err(|e| format!("删除Profile失败: {}", e))?;

        if result == 0 {
            return Err("未找到要删除的Profile".to_string());
        }

        Ok(())
    }
    
    // Removed activate_profile and get_active_profile methods as they are no longer needed
    // The launch functionality now works directly with profile_id without requiring activation state
}