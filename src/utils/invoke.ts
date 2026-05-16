import { invoke } from '@tauri-apps/api/core';
import type { ModInfo, Profile, ProfileMod, AppConfig } from '../types';

// NOTE: Tauri automatically maps camelCase JS arg keys to snake_case Rust params.
// Keep frontend calls using camelCase (profileId, uniqueId, modsDirectory, smapiPath, isEnabled).

// Config commands
export const loadAppConfig = (): Promise<AppConfig> =>
  invoke('load_app_config');

export const saveAppConfig = (smapiPath?: string, modsDirectory?: string): Promise<void> =>
  invoke('save_app_config', { smapiPath, modsDirectory });

// Mod commands
export const getAllMods = (): Promise<ModInfo[]> =>
  invoke('get_all_mods');

export const scanAndRegisterMods = (modsDirectory: string): Promise<ModInfo[]> =>
  invoke('scan_and_register_mods', { modsDirectory });

// Profile commands
export const getAllProfiles = (): Promise<Profile[]> =>
  invoke('get_all_profiles');

export const getProfileById = (profileId: number): Promise<Profile | null> =>
  invoke('get_profile_by_id', { profileId });

export const createProfile = (name: string, description?: string): Promise<number> =>
  invoke('create_profile', { name, description });

export const updateProfile = (profileId: number, name: string, description?: string): Promise<void> =>
  invoke('update_profile', { profileId, name, description });

export const deleteProfile = (profileId: number): Promise<void> =>
  invoke('delete_profile', { profileId });

// Profile-Mod commands
export const getModsForProfile = (profileId: number): Promise<ProfileMod[]> =>
  invoke('get_mods_for_profile', { profileId });

export const addModToProfile = (profileId: number, uniqueId: string): Promise<void> =>
  invoke('add_mod_to_profile', { profileId, uniqueId });

export const removeModFromProfile = (profileId: number, uniqueId: string): Promise<void> =>
  invoke('remove_mod_from_profile', { profileId, uniqueId });

export const toggleModEnabled = (profileId: number, uniqueId: string, isEnabled: boolean): Promise<void> =>
  invoke('toggle_mod_enabled', { profileId, uniqueId, isEnabled });

export const isModInProfile = (profileId: number, uniqueId: string): Promise<boolean> =>
  invoke('is_mod_in_profile', { profileId, uniqueId });

// Game commands
export const launchGameWithProfile = (profileId: number, smapiPath: string): Promise<void> =>
  invoke('launch_game_with_profile', { profileId, smapiPath });

export const validateSmapiInstallation = (smapiPath: string): Promise<boolean> =>
  invoke('validate_smapi_installation', { smapiPath });
