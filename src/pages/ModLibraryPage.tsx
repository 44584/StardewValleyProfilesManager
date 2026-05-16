import { useMemo, useState } from 'react';
import type { ModInfo, Profile } from '../types';
import SearchInput from '../components/SearchInput';
import LoadingButton from '../components/LoadingButton';
import EmptyState from '../components/EmptyState';
import { addModToProfile } from '../utils/invoke';

interface ModLibraryPageProps {
  mods: ModInfo[];
  profiles: Profile[];
  modsDirectory?: string;
  onScanMods: (directory: string) => Promise<void>;
  scanning: boolean;
  onNavigate: (page: string) => void;
}

export default function ModLibraryPage({ mods, profiles, modsDirectory, onScanMods, scanning, onNavigate }: ModLibraryPageProps) {
  const [search, setSearch] = useState('');
  const [addingToProfile, setAddingToProfile] = useState<number | null>(null);

  const filtered = useMemo(() => {
    if (!search.trim()) return mods;
    const q = search.toLowerCase();
    return mods.filter(
      (m) =>
        m.name.toLowerCase().includes(q) ||
        m.author.toLowerCase().includes(q) ||
        (m.description && m.description.toLowerCase().includes(q))
    );
  }, [mods, search]);

  const handleAddToProfile = async (profileId: number, uniqueId: string) => {
    setAddingToProfile(profileId);
    try {
      await addModToProfile(profileId, uniqueId);
    } catch (err: any) {
      alert(err?.message || '添加失败');
    } finally {
      setAddingToProfile(null);
    }
  };

  const getModType = (mod: ModInfo) => {
    if (mod.entryDll) return 'SMAPI模组';
    if (mod.contentPackFor) return '内容包';
    return '模组';
  };

  return (
    <div className="page mod-library-page">
      <div className="page-header">
        <h1 className="page-title">📦 模组库</h1>
        <SearchInput value={search} onChange={setSearch} placeholder="搜索模组名称、作者..." />
      </div>

      <div className="mod-library-toolbar">
        <div className="mod-library-path">
          <span>路径: {modsDirectory || '未配置'}</span>
          {modsDirectory && (
            <LoadingButton loading={scanning} onClick={() => onScanMods(modsDirectory)} variant="secondary">
              {scanning ? '扫描中...' : '🔄 重新扫描'}
            </LoadingButton>
          )}
          {!modsDirectory && (
            <button className="btn btn-secondary" onClick={() => onNavigate('settings')}>
              前往设置
            </button>
          )}
        </div>
        <div className="mod-library-count">共 {filtered.length} 个模组</div>
      </div>

      {filtered.length === 0 ? (
        <EmptyState
          icon="📦"
          title={search ? '未找到匹配的模组' : mods.length === 0 ? '模组库为空' : '暂无模组'}
          description={
            mods.length === 0
              ? '点击"重新扫描"按钮扫描Mods目录'
              : '尝试更换搜索关键词'
          }
          action={
            modsDirectory && mods.length === 0 ? (
              <LoadingButton loading={scanning} onClick={() => onScanMods(modsDirectory)}>
                扫描模组
              </LoadingButton>
            ) : undefined
          }
        />
      ) : (
        <div className="mod-table-wrapper">
          <table className="mod-table">
            <thead>
              <tr>
                <th>模组名称</th>
                <th>作者</th>
                <th>版本</th>
                <th>类型</th>
                <th>操作</th>
              </tr>
            </thead>
            <tbody>
              {filtered.map((mod) => (
                <tr key={mod.uniqueId}>
                  <td>
                    <div className="mod-name">{mod.name}</div>
                    {mod.description && <div className="mod-desc">{mod.description}</div>}
                  </td>
                  <td>{mod.author}</td>
                  <td>v{mod.version}</td>
                  <td>
                    <span className={`tag ${mod.entryDll ? 'tag-smapi' : mod.contentPackFor ? 'tag-content' : 'tag-mod'}`}>
                      {getModType(mod)}
                    </span>
                  </td>
                  <td>
                    <div className="mod-actions-dropdown">
                      {profiles.length === 0 ? (
                        <span className="text-muted">无配置方案</span>
                      ) : (
                        <select
                          className="form-select"
                          value=""
                          onChange={(e) => {
                            const pid = Number(e.target.value);
                            if (pid) {
                              handleAddToProfile(pid, mod.uniqueId);
                              e.target.value = '';
                            }
                          }}
                          disabled={addingToProfile !== null}
                        >
                          <option value="">添加到配置...</option>
                          {profiles.map((p) => (
                            <option key={p.id} value={p.id}>
                              {p.name}
                            </option>
                          ))}
                        </select>
                      )}
                    </div>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
}
