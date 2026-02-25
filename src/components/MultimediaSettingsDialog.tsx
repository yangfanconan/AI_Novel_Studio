import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { X, Save, Eye, EyeOff, Image, Video, Film, Music } from "lucide-react";

interface MultimediaSettingsDialogProps {
  isOpen: boolean;
  onClose: () => void;
}

interface ImageProvider {
  id: string;
  name: string;
  api_key: string;
  api_base: string;
  model: string;
  is_enabled: boolean;
}

const defaultProviders: ImageProvider[] = [
  {
    id: "openai",
    name: "OpenAI DALL-E",
    api_key: "",
    api_base: "https://api.openai.com/v1",
    model: "dall-e-3",
    is_enabled: false,
  },
  {
    id: "stability",
    name: "Stability AI",
    api_key: "",
    api_base: "https://api.stability.ai",
    model: "stable-diffusion-xl",
    is_enabled: false,
  },
  {
    id: "midjourney",
    name: "Midjourney (第三方)",
    api_key: "",
    api_base: "",
    model: "midjourney",
    is_enabled: false,
  },
  {
    id: "comfyui",
    name: "ComfyUI (本地)",
    api_key: "",
    api_base: "http://127.0.0.1:8188",
    model: "sd-xl",
    is_enabled: false,
  },
];

