/// 数据库表结构定义模块
/// 包含mods、profiles、profile_mods三张核心表的SQL定义
pub const CREATE_MODS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS mods (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    unique_id TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    author TEXT NOT NULL,
    version TEXT NOT NULL,
    description TEXT,
    entry_dll TEXT,
    content_pack_for TEXT,
    minimum_api_version TEXT,
    dependencies_json TEXT,
    update_keys_json TEXT,
    mod_path TEXT NOT NULL,
    manifest_hash TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
"#;

pub const CREATE_PROFILES_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS profiles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    profile_path TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
"#;

pub const CREATE_PROFILE_MODS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS profile_mods (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    profile_id INTEGER NOT NULL,
    mod_id INTEGER NOT NULL,
    is_enabled BOOLEAN DEFAULT TRUE,
    link_path TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (profile_id) REFERENCES profiles(id) ON DELETE CASCADE,
    FOREIGN KEY (mod_id) REFERENCES mods(id) ON DELETE CASCADE,
    UNIQUE(profile_id, mod_id)
);
"#;

/// 创建所有必要的数据库索引
pub const CREATE_INDEXES: &[&str] = &[
    "CREATE INDEX IF NOT EXISTS idx_mods_unique_id ON mods(unique_id);",
    "CREATE INDEX IF NOT EXISTS idx_mods_name ON mods(name);",
    "CREATE INDEX IF NOT EXISTS idx_mods_author ON mods(author);",
    "CREATE INDEX IF NOT EXISTS idx_mods_minimum_api_version ON mods(minimum_api_version);",
    "CREATE INDEX IF NOT EXISTS idx_profiles_name ON profiles(name);",
    "CREATE INDEX IF NOT EXISTS idx_profile_mods_profile_id ON profile_mods(profile_id);",
    "CREATE INDEX IF NOT EXISTS idx_profile_mods_mod_id ON profile_mods(mod_id);",
    "CREATE INDEX IF NOT EXISTS idx_profile_mods_is_enabled ON profile_mods(is_enabled);",
];