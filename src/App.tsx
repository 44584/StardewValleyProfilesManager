import { useState, useEffect, useCallback } from 'react';
import './App.css';
import type { Page } from './types';
import { useConfig } from './hooks/useConfig';
import { useMods } from './hooks/useMods';
import { useProfiles } from './hooks/useProfiles';
import { useProfileMods } from './hooks/useProfileMods';
import { getModsForProfile } from './utils/invoke';
import Sidebar from './components/Sidebar';
import HomePage from './pages/HomePage';
import ModLibraryPage from './pages/ModLibraryPage';
import ProfilesPage from './pages/ProfilesPage';
import ProfileDetailPage from './pages/ProfileDetailPage';
import SettingsPage from './pages/SettingsPage';

function App() {
  const [currentPage, setCurrentPage] = useState<Page>('home');
  const [selectedProfileId, setSelectedProfileId] = useState<number | null>(null);
  const [globalError, setGlobalError] = useState<string | null>(null);

  const { config, isConfigured, fetchConfig, updateConfig } = useConfig();
  const { mods, scanning, fetchMods, scanMods } = useMods();
  const {
    profiles,
    fetchProfiles,
    addProfile,
    removeProfile,
  } = useProfiles();

  const {
    profileMods,
    loading: profileModsLoading,
    fetchProfileMods,
    addMod,
    removeMod,
    toggleEnabled,
  } = useProfileMods(selectedProfileId, mods);

  // 初始化加载
  useEffect(() => {
    fetchConfig();
    fetchProfiles();
    fetchMods();
  }, []);

  // 启动时优先检查配置：如果未配置立即跳转到设置页
  useEffect(() => {
    if (!isConfigured) {
      setCurrentPage('settings');
    }
  }, [isConfigured]);

  // 选中 Profile 时加载其模组
  useEffect(() => {
    if (selectedProfileId !== null) {
      fetchProfileMods();
    }
  }, [selectedProfileId, mods]);

  const handleNavigate = useCallback(
    (page: string, profileId?: number) => {
      if (profileId !== undefined) {
        setSelectedProfileId(profileId);
        setCurrentPage('profile-detail');
      } else {
        setCurrentPage(page as Page);
      }
    },
    []
  );

  const handleBack = useCallback(() => {
    setSelectedProfileId(null);
    setCurrentPage('profiles');
  }, []);

  const handleScan = useCallback(
    async (directory: string) => {
      try {
        await scanMods(directory);
      } catch (err: any) {
        setGlobalError(err?.message || '扫描模组失败');
      }
    },
    [scanMods]
  );

  const handleCreateProfile = useCallback(
    async (name: string, description?: string) => {
      await addProfile(name, description);
    },
    [addProfile]
  );

  const handleDeleteProfile = useCallback(
    async (id: number) => {
      await removeProfile(id);
    },
    [removeProfile]
  );

  // 用于通知 ProfilesPage 刷新计数的版本号（写操作成功后自增）
  const [countsRefreshVersion, setCountsRefreshVersion] = useState(0);
  const signalCountsRefresh = useCallback(() => setCountsRefreshVersion((v) => v + 1), []);

  // Wrap profile-mod mutation handlers so we trigger a counts refresh after successful DB change
  const wrappedAddMod = useCallback(
    async (uniqueId: string) => {
      await addMod(uniqueId);
      signalCountsRefresh();
    },
    [addMod, signalCountsRefresh]
  );

  const wrappedRemoveMod = useCallback(
    async (uniqueId: string) => {
      await removeMod(uniqueId);
      signalCountsRefresh();
    },
    [removeMod, signalCountsRefresh]
  );

  const wrappedToggleEnabled = useCallback(
    async (uniqueId: string, isEnabled: boolean) => {
      await toggleEnabled(uniqueId, isEnabled);
      signalCountsRefresh();
    },
    [toggleEnabled, signalCountsRefresh]
  );

  const selectedProfile = profiles.find((p) => p.id === selectedProfileId) || null;

  return (
    <div className="app">
      <Sidebar currentPage={currentPage} onNavigate={setCurrentPage} />
      <main className="main-content">
        {currentPage === 'home' && (
          <HomePage
            profiles={profiles}
            mods={mods}
            config={config}
            onNavigate={handleNavigate}
            onScanMods={handleScan}
            scanning={scanning}
          />
        )}
        {currentPage === 'mods' && (
          <ModLibraryPage
            mods={mods}
            profiles={profiles}
            modsDirectory={config.modsDirectory}
            onScanMods={handleScan}
            scanning={scanning}
            onNavigate={handleNavigate}
          />
        )}
        {currentPage === 'profiles' && (
          <ProfilesPage
            profiles={profiles}
            allMods={mods}
            // ProfilesPage will query backend for each profile's mod count
            getModsForProfile={getModsForProfile}
            countsRefreshVersion={countsRefreshVersion}
            config={config}
            onNavigate={handleNavigate}
            onCreateProfile={handleCreateProfile}
            onDeleteProfile={handleDeleteProfile}
          />
        )}
        {currentPage === 'profile-detail' && selectedProfile && (
          <ProfileDetailPage
            profile={selectedProfile}
            profileMods={profileMods}
            allMods={mods}
            config={config}
            loading={profileModsLoading}
            onBack={handleBack}
            onAddMod={wrappedAddMod}
            onRemoveMod={wrappedRemoveMod}
            onToggleMod={wrappedToggleEnabled}
            onDeleteProfile={handleDeleteProfile}
            onProfileUpdated={fetchProfiles}
          />
        )}
        {currentPage === 'settings' && (
          <SettingsPage config={config} onSave={updateConfig} />
        )}
      </main>

      {globalError && (
        <div className="toast-overlay" onClick={() => setGlobalError(null)}>
          <div className="toast toast-error">
            <span>{globalError}</span>
            <button onClick={() => setGlobalError(null)}>✕</button>
          </div>
        </div>
      )}
    </div>
  );
}

export default App;
