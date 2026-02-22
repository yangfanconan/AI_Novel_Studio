import React, { useState, useEffect } from 'react';
import { X, Sparkles, Loader2, Check, RefreshCw } from 'lucide-react';
import { aiGeneratorService } from '../services/api';
import type {
  GeneratedCharacter,
  GeneratedWorldView,
  GeneratedPlotPoint,
  GeneratedRelation,
} from '../types';

type GenerateType = 'character' | 'worldview' | 'plotpoint' | 'relation';

interface AIGenerateDialogProps {
  isOpen: boolean;
  onClose: () => void;
  type: GenerateType;
  projectId: string;
  onConfirm: (data: any) => Promise<void>;
  existingCharacters?: { id: string; name: string }[];
}

const WORLDVIEW_CATEGORIES = [
  { id: 'geography', name: '地理环境', icon: '🌍' },
  { id: 'history', name: '历史背景', icon: '📜' },
  { id: 'culture', name: '文化风俗', icon: '🎭' },
  { id: 'politics', name: '政治制度', icon: '🏛️' },
  { id: 'economy', name: '经济体系', icon: '💰' },
  { id: 'magic', name: '魔法/科技', icon: '✨' },
  { id: 'religion', name: '宗教信仰', icon: '🕍' },
  { id: 'races', name: '种族生物', icon: '👥' },
  { id: 'other', name: '其他', icon: '📝' },
];

const CHARACTER_TYPES = [
  { id: 'protagonist', name: '主角' },
  { id: 'antagonist', name: '反派' },
  { id: 'supporting', name: '配角' },
  { id: 'minor', name: '次要角色' },
];

