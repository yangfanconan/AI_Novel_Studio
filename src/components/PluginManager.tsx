import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
  Plus,
  Search,
  Settings,
  Trash2,
  Power,
  PowerOff,
  Download,
  Shield,
  Cpu,
  BookOpen,
} from "lucide-react";
import { Plugin, PluginCommand, PermissionStatus } from "../types/plugin";
import { PluginDevGuideDialog } from "./PluginDevGuideDialog";

interface PluginManagerProps {
  onClose?: () => void;
}

const getStateBadge = (state: string) => {
  const badges: Record<string, { text: string; color: string }> = {
    loaded: { text: "已加载", color: "bg-blue-100 text-blue-800" },
    activated: { text: "已激活", color: "bg-green-100 text-green-800" },
    deactivated: { text: "已停用", color: "bg-gray-100 text-gray-800" },
    error: { text: "错误", color: "bg-red-100 text-red-800" },
    unloaded: { text: "未加载", color: "bg-gray-100 text-gray-800" },
  };
  const badge = badges[state] || { text: state, color: "bg-gray-100 text-gray-800" };
  return <span className={`px-2 py-1 text-xs rounded ${badge.color}`}>{badge.text}</span>;
};

const getPluginTypeLabel = (type: string) => {
  const labels: Record<string, string> = {
    editor_extension: "编辑器扩展",
    feature_module: "功能模块",
    theme: "主题",
    language_pack: "语言包",
    ai_adapter: "AI 适配器",
    import_export: "导入/导出",
    utility: "工具",
    integration: "集成",
  };
  return labels[type] || type;
};

