import React, { useState, useEffect } from 'react';
import './App.css';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';

interface Profile {
  id: number;
  name: string;
  description: string;
}

interface ModInfo {
  id: number;
  uniqueId: string;
  name: string;
  author: string;
  version: string;
  description?: string;
  entryDll?: string;
  contentPackFor?: string;
  minimumApiVersion?: string;
  dependenciesJson?: string;
  updateKeysJson?: string;
  modPath: string;
  manifestHash: string;
}

interface ProfileMod {
  id: number;
  profileId: number;
  modId: number;
  isEnabled: boolean;
  linkPath?: string;
}

const App: React.FC = () => {
  // Profile状态
  const [profiles, setProfiles] = useState<Profile[]>([]);
  const [selectedProfileId, setSelectedProfileId] = useState<number | null>(null);
  const [isCreatingProfile, setIsCreatingProfile] = useState(false);
  const [profileName, setProfileName] = useState("");
  const [profileDescription, setProfileDescription] = useState("");
  
  // SMPI路径状态
  const [smapiPath, setSmapiPath] = useState("");
  
  // 模组状态
  const [registeredMods, setRegisteredMods] = useState<ModInfo[]>([]);
  const [profileMods, setProfileMods] = useState<ProfileMod[]>([]);

  // 使用状态更新函数来保存配置
  const [modsDirectory, setModsDirectory] = useState("");

  // 分页状态
  const [profilePage, setProfilePage] = useState(1);
  const [profileModsPage, setProfileModsPage] = useState(1);
  const [registeredModsPage, setRegisteredModsPage] = useState(1);
  const ITEMS_PER_PAGE = 10;
  
  // 加载状态
  const [isLoadingProfiles, setIsLoadingProfiles] = useState(false);
  const [isLoadingProfileMods, setIsLoadingProfileMods] = useState(false);
  const [isLoadingRegisteredMods, setIsLoadingRegisteredMods] = useState(false);
  const [isScanning, setIsScanning] = useState(false);
  
  // 错误状态
  const [error, setError] = useState<string | null>(null);

  // 保存配置到后端
  const saveConfig = async () => {
    try {
      await invoke('save_app_config', {
        smapiPath,
        modsDirectory
      });
    } catch (err) {
      console.warn("Failed to save config:", err);
    }
  };

  // 从后端加载配置
  const loadConfig = async () => {
    try {
      const config: any = await invoke('load_app_config');
      if (config.smapiPath) {
        setSmapiPath(config.smapiPath);
      }
      if (config.modsDirectory) {
        setModsDirectory(config.modsDirectory);
      }
    } catch (err) {
      console.warn("Failed to load config:", err);
    }
  };

  // 分页辅助函数
  const paginate = <T,>(items: T[], page: number, itemsPerPage: number): T[] => {
    const startIndex = (page - 1) * itemsPerPage;
    return items.slice(startIndex, startIndex + itemsPerPage);
  };

  const getTotalPages = (totalItems: number, itemsPerPage: number): number => {
    return Math.ceil(totalItems / itemsPerPage);
  };

  // 加载所有Profiles
  const loadProfiles = async () => {
    setIsLoadingProfiles(true);
    try {
      const profiles = await invoke<Profile[]>("get_all_profiles");
      setProfiles(profiles);
      setProfilePage(1); // 重置到第一页
    } catch (err) {
      console.error("Failed to load profiles:", err);
      const errorMessage = typeof err === 'string' ? err : (err as Error).message;
      setError("加载配置方案失败: " + errorMessage);
    } finally {
      setIsLoadingProfiles(false);
    }
  };

  // 加载已注册的模组
  const loadRegisteredMods = async () => {
    setIsLoadingRegisteredMods(true);
    try {
      const mods = await invoke<ModInfo[]>("get_all_mods");
      console.log("Loaded mods from backend:", mods);
      setRegisteredMods(mods);
      setRegisteredModsPage(1); // 重置到第一页
    } catch (err) {
      console.error("Failed to load registered mods:", err);
      const errorMessage = typeof err === 'string' ? err : (err as Error).message;
      setError("加载已注册模组失败: " + errorMessage);
    } finally {
      setIsLoadingRegisteredMods(false);
    }
  };

  // 加载Profile Mods
  const loadProfileMods = async (profileId: number) => {
    if (profileId <= 0) return;
    
    setIsLoadingProfileMods(true);
    try {
      const mods = await invoke<ProfileMod[]>("get_mods_for_profile", { profileId });
      setProfileMods(mods);
      setProfileModsPage(1); // 重置到第一页
    } catch (err) {
      console.error("Failed to load profile mods:", err);
      const errorMessage = typeof err === 'string' ? err : (err as Error).message;
      setError("加载配置模组失败: " + errorMessage);
    } finally {
      setIsLoadingProfileMods(false);
    }
  };

  // 扫描并注册模组
  const handleScanAndRegisterMods = async () => {
    if (!modsDirectory.trim()) {
      setError("请输入Mods目录路径");
      return;
    }
    
    setIsScanning(true);
    setError(null);
    try {
      const scannedMods = await invoke<ModInfo[]>("scan_and_register_mods", { 
        modsDirectory 
      });
      console.log("Scanned and registered mods:", scannedMods);
      await loadRegisteredMods();
    } catch (err) {
      console.error("Scan and register mods failed:", err);
      const errorMessage = typeof err === 'string' ? err : (err as Error).message;
      setError("扫描并注册模组失败: " + errorMessage);
    } finally {
      setIsScanning(false);
    }
  };

  // 创建Profile
  const handleCreateProfile = async () => {
    if (!profileName.trim()) {
      setError("请输入配置方案名称");
      return;
    }
    
    try {
      await invoke("create_profile", { 
        name: profileName, 
        description: profileDescription 
      });
      setProfileName("");
      setProfileDescription("");
      setIsCreatingProfile(false);
      await loadProfiles();
    } catch (err) {
      console.error("Create profile failed:", err);
      const errorMessage = typeof err === 'string' ? err : (err as Error).message;
      setError("创建配置方案失败: " + errorMessage);
    }
  };

  // 删除Profile
  const handleDeleteProfile = async (profileId: number) => {
    if (!window.confirm("确定要删除这个配置方案吗？这将移除所有关联的模组配置。")) {
      return;
    }
    
    try {
      await invoke("delete_profile", { profileId });
      await loadProfiles();
      if (selectedProfileId === profileId) {
        setSelectedProfileId(null);
        setProfileMods([]);
      }
    } catch (err) {
      console.error("Delete profile failed:", err);
      const errorMessage = typeof err === 'string' ? err : (err as Error).message;
      setError("删除配置方案失败: " + errorMessage);
    }
  };

  // Removed handleLaunchGame method as it's no longer needed

  // 启动游戏（使用指定Profile）
  const handleLaunchGameWithProfile = async (profileId: number) => {
    if (!smapiPath.trim()) {
      setError("请输入SMAPI可执行文件路径");
      return;
    }
    
    try {
      // 验证SMPI路径是否有效
      const isValid = await invoke<boolean>("validate_smapi_installation", { 
        smapiPath 
      });
      
      if (!isValid) {
        setError("SMAPI可执行文件路径无效，请检查路径是否正确");
        return;
      }
      
      // 启动游戏
      await invoke("launch_game_with_profile", { 
        profileId, 
        smapiPath 
      });
      
      // 显示成功消息
      setError("游戏已启动！如果游戏窗口没有出现，请检查SMAPI日志。");
      
    } catch (err) {
      console.error("Launch game with profile failed:", err);
      const errorMessage = typeof err === 'string' ? err : (err as Error).message;
      setError("启动游戏失败: " + errorMessage);
    }
  };

  // 验证SMPI路径
  const handleValidateSmapiPath = async () => {
    if (!smapiPath.trim()) {
      setError("请输入SMAPI可执行文件路径");
      return;
    }
    
    try {
      const isValid = await invoke<boolean>("validate_smapi_installation", { 
        smapiPath 
      });
      
      if (isValid) {
        setError("SMAPI路径有效！");
      } else {
        setError("SMAPI路径无效，请检查路径是否正确");
      }
    } catch (err) {
      console.error("Validate SMPI path failed:", err);
      const errorMessage = typeof err === 'string' ? err : (err as Error).message;
      setError("验证SMPI路径失败: " + errorMessage);
    }
  };

  // 选择SMAPI可执行文件
  const handleSelectSmapiFile = async () => {
    try {
      const selected = await open({
        title: "选择SMAPI可执行文件",
        filters: [{
          name: "可执行文件",
          extensions: ["exe"]
        }],
        multiple: false
      });
      
      if (selected && typeof selected === 'string') {
        setSmapiPath(selected);
      }
    } catch (err) {
      console.error("选择SMAPI文件失败:", err);
      setError("选择SMAPI文件失败: " + (err as Error).message);
    }
  };

  // 选择Mods目录
  const handleSelectModsDirectory = async () => {
    try {
      const selected = await open({
        title: "选择Mods目录",
        directory: true,
        multiple: false
      });
      
      if (selected && typeof selected === 'string') {
        setModsDirectory(selected);
      }
    } catch (err) {
      console.error("选择Mods目录失败:", err);
      setError("选择Mods目录失败: " + (err as Error).message);
    }
  };

  // 向Profile添加Mod
  const handleAddModToProfile = async (uniqueId: string) => {
    if (selectedProfileId === null) {
      setError("请先选择一个配置方案");
      return;
    }
    
    // 添加参数验证
    if (!uniqueId || typeof uniqueId !== 'string') {
      console.error("Invalid uniqueId parameter:", uniqueId);
      setError("无效的Mod Unique ID");
      return;
    }

    console.log(`Added mod ${uniqueId} to profile ${selectedProfileId}`);
    try {
      await invoke("add_mod_to_profile", { 
        profileId: selectedProfileId, 
        uniqueId 
      });
      
      // 重新加载Profile Mods
      await loadProfileMods(selectedProfileId);
    } catch (err) {
      console.error("Add mod to profile failed:", err);
      const errorMessage = typeof err === 'string' ? err : (err as Error).message;
      setError("添加Mod到配置方案失败: " + errorMessage);
    }
  };

  // 从Profile移除Mod
  const handleRemoveModFromProfile = async (modId: number) => {
    if (selectedProfileId === null) {
      setError("请先选择一个配置方案");
      return;
    }
    
    if (!window.confirm("确定要从配置方案中移除这个模组吗？")) {
      return;
    }
    
    try {
      // 需要通过modId找到对应的uniqueId
      const modInfo = registeredMods.find(mod => mod.id === modId);
      if (!modInfo) {
        throw new Error("未找到对应的模组信息");
      }
      
      await invoke("remove_mod_from_profile", { 
        profileId: selectedProfileId, 
        uniqueId: modInfo.uniqueId 
      });
      
      await loadProfileMods(selectedProfileId);
    } catch (err) {
      console.error("Remove mod from profile failed:", err);
      const errorMessage = typeof err === 'string' ? err : (err as Error).message;
      setError("从配置方案移除Mod失败: " + errorMessage);
    }
  };

  // 切换Mod启用状态
  const handleToggleModEnabled = async (modId: number, currentEnabled: boolean) => {
    if (selectedProfileId === null) {
      setError("请先选择一个配置方案");
      return;
    }
    
    try {
      const modInfo = registeredMods.find(mod => mod.id === modId);
      if (!modInfo) {
        throw new Error("未找到对应的模组信息");
      }
      
      await invoke("toggle_mod_enabled", { 
        profileId: selectedProfileId, 
        uniqueId: modInfo.uniqueId,
        isEnabled: !currentEnabled
      });
      
      await loadProfileMods(selectedProfileId);
    } catch (err) {
      console.error("Toggle mod enabled failed:", err);
      const errorMessage = typeof err === 'string' ? err : (err as Error).message;
      setError("切换Mod启用状态失败: " + errorMessage);
    }
  };

  // 检查模组是否已添加到当前Profile
  const isModAddedToProfile = (uniqueId: string): boolean => {
    if (!uniqueId || typeof uniqueId !== 'string') {
      return false;
    }
    
    // 找到对应uniqueId的ModInfo，获取其数字ID
    const modInfo = registeredMods.find(mod => mod.uniqueId === uniqueId);
    if (!modInfo || !modInfo.id) {
      return false;
    }
    // 检查profileMods中是否包含该数字ID
    return profileMods.some(pm => pm.modId === modInfo.id);
  };

  // 初始化加载
  useEffect(() => {
    loadConfig(); // 先加载配置
    loadProfiles();
    loadRegisteredMods();
  }, []);

  // 当选择Profile时，加载对应的模组
  useEffect(() => {
    if (selectedProfileId !== null) {
      loadProfileMods(selectedProfileId);
    } else {
      setProfileMods([]);
    }
  }, [selectedProfileId]);

  // 监听smapiPath变化并保存配置
  useEffect(() => {
    if (smapiPath !== "") {
      saveConfig();
    }
  }, [smapiPath]);

  // 监听modsDirectory变化并保存配置
  useEffect(() => {
    if (modsDirectory !== "") {
      saveConfig();
    }
  }, [modsDirectory]);

  // 获取当前选中的Profile
  const selectedProfile = profiles.find(p => p.id === selectedProfileId);

  return (
    <div className="app">
      <header>
        <h1>Stardew Profiles Manager</h1>
      </header>

      <main className="main-layout">
        {/* 左侧栏：Profile管理和SMPI配置 */}
        <section className="profile-management">
          {/* SMPI路径输入区域 */}
          <div className="smapi-path-section">
            <div className="section-header">
              <h2>SMAPI配置</h2>
            </div>
            <div className="smapi-controls">
              <input
                type="text"
                placeholder="SMAPI可执行文件路径 (如: C:\Program Files (x86)\Steam\steamapps\common\Stardew Valley\StardewModdingAPI.exe)"
                value={smapiPath}
                onChange={(e) => setSmapiPath(e.target.value)}
                className="form-input"
              />
              <div className="smapi-button-group">
                <button
                  className="btn btn-secondary"
                  onClick={handleSelectSmapiFile}
                >
                  浏览
                </button>
                <button
                  className="btn btn-primary"
                  onClick={handleValidateSmapiPath}
                >
                  验证路径
                </button>
              </div>
            </div>
            {smapiPath && (
              <div className="smapi-info">
                <p>当前SMPI路径: {smapiPath}</p>
              </div>
            )}
          </div>

          <div className="section-header">
            <h2>配置方案管理</h2>
            <button 
              className="btn btn-primary"
              onClick={() => setIsCreatingProfile(true)}
            >
              新建配置方案
            </button>
          </div>

          {isCreatingProfile && (
            <div className="create-profile-form">
              <input
                type="text"
                placeholder="配置方案名称"
                value={profileName}
                onChange={(e) => setProfileName(e.target.value)}
                className="form-input"
              />
              <textarea
                placeholder="配置方案描述（可选）"
                value={profileDescription}
                onChange={(e) => setProfileDescription(e.target.value)}
                className="form-textarea"
              />
              <div className="form-actions">
                <button 
                  className="btn btn-success"
                  onClick={handleCreateProfile}
                >
                  创建
                </button>
                <button 
                  className="btn btn-secondary"
                  onClick={() => {
                    setIsCreatingProfile(false);
                    setProfileName("");
                    setProfileDescription("");
                  }}
                >
                  取消
                </button>
              </div>
            </div>
          )}

          {isLoadingProfiles ? (
            <div className="loading">加载配置方案中...</div>
          ) : (
            <>
              <div className="profiles-list">
                {profiles.length === 0 ? (
                  <div className="empty-state">
                    <p>暂无配置方案</p>
                    <button 
                      className="btn btn-secondary"
                      onClick={() => setIsCreatingProfile(true)}
                    >
                      创建第一个配置方案
                    </button>
                  </div>
                ) : (
                  paginate(profiles, profilePage, ITEMS_PER_PAGE).map((profile) => (
                    <div 
                      key={profile.id} 
                      className={`profile-card ${selectedProfileId === profile.id ? 'selected' : ''}`}
                    >
                      <div className="profile-header">
                        <h3>{profile.name}</h3>
                      </div>
                      {profile.description && (
                        <p className="profile-description">{profile.description}</p>
                      )}
                      <div className="profile-actions">
                        <button
                          className={`btn ${selectedProfileId === profile.id ? 'btn-secondary' : 'btn-outline'}`}
                          onClick={() => setSelectedProfileId(profile.id)}
                        >
                          {selectedProfileId === profile.id ? '已选中' : '选择'}
                        </button>
                        <button
                          className="btn btn-success"
                          onClick={() => handleLaunchGameWithProfile(profile.id)}
                        >
                          启动游戏
                        </button>
                        <button
                          className="btn btn-danger"
                          onClick={() => handleDeleteProfile(profile.id)}
                        >
                          删除
                        </button>
                      </div>
                    </div>
                  ))
                )}
              </div>
              
              {profiles.length > ITEMS_PER_PAGE && (
                <div className="pagination">
                  <button 
                    onClick={() => setProfilePage(Math.max(1, profilePage - 1))}
                    disabled={profilePage <= 1}
                  >
                    上一页
                  </button>
                  <span className="pagination-info">
                    第 {profilePage} 页，共 {getTotalPages(profiles.length, ITEMS_PER_PAGE)} 页
                  </span>
                  <button 
                    onClick={() => setProfilePage(Math.min(getTotalPages(profiles.length, ITEMS_PER_PAGE), profilePage + 1))}
                    disabled={profilePage >= getTotalPages(profiles.length, ITEMS_PER_PAGE)}
                  >
                    下一页
                  </button>
                </div>
              )}
            </>
          )}
        </section>

        {/* 中间栏：已添加模组区 */}
        <section className="added-mods">
          <div className="section-header">
            <h2>
              {selectedProfile 
                ? `当前配置的模组 (${profileMods.length}个)` 
                : '请选择配置方案'}
            </h2>
            {selectedProfile && (
              <div className="profile-info">
                <span>配置方案: {selectedProfile.name}</span>
                {/* Removed duplicate launch button - already available in profile cards */}
              </div>
            )}
          </div>

          {!selectedProfile ? (
            <div className="empty-state">
              <p>请在左侧选择一个配置方案以管理模组</p>
            </div>
          ) : isLoadingProfileMods ? (
            <div className="loading">加载模组中...</div>
          ) : profileMods.length === 0 ? (
            <div className="empty-state">
              <p>当前配置方案没有添加任何模组</p>
            </div>
          ) : (
            <>
              <div className="mods-grid">
                {paginate(profileMods, profileModsPage, ITEMS_PER_PAGE).map((profileMod) => {
                  const mod = registeredMods.find(m => m.id === profileMod.modId);
                  if (!mod) {
                    console.warn("Mod not found in registeredMods for modId:", profileMod.modId);
                    return null;
                  }
                  
                  return (
                    <div key={profileMod.id} className="mod-card added">
                      <div className="mod-header">
                        <h4>{mod.name}</h4>
                        <div className="mod-actions">
                          <div className="mod-toggle">
                            <label className="switch">
                              <input
                                type="checkbox"
                                checked={profileMod.isEnabled}
                                onChange={() => handleToggleModEnabled(profileMod.modId, profileMod.isEnabled)}
                              />
                              <span className="slider"></span>
                            </label>
                            <span className="toggle-label">
                              {profileMod.isEnabled ? '已启用' : '已禁用'}
                            </span>
                          </div>
                          <button
                            className="btn btn-danger remove-btn-small"
                            onClick={() => handleRemoveModFromProfile(profileMod.modId)}
                          >
                            移除
                          </button>
                        </div>
                      </div>
                      <div className="mod-details">
                        <p><strong>作者:</strong> {mod.author}</p>
                        <p><strong>版本:</strong> {mod.version}</p>
                      </div>
                    </div>
                  );
                })}
              </div>
              
              {profileMods.length > ITEMS_PER_PAGE && (
                <div className="pagination">
                  <button 
                    onClick={() => setProfileModsPage(Math.max(1, profileModsPage - 1))}
                    disabled={profileModsPage <= 1}
                  >
                    上一页
                  </button>
                  <span className="pagination-info">
                    第 {profileModsPage} 页，共 {getTotalPages(profileMods.length, ITEMS_PER_PAGE)} 页
                  </span>
                  <button 
                    onClick={() => setProfileModsPage(Math.min(getTotalPages(profileMods.length, ITEMS_PER_PAGE), profileModsPage + 1))}
                    disabled={profileModsPage >= getTotalPages(profileMods.length, ITEMS_PER_PAGE)}
                  >
                    下一页
                  </button>
                </div>
              )}
            </>
          )}
        </section>

        {/* 右侧栏：可用模组区 */}
        <section className="available-mods">
          <div className="section-header">
            <h2>可用模组</h2>
          </div>

          <div className="scan-controls">
            <input
              type="text"
              placeholder="Mods目录路径"
              value={modsDirectory}
              onChange={(e) => setModsDirectory(e.target.value)}
              className="form-input"
            />
            <div className="mods-button-group">
              <button
                className="btn btn-secondary"
                onClick={handleSelectModsDirectory}
              >
                浏览
              </button>
              <button
                className={`btn ${isScanning ? 'btn-disabled' : 'btn-primary'}`}
                onClick={handleScanAndRegisterMods}
                disabled={isScanning}
              >
                {isScanning ? '扫描中...' : '扫描'}
              </button>
            </div>
          </div>

          {isLoadingRegisteredMods ? (
            <div className="loading">加载可用模组中...</div>
          ) : registeredMods.length === 0 ? (
            <div className="empty-state">
              <p>暂无可使用模组</p>
              <p>请先扫描Mods目录</p>
            </div>
          ) : (
            <>
              <div className="mods-count">
                可用模组 ({registeredMods.length} 个) - 点击"添加到配置"按钮
              </div>
              <div className="mods-grid">
                {paginate(registeredMods, registeredModsPage, ITEMS_PER_PAGE).map((mod) => {
                  // 添加防御性检查
                  if (!mod.uniqueId) {
                    console.warn("Mod missing uniqueId:", mod);
                    return null;
                  }
                  
                  const isAdded = isModAddedToProfile(mod.uniqueId);
                  return (
                    <div 
                      key={mod.uniqueId} 
                      className={`mod-card available ${isAdded ? 'added' : ''}`}
                    >
                      <div className="mod-header">
                        <h4>{mod.name}</h4>
                        <button
                          className={`btn ${isAdded ? 'btn-disabled' : 'btn-success'}`}
                          onClick={() => !isAdded && handleAddModToProfile(mod.uniqueId)}
                          disabled={isAdded || selectedProfileId === null}
                        >
                          {isAdded ? '已添加' : '添加'}
                        </button>
                      </div>
                      <p><strong>作者:</strong> {mod.author}</p>
                      <p><strong>版本:</strong> {mod.version}</p>
                      {mod.description && <p><strong>描述:</strong> {mod.description}</p>}
                    </div>
                  );
                })}
              </div>
              
              {registeredMods.length > ITEMS_PER_PAGE && (
                <div className="pagination">
                  <button 
                    onClick={() => setRegisteredModsPage(Math.max(1, registeredModsPage - 1))}
                    disabled={registeredModsPage <= 1}
                  >
                    上一页
                  </button>
                  <span className="pagination-info">
                    第 {registeredModsPage} 页，共 {getTotalPages(registeredMods.length, ITEMS_PER_PAGE)} 页
                  </span>
                  <button 
                    onClick={() => setRegisteredModsPage(Math.min(getTotalPages(registeredMods.length, ITEMS_PER_PAGE), registeredModsPage + 1))}
                    disabled={registeredModsPage >= getTotalPages(registeredMods.length, ITEMS_PER_PAGE)}
                  >
                    下一页
                  </button>
                </div>
              )}
            </>
          )}
        </section>
      </main>

      {/* 全局错误提示 */}
      {error && (
        <div className="error-overlay">
          <div className="error-message">
            <p>{error}</p>
            <button onClick={() => setError(null)}>关闭</button>
          </div>
        </div>
      )}
    </div>
  );
};

export default App;