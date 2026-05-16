import { useState, useEffect } from 'react';
import type { Profile, ModInfo, ProfileMod } from '../types';
import ProfileCard from '../components/ProfileCard';
import EmptyState from '../components/EmptyState';
import CreateProfileModal from '../modals/CreateProfileModal';
import ConfirmDialog from '../components/ConfirmDialog';
import { launchGameWithProfile } from '../utils/invoke';

interface ProfilesPageProps {
  profiles: Profile[];
  allMods: ModInfo[];
  getModsForProfile: (profileId: number) => Promise<ProfileMod[]>;
  countsRefreshVersion: number;
  config: { smapiPath?: string };
  onNavigate: (page: string, profileId?: number) => void;
  onCreateProfile: (name: string, description?: string) => Promise<void>;
  onDeleteProfile: (id: number) => Promise<void>;
}

export default function ProfilesPage({
  profiles,
  getModsForProfile,
  countsRefreshVersion,
  config,
  onNavigate,
  onCreateProfile,
  onDeleteProfile,
}: ProfilesPageProps) {
  const [showCreate, setShowCreate] = useState(false);
  const [deletingId, setDeletingId] = useState<number | null>(null);
  const [launchingId, setLaunchingId] = useState<number | null>(null);

  const handleLaunch = async (profileId: number) => {
    if (!config.smapiPath) {
      alert('请先配置 SMAPI 路径');
      onNavigate('settings');
      return;
    }
    setLaunchingId(profileId);
    try {
      await launchGameWithProfile(profileId, config.smapiPath);
    } catch (err: any) {
      alert(err?.message || '启动游戏失败');
    } finally {
      setLaunchingId(null);
    }
  };

  const handleDelete = async () => {
    if (deletingId === null) return;
    try {
      console.log('[ProfilesPage] deleteProfile:start', { profileId: deletingId });
      await onDeleteProfile(deletingId);
      console.log('[ProfilesPage] deleteProfile:success', { profileId: deletingId });
    } catch (err: any) {
      console.error('[ProfilesPage] deleteProfile:error', { profileId: deletingId, err });
      alert(err?.message || '删除失败');
    } finally {
      setDeletingId(null);
    }
  };

  const [counts, setCounts] = useState<Record<number, number>>({});

  // Fetch counts for visible profiles from backend whenever profiles list or refresh version changes
  useEffect(() => {
    let mounted = true;
    const load = async () => {
      const map: Record<number, number> = {};
      await Promise.all(
        profiles.map(async (p) => {
          if (!p.id) return;
          try {
            const mods = await getModsForProfile(p.id);
            map[p.id!] = mods.length;
          } catch (err) {
            console.error('[ProfilesPage] load count error', { profileId: p.id, err });
            map[p.id!] = 0;
          }
        })
      );
      if (mounted) setCounts(map);
    };
    load();
    return () => {
      mounted = false;
    };
  }, [profiles, getModsForProfile, countsRefreshVersion]);

  return (
    <div className="page profiles-page">
      <div className="page-header">
        <h1 className="page-title">⚙️ 配置方案</h1>
        <button className="btn btn-primary" onClick={() => setShowCreate(true)}>
          ➕ 新建配置
        </button>
      </div>

      {profiles.length === 0 ? (
        <EmptyState
          icon="⚙️"
          title="还没有配置方案"
          description="创建你的第一个配置方案，开始管理模组吧！"
          action={
            <button className="btn btn-primary" onClick={() => setShowCreate(true)}>
              ➕ 创建配置方案
            </button>
          }
        />
      ) : (
        <div className="profiles-grid">
          {profiles.map((profile) => (
            <ProfileCard
              key={profile.id}
              profile={profile}
              modCount={counts[profile.id as number] || 0}
              onManage={() => onNavigate('profile-detail', profile.id)}
              onLaunch={() => handleLaunch(profile.id!)}
              onDelete={() => setDeletingId(profile.id!)}
              launching={launchingId === profile.id}
            />
          ))}
        </div>
      )}

      {showCreate && <CreateProfileModal onClose={() => setShowCreate(false)} onCreate={onCreateProfile} />}

      {deletingId !== null && (
        <ConfirmDialog
          title="删除配置方案"
          message="确定要删除这个配置方案吗？此操作不可恢复，关联的模组配置也将被移除。"
          confirmText="删除"
          isDanger
          onConfirm={handleDelete}
          onCancel={() => setDeletingId(null)}
        />
      )}
    </div>
  );
}
