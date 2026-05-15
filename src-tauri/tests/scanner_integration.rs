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
    let mods = scanner
        .scan_mods_directory(temp_dir.path().to_str().unwrap())
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
    let mods = scanner
        .scan_mods_directory(temp_dir.path().to_str().unwrap())
        .expect("Should parse content pack successfully");

    assert_eq!(mods.len(), 1);
    let mod_info = &mods[0];

    assert_eq!(mod_info.name, "Test Content Pack");
    assert_eq!(
        mod_info.content_pack_for,
        Some("Pathoschild.ContentPatcher".to_string())
    );
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
    let mods = scanner
        .scan_mods_directory(temp_dir.path().to_str().unwrap())
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
    let mods = scanner
        .scan_mods_directory(temp_dir.path().to_str().unwrap())
        .expect("Should handle missing manifest gracefully");

    // 只应该返回有效的模组
    assert_eq!(mods.len(), 1);
}

#[test]
fn test_real_mods() {
    // C:\Program Files (x86)\Steam\steamapps\common\Stardew Valley\Mods_simple
    let scanner = ModScanner;
    let mods = scanner
        .scan_mods_directory(
            "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Stardew Valley\\Mods",
        )
        .expect("Should scan real mods successfully");

    assert!(mods.len() > 0);

    // 打印模组名
    println!("Found {} mods:", mods.len());
    for mod_info in &mods {
        println!("{}", mod_info.name);
    }
}

#[test]
fn test_real_console_commands_manifest() {
    let temp_dir = TempDir::new().unwrap();

    // 使用真实的Console Commands manifest格式 (UniqueId - 小写d)
    let manifest_content = r#"{
        "Name": "Console Commands",
        "Author": "SMAPI",
        "Version": "4.3.2",
        "Description": "Adds SMAPI console commands that let you manipulate the game.",
        "UniqueId": "SMAPI.ConsoleCommands",
        "EntryDll": "ConsoleCommands.dll",
        "MinimumApiVersion": "4.3.2"
    }"#;

    let mod_dir = create_test_manifest(&temp_dir, manifest_content);

    let scanner = ModScanner;
    let mods = scanner
        .scan_mods_directory(temp_dir.path().to_str().unwrap())
        .expect("Should parse real Console Commands manifest successfully");

    assert_eq!(mods.len(), 1);
    let mod_info = &mods[0];

    assert_eq!(mod_info.name, "Console Commands");
    assert_eq!(mod_info.author, "SMAPI");
    assert_eq!(mod_info.version, "4.3.2");
    assert_eq!(mod_info.unique_id, "SMAPI.ConsoleCommands");
    assert_eq!(mod_info.entry_dll, Some("ConsoleCommands.dll".to_string()));
    assert_eq!(mod_info.minimum_api_version, Some("4.3.2".to_string()));
    assert_eq!(mod_info.mod_path, mod_dir);
}

#[test]
fn test_unique_id_uppercase_d_format() {
    let temp_dir = TempDir::new().unwrap();

    // 测试 UniqueID 格式 (大写D)
    let manifest_content = r#"{
        "Name": "Test Mod UpperCase D",
        "Author": "Test Author",
        "Version": "1.0.0",
        "Description": "A test mod with UniqueID (uppercase D)",
        "UniqueID": "TestAuthor.TestModUpperD",
        "EntryDll": "TestMod.dll",
        "MinimumApiVersion": "3.8.0"
    }"#;

    let mod_dir = create_test_manifest(&temp_dir, manifest_content);

    let scanner = ModScanner;
    let mods = scanner
        .scan_mods_directory(temp_dir.path().to_str().unwrap())
        .expect("Should parse UniqueID (uppercase D) format successfully");

    assert_eq!(mods.len(), 1);
    let mod_info = &mods[0];

    assert_eq!(mod_info.name, "Test Mod UpperCase D");
    assert_eq!(mod_info.unique_id, "TestAuthor.TestModUpperD");
    assert_eq!(mod_info.entry_dll, Some("TestMod.dll".to_string()));
    assert_eq!(mod_info.mod_path, mod_dir);
}

