/// Mod数据访问层
/// 提供mods表的CRUD操作和查询功能
use crate::models::mod_info::ModInfo;
use rusqlite::{Connection, params, OptionalExtension};

pub struct ModRepository {
    connection: Connection,
}

impl ModRepository {
    /// 创建新的ModRepository实例
    pub fn new(connection: Connection) -> Self {
        Self { connection }
    }
    
    /// 保存或更新Mod信息（基于unique_id进行upsert）
    pub fn save_or_update(&self, mod_info: &ModInfo) -> Result<(), String> {
        // 检查是否已存在相同unique_id的记录
        let exists = self.connection
            .prepare("SELECT COUNT(*) FROM mods WHERE unique_id = ?1")
            .map_err(|e| format!("准备查询语句失败: {}", e))?
            .query_row(params![&mod_info.unique_id], |row| row.get::<_, i32>(0))
            .map_err(|e| format!("查询existing mod失败: {}", e))? > 0;
        
        if exists {
            // 更新现有记录
            self.connection
                .execute(
                    r#"UPDATE mods SET 
                        name = ?1, author = ?2, version = ?3, description = ?4,
                        entry_dll = ?5, content_pack_for = ?6, minimum_api_version = ?7,
                        dependencies_json = ?8, update_keys_json = ?9, mod_path = ?10,
                        manifest_hash = ?11, updated_at = CURRENT_TIMESTAMP
                    WHERE unique_id = ?12"#,
                    params![
                        &mod_info.name,
                        &mod_info.author,
                        &mod_info.version,
                        &mod_info.description,
                        &mod_info.entry_dll,
                        &mod_info.content_pack_for,
                        &mod_info.minimum_api_version,
                        &mod_info.dependencies_json,
                        &mod_info.update_keys_json,
                        mod_info.mod_path.to_str().unwrap_or(""),
                        &mod_info.manifest_hash,
                        &mod_info.unique_id,
                    ],
                )
                .map_err(|e| format!("更新mod失败: {}", e))?;
        } else {
            // 插入新记录
            self.connection
                .execute(
                    r#"INSERT INTO mods (
                        unique_id, name, author, version, description,
                        entry_dll, content_pack_for, minimum_api_version,
                        dependencies_json, update_keys_json, mod_path, manifest_hash
                    ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)"#,
                    params![
                        &mod_info.unique_id,
                        &mod_info.name,
                        &mod_info.author,
                        &mod_info.version,
                        &mod_info.description,
                        &mod_info.entry_dll,
                        &mod_info.content_pack_for,
                        &mod_info.minimum_api_version,
                        &mod_info.dependencies_json,
                        &mod_info.update_keys_json,
                        mod_info.mod_path.to_str().unwrap_or(""),
                        &mod_info.manifest_hash,
                    ],
                )
                .map_err(|e| format!("插入mod失败: {}", e))?;
        }
        
        Ok(())
    }
    
    /// 获取所有已注册的模组
    pub fn get_all_mods(&self) -> Result<Vec<ModInfo>, String> {
        let mut stmt = self.connection
            .prepare(
                "SELECT id, unique_id, name, author, version, description, entry_dll, content_pack_for, minimum_api_version, dependencies_json, update_keys_json, mod_path, manifest_hash FROM mods"
            )
            .map_err(|e| format!("准备查询语句失败: {}", e))?;
        
        let mods: Vec<ModInfo> = stmt
            .query_map([], |row| {
                Ok(ModInfo {
                    id: Some(row.get("id")?),
                    unique_id: row.get("unique_id")?,
                    name: row.get("name")?,
                    author: row.get("author")?,
                    version: row.get("version")?,
                    description: row.get("description").ok(),
                    entry_dll: row.get("entry_dll").ok(),
                    content_pack_for: row.get("content_pack_for").ok(),
                    minimum_api_version: row.get("minimum_api_version").ok(),
                    dependencies_json: row.get("dependencies_json").ok(),
                    update_keys_json: row.get("update_keys_json").ok(),
                    mod_path: std::path::PathBuf::from(row.get::<_, String>("mod_path")?),
                    manifest_hash: row.get("manifest_hash")?,
                })
            })
            .map_err(|e| format!("查询所有mods失败: {}", e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("收集mods结果失败: {}", e))?;
        
        Ok(mods)
    }
    
    /// 根据unique_id查找模组
    pub fn find_by_unique_id(&self, unique_id: &str) -> Result<Option<ModInfo>, String> {
        let mut stmt = self.connection
            .prepare(
                "SELECT id, unique_id, name, author, version, description, entry_dll, content_pack_for, minimum_api_version, dependencies_json, update_keys_json, mod_path, manifest_hash FROM mods WHERE unique_id = ?1"
            )
            .map_err(|e| format!("准备查询语句失败: {}", e))?;
        
        let result = stmt
            .query_row(params![unique_id], |row| {
                Ok(ModInfo {
                    id: Some(row.get("id")?),
                    unique_id: row.get("unique_id")?,
                    name: row.get("name")?,
                    author: row.get("author")?,
                    version: row.get("version")?,
                    description: row.get("description").ok(),
                    entry_dll: row.get("entry_dll").ok(),
                    content_pack_for: row.get("content_pack_for").ok(),
                    minimum_api_version: row.get("minimum_api_version").ok(),
                    dependencies_json: row.get("dependencies_json").ok(),
                    update_keys_json: row.get("update_keys_json").ok(),
                    mod_path: std::path::PathBuf::from(row.get::<_, String>("mod_path")?),
                    manifest_hash: row.get("manifest_hash")?,
                })
            })
            .optional()
            .map_err(|e| format!("查询mod失败: {}", e))?;
        
        Ok(result)
    }
    
    /// 根据ID查找模组
    pub fn get_by_id(&self, id: i32) -> Result<Option<ModInfo>, String> {
        let mut stmt = self.connection
            .prepare(
                "SELECT id, unique_id, name, author, version, description, entry_dll, content_pack_for, minimum_api_version, dependencies_json, update_keys_json, mod_path, manifest_hash FROM mods WHERE id = ?1"
            )
            .map_err(|e| format!("准备查询语句失败: {}", e))?;
        
        let result = stmt
            .query_row(params![id], |row| {
                Ok(ModInfo {
                    id: Some(row.get("id")?),
                    unique_id: row.get("unique_id")?,
                    name: row.get("name")?,
                    author: row.get("author")?,
                    version: row.get("version")?,
                    description: row.get("description").ok(),
                    entry_dll: row.get("entry_dll").ok(),
                    content_pack_for: row.get("content_pack_for").ok(),
                    minimum_api_version: row.get("minimum_api_version").ok(),
                    dependencies_json: row.get("dependencies_json").ok(),
                    update_keys_json: row.get("update_keys_json").ok(),
                    mod_path: std::path::PathBuf::from(row.get::<_, String>("mod_path")?),
                    manifest_hash: row.get("manifest_hash")?,
                })
            })
            .optional()
            .map_err(|e| format!("查询mod失败: {}", e))?;
        
        Ok(result)
    }
}