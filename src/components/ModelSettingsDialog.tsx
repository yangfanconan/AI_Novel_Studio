import React, { useState, useEffect } from 'react';
import { X, Loader2, Eye, EyeOff, Check, Star, Key, Sliders } from 'lucide-react';
import { invoke } from '@tauri-apps/api/core';
import { logger } from '../utils/logger';
import { uiLogger } from '../utils/uiLogger';

interface ModelSettingsDialogProps {
  open: boolean;
  onClose: () => void;
}

interface ModelInfo {
  id: string;
  name: string;
  provider: string;
  is_default: boolean;
}

interface AIParams {
  temperature: number;
  max_tokens: number;
  top_p: number;
}

interface APIKeyInfo {
  provider: string;
  provider_name: string;
  is_configured: boolean;
  masked_key: string | null;
}

type TabType = 'models' | 'params' | 'keys';

export const ModelSettingsDialog: React.FC<ModelSettingsDialogProps> = ({
  open,
  onClose,
}) => {
  console.log('ModelSettingsDialog render, open:', open);

  // Tab 状态
  const [activeTab, setActiveTab] = useState<TabType>('models');

  // 模型相关状态
  const [models, setModels] = useState<ModelInfo[]>([]);
  const [isLoadingModels, setIsLoadingModels] = useState(false);

  // AI 参数状态
  const [aiParams, setAIParams] = useState<AIParams>({
    temperature: 0.7,
    max_tokens: 2000,
    top_p: 0.9,
  });
  const [originalAIParams, setOriginalAIParams] = useState<AIParams>(aiParams);

  // API 密钥状态
  const [apiKeys, setAPIKeys] = useState<APIKeyInfo[]>([]);
  const [editingProvider, setEditingProvider] = useState<string | null>(null);
  const [newApiKey, setNewApiKey] = useState('');
  const [showApiKey, setShowApiKey] = useState(false);

  // 通用状态
  const [isSaving, setIsSaving] = useState(false);
  const [saveStatus, setSaveStatus] = useState<'idle' | 'saving' | 'saved' | 'error'>('idle');
  const [hasChanges, setHasChanges] = useState(false);

  // 加载模型列表
  const loadModels = async () => {
    console.log('loadModels called');
    setIsLoadingModels(true);
    try {
      const availableModels = await invoke<ModelInfo[]>('get_models_with_default');
      logger.info('Loaded models with default', {
        feature: 'model-settings',
        data: { count: availableModels.length }
      });
      setModels(availableModels);
    } catch (error) {
      logger.error('Failed to load models', error, { feature: 'model-settings' });
    } finally {
      setIsLoadingModels(false);
    }
  };

  // 加载 AI 参数
  const loadAIParams = async () => {
    try {
      const params = await invoke<AIParams>('get_ai_params');
      setAIParams(params);
      setOriginalAIParams(params);
    } catch (error) {
      logger.error('Failed to load AI params', error, { feature: 'model-settings' });
    }
  };

  // 加载 API 密钥列表
  const loadAPIKeys = async () => {
    try {
      const keys = await invoke<APIKeyInfo[]>('get_api_keys');
      setAPIKeys(keys);
    } catch (error) {
      logger.error('Failed to load API keys', error, { feature: 'model-settings' });
    }
  };

  // 检测参数变化
  useEffect(() => {
    const paramsChanged =
      aiParams.temperature !== originalAIParams.temperature ||
      aiParams.max_tokens !== originalAIParams.max_tokens ||
      aiParams.top_p !== originalAIParams.top_p;
    setHasChanges(paramsChanged);
  }, [aiParams, originalAIParams]);

  useEffect(() => {
    console.log('ModelSettingsDialog useEffect called, open:', open);
    uiLogger.open('ModelSettingsDialog');

    if (open) {
      console.log('open is true, loading all settings');
      loadModels();
      loadAIParams();
      loadAPIKeys();
      setSaveStatus('idle');
      setActiveTab('models');
    }
  }, [open]);

  // 设置默认模型
  const handleSetDefaultModel = async (modelId: string) => {
    uiLogger.click('ModelSettingsDialog', 'set_default_model', { modelId });

    setIsSaving(true);
    try {
      await invoke('set_default_model', { modelId });
      logger.info('Default model set', {
        feature: 'model-settings',
        data: { modelId }
      });
      // 刷新模型列表
      await loadModels();
    } catch (error) {
      logger.error('Failed to set default model', error, { feature: 'model-settings' });
      alert('设置默认模型失败: ' + (error as Error).message);
    } finally {
      setIsSaving(false);
    }
  };

  // 保存 API 密钥
  const handleSaveAPIKey = async (provider: string) => {
    uiLogger.click('ModelSettingsDialog', 'save_api_key', { provider });

    if (!newApiKey.trim()) {
      logger.warn('API key is empty', { feature: 'model-settings' });
      return;
    }

    setIsSaving(true);
    try {
      await invoke('set_api_key', { provider, apiKey: newApiKey });
      logger.info('API key saved', {
        feature: 'model-settings',
        data: { provider }
      });
      setNewApiKey('');
      setEditingProvider(null);
      setShowApiKey(false);
      // 刷新密钥列表
      await loadAPIKeys();
      // 如果是 bigmodel，也刷新模型列表
      if (provider === 'bigmodel') {
        await loadModels();
      }
    } catch (error) {
      logger.error('Failed to save API key', error, { feature: 'model-settings' });
      alert('保存 API 密钥失败: ' + (error as Error).message);
    } finally {
      setIsSaving(false);
    }
  };

  // 保存所有设置
  const handleSaveAll = async () => {
    uiLogger.click('ModelSettingsDialog', 'save_all', aiParams);

    setSaveStatus('saving');
    setIsSaving(true);
    try {
      // 保存 AI 参数
      await invoke('set_ai_params', { params: aiParams });
      setOriginalAIParams(aiParams);
      setHasChanges(false);
      
      logger.info('AI params saved', {
        feature: 'model-settings',
        data: aiParams
      });
      
      setSaveStatus('saved');
      setTimeout(() => setSaveStatus('idle'), 2000);
    } catch (error) {
      logger.error('Failed to save settings', error, { feature: 'model-settings' });
      setSaveStatus('error');
      alert('保存设置失败: ' + (error as Error).message);
    } finally {
      setIsSaving(false);
    }
  };

  // 取消更改
  const handleCancel = () => {
    setAIParams(originalAIParams);
    setHasChanges(false);
    setEditingProvider(null);
    setNewApiKey('');
  };

  console.log('ModelSettingsDialog render check, open:', open);

  if (!open) {
    console.log('ModelSettingsDialog returning null because open is false');
    return null;
  }

  console.log('ModelSettingsDialog rendering dialog');

  const tabs = [
    { id: 'models' as TabType, label: '模型管理', icon: Star },
    { id: 'params' as TabType, label: 'AI 参数', icon: Sliders },
    { id: 'keys' as TabType, label: 'API 密钥', icon: Key },
  ];

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-white dark:bg-slate-900 w-[600px] max-h-[80vh] rounded-lg shadow-lg flex flex-col">
        {/* 标题栏 */}
        <div className="flex items-center justify-between px-6 py-4 border-b border-slate-200 dark:border-slate-700 shrink-0">
          <h2 className="text-lg font-semibold text-slate-900 dark:text-white">系统设置</h2>
          <div className="flex items-center gap-2">
            {saveStatus === 'saved' && (
              <span className="text-sm text-green-600 dark:text-green-400 flex items-center gap-1">
                <Check className="w-4 h-4" /> 已保存
              </span>
            )}
            {saveStatus === 'saving' && (
              <span className="text-sm text-blue-600 dark:text-blue-400 flex items-center gap-1">
                <Loader2 className="w-4 h-4 animate-spin" /> 保存中...
              </span>
            )}
            <button
              onClick={onClose}
              className="p-1 hover:bg-accent rounded transition-colors"
            >
              <X className="w-5 h-5" />
            </button>
          </div>
        </div>

        {/* Tab 栏 */}
        <div className="flex border-b border-slate-200 dark:border-slate-700 shrink-0">
          {tabs.map((tab) => (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id)}
              className={`flex items-center gap-2 px-6 py-3 text-sm font-medium transition-colors border-b-2 -mb-px ${
                activeTab === tab.id
                  ? 'text-blue-600 dark:text-blue-400 border-blue-600 dark:border-blue-400'
                  : 'text-slate-600 dark:text-slate-400 border-transparent hover:text-slate-900 dark:hover:text-white'
              }`}
            >
              <tab.icon className="w-4 h-4" />
              {tab.label}
            </button>
          ))}
        </div>

        {/* 内容区 */}
        <div className="flex-1 overflow-y-auto p-6">
          {/* 模型管理 Tab */}
          {activeTab === 'models' && (
            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <h3 className="text-sm font-medium text-slate-700 dark:text-slate-300">
                  可用模型列表
                </h3>
                <span className="text-xs text-slate-500 dark:text-slate-400">
                  点击"设为默认"选择默认使用的模型
                </span>
              </div>

              {isLoadingModels ? (
                <div className="flex items-center justify-center py-8 text-slate-500 dark:text-slate-400">
                  <Loader2 className="w-5 h-5 animate-spin mr-2" />
                  加载中...
                </div>
              ) : (
                <div className="space-y-2">
                  {models.length === 0 ? (
                    <p className="text-sm text-slate-500 dark:text-slate-400 text-center py-8">
                      暂无可用模型，请先配置 API 密钥
                    </p>
                  ) : (
                    models.map((model) => (
                      <div
                        key={model.id}
                        className={`flex items-center justify-between p-3 rounded-lg border ${
                          model.is_default
                            ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/20'
                            : 'border-slate-200 dark:border-slate-700 bg-slate-50 dark:bg-slate-800'
                        }`}
                      >
                        <div className="flex items-center gap-3">
                          {model.is_default && (
                            <Star className="w-4 h-4 text-yellow-500 fill-yellow-500" />
                          )}
                          <div>
                            <span className="text-sm font-medium text-slate-900 dark:text-white">
                              {model.name}
                            </span>
                            <span className="ml-2 text-xs px-2 py-0.5 bg-blue-100 dark:bg-blue-900 text-blue-600 dark:text-blue-400 rounded">
                              {model.provider}
                            </span>
                          </div>
                        </div>
                        {model.is_default ? (
                          <span className="text-xs text-blue-600 dark:text-blue-400 font-medium">
                            当前默认
                          </span>
                        ) : (
                          <button
                            onClick={() => handleSetDefaultModel(model.id)}
                            disabled={isSaving}
                            className="text-xs px-3 py-1 bg-blue-500 text-white rounded hover:bg-blue-600 disabled:bg-blue-400 disabled:cursor-not-allowed transition-colors"
                          >
                            设为默认
                          </button>
                        )}
                      </div>
                    ))
                  )}
                </div>
              )}
            </div>
          )}

          {/* AI 参数 Tab */}
          {activeTab === 'params' && (
            <div className="space-y-6">
              <p className="text-sm text-slate-500 dark:text-slate-400">
                调整 AI 生成内容的参数设置，这些参数会影响所有 AI 功能的输出效果。
              </p>

              {/* Temperature */}
              <div className="space-y-2">
                <div className="flex items-center justify-between">
                  <label className="text-sm font-medium text-slate-700 dark:text-slate-300">
                    Temperature（温度）
                  </label>
                  <span className="text-sm text-slate-600 dark:text-slate-400 font-mono">
                    {aiParams.temperature.toFixed(2)}
                  </span>
                </div>
                <input
                  type="range"
                  min="0"
                  max="2"
                  step="0.1"
                  value={aiParams.temperature}
                  onChange={(e) =>
                    setAIParams({ ...aiParams, temperature: parseFloat(e.target.value) })
                  }
                  className="w-full h-2 bg-slate-200 dark:bg-slate-700 rounded-lg appearance-none cursor-pointer accent-blue-500"
                />
                <div className="flex justify-between text-xs text-slate-500 dark:text-slate-400">
                  <span>精确 (0.0)</span>
                  <span>创意 (2.0)</span>
                </div>
              </div>

              {/* Max Tokens */}
              <div className="space-y-2">
                <div className="flex items-center justify-between">
                  <label className="text-sm font-medium text-slate-700 dark:text-slate-300">
                    Max Tokens（最大 Token 数）
                  </label>
                  <span className="text-sm text-slate-600 dark:text-slate-400 font-mono">
                    {aiParams.max_tokens}
                  </span>
                </div>
                <input
                  type="range"
                  min="100"
                  max="4000"
                  step="100"
                  value={aiParams.max_tokens}
                  onChange={(e) =>
                    setAIParams({ ...aiParams, max_tokens: parseInt(e.target.value) })
                  }
                  className="w-full h-2 bg-slate-200 dark:bg-slate-700 rounded-lg appearance-none cursor-pointer accent-blue-500"
                />
                <div className="flex justify-between text-xs text-slate-500 dark:text-slate-400">
                  <span>简洁 (100)</span>
                  <span>详细 (4000)</span>
                </div>
              </div>

              {/* Top P */}
              <div className="space-y-2">
                <div className="flex items-center justify-between">
                  <label className="text-sm font-medium text-slate-700 dark:text-slate-300">
                    Top P（核采样）
                  </label>
                  <span className="text-sm text-slate-600 dark:text-slate-400 font-mono">
                    {aiParams.top_p.toFixed(2)}
                  </span>
                </div>
                <input
                  type="range"
                  min="0"
                  max="1"
                  step="0.1"
                  value={aiParams.top_p}
                  onChange={(e) =>
                    setAIParams({ ...aiParams, top_p: parseFloat(e.target.value) })
                  }
                  className="w-full h-2 bg-slate-200 dark:bg-slate-700 rounded-lg appearance-none cursor-pointer accent-blue-500"
                />
                <div className="flex justify-between text-xs text-slate-500 dark:text-slate-400">
                  <span>保守 (0.0)</span>
                  <span>多样 (1.0)</span>
                </div>
              </div>

              {/* 参数说明 */}
              <div className="p-4 bg-slate-50 dark:bg-slate-800 rounded-lg space-y-2">
                <h4 className="text-sm font-medium text-slate-700 dark:text-slate-300">
                  参数说明
                </h4>
                <ul className="text-xs text-slate-500 dark:text-slate-400 space-y-1">
                  <li>
                    <strong>Temperature</strong>：控制输出的随机性。较低的值使输出更确定，较高的值增加创意性。
                  </li>
                  <li>
                    <strong>Max Tokens</strong>：限制生成的最大 Token 数量，影响输出长度。
                  </li>
                  <li>
                    <strong>Top P</strong>：核采样参数，控制从概率最高的 Token 中采样的范围。
                  </li>
                </ul>
              </div>
            </div>
          )}

          {/* API 密钥 Tab */}
          {activeTab === 'keys' && (
            <div className="space-y-4">
              <p className="text-sm text-slate-500 dark:text-slate-400">
                配置各 AI 服务的 API 密钥。密钥将安全存储在本地数据库中。
              </p>

              <div className="space-y-3">
                {apiKeys.map((keyInfo) => (
                  <div
                    key={keyInfo.provider}
                    className="p-4 border border-slate-200 dark:border-slate-700 rounded-lg"
                  >
                    <div className="flex items-center justify-between mb-2">
                      <div className="flex items-center gap-2">
                        <span className="text-sm font-medium text-slate-900 dark:text-white">
                          {keyInfo.provider_name}
                        </span>
                        {keyInfo.is_configured && (
                          <span className="text-xs px-2 py-0.5 bg-green-100 dark:bg-green-900 text-green-600 dark:text-green-400 rounded">
                            已配置
                          </span>
                        )}
                      </div>
                      {editingProvider !== keyInfo.provider && (
                        <button
                          onClick={() => {
                            setEditingProvider(keyInfo.provider);
                            setNewApiKey('');
                          }}
                          className="text-xs text-blue-600 dark:text-blue-400 hover:underline"
                        >
                          {keyInfo.is_configured ? '更新密钥' : '配置密钥'}
                        </button>
                      )}
                    </div>

                    {editingProvider === keyInfo.provider ? (
                      <div className="space-y-2">
                        <div className="relative">
                          <input
                            type={showApiKey ? 'text' : 'password'}
                            value={newApiKey}
                            onChange={(e) => setNewApiKey(e.target.value)}
                            className="w-full px-3 py-2 pr-10 bg-slate-50 dark:bg-slate-800 border border-slate-300 dark:border-slate-700 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 text-slate-900 dark:text-white text-sm"
                            placeholder="请输入新的 API 密钥"
                          />
                          <button
                            type="button"
                            onClick={() => setShowApiKey(!showApiKey)}
                            className="absolute right-2 top-1/2 -translate-y-1/2 text-slate-400 hover:text-slate-600 dark:hover:text-slate-300"
                          >
                            {showApiKey ? (
                              <EyeOff className="w-4 h-4" />
                            ) : (
                              <Eye className="w-4 h-4" />
                            )}
                          </button>
                        </div>
                        <div className="flex gap-2">
                          <button
                            onClick={() => handleSaveAPIKey(keyInfo.provider)}
                            disabled={isSaving || !newApiKey.trim()}
                            className="text-xs px-3 py-1 bg-blue-500 text-white rounded hover:bg-blue-600 disabled:bg-blue-400 disabled:cursor-not-allowed transition-colors"
                          >
                            {isSaving ? '保存中...' : '保存'}
                          </button>
                          <button
                            onClick={() => {
                              setEditingProvider(null);
                              setNewApiKey('');
                              setShowApiKey(false);
                            }}
                            className="text-xs px-3 py-1 bg-slate-200 dark:bg-slate-700 text-slate-700 dark:text-slate-300 rounded hover:bg-slate-300 dark:hover:bg-slate-600 transition-colors"
                          >
                            取消
                          </button>
                        </div>
                      </div>
                    ) : (
                      <div className="text-sm text-slate-500 dark:text-slate-400">
                        {keyInfo.is_configured ? (
                          <span>当前密钥: {keyInfo.masked_key}</span>
                        ) : (
                          <span className="text-yellow-600 dark:text-yellow-400">
                            尚未配置 API 密钥
                          </span>
                        )}
                      </div>
                    )}

                    {/* 提供商特定说明 */}
                    {keyInfo.provider === 'bigmodel' && (
                      <p className="text-xs text-slate-400 dark:text-slate-500 mt-2">
                        从{' '}
                        <a
                          href="https://open.bigmodel.cn/"
                          target="_blank"
                          rel="noopener noreferrer"
                          className="text-blue-600 dark:text-blue-400 hover:underline"
                        >
                          智谱AI开放平台
                        </a>{' '}
                        获取 API 密钥
                      </p>
                    )}
                    {keyInfo.provider === 'openai' && (
                      <p className="text-xs text-slate-400 dark:text-slate-500 mt-2">
                        从{' '}
                        <a
                          href="https://platform.openai.com/api-keys"
                          target="_blank"
                          rel="noopener noreferrer"
                          className="text-blue-600 dark:text-blue-400 hover:underline"
                        >
                          OpenAI Platform
                        </a>{' '}
                        获取 API 密钥
                      </p>
                    )}
                    {keyInfo.provider === 'anthropic' && (
                      <p className="text-xs text-slate-400 dark:text-slate-500 mt-2">
                        从{' '}
                        <a
                          href="https://console.anthropic.com/"
                          target="_blank"
                          rel="noopener noreferrer"
                          className="text-blue-600 dark:text-blue-400 hover:underline"
                        >
                          Anthropic Console
                        </a>{' '}
                        获取 API 密钥
                      </p>
                    )}
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>

        {/* 底部按钮栏 */}
        <div className="flex items-center justify-between px-6 py-4 border-t border-slate-200 dark:border-slate-700 shrink-0">
          <div className="text-sm text-slate-500 dark:text-slate-400">
            {hasChanges && <span className="text-yellow-600 dark:text-yellow-400">有未保存的更改</span>}
          </div>
          <div className="flex gap-3">
            {hasChanges && (
              <button
                onClick={handleCancel}
                className="px-4 py-2 text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-md transition-colors"
              >
                取消
              </button>
            )}
            <button
              onClick={handleSaveAll}
              disabled={isSaving || !hasChanges}
              className="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 disabled:bg-blue-400 disabled:cursor-not-allowed transition-colors"
            >
              {isSaving ? '保存中...' : '保存设置'}
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};
