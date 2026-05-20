import { useState } from 'react';
import type { ModInfo, Profile } from '../types';
import LoadingButton from '../components/LoadingButton';
import { addModToProfile } from '../utils/invoke';

interface ModDetailPageProps {
  mod: ModInfo;
  profiles: Profile[];
  onNavigate: (page: string) => void;
}

export default function ModDetailPage({ mod, profiles, onNavigate }: ModDetailPageProps) {
  const [addingToProfile, setAddingToProfile] = useState<number | null>(null);

  const handleAddToProfile = async (profileId: number) => {
    setAddingToProfile(profileId);
    try {
      await addModToProfile(profileId, mod.uniqueId);
      // 添加成功后可以显示提示或保持在详情页
    } catch (err: any) {
      alert(err?.message || '添加失败');
    } finally {
      setAddingToProfile(null);
    }
  };

  const getModType = () => {
    if (mod.entryDll) return 'SMAPI模组';
    if (mod.contentPackFor) return '内容包';
    return '模组';
  };

  const parseJsonField = (jsonString?: string) => {
    if (!jsonString) return null;
    try {
      return JSON.parse(jsonString);
    } catch {
      return jsonString; // 如果解析失败，直接返回原始字符串
    }
  };

  const dependencies = parseJsonField(mod.dependenciesJson);
  const updateKeys = parseJsonField(mod.updateKeysJson);

  return (
    <div className="page mod-detail-page">
      <div className="page-header">
        <h1 className="page-title">📦 模组详情</h1>
        <button className="btn btn-secondary" onClick={() => onNavigate('mods')}>
          ← 返回模组库
        </button>
      </div>

      <div className="mod-detail-content">
        <div className="mod-detail-header">
          <h2>{mod.name}</h2>
          <div className="mod-detail-meta">
            <span className={`tag ${mod.entryDll ? 'tag-smapi' : mod.contentPackFor ? 'tag-content' : 'tag-mod'}`}>
              {getModType()}
            </span>
            <span>v{mod.version}</span>
            <span>by {mod.author}</span>
          </div>
        </div>

        {mod.description && (
          <div className="mod-detail-section">
            <h3>描述</h3>
            <p>{mod.description}</p>
          </div>
        )}

        <div className="mod-detail-section">
          <h3>基本信息</h3>
          <div className="mod-detail-grid">
            <div className="mod-detail-row">
              <span className="mod-detail-label">唯一标识符:</span>
              <span className="mod-detail-value">{mod.uniqueId}</span>
            </div>
            {mod.minimumApiVersion && (
              <div className="mod-detail-row">
                <span className="mod-detail-label">最低 SMAPI 版本:</span>
                <span className="mod-detail-value">{mod.minimumApiVersion}</span>
              </div>
            )}
            <div className="mod-detail-row">
              <span className="mod-detail-label">文件路径:</span>
              <span className="mod-detail-value mod-path">{mod.modPath}</span>
            </div>
            <div className="mod-detail-row">
              <span className="mod-detail-label">Manifest 哈希:</span>
              <span className="mod-detail-value mod-hash">{mod.manifestHash}</span>
            </div>
          </div>
        </div>

        {dependencies && (
          <div className="mod-detail-section">
            <h3>依赖项</h3>
            <div className="mod-dependencies">
              {Array.isArray(dependencies) ? (
                dependencies.map((dep: any, index: number) => (
                  <div key={index} className="mod-dependency-item">
                    <div className="dependency-name">{dep.uniqueId || dep.UniqueID}</div>
                    {dep.minimumVersion && (
                      <div className="dependency-version">最低版本: {dep.minimumVersion}</div>
                    )}
                    {dep.isRequired !== undefined && (
                      <div className="dependency-required">
                        必需: {dep.isRequired ? '是' : '否'}
                      </div>
                    )}
                  </div>
                ))
              ) : (
                <pre className="mod-json">{JSON.stringify(dependencies, null, 2)}</pre>
              )}
            </div>
          </div>
        )}

        {updateKeys && (
          <div className="mod-detail-section">
            <h3>更新键</h3>
            <div className="mod-update-keys">
              {Array.isArray(updateKeys) ? (
                updateKeys.map((key: string, index: number) => (
                  <span key={index} className="tag tag-update-key">{key}</span>
                ))
              ) : (
                <pre className="mod-json">{JSON.stringify(updateKeys, null, 2)}</pre>
              )}
            </div>
          </div>
        )}

        {mod.entryDll && (
          <div className="mod-detail-section">
            <h3>SMAPI 模组信息</h3>
            <div className="mod-detail-row">
              <span className="mod-detail-label">入口 DLL:</span>
              <span className="mod-detail-value">{mod.entryDll}</span>
            </div>
          </div>
        )}

        {mod.contentPackFor && (
          <div className="mod-detail-section">
            <h3>内容包信息</h3>
            <div className="mod-detail-row">
              <span className="mod-detail-label">适用模组:</span>
              <span className="mod-detail-value">{mod.contentPackFor}</span>
            </div>
          </div>
        )}

        <div className="mod-detail-section">
          <h3>添加到配置方案</h3>
          {profiles.length === 0 ? (
            <p className="text-muted">暂无配置方案，请先创建一个配置方案。</p>
          ) : (
            <div className="mod-add-profiles">
              {profiles.map((profile) => (
                <LoadingButton
                  key={profile.id}
                  loading={addingToProfile === profile.id}
                  onClick={() => handleAddToProfile(profile.id!)}
                  variant="primary"
                  className="btn-add-profile"
                >
                  添加到 "{profile.name}"
                </LoadingButton>
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}