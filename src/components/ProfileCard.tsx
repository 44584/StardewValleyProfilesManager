import type { Profile } from '../types';
import LoadingButton from './LoadingButton';

interface ProfileCardProps {
  profile: Profile;
  modCount: number;
  onManage: () => void;
  onLaunch: () => void;
  onDelete: () => void;
  launching?: boolean;
}

export default function ProfileCard({
  profile,
  modCount,
  onManage,
  onLaunch,
  onDelete,
  launching = false,
}: ProfileCardProps) {
  return (
    <div className="profile-card">
      <div className="profile-card-header">
        <span className="profile-card-icon">🎮</span>
        <h3 className="profile-card-name">{profile.name}</h3>
      </div>
      {profile.description && (
        <p className="profile-card-desc">{profile.description}</p>
      )}
      <div className="profile-card-stats">
        <span>{modCount} 个模组</span>
      </div>
      <div className="profile-card-actions">
        <LoadingButton variant="success" onClick={onLaunch} loading={launching} className="profile-launch-btn">
          🚀 启动游戏
        </LoadingButton>
        <button className="btn btn-secondary btn-sm" onClick={onManage}>
          ⚙️ 管理
        </button>
        <button className="btn btn-danger btn-sm" onClick={onDelete}>
          🗑️
        </button>
      </div>
    </div>
  );
}
