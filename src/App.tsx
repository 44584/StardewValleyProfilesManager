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
import ModDetailPage from './pages/ModDetailPage';
import SettingsPage from './pages/SettingsPage';

function App() {
  const [currentPage, setCurrentPage] = useState<Page>('home');
  const [selectedProfileId, setSelectedProfileId] = useState<number | null>(null);
  const [selectedModUniqueId, setSelectedModUniqueId] = useState<string | null>(null);
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
    (page: string, id?: number | string) => {
      if (typeof id === 'number') {
        setSelectedProfileId(id);
        setCurrentPage('profile-detail');
      } else if (typeof id === 'string') {
        setSelectedModUniqueId(id);
        setCurrentPage('mod-detail');
      } else {
        setCurrentPage(page as Page);
      }
    },
    []
  );

  const handleBack = useCallback(() => {
    if (currentPage === 'profile-detail') {
      setSelectedProfileId(null);
      setCurrentPage('profiles');
    } else if (currentPage === 'mod-detail') {
      setSelectedModUniqueId(null);
      setCurrentPage('mods');
    }
  }, [currentPage]);

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

  // 渲染当前页面
  const renderCurrentPage = () => {
    switch (currentPage) {
      case 'home':
        return (
          <HomePage
            profiles={profiles}
            mods={mods}
            config={config}
            onNavigate={handleNavigate}
            onScanMods={handleScan}
            scanning={scanning}
          />
        );
      case 'mods':
        return (
          <ModLibraryPage
            mods={mods}
            profiles={profiles}
            modsDirectory={config?.modsDirectory}
            onScanMods={handleScan}
            scanning={scanning}
            onNavigate={handleNavigate}
          />
        );
      case 'profiles':
        return (
          <ProfilesPage
            profiles={profiles}
            allMods={mods}
            getModsForProfile={getModsForProfile}
            countsRefreshVersion={countsRefreshVersion}
            config={{ smapiPath: config?.smapiPath }}
            onNavigate={handleNavigate}
            onCreateProfile={handleCreateProfile}
            onDeleteProfile={handleDeleteProfile}
          />
        );
      case 'profile-detail':
        return selectedProfileId !== null ? (
          <ProfileDetailPage
            profile={profiles.find(p => p.id === selectedProfileId)!}
            profileMods={profileMods}
            allMods={mods}
            config={{ smapiPath: config?.smapiPath }}
            loading={profileModsLoading}
            onBack={handleBack}
            onAddMod={addMod}
            onRemoveMod={removeMod}
            onToggleMod={toggleEnabled}
            onProfileUpdated={() => {}}
          />
        ) : null;
      case 'mod-detail':
        return selectedModUniqueId !== null ? (
          <ModDetailPage
            mod={mods.find(m => m.uniqueId === selectedModUniqueId)!}
            profiles={profiles}
            onNavigate={handleNavigate}
          />
        ) : null;
      case 'settings':
        return (
          <SettingsPage
            config={config || { smapiPath: '', modsDirectory: '' }}
            onSave={updateConfig}
          />
        );
      default:
        return (
          <HomePage
            profiles={profiles}
            mods={mods}
            config={config}
            onNavigate={handleNavigate}
            onScanMods={handleScan}
            scanning={scanning}
          />
        );
    }
  };

  return (
    <div className="app">
      <Sidebar currentPage={currentPage} onNavigate={setCurrentPage} />
      <main className="main-content">
        {renderCurrentPage()}
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