#[test]
fn test_content_pack_for_field_name_compatibility() {
    let temp_dir = TempDir::new().unwrap();

    // 测试 ContentPackFor 使用 UniqueId (小写d) 格式
    let manifest_content = r#"{
        "Name": "Test Content Pack Lowercase d",
        "Author": "Test Author",
        "Version": "1.0.0",
        "Description": "A test content pack with UniqueId (lowercase d)",
        "UniqueId": "TestAuthor.TestContentPackLowerD",
        "ContentPackFor": {
            "UniqueId": "Pathoschild.ContentPatcher"
        }
    }"#;

    let mod_dir = create_test_manifest(&temp_dir, manifest_content);

    let scanner = ModScanner;
    let mods = scanner.scan_mods_directory(temp_dir.path().to_str().unwrap())
        .expect("Should parse ContentPackFor with UniqueId (lowercase d) successfully");

    assert_eq!(mods.len(), 1);
    let mod_info = &mods[0];

    assert_eq!(mod_info.name, "Test Content Pack Lowercase d");
    assert_eq!(mod_info.unique_id, "TestAuthor.TestContentPackLowerD");
    assert_eq!(mod_info.content_pack_for, Some("Pathoschild.ContentPatcher".to_string()));
    assert_eq!(mod_info.mod_path, mod_dir);
}

#[test]
fn test_manifest_with_bom() {
    let temp_dir = TempDir::new().unwrap();

    // 创建带有UTF-8 BOM的manifest.json
    let manifest_content_with_bom = format!(
        "\u{feff}{}",
        r#"{
        "Name": "Test Mod With BOM",
        "Author": "Test Author",
        "Version": "1.0.0",
        "Description": "A test mod with UTF-8 BOM",
        "UniqueId": "TestAuthor.TestModWithBOM",
        "EntryDll": "TestMod.dll",
        "MinimumApiVersion": "3.8.0"
    }"#
    );

    let mod_dir = create_test_manifest(&temp_dir, &manifest_content_with_bom);

    let scanner = ModScanner;
    let mods = scanner
        .scan_mods_directory(temp_dir.path().to_str().unwrap())
        .expect("Should parse manifest with BOM successfully");

    assert_eq!(mods.len(), 1);
    let mod_info = &mods[0];

    assert_eq!(mod_info.name, "Test Mod With BOM");
    assert_eq!(mod_info.unique_id, "TestAuthor.TestModWithBOM");
    assert_eq!(mod_info.entry_dll, Some("TestMod.dll".to_string()));
    assert_eq!(mod_info.mod_path, mod_dir);
}

#[test]
fn test_generic_mod_config_menu_with_comments() {
    let temp_dir = TempDir::new().unwrap();

    // 测试带有注释的GenericModConfigMenu manifest
    let manifest_content = r#"{
  /*
   | This file is automatically updated by ModManifestBuilder
   | when the project is compiled.
   | 
   | Changes made to this file may be overwritten.
   | 
   */
  "$schema": "https://smapi.io/schemas/manifest.json",
  "UniqueId": "spacechase0.GenericModConfigMenu",
  "Name": "Generic Mod Config Menu",
  "Author": "kittycatcasey",
  "Version": "1.16.0",
  "Description": "Adds an in-game UI to edit other mods' config options (for mods which support it).",
  "MinimumApiVersion": "4.1",
  "MinimumGameVersion": "1.6.14",
  "EntryDll": "GenericModConfigMenu.dll",
  "UpdateKeys": [
    "Nexus:5098"
  ]
}"#;

    let mod_dir = create_test_manifest(&temp_dir, manifest_content);

    let scanner = ModScanner;
    let mods = scanner
        .scan_mods_directory(temp_dir.path().to_str().unwrap())
        .expect("Should parse GenericModConfigMenu with comments successfully");

    assert_eq!(mods.len(), 1);
    let mod_info = &mods[0];

    assert_eq!(mod_info.name, "Generic Mod Config Menu");
    assert_eq!(mod_info.unique_id, "spacechase0.GenericModConfigMenu");
    assert_eq!(
        mod_info.entry_dll,
        Some("GenericModConfigMenu.dll".to_string())
    );
    assert_eq!(mod_info.mod_path, mod_dir);
}

