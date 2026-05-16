import { useState, useCallback } from 'react';
import type { ModInfo } from '../types';
import { getAllMods, scanAndRegisterMods } from '../utils/invoke';

export function useMods() {
  const [mods, setMods] = useState<ModInfo[]>([]);
  const [loading, setLoading] = useState(false);
  const [scanning, setScanning] = useState(false);

  const fetchMods = useCallback(async () => {
    setLoading(true);
    try {
      const result = await getAllMods();
      setMods(result);
    } catch (err) {
      console.error('Failed to fetch mods:', err);
      throw err;
    } finally {
      setLoading(false);
    }
  }, []);

  const scanMods = useCallback(async (directory: string) => {
    setScanning(true);
    try {
      const result = await scanAndRegisterMods(directory);
      setMods(result);
    } catch (err) {
      console.error('Failed to scan mods:', err);
      throw err;
    } finally {
      setScanning(false);
    }
  }, []);

  return { mods, loading, scanning, fetchMods, scanMods };
}