export default function PluginManager({ onClose }: PluginManagerProps) {
  const [plugins, setPlugins] = useState<Plugin[]>([]);
  const [commands, setCommands] = useState<PluginCommand[]>([]);
  const [searchQuery, setSearchQuery] = useState("");
  const [selectedPlugin, setSelectedPlugin] = useState<Plugin | null>(null);
  const [showInstallDialog, setShowInstallDialog] = useState(false);
  const [showDevGuide, setShowDevGuide] = useState(false);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadPlugins();
    loadCommands();
  }, []);

  const loadPlugins = async () => {
    try {
      const result = await invoke("plugin_get_all");
      console.log("Raw plugins result:", result, typeof result);
      const parsedPlugins = Array.isArray(result) ? result : [];
      console.log("Parsed plugins:", parsedPlugins);
      setPlugins(parsedPlugins);
    } catch (error) {
      console.error("Failed to load plugins:", error);
      setPlugins([]);
    } finally {
      setLoading(false);
    }
  };

  const loadCommands = async () => {
    try {
      const result = await invoke("plugin_get_commands");
      console.log("Raw commands result:", result, typeof result);
      const parsedCommands = Array.isArray(result) ? result : [];
      console.log("Parsed commands:", parsedCommands);
      setCommands(parsedCommands);
    } catch (error) {
      console.error("Failed to load commands:", error);
      setCommands([]);
    }
  };

  const handleActivate = async (pluginId: string) => {
    try {
      await invoke("plugin_activate", { pluginId });
      await loadPlugins();
      await loadCommands();
    } catch (error) {
      console.error("Failed to activate plugin:", error);
      alert(`激活插件失败: ${error}`);
    }
  };

  const handleDeactivate = async (pluginId: string) => {
    try {
      await invoke("plugin_deactivate", { pluginId });
      await loadPlugins();
      await loadCommands();
    } catch (error) {
      console.error("Failed to deactivate plugin:", error);
      alert(`停用插件失败: ${error}`);
    }
  };

  const handleUninstall = async (pluginId: string) => {
    if (!confirm(`确定要卸载此插件吗？此操作不可撤销。`)) {
      return;
    }

    try {
      await invoke("plugin_uninstall", { pluginId });
      await loadPlugins();
      await loadCommands();
      if (selectedPlugin?.manifest.info.id === pluginId) {
        setSelectedPlugin(null);
      }
    } catch (error) {
      console.error("Failed to uninstall plugin:", error);
      alert(`卸载插件失败: ${error}`);
    }
  };

  const handleSearch = async (query: string) => {
    setSearchQuery(query);
    if (query.trim()) {
      try {
        const result = await invoke<Plugin[]>("plugin_search", { query });
        setPlugins(result);
      } catch (error) {
        console.error("Failed to search plugins:", error);
      }
    } else {
      await loadPlugins();
    }
  };

  const filteredPlugins = plugins.filter((p) => {
    if (searchQuery) {
      return true;
    }
    return p.manifest.info.id !== selectedPlugin?.manifest.info.id;
  });

  return (
    <div className="h-full flex flex-col bg-background">
      <div className="border-b border-border p-4">
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-xl font-semibold">插件管理</h2>
          {onClose && (
            <button onClick={onClose} className="text-muted-foreground hover:text-foreground">
              ×
            </button>
          )}
        </div>

        <div className="flex gap-2">
          <div className="flex-1 relative">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground" />
            <input
              type="text"
              placeholder="搜索插件..."
              value={searchQuery}
              onChange={(e) => handleSearch(e.target.value)}
              className="w-full pl-10 pr-4 py-2 border border-border rounded-lg bg-background"
            />
          </div>
          <button
            onClick={() => setShowInstallDialog(true)}
            className="px-4 py-2 bg-primary text-primary-foreground rounded-lg flex items-center gap-2 hover:bg-primary/90"
          >
            <Download className="w-4 h-4" />
            安装插件
          </button>
          <button
            onClick={() => setShowDevGuide(true)}
            className="px-4 py-2 bg-secondary text-secondary-foreground rounded-lg flex items-center gap-2 hover:bg-secondary/80"
          >
            <BookOpen className="w-4 h-4" />
            开发指南
          </button>
        </div>
      </div>

      <div className="flex-1 flex overflow-hidden">
        <div className="w-80 border-r border-border overflow-y-auto">
          {loading ? (
            <div className="p-4 text-center text-muted-foreground">加载中...</div>
          ) : filteredPlugins.length === 0 ? (
            <div className="p-4 text-center text-muted-foreground">
              {searchQuery ? "未找到匹配的插件" : "暂无已安装的插件"}
            </div>
          ) : (
            <div className="divide-y divide-border">
              {filteredPlugins.map((plugin) => (
                <div
                  key={plugin.manifest.info.id}
                  onClick={() => setSelectedPlugin(plugin)}
                  className={`p-4 cursor-pointer hover:bg-accent ${
                    selectedPlugin?.manifest.info.id === plugin.manifest.info.id ? "bg-accent" : ""
                  }`}
                >
                  <div className="flex items-start gap-3">
                    {plugin.manifest.info.icon && (
                      <img
                        src={plugin.manifest.info.icon}
                        alt={plugin.manifest.info.name}
                        className="w-10 h-10 rounded"
                      />
                    )}
                    <div className="flex-1 min-w-0">
                      <h3 className="font-medium truncate">{plugin.manifest.info.name}</h3>
                      <p className="text-sm text-muted-foreground truncate">
                        {plugin.manifest.info.description}
                      </p>
                      <div className="flex items-center gap-2 mt-2">
                        {getStateBadge(plugin.state)}
                        <span className="text-xs text-muted-foreground">
                          {plugin.manifest.info.version}
                        </span>
                      </div>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>

        {selectedPlugin && (
          <PluginDetailPanel
            plugin={selectedPlugin}
            commands={commands.filter((c) => c.plugin_id === selectedPlugin.manifest.info.id)}
            onActivate={handleActivate}
            onDeactivate={handleDeactivate}
            onUninstall={handleUninstall}
          />
        )}
      </div>

      {showInstallDialog && (
        <InstallPluginDialog
          onClose={() => setShowInstallDialog(false)}
          onInstall={async () => {
            await loadPlugins();
            setShowInstallDialog(false);
          }}
        />
      )}

      {showDevGuide && <PluginDevGuideDialog onClose={() => setShowDevGuide(false)} />}
    </div>
  );
}

interface PluginDetailPanelProps {
  plugin: Plugin;
  commands: PluginCommand[];
  onActivate: (pluginId: string) => void;
  onDeactivate: (pluginId: string) => void;
  onUninstall: (pluginId: string) => void;
}

function PluginDetailPanel({
  plugin,
  commands,
  onActivate,
  onDeactivate,
  onUninstall,
}: PluginDetailPanelProps) {
  const [activeTab, setActiveTab] = useState<"overview" | "permissions" | "settings" | "commands">(
    "overview"
  );
  const [permissions, setPermissions] = useState<PermissionStatus[]>([]);
  const [settings, setSettings] = useState<Record<string, any>>({});

  useEffect(() => {
    loadPermissions();
    loadSettings();
  }, [plugin]);

  const loadPermissions = async () => {
    try {
      const result = await invoke<PermissionStatus[]>("plugin_get_permissions", {
        pluginId: plugin.manifest.info.id,
      });
      setPermissions(result);
    } catch (error) {
      console.error("Failed to load permissions:", error);
    }
  };

  const loadSettings = async () => {
    try {
      const result = await invoke("plugin_get_settings", {
        pluginId: plugin.manifest.info.id,
      });
      setSettings(result as Record<string, any>);
    } catch (error) {
      console.error("Failed to load settings:", error);
    }
  };

  const handleSaveSettings = async () => {
    try {
      await invoke("plugin_update_settings", {
        pluginId: plugin.manifest.info.id,
        settings,
      });
      alert("设置已保存");
    } catch (error) {
      console.error("Failed to save settings:", error);
      alert("保存设置失败");
    }
  };

  const isActive = plugin.state === "activated";

  return (
    <div className="flex-1 flex flex-col overflow-hidden">
      <div className="border-b border-border p-4">
        <div className="flex items-start justify-between">
          <div className="flex items-start gap-4">
            {plugin.manifest.info.icon && (
              <img
                src={plugin.manifest.info.icon}
                alt={plugin.manifest.info.name}
                className="w-16 h-16 rounded-lg"
              />
            )}
            <div>
              <h3 className="text-lg font-semibold">{plugin.manifest.info.name}</h3>
              <p className="text-sm text-muted-foreground mb-2">
                {plugin.manifest.info.description}
              </p>
              <div className="flex items-center gap-3 text-sm">
                <span>v{plugin.manifest.info.version}</span>
                <span>by {plugin.manifest.info.author.name}</span>
                {getStateBadge(plugin.state)}
              </div>
            </div>
          </div>
          <div className="flex gap-2">
            {isActive ? (
              <button
                onClick={() => onDeactivate(plugin.manifest.info.id)}
                className="px-3 py-1.5 bg-secondary text-secondary-foreground rounded flex items-center gap-2 hover:bg-secondary/80"
              >
                <PowerOff className="w-4 h-4" />
                停用
              </button>
            ) : (
              <button
                onClick={() => onActivate(plugin.manifest.info.id)}
                className="px-3 py-1.5 bg-primary text-primary-foreground rounded flex items-center gap-2 hover:bg-primary/90"
              >
                <Power className="w-4 h-4" />
                激活
              </button>
            )}
            <button
              onClick={() => onUninstall(plugin.manifest.info.id)}
              className="px-3 py-1.5 bg-destructive text-destructive-foreground rounded flex items-center gap-2 hover:bg-destructive/90"
            >
              <Trash2 className="w-4 h-4" />
              卸载
            </button>
          </div>
        </div>

        <div className="flex gap-1 mt-4">
          {[
            { id: "overview", label: "概览" },
            { id: "permissions", label: "权限" },
            { id: "settings", label: "设置" },
            { id: "commands", label: "命令" },
          ].map((tab) => (
            <button
              key={tab.id as any}
              onClick={() => setActiveTab(tab.id as any)}
              className={`px-4 py-2 text-sm rounded-t ${
                activeTab === tab.id
                  ? "bg-background border-b-2 border-primary"
                  : "text-muted-foreground hover:text-foreground"
              }`}
            >
              {tab.label}
            </button>
          ))}
        </div>
      </div>

      <div className="flex-1 overflow-y-auto p-4">
        {activeTab === "overview" && (
          <div className="space-y-6">
            <div>
              <h4 className="font-medium mb-2">基本信息</h4>
              <dl className="grid grid-cols-2 gap-4">
                <div>
                  <dt className="text-sm text-muted-foreground">插件 ID</dt>
                  <dd className="text-sm font-mono">{plugin.manifest.info.id}</dd>
                </div>
                <div>
                  <dt className="text-sm text-muted-foreground">类型</dt>
                  <dd>{getPluginTypeLabel(plugin.manifest.info.pluginType)}</dd>
                </div>
                <div>
                  <dt className="text-sm text-muted-foreground">最小应用版本</dt>
                  <dd>{plugin.manifest.info.minAppVersion}</dd>
                </div>
                <div>
                  <dt className="text-sm text-muted-foreground">安装时间</dt>
                  <dd>{new Date(plugin.installed_at).toLocaleString("zh-CN")}</dd>
                </div>
                {plugin.last_activated && (
                  <div>
                    <dt className="text-sm text-muted-foreground">最后激活</dt>
                    <dd>{new Date(plugin.last_activated).toLocaleString("zh-CN")}</dd>
                  </div>
                )}
              </dl>
            </div>

            {plugin.manifest.info.homepage && (
              <div>
                <h4 className="font-medium mb-2">主页</h4>
                <a
                  href={plugin.manifest.info.homepage}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-primary hover:underline"
                >
                  {plugin.manifest.info.homepage}
                </a>
              </div>
            )}

            {plugin.manifest.info.repository && (
              <div>
                <h4 className="font-medium mb-2">代码仓库</h4>
                <a
                  href={plugin.manifest.info.repository}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-primary hover:underline"
                >
                  {plugin.manifest.info.repository}
                </a>
              </div>
            )}

            {plugin.manifest.info.keywords && plugin.manifest.info.keywords.length > 0 && (
              <div>
                <h4 className="font-medium mb-2">关键词</h4>
                <div className="flex flex-wrap gap-2">
                  {plugin.manifest.info.keywords.map((keyword, index) => (
                    <span key={index} className="px-2 py-1 text-xs bg-secondary rounded">
                      {keyword}
                    </span>
                  ))}
                </div>
              </div>
            )}

            {plugin.manifest.capabilities.length > 0 && (
              <div>
                <h4 className="font-medium mb-2">功能能力</h4>
                <div className="flex flex-wrap gap-2">
                  {plugin.manifest.capabilities.map((cap, index) => (
                    <span key={index} className="px-2 py-1 text-xs bg-secondary rounded">
                      {cap}
                    </span>
                  ))}
                </div>
              </div>
            )}
          </div>
        )}

        {activeTab === "permissions" && (
          <div className="space-y-4">
            <PermissionsPanel
              pluginId={plugin.manifest.info.id}
              permissions={permissions}
              onRefresh={loadPermissions}
            />
          </div>
        )}

        {activeTab === "settings" && (
          <div className="space-y-4">
            <div className="flex justify-between items-center mb-4">
              <h4 className="font-medium">插件设置</h4>
              <button
                onClick={handleSaveSettings}
                className="px-4 py-2 bg-primary text-primary-foreground rounded hover:bg-primary/90"
              >
                保存设置
              </button>
            </div>
            <pre className="bg-muted p-4 rounded-lg overflow-auto text-sm">
              {JSON.stringify(settings, null, 2)}
            </pre>
          </div>
        )}

        {activeTab === "commands" && (
          <div className="space-y-4">
            {commands.length === 0 ? (
              <div className="text-center text-muted-foreground py-8">此插件未注册任何命令</div>
            ) : (
              commands.map((command) => (
                <div key={command.command_id} className="p-4 border border-border rounded-lg">
                  <div className="flex items-center justify-between mb-2">
                    <h5 className="font-medium">{command.title}</h5>
                    {command.category && (
                      <span className="text-xs px-2 py-1 bg-secondary rounded">
                        {command.category}
                      </span>
                    )}
                  </div>
                  {command.description && (
                    <p className="text-sm text-muted-foreground mb-2">{command.description}</p>
                  )}
                  {command.keybinding && (
                    <div className="text-sm text-muted-foreground">
                      快捷键:{" "}
                      <kbd className="px-2 py-1 bg-muted rounded text-xs">{command.keybinding}</kbd>
                    </div>
                  )}
                </div>
              ))
            )}
          </div>
        )}
      </div>
    </div>
  );
}

interface PermissionsPanelProps {
  pluginId: string;
  permissions: PermissionStatus[];
  onRefresh: () => void;
}

function PermissionsPanel({ pluginId, permissions, onRefresh }: PermissionsPanelProps) {
  const handleGrantPermission = async (permissionName: string) => {
    try {
      await invoke("plugin_grant_permission", { pluginId, permissionName });
      await onRefresh();
    } catch (error) {
      console.error("Failed to grant permission:", error);
      alert(`授权失败: ${error}`);
    }
  };

  const handleRevokePermission = async (permissionName: string) => {
    try {
      await invoke("plugin_revoke_permission", { pluginId, permissionName });
      await onRefresh();
    } catch (error) {
      console.error("Failed to revoke permission:", error);
      alert(`撤销授权失败: ${error}`);
    }
  };

  const getRiskColor = (risk: string) => {
    switch (risk) {
      case "low":
        return "text-green-600";
      case "medium":
        return "text-yellow-600";
      case "high":
        return "text-red-600";
      default:
        return "text-gray-600";
    }
  };

  const getRiskLabel = (risk: string) => {
    switch (risk) {
      case "low":
        return "低";
      case "medium":
        return "中";
      case "high":
        return "高";
      default:
        return risk;
    }
  };

  return (
    <div className="space-y-4">
      {permissions.length === 0 ? (
        <div className="text-center text-muted-foreground py-8">此插件不需要任何权限</div>
      ) : (
        permissions.map((perm) => (
          <div key={perm.name} className="p-4 border border-border rounded-lg">
            <div className="flex items-center justify-between mb-2">
              <div className="flex items-center gap-2">
                <Shield className="w-4 h-4" />
                <span className="font-medium">{perm.name}</span>
              </div>
              <div className="flex items-center gap-2">
                <span className={`text-xs ${getRiskColor(perm.risk)}`}>
                  风险等级: {getRiskLabel(perm.risk)}
                </span>
                {perm.granted ? (
                  <button
                    onClick={() => handleRevokePermission(perm.name)}
                    className="px-3 py-1 text-sm bg-destructive/10 text-destructive rounded hover:bg-destructive/20"
                  >
                    撤销
                  </button>
                ) : (
                  <button
                    onClick={() => handleGrantPermission(perm.name)}
                    className="px-3 py-1 text-sm bg-primary text-primary-foreground rounded hover:bg-primary/90"
                  >
                    授权
                  </button>
                )}
              </div>
            </div>
            <div className="flex items-center gap-2">
              {perm.granted ? (
                <span className="text-xs text-green-600">已授权</span>
              ) : (
                <span className="text-xs text-muted-foreground">未授权</span>
              )}
            </div>
          </div>
        ))
      )}
    </div>
  );
}

function InstallPluginDialog({
  onClose,
  onInstall,
}: {
  onClose: () => void;
  onInstall: () => void;
}) {
  const [pluginPath, setPluginPath] = useState("");
  const [installing, setInstalling] = useState(false);

  const handleInstall = async () => {
    if (!pluginPath.trim()) {
      alert("请输入插件路径");
      return;
    }

    setInstalling(true);
    try {
      await invoke("plugin_install", { pluginPath });
      await onInstall();
    } catch (error) {
      console.error("Failed to install plugin:", error);
      alert(`安装失败: ${error}`);
    } finally {
      setInstalling(false);
    }
  };

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-background rounded-lg p-6 w-full max-w-md">
        <h3 className="text-lg font-semibold mb-4">安装插件</h3>
        <div className="space-y-4">
          <div>
            <label className="block text-sm font-medium mb-2">插件路径</label>
            <input
              type="text"
              value={pluginPath}
              onChange={(e) => setPluginPath(e.target.value)}
              placeholder="输入插件目录或 plugin.json 文件路径"
              className="w-full px-4 py-2 border border-border rounded-lg bg-background"
            />
            <p className="text-xs text-muted-foreground mt-1">
              支持插件目录或包含 plugin.json 的目录
            </p>
          </div>
        </div>
        <div className="flex justify-end gap-2 mt-6">
          <button
            onClick={onClose}
            disabled={installing}
            className="px-4 py-2 bg-secondary text-secondary-foreground rounded hover:bg-secondary/80 disabled:opacity-50"
          >
            取消
          </button>
          <button
            onClick={handleInstall}
            disabled={installing}
            className="px-4 py-2 bg-primary text-primary-foreground rounded hover:bg-primary/90 disabled:opacity-50"
          >
            {installing ? "安装中..." : "安装"}
          </button>
        </div>
      </div>
    </div>
  );
}
