import { useEffect, useState } from 'react';
import type { Profile, ModInfo } from '../types';
import LoadingButton from '../components/LoadingButton';
import EmptyState from '../components/EmptyState';
import { launchGameWithProfile } from '../utils/invoke';

interface HomePageProps {
  profiles: Profile[];
  mods: ModInfo[];
  config: { smapiPath?: string; modsDirectory?: string };
  onNavigate: (page: string, profileId?: number) => void;
  onScanMods: (directory: string) => Promise<void>;
  scanning: boolean;
}

export default function HomePage({ profiles, mods, config, onNavigate, onScanMods, scanning }: HomePageProps) {
  const [launchingId, setLaunchingId] = useState<number | null>(null);
  const [recentProfiles, setRecentProfiles] = useState<Profile[]>([]);

  useEffect(() => {
    setRecentProfiles(profiles.slice(0, 3));
  }, [profiles]);

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

  const hasConfig = !!(config.smapiPath && config.modsDirectory);

  return (
    <div className="page home-page">
      <h1 className="page-title">🏡 欢迎回来</h1>

      <div className="home-actions">
        <button className="home-action-card" onClick={() => recentProfiles[0] && handleLaunch(recentProfiles[0].id!)}>
          <span className="home-action-icon">🎮</span>
          <span className="home-action-label">启动游戏</span>
          <span className="home-action-sub">{recentProfiles[0] ? `使用「${recentProfiles[0].name}」` : '选择配置方案后启动'}</span>
        </button>
        <button
          className="home-action-card"
          onClick={() => config.modsDirectory ? onScanMods(config.modsDirectory) : onNavigate('settings')}
          disabled={scanning}
        >
          <span className="home-action-icon">📦</span>
          <span className="home-action-label">{scanning ? '扫描中...' : '扫描模组'}</span>
          <span className="home-action-sub">{config.modsDirectory || '请先配置Mods目录'}</span>
        </button>
        <button className="home-action-card" onClick={() => onNavigate('profiles')}>
          <span className="home-action-icon">➕</span>
          <span className="home-action-label">新建配置</span>
          <span className="home-action-sub">创建一个新的模组配置</span>
        </button>
      </div>

      <div className="home-stats">
        <div className="stat-card">
          <div className="stat-value">{mods.length}</div>
          <div className="stat-label">已扫描模组</div>
        </div>
        <div className="stat-card">
          <div className="stat-value">{profiles.length}</div>
          <div className="stat-label">配置方案</div>
        </div>
        <div className="stat-card">
          <div className="stat-value">{config.smapiPath ? '✅' : '❌'}</div>
          <div className="stat-label">SMAPI 配置</div>
        </div>
      </div>

      {!hasConfig && (
        <div className="home-alert">
          <span>⚠️ SMAPI 路径或模组目录未配置，</span>
          <button className="link-btn" onClick={() => onNavigate('settings')}>点击前往设置</button>
        </div>
      )}

      <div className="home-section">
        <h2 className="home-section-title">最近使用的配置方案</h2>
        {recentProfiles.length === 0 ? (
          <EmptyState
            icon="🎮"
            title="还没有配置方案"
            description="创建你的第一个配置方案，开始管理模组吧！"
            action={
              <button className="btn btn-primary" onClick={() => onNavigate('profiles')}>
                创建配置方案
              </button>
            }
          />
        ) : (
          <div className="home-profiles">
            {recentProfiles.map((profile) => (
              <div key={profile.id} className="home-profile-item">
                <div className="home-profile-info">
                  <span className="home-profile-name">{profile.name}</span>
                  {profile.description && <span className="home-profile-desc">{profile.description}</span>}
                </div>
                <div className="home-profile-actions">
                  <LoadingButton
                    variant="success"
                    onClick={() => handleLaunch(profile.id!)}
                    loading={launchingId === profile.id}
                    className="btn-sm"
                  >
                    启动
                  </LoadingButton>
                  <button className="btn btn-secondary btn-sm" onClick={() => onNavigate('profile-detail', profile.id)}>
                    管理
                  </button>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
