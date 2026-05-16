import { useState, useMemo } from 'react';
import type { ModInfo } from '../types';
import SearchInput from '../components/SearchInput';
import LoadingButton from '../components/LoadingButton';
import EmptyState from '../components/EmptyState';

interface AddModsModalProps {
  profileName: string;
  availableMods: ModInfo[];
  onClose: () => void;
  onAdd: (uniqueIds: string[]) => Promise<void>;
}

export default function AddModsModal({ profileName, availableMods, onClose, onAdd }: AddModsModalProps) {
  const [search, setSearch] = useState('');
  const [selected, setSelected] = useState<Set<string>>(new Set());
  const [adding, setAdding] = useState(false);

  const filtered = useMemo(() => {
    if (!search.trim()) return availableMods;
    const q = search.toLowerCase();
    return availableMods.filter(
      (m) =>
        m.name.toLowerCase().includes(q) ||
        m.author.toLowerCase().includes(q) ||
        (m.description && m.description.toLowerCase().includes(q))
    );
  }, [availableMods, search]);

  const toggleSelect = (uniqueId: string) => {
    const next = new Set(selected);
    if (next.has(uniqueId)) {
      next.delete(uniqueId);
    } else {
      next.add(uniqueId);
    }
    setSelected(next);
  };

  const handleAdd = async () => {
    if (selected.size === 0) return;
    setAdding(true);
    try {
      await onAdd(Array.from(selected));
      onClose();
    } catch (err) {
      console.error('Failed to add mods:', err);
    } finally {
      setAdding(false);
    }
  };

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="modal modal-lg" onClick={(e) => e.stopPropagation()}>
        <div className="modal-header">
          <h3>添加模组到「{profileName}」</h3>
          <button className="modal-close" onClick={onClose}>
            ✕
          </button>
        </div>
        <div className="modal-body">
          <div className="modal-toolbar">
            <SearchInput
              value={search}
              onChange={setSearch}
              placeholder="搜索模组名称、作者..."
            />
            <span className="modal-count">已选择 {selected.size} 个</span>
          </div>

          {filtered.length === 0 ? (
            <EmptyState
              icon="🔍"
              title={search ? '未找到匹配的模组' : '没有可添加的模组'}
              description={search ? '尝试更换搜索关键词' : '模组库中没有尚未加入该配置的模组'}
            />
          ) : (
            <div className="modal-list">
              {filtered.map((mod) => (
                <label
                  key={mod.uniqueId}
                  className={`modal-list-item ${selected.has(mod.uniqueId) ? 'selected' : ''}`}
                >
                  <input
                    type="checkbox"
                    checked={selected.has(mod.uniqueId)}
                    onChange={() => toggleSelect(mod.uniqueId)}
                  />
                  <div className="modal-list-content">
                    <div className="modal-list-title">{mod.name}</div>
                    <div className="modal-list-meta">
                      {mod.author} · v{mod.version}
                      {mod.entryDll ? ' · SMAPI模组' : mod.contentPackFor ? ' · 内容包' : ''}
                    </div>
                  </div>
                </label>
              ))}
            </div>
          )}
        </div>
        <div className="modal-footer">
          <button className="btn btn-secondary" onClick={onClose}>
            取消
          </button>
          <LoadingButton onClick={handleAdd} loading={adding} disabled={selected.size === 0}>
            添加选中模组
          </LoadingButton>
        </div>
      </div>
    </div>
  );
}
