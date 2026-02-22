import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import {
  KnowledgeEntry,
  CreateKnowledgeEntryRequest,
  UpdateKnowledgeEntryRequest,
  KnowledgeRelation,
  KnowledgeSearchResult,
} from '../types';
import { Search, Plus, Trash2, Edit, Link, Database, Tag, Star, Check } from 'lucide-react';

interface KnowledgeBaseProps {
  projectId: string;
}

const ENTRY_TYPES = [
  { id: 'character', name: '角色', icon: '👤' },
  { id: 'worldview', name: '世界观', icon: '🌍' },
  { id: 'plot', name: '剧情', icon: '📖' },
  { id: 'item', name: '物品', icon: '🎁' },
  { id: 'location', name: '地点', icon: '📍' },
  { id: 'event', name: '事件', icon: '📅' },
  { id: 'concept', name: '概念', icon: '💡' },
  { id: 'other', name: '其他', icon: '📝' },
];

export function KnowledgeBase({ projectId }: KnowledgeBaseProps) {
  const [entries, setEntries] = useState<KnowledgeEntry[]>([]);
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedType, setSelectedType] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [showEditor, setShowEditor] = useState(false);
  const [editingEntry, setEditingEntry] = useState<KnowledgeEntry | null>(null);
  const [selectedEntry, setSelectedEntry] = useState<KnowledgeEntry | null>(null);
  const [entryRelations, setEntryRelations] = useState<KnowledgeRelation[]>([]);

  const [formData, setFormData] = useState({
    entry_type: 'other',
    title: '',
    content: '',
    keywords: '',
    importance: 0,
  });

  useEffect(() => {
    loadEntries();
  }, [projectId]);

  const loadEntries = async () => {
    setLoading(true);
    try {
      const result = await invoke<KnowledgeEntry[]>('get_knowledge_entries', {
        projectId,
      });
      setEntries(result);
    } catch (error) {
      console.error('Failed to load knowledge entries:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleSearch = async () => {
    if (!searchQuery.trim()) {
      loadEntries();
      return;
    }

    setLoading(true);
    try {
      const result = await invoke<KnowledgeSearchResult[]>('search_knowledge', {
        request: {
          project_id: projectId,
          query: searchQuery,
          entry_types: selectedType ? [selectedType] : null,
          limit: 50,
        },
      });
      setEntries(result.map((r) => r.entry));
    } catch (error) {
      console.error('Failed to search knowledge:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleFilterByType = async (type: string | null) => {
    setSelectedType(type);
    if (type) {
      setLoading(true);
      try {
        const result = await invoke<KnowledgeEntry[]>('get_knowledge_entries_by_type', {
          projectId,
          entryType: type,
        });
        setEntries(result);
      } catch (error) {
        console.error('Failed to filter entries:', error);
      } finally {
        setLoading(false);
      }
    } else {
      loadEntries();
    }
  };

  const handleCreateEntry = async () => {
    if (!formData.title.trim() || !formData.content.trim()) return;

    setLoading(true);
    try {
      const request: CreateKnowledgeEntryRequest = {
        project_id: projectId,
        entry_type: formData.entry_type,
        title: formData.title,
        content: formData.content,
        keywords: formData.keywords || undefined,
        importance: formData.importance,
      };

      await invoke('create_knowledge_entry', { request });
      resetForm();
      loadEntries();
    } catch (error) {
      console.error('Failed to create entry:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleUpdateEntry = async () => {
    if (!editingEntry || !formData.title.trim() || !formData.content.trim()) return;

    setLoading(true);
    try {
      const request: UpdateKnowledgeEntryRequest = {
        id: editingEntry.id,
        entry_type: formData.entry_type,
        title: formData.title,
        content: formData.content,
        keywords: formData.keywords || undefined,
        importance: formData.importance,
      };

      await invoke('update_knowledge_entry', { request });
      resetForm();
      loadEntries();
    } catch (error) {
      console.error('Failed to update entry:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleDeleteEntry = async (entryId: string) => {
    if (!confirm('确定要删除这个知识条目吗？')) return;

    try {
      await invoke('delete_knowledge_entry', { entryId });
      if (selectedEntry?.id === entryId) {
        setSelectedEntry(null);
      }
      loadEntries();
    } catch (error) {
      console.error('Failed to delete entry:', error);
    }
  };

  const handleSyncCharacters = async () => {
    setLoading(true);
    try {
      const characters = await invoke<{ id: string }[]>('get_characters', { projectId });
      for (const char of characters) {
        await invoke('sync_character_to_knowledge', { characterId: char.id });
      }
      loadEntries();
    } catch (error) {
      console.error('Failed to sync characters:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleSyncWorldViews = async () => {
    setLoading(true);
    try {
      const worldviews = await invoke<{ id: string }[]>('get_world_views', { projectId });
      for (const wv of worldviews) {
        await invoke('sync_worldview_to_knowledge', { worldviewId: wv.id });
      }
      loadEntries();
    } catch (error) {
      console.error('Failed to sync worldviews:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleViewEntry = async (entry: KnowledgeEntry) => {
    setSelectedEntry(entry);
    try {
      const relations = await invoke<KnowledgeRelation[]>('get_knowledge_relations', {
        entryId: entry.id,
      });
      setEntryRelations(relations);
    } catch (error) {
      console.error('Failed to load relations:', error);
      setEntryRelations([]);
    }
  };

  const resetForm = () => {
    setShowEditor(false);
    setEditingEntry(null);
    setFormData({
      entry_type: 'other',
      title: '',
      content: '',
      keywords: '',
      importance: 0,
    });
  };

  const startEdit = (entry: KnowledgeEntry) => {
    setEditingEntry(entry);
    setFormData({
      entry_type: entry.entry_type,
      title: entry.title,
      content: entry.content,
      keywords: entry.keywords || '',
      importance: entry.importance,
    });
    setShowEditor(true);
  };

  const getEntryTypeInfo = (type: string) => {
    return ENTRY_TYPES.find((t) => t.id === type) || ENTRY_TYPES[ENTRY_TYPES.length - 1];
  };

  const getImportanceStars = (importance: number) => {
    return '⭐'.repeat(Math.min(importance, 5));
  };

  return (
    <div className="flex h-full">
      {/* 左侧列表 */}
      <div className="w-2/3 border-r border-gray-200 flex flex-col">
        {/* 工具栏 */}
        <div className="p-4 border-b border-gray-200 space-y-3">
          <div className="flex gap-2">
            <div className="flex-1 relative">
              <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-gray-400" />
              <input
                type="text"
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                onKeyDown={(e) => e.key === 'Enter' && handleSearch()}
                placeholder="搜索知识库..."
                className="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>
            <button
              onClick={handleSearch}
              className="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600"
            >
              搜索
            </button>
          </div>

          <div className="flex flex-wrap gap-2">
            <button
              onClick={() => handleFilterByType(null)}
              className={`px-3 py-1 rounded-full text-sm ${
                selectedType === null
                  ? 'bg-blue-500 text-white'
                  : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
              }`}
            >
              全部
            </button>
            {ENTRY_TYPES.map((type) => (
              <button
                key={type.id}
                onClick={() => handleFilterByType(type.id)}
                className={`px-3 py-1 rounded-full text-sm ${
                  selectedType === type.id
                    ? 'bg-blue-500 text-white'
                    : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                }`}
              >
                {type.icon} {type.name}
              </button>
            ))}
          </div>

          <div className="flex gap-2">
            <button
              onClick={() => {
                resetForm();
                setShowEditor(true);
              }}
              className="flex items-center gap-1 px-3 py-1.5 bg-green-500 text-white rounded-md hover:bg-green-600 text-sm"
            >
              <Plus className="w-4 h-4" /> 新建条目
            </button>
            <button
              onClick={handleSyncCharacters}
              disabled={loading}
              className="flex items-center gap-1 px-3 py-1.5 bg-purple-500 text-white rounded-md hover:bg-purple-600 text-sm disabled:opacity-50"
            >
              <Database className="w-4 h-4" /> 同步角色
            </button>
            <button
              onClick={handleSyncWorldViews}
              disabled={loading}
              className="flex items-center gap-1 px-3 py-1.5 bg-orange-500 text-white rounded-md hover:bg-orange-600 text-sm disabled:opacity-50"
            >
              <Database className="w-4 h-4" /> 同步世界观
            </button>
          </div>
        </div>

        {/* 条目列表 */}
        <div className="flex-1 overflow-y-auto p-4">
          {loading ? (
            <div className="text-center py-8 text-gray-500">加载中...</div>
          ) : entries.length === 0 ? (
            <div className="text-center py-8 text-gray-500">
              暂无知识条目，点击"新建条目"或同步现有数据
            </div>
          ) : (
            <div className="space-y-3">
              {entries.map((entry) => {
                const typeInfo = getEntryTypeInfo(entry.entry_type);
                return (
                  <div
                    key={entry.id}
                    onClick={() => handleViewEntry(entry)}
                    className={`p-4 border rounded-lg cursor-pointer transition-colors ${
                      selectedEntry?.id === entry.id
                        ? 'border-blue-500 bg-blue-50'
                        : 'border-gray-200 hover:border-gray-300 hover:bg-gray-50'
                    }`}
                  >
                    <div className="flex items-start justify-between">
                      <div className="flex items-start gap-3">
                        <span className="text-2xl">{typeInfo.icon}</span>
                        <div className="flex-1">
                          <div className="flex items-center gap-2">
                            <h3 className="font-medium text-gray-900">{entry.title}</h3>
                            {entry.is_verified && (
                              <Check className="w-4 h-4 text-green-500" />
                            )}
                            <span className="text-xs text-yellow-600">
                              {getImportanceStars(entry.importance)}
                            </span>
                          </div>
                          <p className="text-sm text-gray-600 mt-1 line-clamp-2">
                            {entry.content}
                          </p>
                          {entry.keywords && (
                            <div className="flex flex-wrap gap-1 mt-2">
                              {entry.keywords.split(',').map((keyword, idx) => (
                                <span
                                  key={idx}
                                  className="px-2 py-0.5 bg-gray-100 text-gray-600 text-xs rounded"
                                >
                                  {keyword.trim()}
                                </span>
                              ))}
                            </div>
                          )}
                        </div>
                      </div>
                      <div className="flex gap-1">
                        <button
                          onClick={(e) => {
                            e.stopPropagation();
                            startEdit(entry);
                          }}
                          className="p-1 text-gray-400 hover:text-blue-500"
                        >
                          <Edit className="w-4 h-4" />
                        </button>
                        <button
                          onClick={(e) => {
                            e.stopPropagation();
                            handleDeleteEntry(entry.id);
                          }}
                          className="p-1 text-gray-400 hover:text-red-500"
                        >
                          <Trash2 className="w-4 h-4" />
                        </button>
                      </div>
                    </div>
                  </div>
                );
              })}
            </div>
          )}
        </div>
      </div>

      {/* 右侧详情 */}
      <div className="w-1/3 flex flex-col bg-gray-50">
        {selectedEntry ? (
          <>
            <div className="p-4 border-b border-gray-200">
              <div className="flex items-center gap-2">
                <span className="text-2xl">{getEntryTypeInfo(selectedEntry.entry_type).icon}</span>
                <h2 className="text-lg font-semibold">{selectedEntry.title}</h2>
              </div>
              <div className="flex items-center gap-2 mt-2 text-sm text-gray-500">
                <span>
                  <Tag className="w-3 h-3 inline mr-1" />
                  {getEntryTypeInfo(selectedEntry.entry_type).name}
                </span>
                <span>
                  <Star className="w-3 h-3 inline mr-1" />
                  重要性: {selectedEntry.importance}
                </span>
                {selectedEntry.is_verified && (
                  <span className="text-green-500">
                    <Check className="w-3 h-3 inline mr-1" />
                    已验证
                  </span>
                )}
              </div>
            </div>

            <div className="flex-1 overflow-y-auto p-4">
              <div className="mb-4">
                <h3 className="text-sm font-medium text-gray-700 mb-2">内容</h3>
                <p className="text-sm text-gray-600 whitespace-pre-wrap">
                  {selectedEntry.content}
                </p>
              </div>

              {selectedEntry.keywords && (
                <div className="mb-4">
                  <h3 className="text-sm font-medium text-gray-700 mb-2">关键词</h3>
                  <div className="flex flex-wrap gap-1">
                    {selectedEntry.keywords.split(',').map((keyword, idx) => (
                      <span
                        key={idx}
                        className="px-2 py-1 bg-blue-100 text-blue-700 text-sm rounded"
                      >
                        {keyword.trim()}
                      </span>
                    ))}
                  </div>
                </div>
              )}

              {entryRelations.length > 0 && (
                <div className="mb-4">
                  <h3 className="text-sm font-medium text-gray-700 mb-2">
                    <Link className="w-3 h-3 inline mr-1" />
                    关联 ({entryRelations.length})
                  </h3>
                  <div className="space-y-2">
                    {entryRelations.map((rel) => (
                      <div
                        key={rel.id}
                        className="p-2 bg-white border border-gray-200 rounded text-sm"
                      >
                        <span className="text-gray-500">{rel.relation_type}</span>
                        {rel.description && (
                          <p className="text-gray-600 mt-1">{rel.description}</p>
                        )}
                      </div>
                    ))}
                  </div>
                </div>
              )}

              <div className="text-xs text-gray-400 mt-4">
                <p>创建: {new Date(selectedEntry.created_at).toLocaleString()}</p>
                <p>更新: {new Date(selectedEntry.updated_at).toLocaleString()}</p>
              </div>
            </div>
          </>
        ) : (
          <div className="flex-1 flex items-center justify-center text-gray-400">
            选择一个条目查看详情
          </div>
        )}
      </div>

      {/* 编辑对话框 */}
      {showEditor && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg shadow-lg w-full max-w-2xl p-6">
            <h2 className="text-lg font-semibold mb-4">
              {editingEntry ? '编辑知识条目' : '新建知识条目'}
            </h2>

            <div className="space-y-4">
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-sm font-medium mb-1">类型</label>
                  <select
                    value={formData.entry_type}
                    onChange={(e) => setFormData({ ...formData, entry_type: e.target.value })}
                    className="w-full px-3 py-2 border border-gray-300 rounded-md"
                  >
                    {ENTRY_TYPES.map((type) => (
                      <option key={type.id} value={type.id}>
                        {type.icon} {type.name}
                      </option>
                    ))}
                  </select>
                </div>
                <div>
                  <label className="block text-sm font-medium mb-1">重要性 (0-5)</label>
                  <input
                    type="number"
                    min="0"
                    max="5"
                    value={formData.importance}
                    onChange={(e) =>
                      setFormData({
                        ...formData,
                        importance: parseInt(e.target.value) || 0,
                      })
                    }
                    className="w-full px-3 py-2 border border-gray-300 rounded-md"
                  />
                </div>
              </div>

              <div>
                <label className="block text-sm font-medium mb-1">标题 *</label>
                <input
                  type="text"
                  value={formData.title}
                  onChange={(e) => setFormData({ ...formData, title: e.target.value })}
                  className="w-full px-3 py-2 border border-gray-300 rounded-md"
                  placeholder="输入标题"
                />
              </div>

              <div>
                <label className="block text-sm font-medium mb-1">内容 *</label>
                <textarea
                  value={formData.content}
                  onChange={(e) => setFormData({ ...formData, content: e.target.value })}
                  rows={6}
                  className="w-full px-3 py-2 border border-gray-300 rounded-md resize-none"
                  placeholder="输入详细内容"
                />
              </div>

              <div>
                <label className="block text-sm font-medium mb-1">关键词</label>
                <input
                  type="text"
                  value={formData.keywords}
                  onChange={(e) => setFormData({ ...formData, keywords: e.target.value })}
                  className="w-full px-3 py-2 border border-gray-300 rounded-md"
                  placeholder="用逗号分隔多个关键词"
                />
              </div>
            </div>

            <div className="flex justify-end gap-2 mt-6">
              <button
                onClick={resetForm}
                className="px-4 py-2 text-gray-600 hover:text-gray-800"
              >
                取消
              </button>
              <button
                onClick={editingEntry ? handleUpdateEntry : handleCreateEntry}
                disabled={!formData.title.trim() || !formData.content.trim() || loading}
                className="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 disabled:opacity-50"
              >
                {editingEntry ? '更新' : '创建'}
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