export default function MultimediaSettingsDialog({
  isOpen,
  onClose,
}: MultimediaSettingsDialogProps) {
  const [providers, setProviders] = useState<ImageProvider[]>(defaultProviders);
  const [selectedProvider, setSelectedProvider] = useState<string>("openai");
  const [showApiKey, setShowApiKey] = useState<Record<string, boolean>>({});
  const [isSaving, setIsSaving] = useState(false);
  const [activeTab, setActiveTab] = useState<"image" | "video" | "audio">("image");

  useEffect(() => {
    if (isOpen) {
      loadSettings();
    }
  }, [isOpen]);

  const loadSettings = async () => {
    try {
      const savedSettings = localStorage.getItem("multimedia_settings");
      if (savedSettings) {
        const parsed = JSON.parse(savedSettings);
        setProviders(parsed.providers || defaultProviders);
      }
    } catch (error) {
      console.error("Failed to load multimedia settings:", error);
    }
  };

  const handleSave = async () => {
    setIsSaving(true);
    try {
      localStorage.setItem("multimedia_settings", JSON.stringify({ providers }));

      const enabledProvider = providers.find((p) => p.is_enabled);
      if (enabledProvider && enabledProvider.api_key) {
        await invoke("set_multimedia_api_key", {
          provider: enabledProvider.id,
          apiKey: enabledProvider.api_key,
        }).catch(() => {});
      }

      alert("设置已保存");
    } catch (error) {
      console.error("Failed to save settings:", error);
      alert("保存失败: " + error);
    } finally {
      setIsSaving(false);
    }
  };

  const updateProvider = (id: string, updates: Partial<ImageProvider>) => {
    setProviders((prev) => prev.map((p) => (p.id === id ? { ...p, ...updates } : p)));
  };

  const handleEnableProvider = (id: string, enabled: boolean) => {
    setProviders((prev) =>
      prev.map((p) => ({
        ...p,
        is_enabled: p.id === id ? enabled : enabled ? false : p.is_enabled,
      }))
    );
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-white dark:bg-slate-800 rounded-lg shadow-xl w-full max-w-2xl max-h-[90vh] overflow-hidden">
        <div className="flex items-center justify-between p-4 border-b border-slate-200 dark:border-slate-700">
          <h2 className="text-lg font-semibold text-slate-800 dark:text-slate-100">
            多媒体生成设置
          </h2>
          <button
            onClick={onClose}
            className="text-slate-500 hover:text-slate-700 dark:text-slate-400 dark:hover:text-slate-200"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        <div className="flex border-b border-slate-200 dark:border-slate-700">
          <button
            onClick={() => setActiveTab("image")}
            className={`flex items-center gap-2 px-4 py-3 text-sm font-medium border-b-2 transition-colors ${
              activeTab === "image"
                ? "border-blue-500 text-blue-600 dark:text-blue-400"
                : "border-transparent text-slate-500 hover:text-slate-700 dark:text-slate-400"
            }`}
          >
            <Image className="w-4 h-4" />
            图像生成
          </button>
          <button
            onClick={() => setActiveTab("video")}
            className={`flex items-center gap-2 px-4 py-3 text-sm font-medium border-b-2 transition-colors ${
              activeTab === "video"
                ? "border-blue-500 text-blue-600 dark:text-blue-400"
                : "border-transparent text-slate-500 hover:text-slate-700 dark:text-slate-400"
            }`}
          >
            <Video className="w-4 h-4" />
            视频生成
          </button>
          <button
            onClick={() => setActiveTab("audio")}
            className={`flex items-center gap-2 px-4 py-3 text-sm font-medium border-b-2 transition-colors ${
              activeTab === "audio"
                ? "border-blue-500 text-blue-600 dark:text-blue-400"
                : "border-transparent text-slate-500 hover:text-slate-700 dark:text-slate-400"
            }`}
          >
            <Music className="w-4 h-4" />
            音频生成
          </button>
        </div>

        <div className="p-4 overflow-y-auto max-h-[60vh]">
          {activeTab === "image" && (
            <div className="space-y-4">
              <div className="text-sm text-slate-600 dark:text-slate-400 mb-4">
                配置图像生成服务提供商。启用后，将为小说生成插图、角色画像等。
              </div>

              <div className="grid gap-4">
                {providers.map((provider) => (
                  <div
                    key={provider.id}
                    className={`p-4 border rounded-lg transition-colors ${
                      provider.is_enabled
                        ? "border-blue-500 bg-blue-50 dark:bg-blue-900/20"
                        : "border-slate-200 dark:border-slate-600"
                    }`}
                  >
                    <div className="flex items-center justify-between mb-3">
                      <div className="flex items-center gap-2">
                        <input
                          type="radio"
                          name="provider"
                          checked={provider.is_enabled}
                          onChange={(e) => handleEnableProvider(provider.id, e.target.checked)}
                          className="w-4 h-4 text-blue-500"
                        />
                        <span className="font-medium text-slate-800 dark:text-slate-200">
                          {provider.name}
                        </span>
                      </div>
                      {provider.is_enabled && (
                        <span className="px-2 py-0.5 text-xs bg-blue-100 text-blue-700 dark:bg-blue-900 dark:text-blue-300 rounded">
                          已启用
                        </span>
                      )}
                    </div>

                    <div className="space-y-3">
                      <div>
                        <label className="block text-xs text-slate-500 dark:text-slate-400 mb-1">
                          API Key
                        </label>
                        <div className="relative">
                          <input
                            type={showApiKey[provider.id] ? "text" : "password"}
                            value={provider.api_key}
                            onChange={(e) =>
                              updateProvider(provider.id, { api_key: e.target.value })
                            }
                            className="w-full px-3 py-2 pr-10 text-sm border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700 text-slate-800 dark:text-slate-200"
                            placeholder="输入 API Key"
                          />
                          <button
                            type="button"
                            onClick={() =>
                              setShowApiKey((prev) => ({
                                ...prev,
                                [provider.id]: !prev[provider.id],
                              }))
                            }
                            className="absolute right-2 top-1/2 -translate-y-1/2 text-slate-400 hover:text-slate-600"
                          >
                            {showApiKey[provider.id] ? (
                              <EyeOff className="w-4 h-4" />
                            ) : (
                              <Eye className="w-4 h-4" />
                            )}
                          </button>
                        </div>
                      </div>

                      <div className="grid grid-cols-2 gap-3">
                        <div>
                          <label className="block text-xs text-slate-500 dark:text-slate-400 mb-1">
                            API Base URL
                          </label>
                          <input
                            type="text"
                            value={provider.api_base}
                            onChange={(e) =>
                              updateProvider(provider.id, { api_base: e.target.value })
                            }
                            className="w-full px-3 py-2 text-sm border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700 text-slate-800 dark:text-slate-200"
                            placeholder="https://api.example.com"
                          />
                        </div>
                        <div>
                          <label className="block text-xs text-slate-500 dark:text-slate-400 mb-1">
                            模型
                          </label>
                          <input
                            type="text"
                            value={provider.model}
                            onChange={(e) => updateProvider(provider.id, { model: e.target.value })}
                            className="w-full px-3 py-2 text-sm border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700 text-slate-800 dark:text-slate-200"
                            placeholder="model-name"
                          />
                        </div>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )}

          {activeTab === "video" && (
            <div className="space-y-4">
              <div className="text-sm text-slate-600 dark:text-slate-400 mb-4">
                视频生成功能即将推出，敬请期待。
              </div>
              <div className="p-8 text-center text-slate-400 dark:text-slate-500">
                <Film className="w-12 h-12 mx-auto mb-3 opacity-50" />
                <p>视频生成功能开发中</p>
              </div>
            </div>
          )}

          {activeTab === "audio" && (
            <div className="space-y-4">
              <div className="text-sm text-slate-600 dark:text-slate-400 mb-4">
                音频生成功能即将推出，敬请期待。
              </div>
              <div className="p-8 text-center text-slate-400 dark:text-slate-500">
                <Music className="w-12 h-12 mx-auto mb-3 opacity-50" />
                <p>音频生成功能开发中</p>
              </div>
            </div>
          )}
        </div>

        <div className="flex items-center justify-end gap-2 p-4 border-t border-slate-200 dark:border-slate-700">
          <button
            onClick={onClose}
            className="px-4 py-2 text-slate-600 dark:text-slate-400 hover:text-slate-800 dark:hover:text-slate-200"
          >
            取消
          </button>
          <button
            onClick={handleSave}
            disabled={isSaving}
            className="flex items-center gap-2 px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors disabled:opacity-50"
          >
            <Save className="w-4 h-4" />
            {isSaving ? "保存中..." : "保存设置"}
          </button>
        </div>
      </div>
    </div>
  );
}
