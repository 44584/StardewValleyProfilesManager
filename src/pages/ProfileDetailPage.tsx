import { useState, useMemo, useEffect } from 'react';
import type { Profile, ModInfo } from '../types';
import type { ProfileModWithInfo } from '../hooks/useProfileMods';
import SearchInput from '../components/SearchInput';
import LoadingButton from '../components/LoadingButton';
import EmptyState from '../components/EmptyState';
import AddModsModal from '../modals/AddModsModal';
import { launchGameWithProfile, updateProfile } from '../utils/invoke';

interface ProfileDetailPageProps {
  profile: Profile;
  profileMods: ProfileModWithInfo[];
  allMods: ModInfo[];
  config: { smapiPath?: string };
  loading: boolean;
  onBack: () => void;
  onAddMod: (uniqueId: string) => Promise<void>;
  onRemoveMod: (uniqueId: string) => Promise<void>;
  onToggleMod: (uniqueId: string, enabled: boolean) => Promise<void>;
  onProfileUpdated: () => void;
}

export default function ProfileDetailPage({
  profile,
  profileMods,
  allMods,
  config,
  loading,
  onBack,
  onAddMod,
  onRemoveMod,
  onToggleMod,
  onProfileUpdated,
}: ProfileDetailPageProps) {
  const [search, setSearch] = useState('');
  const [showAddModal, setShowAddModal] = useState(false);

  const [launching, setLaunching] = useState(false);
  const [editing, setEditing] = useState(false);
  const [editName, setEditName] = useState(profile.name);
  const [editDesc, setEditDesc] = useState(profile.description || '');
  const [savingEdit, setSavingEdit] = useState(false);
  const [showAll, setShowAll] = useState(true);

  useEffect(() => {
    setEditName(profile.name);
    setEditDesc(profile.description || '');
  }, [profile]);

  const filtered = useMemo(() => {
    let list = profileMods;
    if (!showAll) {
      list = list.filter((pm) => pm.isEnabled);
    }
    if (!search.trim()) return list;
    const q = search.toLowerCase();
    return list.filter(
      (pm) =>
        pm.modInfo.name.toLowerCase().includes(q) ||
        pm.modInfo.author.toLowerCase().includes(q)
    );
  }, [profileMods, search, showAll]);

  const availableMods = useMemo(() => {
    const inProfile = new Set(profileMods.map((pm) => pm.modInfo.uniqueId));
    return allMods.filter((m) => !inProfile.has(m.uniqueId));
  }, [allMods, profileMods]);

  const enabledCount = profileMods.filter((pm) => pm.isEnabled).length;

  const handleLaunch = async () => {
    if (!config.smapiPath) {
      alert('请先配置 SMAPI 路径');
      return;
    }
    setLaunching(true);
    try {
      await launchGameWithProfile(profile.id!, config.smapiPath);
    } catch (err: any) {
      alert(err?.message || '启动游戏失败');
    } finally {
      setLaunching(false);
    }
  };

  const handleSaveEdit = async () => {
    const trimmed = editName.trim();
    if (!trimmed) return;
    setSavingEdit(true);
    try {
      await updateProfile(profile.id!, trimmed, editDesc.trim() || undefined);
      setEditing(false);
      onProfileUpdated();
    } catch (err: any) {
      alert(err?.message || '保存失败');
    } finally {
      setSavingEdit(false);
    }
  };

  const handleToggleMod = async (uniqueId: string, nextEnabled: boolean) => {
    console.log('[ProfileDetailPage] toggleMod:start', {
      profileId: profile.id,
      uniqueId,
      nextEnabled,
    });
    await onToggleMod(uniqueId, nextEnabled);
    console.log('[ProfileDetailPage] toggleMod:success', {
      profileId: profile.id,
      uniqueId,
      nextEnabled,
    });
  };

  const handleRemoveMod = async (uniqueId: string) => {
    console.log('[ProfileDetailPage] removeMod:start', { profileId: profile.id, uniqueId });
    await onRemoveMod(uniqueId);
    console.log('[ProfileDetailPage] removeMod:success', { profileId: profile.id, uniqueId });
  };

  const handleAddMods = async (uniqueIds: string[]) => {
    console.log('[ProfileDetailPage] addMods:start', {
      profileId: profile.id,
      uniqueIds,
    });
    for (const uid of uniqueIds) {
      console.log('[ProfileDetailPage] addMods:item:start', { profileId: profile.id, uniqueId: uid });
      await onAddMod(uid);
      console.log('[ProfileDetailPage] addMods:item:success', { profileId: profile.id, uniqueId: uid });
    }
    console.log('[ProfileDetailPage] addMods:success', {
      profileId: profile.id,
      count: uniqueIds.length,
    });
  };

  return (
    <div className="page profile-detail-page">
      <div className="page-header-with-back">
        <button className="btn btn-secondary" onClick={onBack}>
          ← 返回配置列表
        </button>
      </div>

      <div className="profile-detail-header">
        {editing ? (
          <div className="profile-detail-edit">
            <input
              className="form-input"
              value={editName}
              onChange={(e) => setEditName(e.target.value)}
              autoFocus
            />
            <textarea
              className="form-textarea"
              value={editDesc}
              onChange={(e) => setEditDesc(e.target.value)}
              rows={2}
              placeholder="描述（可选）"
            />
            <div className="profile-detail-edit-actions">
              <button className="btn btn-secondary btn-sm" onClick={() => setEditing(false)}>
                取消
              </button>
              <LoadingButton onClick={handleSaveEdit} loading={savingEdit} className="btn-sm">
                保存
              </LoadingButton>
            </div>
          </div>
        ) : (
          <>
            <div className="profile-detail-title-row">
              <h1 className="page-title">{profile.name}</h1>
              <div className="profile-detail-actions">
                <button className="btn btn-secondary btn-sm" onClick={() => setEditing(true)} title="编辑">
                  ✏️
                </button>
              </div>
            </div>
            {profile.description && <p className="profile-detail-desc">{profile.description}</p>}
          </>
        )}
      </div>

      <div className="profile-detail-toolbar">
        <LoadingButton variant="success" onClick={handleLaunch} loading={launching}>
          🚀 启动游戏
        </LoadingButton>
        <button className="btn btn-primary" onClick={() => setShowAddModal(true)}>
          ➕ 添加模组
        </button>
        <SearchInput value={search} onChange={setSearch} placeholder="搜索模组..." />
        <label className="toggle-label-inline">
          <input type="checkbox" checked={showAll} onChange={(e) => setShowAll(e.target.checked)} />
          显示全部
        </label>
      </div>

      <div className="profile-detail-count">
        共 {profileMods.length} 个模组 · {enabledCount} 个已启用
      </div>

      {loading ? (
        <div className="loading-center">加载中...</div>
      ) : profileMods.length === 0 ? (
        <EmptyState
          icon="📦"
          title="该配置方案还没有模组"
          description='点击"添加模组"按钮，从模组库中选择要添加的模组'
          action={
            <button className="btn btn-primary" onClick={() => setShowAddModal(true)}>
              ➕ 添加模组
            </button>
          }
        />
      ) : (
        <div className="mod-table-wrapper">
          <table className="mod-table">
            <thead>
              <tr>
                <th style={{ width: '60px' }}>启用</th>
                <th>模组名称</th>
                <th>作者</th>
                <th>版本</th>
                <th>操作</th>
              </tr>
            </thead>
            <tbody>
              {filtered.map((pm) => (
                <tr key={pm.id} className={!pm.isEnabled ? 'mod-row-disabled' : ''}>
                  <td>
                    <label className="switch">
                      <input
                        type="checkbox"
                        checked={pm.isEnabled}
                        onChange={() => handleToggleMod(pm.modInfo.uniqueId, !pm.isEnabled)}
                      />
                      <span className="slider" />
                    </label>
                  </td>
                  <td>
                    <div className="mod-name">{pm.modInfo.name}</div>
                    {pm.modInfo.description && <div className="mod-desc">{pm.modInfo.description}</div>}
                  </td>
                  <td>{pm.modInfo.author}</td>
                  <td>v{pm.modInfo.version}</td>
                  <td>
                    <button
                      className="btn btn-danger btn-sm"
                      onClick={() => handleRemoveMod(pm.modInfo.uniqueId)}
                    >
                      移除
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      {showAddModal && (
        <AddModsModal
          profileName={profile.name}
          availableMods={availableMods}
          onClose={() => setShowAddModal(false)}
          onAdd={handleAddMods}
        />
      )}


    </div>
  );
}