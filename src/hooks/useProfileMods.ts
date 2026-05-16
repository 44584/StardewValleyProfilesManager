import { useState, useCallback } from 'react';
import type { ProfileMod, ModInfo } from '../types';
import {
  getModsForProfile,
  addModToProfile,
  removeModFromProfile,
  toggleModEnabled,
} from '../utils/invoke';

export interface ProfileModWithInfo extends ProfileMod {
  modInfo: ModInfo;
}

export function useProfileMods(
  profileId: number | null,
  allMods: ModInfo[]
) {
  const [profileMods, setProfileMods] = useState<ProfileModWithInfo[]>([]);
  const [loading, setLoading] = useState(false);

  const fetchProfileMods = useCallback(async () => {
    if (profileId === null) {
      setProfileMods([]);
      return;
    }
    setLoading(true);
    try {
      console.log('[useProfileMods] fetchProfileMods:start', {
        profileId,
        allModsCount: allMods.length,
      });
      const pms = await getModsForProfile(profileId);
      console.log('[useProfileMods] fetchProfileMods:rawResponse', {
        profileId,
        profileModsCount: pms.length,
        profileMods: pms,
      });
      const enriched = pms
        .map((pm) => {
          const modInfo = allMods.find((m) => m.id === pm.modId);
          return modInfo ? { ...pm, modInfo } : null;
        })
        .filter(Boolean) as ProfileModWithInfo[];
      console.log('[useProfileMods] fetchProfileMods:enriched', {
        profileId,
        enrichedCount: enriched.length,
      });
      setProfileMods(enriched);
    } catch (err) {
      console.error('[useProfileMods] fetchProfileMods:error', { profileId, err });
      throw err;
    } finally {
      setLoading(false);
    }
  }, [profileId, allMods]);

  const addMod = useCallback(
    async (uniqueId: string) => {
      if (profileId === null) return;
      console.log('[useProfileMods] addMod:start', { profileId, uniqueId });
      try {
        await addModToProfile(profileId, uniqueId);
        console.log('[useProfileMods] addMod:addModToProfile:success', { profileId, uniqueId });
        await fetchProfileMods();
        console.log('[useProfileMods] addMod:fetchProfileMods:success', { profileId, uniqueId });
      } catch (err) {
        console.error('[useProfileMods] addMod:error', { profileId, uniqueId, err });
        throw err;
      }
    },
    [profileId, fetchProfileMods]
  );

  const removeMod = useCallback(
    async (uniqueId: string) => {
      if (profileId === null) return;
      console.log('[useProfileMods] removeMod:start', { profileId, uniqueId });
      try {
        await removeModFromProfile(profileId, uniqueId);
        console.log('[useProfileMods] removeMod:removeModFromProfile:success', { profileId, uniqueId });
        await fetchProfileMods();
        console.log('[useProfileMods] removeMod:fetchProfileMods:success', { profileId, uniqueId });
      } catch (err) {
        console.error('[useProfileMods] removeMod:error', { profileId, uniqueId, err });
        throw err;
      }
    },
    [profileId, fetchProfileMods]
  );

  const toggleEnabled = useCallback(
    async (uniqueId: string, isEnabled: boolean) => {
      if (profileId === null) return;
      console.log('[useProfileMods] toggleEnabled:start', { profileId, uniqueId, isEnabled });
      try {
        await toggleModEnabled(profileId, uniqueId, isEnabled);
        console.log('[useProfileMods] toggleEnabled:toggleModEnabled:success', {
          profileId,
          uniqueId,
          isEnabled,
        });
        await fetchProfileMods();
        console.log('[useProfileMods] toggleEnabled:fetchProfileMods:success', {
          profileId,
          uniqueId,
          isEnabled,
        });
      } catch (err) {
        console.error('[useProfileMods] toggleEnabled:error', { profileId, uniqueId, isEnabled, err });
        throw err;
      }
    },
    [profileId, fetchProfileMods]
  );

  return {
    profileMods,
    loading,
    fetchProfileMods,
    addMod,
    removeMod,
    toggleEnabled,
  };
}
