export type PluginType = 'editor_extension' | 'feature_module' | 'theme' | 'language_pack' | 'ai_adapter' | 'import_export' | 'utility' | 'integration';

export type PluginCapability = 'editor' | 'project' | 'ai' | 'filesystem' | 'network' | 'ui' | 'storage';

export type PermissionRisk = 'low' | 'medium' | 'high';

export type PluginState = 'loaded' | 'activated' | 'deactivated' | 'error' | 'unloaded';

export interface PluginPermission {
  name: string;
  description: string;
  risk: PermissionRisk;
}

export interface PluginContribution {
  type: string;
  id: string;
  label: string;
  description?: string;
  icon?: string;
  enabled_by_default?: boolean;
  [key: string]: any;
}

export interface PluginScript {
  language: string;
  entry_point: string;
  dependencies?: Record<string, string>;
}

export interface PluginAuthor {
  name: string;
  email?: string;
  url?: string;
}

export interface PluginInfo {
  id: string;
  version: string;
  name: string;
  description: string;
  author: PluginAuthor;
  pluginType: PluginType;
  homepage?: string;
  repository?: string;
  license?: string;
  minAppVersion: string;
  icon?: string;
  keywords?: string[];
}

export interface PluginManifest {
  info: PluginInfo;
  permissions: PluginPermission[];
  capabilities: PluginCapability[];
  contributes: PluginContribution[];
  script?: PluginScript;
  settings?: any;
  dependencies: Record<string, string>;
}

export interface Plugin {
  manifest: PluginManifest;
  path: string;
  state: PluginState;
  error?: string;
  settings: Record<string, any>;
  installed_at: string;
  last_activated?: string;
}

export interface PluginCommand {
  plugin_id: string;
  command_id: string;
  title: string;
  description?: string;
  category?: string;
  icon?: string;
  keybinding?: string;
}

export interface PluginStorageItem {
  plugin_id: string;
  key: string;
  value: any;
  updated_at: string;
}

export interface PluginEvent {
  id: string;
  plugin_id: string;
  type: string;
  payload: any;
  timestamp: string;
}

export interface PluginError {
  plugin_id: string;
  error_type: string;
  message: string;
  timestamp: string;
  stack_trace?: string;
}

export interface PermissionStatus {
  name: string;
  granted: boolean;
  risk: PermissionRisk;
}

export interface PermissionDialogInfo {
  plugin_id: string;
  plugin_name: string;
  permission_name: string;
  permission_description: string;
  risk: PermissionRisk;
}

export interface ResourceUsageStats {
  memory_bytes: number;
  memory_limit_bytes: number;
  file_descriptors: number;
  file_descriptors_limit: number;
  network_connections: number;
  network_connections_limit: number;
  execution_duration_seconds: number;
  execution_timeout_seconds: number;
}
