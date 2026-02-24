import React, { useState, useEffect } from 'react';
import {
  X,
  Sparkles,
  Loader2,
  Download,
  Check,
  Palette,
  Image as ImageIcon,
  Maximize2,
  RefreshCw,
  Copy
} from 'lucide-react';
import { multimediaService } from '../services/multimedia.service';
import type {
  Illustration,
  IllustrationOptions,
  VisualStyle
} from '../types/multimedia';
import {
  VisualStyle as VS,
  VISUAL_STYLE_LABELS
} from '../types/multimedia';
import type { Chapter, Character } from '../types';

interface IllustrationGeneratorDialogProps {
  isOpen: boolean;
  onClose: () => void;
  projectId: string;
  chapters: Chapter[];
  characters: Character[];
  currentChapterId?: string;
  selectedText?: string;
}

export function IllustrationGeneratorDialog({
  isOpen,
  onClose,
  projectId,
  chapters,
  characters,
  currentChapterId,
  selectedText,
}: IllustrationGeneratorDialogProps) {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [illustration, setIllustration] = useState<Illustration | null>(null);
  const [copied, setCopied] = useState(false);
  const [options, setOptions] = useState<IllustrationOptions>(
    multimediaService.getIllustrationDefaults()
  );
  const [customDescription, setCustomDescription] = useState('');
  const [selectedCharacters, setSelectedCharacters] = useState<string[]>([]);
  const [showAdvanced, setShowAdvanced] = useState(false);

  useEffect(() => {
    if (isOpen) {
      setIllustration(null);
      setError(null);
      setCustomDescription(selectedText || '');
      setSelectedCharacters([]);
    }
  }, [isOpen, selectedText]);

  const handleGenerate = async () => {
    if (!customDescription.trim()) {
      setError('请输入场景描述');
      return;
    }

    setLoading(true);
    setError(null);
    setIllustration(null);

    try {
      const result = await multimediaService.generateIllustration({
        content: customDescription,
        characterIds: selectedCharacters.length > 0 ? selectedCharacters : undefined,
        options
      });
      setIllustration(result);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(`生成失败: ${errorMessage}`);
    } finally {
      setLoading(false);
    }
  };

  const handleCopyPrompt = async () => {
    if (!illustration?.prompt) return;
    try {
      await navigator.clipboard.writeText(illustration.prompt);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch (err) {
      console.error('Failed to copy:', err);
    }
  };

  const handleDownload = () => {
    if (!illustration?.imageData) return;
    const a = document.createElement('a');
    a.href = illustration.imageData;
    a.download = `插画_${illustration.title}_${new Date().toISOString().slice(0, 10)}.png`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
  };

  const handleRegenerate = () => {
    handleGenerate();
  };

  const toggleCharacter = (charId: string) => {
    setSelectedCharacters((prev) =>
      prev.includes(charId)
        ? prev.filter((id) => id !== charId)
        : [...prev, charId]
    );
  };

  const aspectRatioOptions = [
    { value: '1:1', label: '1:1 (正方形)', icon: '⬜' },
    { value: '16:9', label: '16:9 (横版)', icon: '▬' },
    { value: '9:16', label: '9:16 (竖版)', icon: '▮' },
    { value: '4:3', label: '4:3 (传统)', icon: '▭' },
    { value: '3:4', label: '3:4 (肖像)', icon: '▯' },
    { value: '21:9', label: '21:9 (超宽)', icon: '▬▬' },
  ];

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
      <div className="w-full max-w-4xl bg-white dark:bg-slate-800 rounded-lg shadow-xl max-h-[90vh] flex flex-col">
        <div className="flex items-center justify-between p-4 border-b border-slate-200 dark:border-slate-700">
          <div className="flex items-center gap-2">
            <Palette className="w-5 h-5 text-pink-500" />
            <h3 className="font-semibold text-slate-900 dark:text-slate-100">AI 插画生成</h3>
          </div>
          <button
            onClick={onClose}
            className="p-1 hover:bg-slate-100 dark:hover:bg-slate-700 rounded"
          >
            <X className="w-5 h-5 text-slate-500" />
          </button>
        </div>

        <div className="p-4 border-b border-slate-200 dark:border-slate-700">
          <div className="mb-4">
            <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">
              场景描述
            </label>
            <textarea
              value={customDescription}
              onChange={(e) => setCustomDescription(e.target.value)}
              placeholder="描述您想要生成的插画场景，例如：一个穿着白色长裙的少女站在樱花树下，夕阳西下..."
              className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-pink-500 min-h-[100px] resize-y"
            />
          </div>

          {characters.length > 0 && (
            <div className="mb-4">
              <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                包含角色 (可选)
              </label>
              <div className="flex flex-wrap gap-2">
                {characters.map((char) => (
                  <button
                    key={char.id}
                    onClick={() => toggleCharacter(char.id)}
                    className={`px-3 py-1.5 text-sm rounded-full transition-colors ${
                      selectedCharacters.includes(char.id)
                        ? 'bg-pink-500 text-white'
                        : 'bg-slate-100 dark:bg-slate-700 text-slate-600 dark:text-slate-300 hover:bg-slate-200 dark:hover:bg-slate-600'
                    }`}
                  >
                    {char.name}
                  </button>
                ))}
              </div>
            </div>
          )}

          <div className="flex items-center gap-4 mb-4">
            <div className="flex-1">
              <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">
                视觉风格
              </label>
              <select
                value={options.style}
                onChange={(e) => setOptions({ ...options, style: e.target.value as VisualStyle })}
                className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-pink-500"
              >
                {Object.entries(VISUAL_STYLE_LABELS).map(([key, label]) => (
                  <option key={key} value={key}>{label}</option>
                ))}
              </select>
            </div>
            <div className="flex-1">
              <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">
                画面比例
              </label>
              <select
                value={options.aspectRatio}
                onChange={(e) => setOptions({ ...options, aspectRatio: e.target.value as IllustrationOptions['aspectRatio'] })}
                className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-pink-500"
              >
                {aspectRatioOptions.map((opt) => (
                  <option key={opt.value} value={opt.value}>
                    {opt.icon} {opt.label}
                  </option>
                ))}
              </select>
            </div>
          </div>

          <button
            onClick={() => setShowAdvanced(!showAdvanced)}
            className="text-sm text-slate-600 dark:text-slate-400 hover:text-slate-800 dark:hover:text-slate-200 mb-2"
          >
            {showAdvanced ? '▼ 高级选项' : '▶ 高级选项'}
          </button>

          {showAdvanced && (
            <div className="p-3 bg-slate-50 dark:bg-slate-700/50 rounded-lg space-y-3 mb-4">
              <div className="flex items-center gap-4">
                <div className="flex-1">
                  <label className="block text-xs font-medium text-slate-600 dark:text-slate-400 mb-1">
                    图像质量
                  </label>
                  <select
                    value={options.quality}
                    onChange={(e) => setOptions({ ...options, quality: e.target.value as IllustrationOptions['quality'] })}
                    className="w-full px-2 py-1.5 text-sm border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100"
                  >
                    <option value="standard">标准</option>
                    <option value="high">高清</option>
                    <option value="ultra">超高清</option>
                  </select>
                </div>
                <div className="flex-1">
                  <label className="block text-xs font-medium text-slate-600 dark:text-slate-400 mb-1">
                    图像服务
                  </label>
                  <select
                    value={options.imageProvider || 'stability'}
                    onChange={(e) => setOptions({ ...options, imageProvider: e.target.value as IllustrationOptions['imageProvider'] })}
                    className="w-full px-2 py-1.5 text-sm border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100"
                  >
                    <option value="stability">Stability AI</option>
                    <option value="dalle">DALL-E</option>
                    <option value="comfyui">ComfyUI</option>
                    <option value="midjourney">Midjourney</option>
                  </select>
                </div>
              </div>
              <div>
                <label className="block text-xs font-medium text-slate-600 dark:text-slate-400 mb-1">
                  自定义提示词补充
                </label>
                <input
                  type="text"
                  value={options.customPrompt || ''}
                  onChange={(e) => setOptions({ ...options, customPrompt: e.target.value })}
                  placeholder="例如：cinematic lighting, detailed, 8k"
                  className="w-full px-2 py-1.5 text-sm border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100"
                />
              </div>
              <div>
                <label className="block text-xs font-medium text-slate-600 dark:text-slate-400 mb-1">
                  负面提示词 (避免出现的内容)
                </label>
                <input
                  type="text"
                  value={options.negativePrompt || ''}
                  onChange={(e) => setOptions({ ...options, negativePrompt: e.target.value })}
                  placeholder="例如：blurry, low quality, distorted"
                  className="w-full px-2 py-1.5 text-sm border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100"
                />
              </div>
              <div className="flex items-center gap-4">
                <label className="flex items-center gap-1.5 text-sm text-slate-600 dark:text-slate-400">
                  <input
                    type="checkbox"
                    checked={options.characterConsistency}
                    onChange={(e) => setOptions({ ...options, characterConsistency: e.target.checked })}
                    className="rounded border-slate-300"
                  />
                  角色一致性 (需要角色参考图)
                </label>
              </div>
            </div>
          )}

          <div className="flex items-center gap-3">
            <button
              onClick={handleGenerate}
              disabled={loading || !customDescription.trim()}
              className="flex items-center gap-2 px-4 py-2 bg-pink-500 hover:bg-pink-600 text-white rounded-lg font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {loading ? (
                <>
                  <Loader2 className="w-4 h-4 animate-spin" />
                  生成中...
                </>
              ) : (
                <>
                  <Sparkles className="w-4 h-4" />
                  生成插画
                </>
              )}
            </button>
            {illustration && (
              <button
                onClick={handleRegenerate}
                disabled={loading}
                className="flex items-center gap-2 px-4 py-2 bg-slate-100 dark:bg-slate-700 hover:bg-slate-200 dark:hover:bg-slate-600 text-slate-700 dark:text-slate-300 rounded-lg font-medium transition-colors"
              >
                <RefreshCw className="w-4 h-4" />
                重新生成
              </button>
            )}
          </div>

          {error && (
            <div className="mt-3 p-3 bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400 rounded-lg text-sm">
              {error}
            </div>
          )}
        </div>

        <div className="flex-1 overflow-y-auto p-4">
          {!illustration && !loading && (
            <div className="flex flex-col items-center justify-center h-64 text-slate-400">
              <ImageIcon className="w-12 h-12 mb-4" />
              <p className="text-sm">输入场景描述后点击"生成插画"</p>
              <p className="text-xs mt-1">AI 将根据您的描述生成精美的插画</p>
            </div>
          )}

          {loading && (
            <div className="flex flex-col items-center justify-center h-64 text-slate-400">
              <Loader2 className="w-12 h-12 animate-spin mb-4" />
              <p className="text-sm">正在生成插画...</p>
              <p className="text-xs mt-1">AI 正在创作中，请稍候</p>
            </div>
          )}

          {illustration && (
            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <h4 className="font-medium text-slate-900 dark:text-slate-100">
                  {illustration.title}
                </h4>
                {illustration.imageData && (
                  <button
                    onClick={handleDownload}
                    className="flex items-center gap-1 px-3 py-1.5 text-sm bg-pink-100 dark:bg-pink-900/30 text-pink-600 dark:text-pink-400 rounded-lg hover:bg-pink-200 dark:hover:bg-pink-900/50 transition-colors"
                  >
                    <Download className="w-4 h-4" />
                    下载图片
                  </button>
                )}
              </div>

              {illustration.imageData ? (
                <div className="relative bg-slate-100 dark:bg-slate-700 rounded-lg overflow-hidden">
                  <img
                    src={illustration.imageData}
                    alt={illustration.title}
                    className="w-full h-auto max-h-[400px] object-contain"
                  />
                </div>
              ) : (
                <div className="bg-slate-100 dark:bg-slate-700 rounded-lg p-8 flex flex-col items-center justify-center h-64">
                  <ImageIcon className="w-12 h-12 text-slate-400 mb-2" />
                  <p className="text-sm text-slate-500 dark:text-slate-400">
                    图像未生成 (需要配置图像服务)
                  </p>
                </div>
              )}

              <div className="p-3 bg-slate-50 dark:bg-slate-700/50 rounded-lg">
                <div className="flex items-center justify-between mb-2">
                  <span className="text-sm font-medium text-slate-700 dark:text-slate-300">
                    生成提示词
                  </span>
                  <button
                    onClick={handleCopyPrompt}
                    className="flex items-center gap-1 px-2 py-1 text-xs bg-slate-200 dark:bg-slate-600 hover:bg-slate-300 dark:hover:bg-slate-500 rounded transition-colors"
                  >
                    {copied ? (
                      <>
                        <Check className="w-3 h-3 text-green-500" />
                        已复制
                      </>
                    ) : (
                      <>
                        <Copy className="w-3 h-3" />
                        复制
                      </>
                    )}
                  </button>
                </div>
                <p className="text-sm text-slate-600 dark:text-slate-400 font-mono">
                  {illustration.prompt}
                </p>
              </div>

              {illustration.negativePrompt && (
                <div className="p-3 bg-red-50 dark:bg-red-900/20 rounded-lg">
                  <span className="text-sm font-medium text-red-700 dark:text-red-400">
                    负面提示词
                  </span>
                  <p className="text-sm text-red-600 dark:text-red-400 font-mono mt-1">
                    {illustration.negativePrompt}
                  </p>
                </div>
              )}

              <div className="grid grid-cols-3 gap-4 text-sm">
                <div className="p-2 bg-slate-50 dark:bg-slate-700/50 rounded">
                  <span className="text-slate-500 dark:text-slate-400">风格: </span>
                  <span className="text-slate-700 dark:text-slate-300">
                    {VISUAL_STYLE_LABELS[illustration.style as VisualStyle] || illustration.style}
                  </span>
                </div>
                <div className="p-2 bg-slate-50 dark:bg-slate-700/50 rounded">
                  <span className="text-slate-500 dark:text-slate-400">比例: </span>
                  <span className="text-slate-700 dark:text-slate-300">{illustration.aspectRatio}</span>
                </div>
                <div className="p-2 bg-slate-50 dark:bg-slate-700/50 rounded">
                  <span className="text-slate-500 dark:text-slate-400">生成时间: </span>
                  <span className="text-slate-700 dark:text-slate-300">
                    {new Date(illustration.metadata.generatedAt).toLocaleString()}
                  </span>
                </div>
              </div>
            </div>
          )}
        </div>

        <div className="p-4 border-t border-slate-200 dark:border-slate-700 flex justify-end">
          <button
            onClick={onClose}
            className="px-4 py-2 text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-700 rounded-lg font-medium transition-colors"
          >
            关闭
          </button>
        </div>
      </div>
    </div>
  );
}
