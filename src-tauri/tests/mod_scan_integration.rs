#[cfg(test)]
mod tests {
    use std::fs;
    use tempfile::TempDir;
    use stardewprofilesmanager_lib::commands::mod_commands;
    
    #[test]
    fn test_scan_and_register_mods() {
        // 创建临时目录用于测试
        let temp_dir = TempDir::new().expect("无法创建临时目录");
        let mods_dir = temp_dir.path().join("Mods");
        fs::create_dir(&mods_dir).expect("无法创建Mods目录");
        
        // 创建测试模组目录
        let test_mod_dir = mods_dir.join("TestMod");
        fs::create_dir(&test_mod_dir).expect("无法创建测试模组目录");
        
        // 创建测试manifest.json
        let manifest_content = r#"{
            "Name": "Test Mod",
            "Author": "Test Author",
            "Version": "1.0.0",
            "Description": "A test mod for integration testing",
            "UniqueID": "TestAuthor.TestMod",
            "EntryDll": "TestMod.dll",
            "MinimumApiVersion": "3.0.0",
            "UpdateKeys": ["TestAuthor.TestMod"]
        }"#;
        
        fs::write(test_mod_dir.join("manifest.json"), manifest_content)
            .expect("无法写入manifest.json");
        
        // 调用扫描和注册功能
        let result = mod_commands::scan_and_register_mods(
            mods_dir.to_str().unwrap().to_string()
        );
        
        assert!(result.is_ok(), "扫描和注册应该成功");
        let scanned_mods = result.unwrap();
        assert_eq!(scanned_mods.len(), 1, "应该扫描到1个模组");
        
        let mod_info = &scanned_mods[0];
        assert_eq!(mod_info.unique_id, "TestAuthor.TestMod");
        assert_eq!(mod_info.name, "Test Mod");
        assert_eq!(mod_info.author, "Test Author");
        assert_eq!(mod_info.version, "1.0.0");
        assert_eq!(mod_info.entry_dll, Some("TestMod.dll".to_string()));
        assert_eq!(mod_info.minimum_api_version, Some("3.0.0".to_string()));
        
        // 验证数据库中确实存在该记录
        let all_mods = mod_commands::get_all_mods().unwrap();
        assert_eq!(all_mods.len(), 1, "数据库中应该有1个模组");
        assert_eq!(all_mods[0].unique_id, "TestAuthor.TestMod");
    }
}