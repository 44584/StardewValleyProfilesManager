import { useState } from 'react';
import LoadingButton from '../components/LoadingButton';

interface CreateProfileModalProps {
  onClose: () => void;
  onCreate: (name: string, description?: string) => Promise<void>;
}

export default function CreateProfileModal({ onClose, onCreate }: CreateProfileModalProps) {
  const [name, setName] = useState('');
  const [description, setDescription] = useState('');
  const [creating, setCreating] = useState(false);
  const [error, setError] = useState('');

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    const trimmed = name.trim();
    if (!trimmed) {
      setError('配置名称不能为空');
      return;
    }
    setError('');
    setCreating(true);
    try {
      await onCreate(trimmed, description.trim() || undefined);
      onClose();
    } catch (err: any) {
      setError(err?.message || '创建失败');
    } finally {
      setCreating(false);
    }
  };

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="modal" onClick={(e) => e.stopPropagation()}>
        <div className="modal-header">
          <h3>新建配置方案</h3>
          <button className="modal-close" onClick={onClose}>
            ✕
          </button>
        </div>
        <form onSubmit={handleSubmit}>
          <div className="modal-body">
            {error && <div className="form-error">{error}</div>}
            <div className="form-group">
              <label>
                配置名称 <span className="required">*</span>
              </label>
              <input
                type="text"
                className="form-input"
                value={name}
                onChange={(e) => setName(e.target.value)}
                placeholder="如：休闲种田"
                autoFocus
              />
            </div>
            <div className="form-group">
              <label>描述（可选）</label>
              <textarea
                className="form-textarea"
                value={description}
                onChange={(e) => setDescription(e.target.value)}
                placeholder="描述这个配置的用途..."
                rows={3}
              />
            </div>
          </div>
          <div className="modal-footer">
            <button type="button" className="btn btn-secondary" onClick={onClose}>
              取消
            </button>
            <LoadingButton type="submit" loading={creating}>
              创建
            </LoadingButton>
          </div>
        </form>
      </div>
    </div>
  );
}
