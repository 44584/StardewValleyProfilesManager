import { useState, useCallback } from 'react';
import type { Profile } from '../types';
import { getAllProfiles, createProfile, updateProfile, deleteProfile } from '../utils/invoke';

export function useProfiles() {
  const [profiles, setProfiles] = useState<Profile[]>([]);
  const [loading, setLoading] = useState(false);

  const fetchProfiles = useCallback(async () => {
    setLoading(true);
    try {
      const result = await getAllProfiles();
      setProfiles(result);
    } catch (err) {
      console.error('Failed to fetch profiles:', err);
      throw err;
    } finally {
      setLoading(false);
    }
  }, []);

  const addProfile = useCallback(async (name: string, description?: string) => {
    const id = await createProfile(name, description);
    await fetchProfiles();
    return id;
  }, [fetchProfiles]);

  const editProfile = useCallback(async (profileId: number, name: string, description?: string) => {
    await updateProfile(profileId, name, description);
    await fetchProfiles();
  }, [fetchProfiles]);

  const removeProfile = useCallback(async (profileId: number) => {
    console.log('[useProfiles] removeProfile:start', { profileId });
    try {
      await deleteProfile(profileId);
      console.log('[useProfiles] removeProfile:deleteProfile:success', { profileId });
      await fetchProfiles();
      console.log('[useProfiles] removeProfile:fetchProfiles:success', { profileId });
    } catch (err) {
      console.error('[useProfiles] removeProfile:error', { profileId, err });
      throw err;
    }
  }, [fetchProfiles]);

  return { profiles, loading, fetchProfiles, addProfile, editProfile, removeProfile };
}
