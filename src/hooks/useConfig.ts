import { useState, useEffect, useCallback } from 'react';
import type { AppConfig } from '../types';
import { loadAppConfig, saveAppConfig } from '../utils/invoke';

export function useConfig() {
  const [config, setConfig] = useState<AppConfig>({});
  const [loading, setLoading] = useState(true);
  const [isConfigured, setIsConfigured] = useState(false);

  const fetchConfig = useCallback(async () => {
    setLoading(true);
    try {
      const raw = await loadAppConfig();
      // backend returns snake_case keys (smapi_path, mods_directory). Normalize to camelCase for frontend.
      const cfg = {
        smapiPath: (raw as any).smapiPath ?? (raw as any).smapi_path ?? undefined,
        modsDirectory: (raw as any).modsDirectory ?? (raw as any).mods_directory ?? undefined,
      };
      setConfig(cfg);
      setIsConfigured(!!(cfg.smapiPath && cfg.modsDirectory));
    } catch (err) {
      console.warn('Failed to load config:', err);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchConfig();
  }, [fetchConfig]);

  const updateConfig = useCallback(async (newConfig: AppConfig) => {
    try {
      await saveAppConfig(newConfig.smapiPath, newConfig.modsDirectory);
      setConfig(newConfig);
      setIsConfigured(!!(newConfig.smapiPath && newConfig.modsDirectory));
    } catch (err) {
      console.error('Failed to save config:', err);
      throw err;
    }
  }, []);

  return { config, loading, isConfigured, fetchConfig, updateConfig };
}
