//! Scanner 模块集成测试
//! 测试模组扫描和manifest解析功能

use stardewprofilesmanager_lib::scanner::mod_scanner::ModScanner;
use std::fs;
use std::io::Write;
use tempfile::TempDir;

/// 创建测试用的manifest.json文件
fn create_test_manifest(temp_dir: &TempDir, manifest_content: &str) -> std::path::PathBuf {
    let mod_dir = temp_dir.path().join("TestMod");
    fs::create_dir_all(&mod_dir).unwrap();
    
    let manifest_path = mod_dir.join("manifest.json");
    let mut file = fs::File::create(&manifest_path).unwrap();
    file.write_all(manifest_content.as_bytes()).unwrap();
    
    mod_dir
}

#[test]
fn test_smapi_mod_parsing() {
    let temp_dir = TempDir::new().unwrap();
    
    let manifest_content = r#"{
        "Name": "Test Mod",
        "Author": "Test Author",
        "Version": "1.0.0",
        "Description": "A test SMAPI mod",
        "UniqueID": "TestAuthor.TestMod",
        "EntryDll": "TestMod.dll",
        "MinimumApiVersion": "3.8.0",
        "UpdateKeys": ["Nexus:12345"]
    }"#;
    
    let mod_dir = create_test_manifest(&temp_dir, manifest_content);
    
    let scanner = ModScanner;
    let mods = scanner.scan_mods_directory(temp_dir.path().to_str().unwrap())
        .expect("Should parse SMAPI mod successfully");
    
    assert_eq!(mods.len(), 1);
    let mod_info = &mods[0];
    
    assert_eq!(mod_info.name, "Test Mod");
    assert_eq!(mod_info.author, "Test Author");
    assert_eq!(mod_info.version, "1.0.0");
    assert_eq!(mod_info.unique_id, "TestAuthor.TestMod");
    assert_eq!(mod_info.entry_dll, Some("TestMod.dll".to_string()));
    assert_eq!(mod_info.minimum_api_version, Some("3.8.0".to_string()));
    assert!(mod_info.update_keys_json.is_some());
    assert_eq!(mod_info.mod_path, mod_dir);
}

#[test]
fn test_content_pack_parsing() {
    let temp_dir = TempDir::new().unwrap();
    
    let manifest_content = r#"{
        "Name": "Test Content Pack",
        "Author": "Test Author",
        "Version": "1.0.0",
        "Description": "A test content pack",
        "UniqueID": "TestAuthor.TestContentPack",
        "ContentPackFor": {
            "UniqueID": "Pathoschild.ContentPatcher"
        },
        "Dependencies": [
            {
                "UniqueID": "Some.Dependency",
                "MinimumVersion": "1.0.0",
                "IsRequired": true
            }
        ]
    }"#;
    
    let mod_dir = create_test_manifest(&temp_dir, manifest_content);
    
    let scanner = ModScanner;
    let mods = scanner.scan_mods_directory(temp_dir.path().to_str().unwrap())
        .expect("Should parse content pack successfully");
    
    assert_eq!(mods.len(), 1);
    let mod_info = &mods[0];
    
    assert_eq!(mod_info.name, "Test Content Pack");
    assert_eq!(mod_info.content_pack_for, Some("Pathoschild.ContentPatcher".to_string()));
    assert!(mod_info.dependencies_json.is_some());
    assert_eq!(mod_info.mod_path, mod_dir);
}

#[test]
fn test_invalid_manifest_handling() {
    let temp_dir = TempDir::new().unwrap();
    
    // 创建无效的manifest.json
    let invalid_mod_dir = temp_dir.path().join("InvalidMod");
    fs::create_dir_all(&invalid_mod_dir).unwrap();
    
    let manifest_path = invalid_mod_dir.join("manifest.json");
    fs::write(manifest_path, "{ invalid json }").unwrap();
    
    let scanner = ModScanner;
    let mods = scanner.scan_mods_directory(temp_dir.path().to_str().unwrap())
        .expect("Should skip invalid manifest without panicking");
    
    // 无效的manifest应该被跳过，不返回任何模组
    assert_eq!(mods.len(), 0);
}

#[test]
fn test_missing_manifest_handling() {
    let temp_dir = TempDir::new().unwrap();
    
    // 创建没有manifest.json的目录
    let no_manifest_dir = temp_dir.path().join("NoManifestMod");
    fs::create_dir_all(&no_manifest_dir).unwrap();
    
    // 创建一个有效的manifest用于对比
    let valid_manifest_content = r#"{
        "Name": "Valid Mod",
        "Author": "Author",
        "Version": "1.0.0",
        "UniqueID": "Author.ValidMod",
        "EntryDll": "ValidMod.dll"
    }"#;
    create_test_manifest(&temp_dir, valid_manifest_content);
    
    let scanner = ModScanner;
    let mods = scanner.scan_mods_directory(temp_dir.path().to_str().unwrap())
        .expect("Should handle missing manifest gracefully");
    
    // 只应该返回有效的模组
    assert_eq!(mods.len(), 1);
}