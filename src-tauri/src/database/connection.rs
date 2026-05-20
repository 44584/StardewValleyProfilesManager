use dirs::data_dir;
use rusqlite::Connection;
use std::path::PathBuf;

/// 数据库连接管理模块
/// 负责SQLite数据库的连接、初始化和事务管理

/// 获取数据库文件的完整路径
/// 数据库文件位于用户数据目录下的StardewProfilesManager子目录中
pub fn get_database_path() -> Result<PathBuf, String> {
    let mut data_path = data_dir().ok_or_else(|| "无法获取用户数据目录".to_string())?;
    data_path.push("StardewProfilesManager");

    // 创建目录（如果不存在）
    std::fs::create_dir_all(&data_path).map_err(|e| format!("无法创建数据库目录: {}", e))?;

    data_path.push("profiles.db");
    Ok(data_path)
}

/// 初始化数据库连接
pub fn init_database() -> Result<Connection, String> {
    let db_path = get_database_path()?;
    let conn = Connection::open(&db_path)
        .map_err(|e| format!("无法打开数据库 {}: {}", db_path.display(), e))?;

    // 启用外键约束
    conn.execute("PRAGMA foreign_keys = ON;", [])
        .map_err(|e| format!("无法启用外键约束: {}", e))?;

    Ok(conn)
}

/// 执行数据库迁移（创建表结构）
pub fn run_migrations(conn: &Connection) -> Result<(), String> {
    // 导入表结构定义
    use crate::database::schema::*;

    // 创建mods表
    conn.execute(CREATE_MODS_TABLE, [])
        .map_err(|e| format!("创建mods表失败: {}", e))?;

    // 创建profiles表
    conn.execute(CREATE_PROFILES_TABLE, [])
        .map_err(|e| format!("创建profiles表失败: {}", e))?;

    // 创建profile_mods表
    conn.execute(CREATE_PROFILE_MODS_TABLE, [])
        .map_err(|e| format!("创建profile_mods表失败: {}", e))?;

    // 创建索引
    for create_index_sql in CREATE_INDEXES {
        conn.execute(create_index_sql, [])
            .map_err(|e| format!("创建索引失败: {}", e))?;
    }

    Ok(())
}