#[test]
fn test_artisan_goods_keep_quality_with_trailing_commas() {
    let temp_dir = TempDir::new().unwrap();

    // 准确复现原始错误中的JSON格式，使用UniqueID（大写D）
    let manifest_content = r#"{
    "Name": "Artisan Goods Keep Quality",
    "Author": "voiddreams",
    "Version": "1.4.7",
    "Description": "A mod to let artisan goods keep the quality of their input",
    "UniqueID": "voiddreams.ArtisanGoodsKeepQuality",
    "UpdateKeys": [ "Nexus:21278" ],
    "ContentPackFor": {
        "UniqueID": "Pathoschild.ContentPatcher",
    }
}"#;

    let mod_dir = create_test_manifest(&temp_dir, manifest_content);

    let scanner = ModScanner;
    let mods = scanner
        .scan_mods_directory(temp_dir.path().to_str().unwrap())
        .expect("Should parse ArtisanGoodsKeepQuality with trailing commas successfully");

    assert_eq!(mods.len(), 1);
    let mod_info = &mods[0];

    assert_eq!(mod_info.name, "Artisan Goods Keep Quality");
    assert_eq!(mod_info.unique_id, "voiddreams.ArtisanGoodsKeepQuality");
    assert_eq!(
        mod_info.content_pack_for,
        Some("Pathoschild.ContentPatcher".to_string())
    );
    assert_eq!(mod_info.mod_path, mod_dir);
}

#[test]
fn test_stardew_hack_manifest() {
    let temp_dir = TempDir::new().unwrap();

    let manifest_content = r#"{
   "Name": "星露骇客框架",
   "Author": "bcmpinc",
   "Version": "7.4",
   "Description": "Transpilation library used by my other mods. Doesn't do much on its own.",
   "UniqueID": "bcmpinc.StardewHack",
   "EntryDll": "StardewHack.dll",
   "MinimumApiVersion": "4.0.0",
   "UpdateKeys": ["Nexus:3213"],
   "Dependencies": [{
      "UniqueId": "spacechase0.GenericModConfigMenu",
      "MinimumVersion": "1.12",
      "IsRequired": false
   }]
}"#;

    let mod_dir = create_test_manifest(&temp_dir, manifest_content);

    let scanner = ModScanner;
    let mods = scanner.scan_mods_directory(temp_dir.path().to_str().unwrap())
        .expect("Should parse StardewHack manifest successfully");

    assert_eq!(mods.len(), 1);
    let mod_info = &mods[0];

    assert_eq!(mod_info.name, "星露骇客框架");
    assert_eq!(mod_info.unique_id, "bcmpinc.StardewHack");
    assert_eq!(mod_info.entry_dll, Some("StardewHack.dll".to_string()));
    assert_eq!(mod_info.mod_path, mod_dir);
}

#[test]
fn test_gmcm_options_manifest() {
    let temp_dir = TempDir::new().unwrap();

    let manifest_content = r#"{
    "Name": "GMCM Options",
    "Author": "Jamie Taylor",
    "Version": "2.1.0",
    "Description": "Provides complex Option types for Generic Mod Config Menu",
    "UniqueID": "jltaylor-us.GMCMOptions",
    "EntryDll": "GMCMOptions.dll",
    "MinimumApiVersion": "4.0.0",
    "Dependencies": [ { "UniqueId": "spacechase0.GenericModConfigMenu", "MinimumVersion":  "1.8.0" } ],
    "UpdateKeys": [ "Nexus:10505", "GitHub:jltaylor-us/StardewGMCMOptions" ]
}"#;

    let mod_dir = create_test_manifest(&temp_dir, manifest_content);

    let scanner = ModScanner;
    let mods = scanner.scan_mods_directory(temp_dir.path().to_str().unwrap())
        .expect("Should parse GMCM Options manifest successfully");

    assert_eq!(mods.len(), 1);
    let mod_info = &mods[0];

    assert_eq!(mod_info.name, "GMCM Options");
    assert_eq!(mod_info.unique_id, "jltaylor-us.GMCMOptions");
    assert_eq!(mod_info.entry_dll, Some("GMCMOptions.dll".to_string()));
    assert_eq!(mod_info.mod_path, mod_dir);
}