export function AIGenerateDialog({
  isOpen,
  onClose,
  type,
  projectId,
  onConfirm,
  existingCharacters = [],
}: AIGenerateDialogProps) {
  const [loading, setLoading] = useState(false);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  
  // 生成参数
  const [characterType, setCharacterType] = useState('supporting');
  const [description, setDescription] = useState('');
  const [worldViewCategory, setWorldViewCategory] = useState('geography');
  const [plotContext, setPlotContext] = useState('');
  const [plotDirection, setPlotDirection] = useState('');
  
  // 生成结果
  const [generatedCharacter, setGeneratedCharacter] = useState<GeneratedCharacter | null>(null);
  const [generatedWorldView, setGeneratedWorldView] = useState<GeneratedWorldView | null>(null);
  const [generatedPlotPoints, setGeneratedPlotPoints] = useState<GeneratedPlotPoint[]>([]);
  const [generatedRelations, setGeneratedRelations] = useState<GeneratedRelation[]>([]);
  
  // 编辑状态
  const [editingCharacter, setEditingCharacter] = useState<GeneratedCharacter | null>(null);
  const [editingWorldView, setEditingWorldView] = useState<GeneratedWorldView | null>(null);
  const [editingPlotIndex, setEditingPlotIndex] = useState<number | null>(null);
  const [editingPlotPoint, setEditingPlotPoint] = useState<GeneratedPlotPoint | null>(null);
  
  // 选中的情节点
  const [selectedPlotIndices, setSelectedPlotIndices] = useState<Set<number>>(new Set());

  useEffect(() => {
    if (isOpen) {
      // 重置状态
      setLoading(false);
      setSaving(false);
      setError(null);
      setDescription('');
      setPlotContext('');
      setPlotDirection('');
      setGeneratedCharacter(null);
      setGeneratedWorldView(null);
      setGeneratedPlotPoints([]);
      setGeneratedRelations([]);
      setEditingCharacter(null);
      setEditingWorldView(null);
      setEditingPlotIndex(null);
      setEditingPlotPoint(null);
      setSelectedPlotIndices(new Set());
    }
  }, [isOpen, type]);

  const handleGenerate = async () => {
    setLoading(true);
    setError(null);
    
    try {
      switch (type) {
        case 'character':
          const charResult = await aiGeneratorService.generateCharacter(projectId, {
            type: characterType,
            description: description || undefined,
          });
          setGeneratedCharacter(charResult);
          setEditingCharacter(charResult);
          break;
          
        case 'worldview':
          const worldResult = await aiGeneratorService.generateWorldView(projectId, worldViewCategory);
          setGeneratedWorldView(worldResult);
          setEditingWorldView(worldResult);
          break;
          
        case 'plotpoint':
          const plotResult = await aiGeneratorService.generatePlotPoints(
            projectId,
            plotContext || undefined,
            plotDirection || undefined
          );
          setGeneratedPlotPoints(plotResult);
          setSelectedPlotIndices(new Set(plotResult.map((_, i) => i)));
          break;
          
        case 'relation':
          const relationResult = await aiGeneratorService.generateCharacterRelations(projectId);
          setGeneratedRelations(relationResult);
          break;
      }
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(`生成失败: ${errorMessage}`);
    } finally {
      setLoading(false);
    }
  };

  const handleConfirm = async () => {
    setSaving(true);
    setError(null);
    
    try {
      switch (type) {
        case 'character':
          if (editingCharacter) {
            await onConfirm({
              name: editingCharacter.name,
              role_type: editingCharacter.role_type,
              race: editingCharacter.race,
              age: editingCharacter.age,
              gender: editingCharacter.gender,
              birth_date: editingCharacter.birth_date,
              appearance: editingCharacter.appearance,
              personality: editingCharacter.personality,
              background: editingCharacter.background,
              mbti: editingCharacter.mbti,
              enneagram: editingCharacter.enneagram,
              bazi: editingCharacter.bazi,
              ziwei: editingCharacter.ziwei,
              skills: editingCharacter.skills,
              status: editingCharacter.status,
              items: editingCharacter.items,
            });
          }
          break;
          
        case 'worldview':
          if (editingWorldView) {
            await onConfirm({
              category: editingWorldView.category,
              title: editingWorldView.title,
              content: editingWorldView.content,
              tags: editingWorldView.tags?.join(', '),
            });
          }
          break;
          
        case 'plotpoint':
          const selectedPoints = generatedPlotPoints.filter((_, i) => selectedPlotIndices.has(i));
          for (const point of selectedPoints) {
            await onConfirm({
              title: point.title,
              description: point.description,
              note: point.note,
            });
          }
          break;
          
        case 'relation':
          for (const relation of generatedRelations) {
            await onConfirm(relation);
          }
          break;
      }
      onClose();
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(`保存失败: ${errorMessage}`);
    } finally {
      setSaving(false);
    }
  };

  const togglePlotSelection = (index: number) => {
    setSelectedPlotIndices(prev => {
      const newSet = new Set(prev);
      if (newSet.has(index)) {
        newSet.delete(index);
      } else {
        newSet.add(index);
      }
      return newSet;
    });
  };

  const handleEditPlotPoint = (index: number) => {
    setEditingPlotIndex(index);
    setEditingPlotPoint({ ...generatedPlotPoints[index] });
  };

  const handleSavePlotPoint = () => {
    if (editingPlotIndex !== null && editingPlotPoint) {
      const newPoints = [...generatedPlotPoints];
      newPoints[editingPlotIndex] = editingPlotPoint;
      setGeneratedPlotPoints(newPoints);
      setEditingPlotIndex(null);
      setEditingPlotPoint(null);
    }
  };

  if (!isOpen) return null;

  const getTitle = () => {
    switch (type) {
      case 'character': return 'AI 生成角色';
      case 'worldview': return 'AI 生成世界观';
      case 'plotpoint': return 'AI 生成情节点';
      case 'relation': return 'AI 生成关系';
    }
  };

  const renderGenerateForm = () => {
    switch (type) {
      case 'character':
        return (
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                角色类型
              </label>
              <div className="grid grid-cols-2 gap-2">
                {CHARACTER_TYPES.map((t) => (
                  <button
                    key={t.id}
                    type="button"
                    onClick={() => setCharacterType(t.id)}
                    className={`p-2 rounded-lg text-sm transition-colors ${
                      characterType === t.id
                        ? 'bg-blue-500 text-white'
                        : 'bg-slate-100 dark:bg-slate-700 hover:bg-slate-200 dark:hover:bg-slate-600'
                    }`}
                  >
                    {t.name}
                  </button>
                ))}
              </div>
            </div>
            <div>
              <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                额外描述（可选）
              </label>
              <textarea
                value={description}
                onChange={(e) => setDescription(e.target.value)}
                rows={3}
                className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-blue-500 resize-none"
                placeholder="例如：一个神秘的老人，喜欢在森林中散步..."
              />
            </div>
          </div>
        );
        
      case 'worldview':
        return (
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                选择类别
              </label>
              <div className="grid grid-cols-3 gap-2">
                {WORLDVIEW_CATEGORIES.map((cat) => (
                  <button
                    key={cat.id}
                    type="button"
                    onClick={() => setWorldViewCategory(cat.id)}
                    className={`p-2 rounded-lg text-sm transition-colors flex items-center justify-center gap-1 ${
                      worldViewCategory === cat.id
                        ? 'bg-blue-500 text-white'
                        : 'bg-slate-100 dark:bg-slate-700 hover:bg-slate-200 dark:hover:bg-slate-600'
                    }`}
                  >
                    <span>{cat.icon}</span>
                    <span>{cat.name}</span>
                  </button>
                ))}
              </div>
            </div>
          </div>
        );
        
      case 'plotpoint':
        return (
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                上下文背景（可选）
              </label>
              <textarea
                value={plotContext}
                onChange={(e) => setPlotContext(e.target.value)}
                rows={3}
                className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-blue-500 resize-none"
                placeholder="描述当前故事背景..."
              />
            </div>
            <div>
              <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                剧情方向（可选）
              </label>
              <textarea
                value={plotDirection}
                onChange={(e) => setPlotDirection(e.target.value)}
                rows={2}
                className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-blue-500 resize-none"
                placeholder="例如：需要增加冲突、发展感情线..."
              />
            </div>
          </div>
        );
        
      case 'relation':
        return (
          <div className="text-center py-4">
            <p className="text-sm text-slate-600 dark:text-slate-400">
              将根据项目中已有的角色自动生成关系建议
            </p>
            {existingCharacters.length < 2 && (
              <p className="text-sm text-amber-500 mt-2">
                需要至少 2 个角色才能生成关系
              </p>
            )}
          </div>
        );
    }
  };

  const renderResultPreview = () => {
    switch (type) {
      case 'character':
        if (!editingCharacter) return null;
        return (
          <div className="space-y-4">
            <h4 className="font-medium text-slate-900 dark:text-slate-100">生成结果预览</h4>
            <div className="space-y-3 p-4 bg-slate-50 dark:bg-slate-800 rounded-lg max-h-96 overflow-y-auto">
              <div>
                <label className="block text-xs text-slate-500 mb-1">姓名 *</label>
                <input
                  type="text"
                  value={editingCharacter.name}
                  onChange={(e) => setEditingCharacter({ ...editingCharacter, name: e.target.value })}
                  className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100"
                />
              </div>
              <div className="grid grid-cols-3 gap-3">
                <div>
                  <label className="block text-xs text-slate-500 mb-1">身份</label>
                  <select
                    value={editingCharacter.role_type || ''}
                    onChange={(e) => setEditingCharacter({ ...editingCharacter, role_type: e.target.value || undefined })}
                    className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100"
                  >
                    <option value="">未设置</option>
                    <option value="protagonist">主角</option>
                    <option value="deuteragonist">第二主角</option>
                    <option value="antagonist">反派</option>
                    <option value="supporting">配角</option>
                    <option value="minor">小角色</option>
                  </select>
                </div>
                <div>
                  <label className="block text-xs text-slate-500 mb-1">种族</label>
                  <input
                    type="text"
                    value={editingCharacter.race || ''}
                    onChange={(e) => setEditingCharacter({ ...editingCharacter, race: e.target.value || undefined })}
                    className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100"
                  />
                </div>
                <div>
                  <label className="block text-xs text-slate-500 mb-1">年龄</label>
                  <input
                    type="number"
                    value={editingCharacter.age || ''}
                    onChange={(e) => setEditingCharacter({ ...editingCharacter, age: e.target.value ? parseInt(e.target.value) : undefined })}
                    className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100"
                  />
                </div>
              </div>
              <div className="grid grid-cols-2 gap-3">
                <div>
                  <label className="block text-xs text-slate-500 mb-1">性别</label>
                  <input
                    type="text"
                    value={editingCharacter.gender || ''}
                    onChange={(e) => setEditingCharacter({ ...editingCharacter, gender: e.target.value })}
                    className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100"
                  />
                </div>
                <div>
                  <label className="block text-xs text-slate-500 mb-1">出生日期</label>
                  <input
                    type="text"
                    value={editingCharacter.birth_date || ''}
                    onChange={(e) => setEditingCharacter({ ...editingCharacter, birth_date: e.target.value || undefined })}
                    className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100"
                    placeholder="如：龙历三千年三月初三"
                  />
                </div>
              </div>
              <div>
                <label className="block text-xs text-slate-500 mb-1">外貌</label>
                <textarea
                  value={editingCharacter.appearance || ''}
                  onChange={(e) => setEditingCharacter({ ...editingCharacter, appearance: e.target.value })}
                  rows={2}
                  className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100 resize-none"
                />
              </div>
              <div>
                <label className="block text-xs text-slate-500 mb-1">性格</label>
                <textarea
                  value={editingCharacter.personality || ''}
                  onChange={(e) => setEditingCharacter({ ...editingCharacter, personality: e.target.value })}
                  rows={2}
                  className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100 resize-none"
                />
              </div>
              <div>
                <label className="block text-xs text-slate-500 mb-1">背景</label>
                <textarea
                  value={editingCharacter.background || ''}
                  onChange={(e) => setEditingCharacter({ ...editingCharacter, background: e.target.value })}
                  rows={3}
                  className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100 resize-none"
                />
              </div>
              <div className="grid grid-cols-2 gap-3">
                <div>
                  <label className="block text-xs text-slate-500 mb-1">MBTI</label>
                  <input
                    type="text"
                    value={editingCharacter.mbti || ''}
                    onChange={(e) => setEditingCharacter({ ...editingCharacter, mbti: e.target.value || undefined })}
                    className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100"
                    placeholder="如：INTJ"
                  />
                </div>
                <div>
                  <label className="block text-xs text-slate-500 mb-1">九型人格</label>
                  <input
                    type="text"
                    value={editingCharacter.enneagram || ''}
                    onChange={(e) => setEditingCharacter({ ...editingCharacter, enneagram: e.target.value || undefined })}
                    className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100"
                    placeholder="如：3号-成就型"
                  />
                </div>
              </div>
              <div className="grid grid-cols-2 gap-3">
                <div>
                  <label className="block text-xs text-slate-500 mb-1">八字</label>
                  <input
                    type="text"
                    value={editingCharacter.bazi || ''}
                    onChange={(e) => setEditingCharacter({ ...editingCharacter, bazi: e.target.value || undefined })}
                    className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100"
                  />
                </div>
                <div>
                  <label className="block text-xs text-slate-500 mb-1">紫微斗数</label>
                  <input
                    type="text"
                    value={editingCharacter.ziwei || ''}
                    onChange={(e) => setEditingCharacter({ ...editingCharacter, ziwei: e.target.value || undefined })}
                    className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100"
                  />
                </div>
              </div>
              <div>
                <label className="block text-xs text-slate-500 mb-1">技能</label>
                <textarea
                  value={editingCharacter.skills || ''}
                  onChange={(e) => setEditingCharacter({ ...editingCharacter, skills: e.target.value || undefined })}
                  rows={2}
                  className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100 resize-none"
                  placeholder="用顿号分隔多个技能"
                />
              </div>
              <div>
                <label className="block text-xs text-slate-500 mb-1">当前状态</label>
                <textarea
                  value={editingCharacter.status || ''}
                  onChange={(e) => setEditingCharacter({ ...editingCharacter, status: e.target.value || undefined })}
                  rows={2}
                  className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100 resize-none"
                />
              </div>
              <div>
                <label className="block text-xs text-slate-500 mb-1">随身物品</label>
                <textarea
                  value={editingCharacter.items || ''}
                  onChange={(e) => setEditingCharacter({ ...editingCharacter, items: e.target.value || undefined })}
                  rows={2}
                  className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100 resize-none"
                  placeholder="用顿号分隔多个物品"
                />
              </div>
            </div>
          </div>
        );
        
      case 'worldview':
        if (!editingWorldView) return null;
        return (
          <div className="space-y-4">
            <h4 className="font-medium text-slate-900 dark:text-slate-100">生成结果预览</h4>
            <div className="space-y-3 p-4 bg-slate-50 dark:bg-slate-800 rounded-lg">
              <div>
                <label className="block text-xs text-slate-500 mb-1">标题</label>
                <input
                  type="text"
                  value={editingWorldView.title}
                  onChange={(e) => setEditingWorldView({ ...editingWorldView, title: e.target.value })}
                  className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100"
                />
              </div>
              <div>
                <label className="block text-xs text-slate-500 mb-1">内容</label>
                <textarea
                  value={editingWorldView.content}
                  onChange={(e) => setEditingWorldView({ ...editingWorldView, content: e.target.value })}
                  rows={6}
                  className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100 resize-none"
                />
              </div>
              <div>
                <label className="block text-xs text-slate-500 mb-1">标签</label>
                <input
                  type="text"
                  value={editingWorldView.tags?.join(', ') || ''}
                  onChange={(e) => setEditingWorldView({ ...editingWorldView, tags: e.target.value.split(',').map(t => t.trim()).filter(Boolean) })}
                  className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100"
                  placeholder="用逗号分隔多个标签"
                />
              </div>
            </div>
          </div>
        );
        
      case 'plotpoint':
        if (generatedPlotPoints.length === 0) return null;
        return (
          <div className="space-y-4">
            <h4 className="font-medium text-slate-900 dark:text-slate-100">
              生成结果预览 
              <span className="text-sm font-normal text-slate-500 ml-2">
                (已选择 {selectedPlotIndices.size}/{generatedPlotPoints.length} 个)
              </span>
            </h4>
            <div className="space-y-2 max-h-64 overflow-y-auto">
              {generatedPlotPoints.map((point, index) => (
                <div
                  key={index}
                  className={`p-3 border rounded-lg cursor-pointer transition-colors ${
                    selectedPlotIndices.has(index)
                      ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/20'
                      : 'border-slate-200 dark:border-slate-700 hover:border-slate-300 dark:hover:border-slate-600'
                  }`}
                  onClick={() => togglePlotSelection(index)}
                >
                  {editingPlotIndex === index ? (
                    <div className="space-y-2" onClick={(e) => e.stopPropagation()}>
                      <input
                        type="text"
                        value={editingPlotPoint?.title || ''}
                        onChange={(e) => setEditingPlotPoint({ ...editingPlotPoint!, title: e.target.value })}
                        className="w-full px-2 py-1 border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700 text-sm"
                      />
                      <textarea
                        value={editingPlotPoint?.description || ''}
                        onChange={(e) => setEditingPlotPoint({ ...editingPlotPoint!, description: e.target.value })}
                        rows={2}
                        className="w-full px-2 py-1 border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700 text-sm resize-none"
                      />
                      <div className="flex gap-2">
                        <button
                          onClick={handleSavePlotPoint}
                          className="px-2 py-1 bg-blue-500 text-white rounded text-xs"
                        >
                          保存
                        </button>
                        <button
                          onClick={() => { setEditingPlotIndex(null); setEditingPlotPoint(null); }}
                          className="px-2 py-1 bg-slate-200 dark:bg-slate-600 rounded text-xs"
                        >
                          取消
                        </button>
                      </div>
                    </div>
                  ) : (
                    <div className="flex items-start gap-2">
                      <div className={`w-5 h-5 rounded border flex items-center justify-center flex-shrink-0 mt-0.5 ${
                        selectedPlotIndices.has(index)
                          ? 'bg-blue-500 border-blue-500 text-white'
                          : 'border-slate-300 dark:border-slate-600'
                      }`}>
                        {selectedPlotIndices.has(index) && <Check className="w-3 h-3" />}
                      </div>
                      <div className="flex-1 min-w-0">
                        <div className="flex items-center gap-2">
                          <span className="font-medium text-sm text-slate-900 dark:text-slate-100">
                            {point.title}
                          </span>
                          {point.priority && (
                            <span className={`px-1.5 py-0.5 rounded text-xs ${
                              point.priority === 'high' ? 'bg-red-100 text-red-600' :
                              point.priority === 'medium' ? 'bg-yellow-100 text-yellow-600' :
                              'bg-slate-100 text-slate-600'
                            }`}>
                              {point.priority === 'high' ? '高' : point.priority === 'medium' ? '中' : '低'}
                            </span>
                          )}
                        </div>
                        {point.description && (
                          <p className="text-xs text-slate-500 dark:text-slate-400 mt-1 line-clamp-2">
                            {point.description}
                          </p>
                        )}
                      </div>
                      <button
                        onClick={(e) => { e.stopPropagation(); handleEditPlotPoint(index); }}
                        className="p-1 hover:bg-slate-200 dark:hover:bg-slate-600 rounded"
                      >
                        <svg className="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                          <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7" />
                          <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z" />
                        </svg>
                      </button>
                    </div>
                  )}
                </div>
              ))}
            </div>
          </div>
        );
        
      case 'relation':
        if (generatedRelations.length === 0) return null;
        return (
          <div className="space-y-4">
            <h4 className="font-medium text-slate-900 dark:text-slate-100">生成的关系建议</h4>
            <div className="space-y-2 max-h-64 overflow-y-auto">
              {generatedRelations.map((relation, index) => (
                <div
                  key={index}
                  className="p-3 border border-slate-200 dark:border-slate-700 rounded-lg bg-slate-50 dark:bg-slate-800"
                >
                  <div className="flex items-center gap-2">
                    <span className="font-medium text-sm text-slate-900 dark:text-slate-100">
                      {relation.from_character_name}
                    </span>
                    <span className="text-blue-500">→</span>
                    <span className="font-medium text-sm text-slate-900 dark:text-slate-100">
                      {relation.to_character_name}
                    </span>
                  </div>
                  <div className="mt-1">
                    <span className="px-2 py-0.5 bg-blue-100 dark:bg-blue-900/30 text-blue-600 dark:text-blue-400 rounded text-xs">
                      {relation.relation_type}
                    </span>
                  </div>
                  {relation.description && (
                    <p className="text-xs text-slate-500 dark:text-slate-400 mt-2">
                      {relation.description}
                    </p>
                  )}
                </div>
              ))}
            </div>
          </div>
        );
    }
  };

  const hasResult = () => {
    switch (type) {
      case 'character': return !!generatedCharacter;
      case 'worldview': return !!generatedWorldView;
      case 'plotpoint': return generatedPlotPoints.length > 0;
      case 'relation': return generatedRelations.length > 0;
    }
  };

  const canConfirm = () => {
    switch (type) {
      case 'character': return !!editingCharacter?.name;
      case 'worldview': return !!editingWorldView?.title && !!editingWorldView?.content;
      case 'plotpoint': return selectedPlotIndices.size > 0;
      case 'relation': return generatedRelations.length > 0;
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
      <div className="w-full max-w-lg bg-white dark:bg-slate-800 rounded-lg shadow-xl">
        <div className="flex items-center justify-between p-4 border-b border-slate-200 dark:border-slate-700">
          <div className="flex items-center gap-2">
            <Sparkles className="w-5 h-5 text-blue-500" />
            <h3 className="font-semibold text-slate-900 dark:text-slate-100">{getTitle()}</h3>
          </div>
          <button
            onClick={onClose}
            className="p-1 hover:bg-slate-100 dark:hover:bg-slate-700 rounded"
          >
            <X className="w-5 h-5 text-slate-500" />
          </button>
        </div>

        <div className="p-4 space-y-4 max-h-[60vh] overflow-y-auto">
          {error && (
            <div className="p-3 bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400 rounded-lg text-sm">
              {error}
            </div>
          )}
          
          {!hasResult() && renderGenerateForm()}
          {hasResult() && renderResultPreview()}
        </div>

        <div className="flex items-center justify-between p-4 border-t border-slate-200 dark:border-slate-700">
          <div className="flex gap-2">
            {!hasResult() ? (
              <button
                onClick={handleGenerate}
                disabled={loading || (type === 'relation' && existingCharacters.length < 2)}
                className="flex items-center gap-2 px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded-lg font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {loading ? (
                  <>
                    <Loader2 className="w-4 h-4 animate-spin" />
                    生成中...
                  </>
                ) : (
                  <>
                    <Sparkles className="w-4 h-4" />
                    开始生成
                  </>
                )}
              </button>
            ) : (
              <button
                onClick={() => {
                  setGeneratedCharacter(null);
                  setGeneratedWorldView(null);
                  setGeneratedPlotPoints([]);
                  setGeneratedRelations([]);
                  setEditingCharacter(null);
                  setEditingWorldView(null);
                }}
                disabled={loading}
                className="flex items-center gap-2 px-4 py-2 bg-slate-100 dark:bg-slate-700 hover:bg-slate-200 dark:hover:bg-slate-600 rounded-lg font-medium transition-colors"
              >
                <RefreshCw className="w-4 h-4" />
                重新生成
              </button>
            )}
          </div>
          
          <div className="flex gap-2">
            <button
              onClick={onClose}
              className="px-4 py-2 text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-700 rounded-lg font-medium transition-colors"
            >
              取消
            </button>
            {hasResult() && (
              <button
                onClick={handleConfirm}
                disabled={saving || !canConfirm()}
                className="flex items-center gap-2 px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded-lg font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {saving ? (
                  <>
                    <Loader2 className="w-4 h-4 animate-spin" />
                    保存中...
                  </>
                ) : (
                  <>
                    <Check className="w-4 h-4" />
                    确认保存
                  </>
                )}
              </button>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
