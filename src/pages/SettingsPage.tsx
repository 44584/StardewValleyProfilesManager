import { useState, useEffect } from 'react';
import { open } from '@tauri-apps/plugin-dialog';
import type { AppConfig } from '../types';
import LoadingButton from '../components/LoadingButton';
import { validateSmapiInstallation } from '../utils/invoke';

interface SettingsPageProps {
  config: AppConfig;
  onSave: (config: AppConfig) => Promise<void>;
}

export default function SettingsPage({ config, onSave }: SettingsPageProps) {
  const [smapiPath, setSmapiPath] = useState(config.smapiPath || '');
  const [modsDirectory, setModsDirectory] = useState(config.modsDirectory || '');
  const [validating, setValidating] = useState(false);
  const [smapiValid, setSmapiValid] = useState<boolean | null>(null);
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    setSmapiPath(config.smapiPath || '');
    setModsDirectory(config.modsDirectory || '');
  }, [config]);

  const handleSelectSmapi = async () => {
    const selected = await open({
      title: '选择 SMAPI 可执行文件',
      filters: [{ name: '可执行文件', extensions: ['exe'] }],
      multiple: false,
    });
    if (selected && typeof selected === 'string') {
      console.log("selected smapi: ", selected);
      setSmapiPath(selected);
      setSmapiValid(null);
    }
  };

  const handleSelectModsDir = async () => {
    const selected = await open({
      title: '选择 Mods 目录',
      directory: true,
      multiple: false,
    });
    if (selected && typeof selected === 'string') {
      setModsDirectory(selected);
    }
  };

  const handleValidateSmapi = async () => {
    if (!smapiPath) return;
    setValidating(true);
    try {
      const valid = await validateSmapiInstallation(smapiPath);
      setSmapiValid(valid);
    } catch {
      setSmapiValid(false);
    } finally {
      setValidating(false);
    }
  };

  const handleSave = async () => {
    setSaving(true);
    try {
      await onSave({ smapiPath: smapiPath || undefined, modsDirectory: modsDirectory || undefined });
    } finally {
      setSaving(false);
    }
  };

  const hasChanges = smapiPath !== (config.smapiPath || '') || modsDirectory !== (config.modsDirectory || '');

  return (
    <div className="page settings-page">
      <h1 className="page-title">🔧 设置</h1>

      <div className="settings-card">
        <h2 className="settings-section-title">游戏配置</h2>

        <div className="form-group">
          <label>
            SMAPI 可执行文件路径 <span className="required">*</span>
          </label>
          <div className="path-input-row">
            <input
              type="text"
              className="form-input"
              value={smapiPath}
              onChange={(e) => { setSmapiPath(e.target.value); setSmapiValid(null); }}
              placeholder="如：C:\\...\\StardewModdingAPI.exe"
            />
            <button className="btn btn-secondary" onClick={handleSelectSmapi}>
              📂 浏览...
            </button>
            <LoadingButton loading={validating} onClick={handleValidateSmapi} disabled={!smapiPath}>
              验证
            </LoadingButton>
          </div>
          {smapiValid === true && <div className="form-hint success">✅ SMAPI 路径有效</div>}
          {smapiValid === false && <div className="form-hint error">❌ SMAPI 路径无效</div>}
        </div>

        <div className="form-group">
          <label>
            模组目录路径 <span className="required">*</span>
          </label>
          <div className="path-input-row">
            <input
              type="text"
              className="form-input"
              value={modsDirectory}
              onChange={(e) => setModsDirectory(e.target.value)}
              placeholder="如：C:\\...\\Stardew Valley\\Mods"
            />
            <button className="btn btn-secondary" onClick={handleSelectModsDir}>
              📂 浏览...
            </button>
          </div>
        </div>

        {hasChanges && (
          <div className="settings-actions">
            <LoadingButton loading={saving} onClick={handleSave}>
              保存配置
            </LoadingButton>
          </div>
        )}
      </div>

      {!config.smapiPath && !config.modsDirectory && (
        <div className="settings-hint">
          ℹ️ 请配置 SMAPI 路径和模组目录以开始使用本应用
        </div>
      )}
    </div>
  );
}
