export interface ModInfo {
  id?: number;
  uniqueId: string;
  name: string;
  author: string;
  version: string;
  description?: string;
  entryDll?: string;
  contentPackFor?: string;
  minimumApiVersion?: string;
  dependenciesJson?: string;
  updateKeysJson?: string;
  modPath: string;
  manifestHash: string;
}

export interface Profile {
  id?: number;
  name: string;
  description?: string;
  profile_path: string;
}

export interface ProfileMod {
  id?: number;
  profileId: number;
  modId: number;
  isEnabled: boolean;
  linkPath?: string;
}

export interface AppConfig {
  smapiPath?: string;
  modsDirectory?: string;
}

export type Page = 'home' | 'mods' | 'profiles' | 'profile-detail' | 'settings' | 'mod-detail';
