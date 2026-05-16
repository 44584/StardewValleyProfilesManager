/// 数据库操作模块
pub mod connection;
pub mod schema;
pub mod mod_repository;
pub mod profile_repository;
pub mod profile_mod_repository;

use rusqlite::Connection;

/// 初始化数据库连接并执行迁移
pub fn initialize_database() -> Result<Connection, String> {
    let conn = connection::init_database()?;
    connection::run_migrations(&conn)?;
    Ok(conn)
}