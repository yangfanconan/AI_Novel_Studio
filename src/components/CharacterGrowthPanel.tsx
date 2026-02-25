import React, { useState, useEffect } from 'react';
import {
  TrendingUp,
  Tag,
  Plus,
  Search,
  Filter,
  Trash2,
  BarChart3,
  Calendar,
  ArrowUpRight,
  ArrowDownRight,
  Minus,
} from 'lucide-react';
import {
  characterGrowthService,
  CharacterGrowthTimeline,
  CharacterTagCollection,
  CharacterTag,
  TagLibrary,
  TagStatistics,
  TagWeight,
} from '../services/characterGrowth.service';

interface CharacterGrowthPanelProps {
  projectId: string;
  characterId: string;
  characterName: string;
}

export const CharacterGrowthPanel: React.FC<CharacterGrowthPanelProps> = ({
  projectId,
  characterId,
  characterName,
}) => {
  const [activeTab, setActiveTab] = useState<'growth' | 'tags'>('growth');
  const [timeline, setTimeline] = useState<CharacterGrowthTimeline | null>(null);
  const [tagCollection, setTagCollection] = useState<CharacterTagCollection | null>(null);
  const [tagLibrary, setTagLibrary] = useState<TagLibrary | null>(null);
  const [statistics, setStatistics] = useState<TagStatistics | null>(null);
  const [loading, setLoading] = useState(false);
  const [showAddTagModal, setShowAddTagModal] = useState(false);
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedTagType, setSelectedTagType] = useState<string>('');
  const [newTagName, setNewTagName] = useState('');
  const [newTagDescription, setNewTagDescription] = useState('');
  const [newTagColor, setNewTagColor] = useState('#4ECDC4');

  useEffect(() => {
    loadData();
  }, [characterId, projectId]);

  const loadData = async () => {
    setLoading(true);
    try {
      const [timelineData, tagsData, libraryData, statsData] = await Promise.all([
        characterGrowthService.getGrowthTimeline(characterId),
        characterGrowthService.getCharacterTags(characterId),
        characterGrowthService.getTagLibrary(),
        characterGrowthService.getTagStatistics(projectId),
      ]);
      setTimeline(timelineData);
      setTagCollection(tagsData);
      setTagLibrary(libraryData);
      setStatistics(statsData);
    } catch (error) {
      console.error('Failed to load data:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleAddTag = async () => {
    if (!newTagName.trim()) return;

    setLoading(true);
    try {
      await characterGrowthService.createCharacterTag(
        characterId,
        selectedTagType as any,
        newTagName,
        newTagColor,
        'medium' as TagWeight,
        undefined,
        newTagDescription,
        false,
        'manual'
      );
      setShowAddTagModal(false);
      setNewTagName('');
      setNewTagDescription('');
      loadData();
    } catch (error) {
      console.error('Failed to add tag:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleDeleteTag = async (tagId: string) => {
    if (!confirm('确定要删除此标签吗？')) return;

    setLoading(true);
    try {
      await characterGrowthService.deleteCharacterTag(tagId);
      loadData();
    } catch (error) {
      console.error('Failed to delete tag:', error);
    } finally {
      setLoading(false);
    }
  };

  const getSignificanceColor = (significance: string) => {
    switch (significance) {
      case 'minor': return 'bg-gray-100 text-gray-800';
      case 'moderate': return 'bg-blue-100 text-blue-800';
      case 'major': return 'bg-orange-100 text-orange-800';
      case 'critical': return 'bg-red-100 text-red-800';
      default: return 'bg-gray-100 text-gray-800';
    }
  };

  const getSignificanceIcon = (significance: string) => {
    switch (significance) {
      case 'minor': return <Minus className="w-3 h-3" />;
      case 'moderate': return <ArrowUpRight className="w-3 h-3" />;
      case 'major': return <TrendingUp className="w-3 h-3" />;
      case 'critical': return <TrendingUp className="w-3 h-3 text-red-600" />;
      default: return <Minus className="w-3 h-3" />;
    }
  };

  const formatDate = (timestamp: number) => {
    return new Date(timestamp * 1000).toLocaleDateString('zh-CN', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
    });
  };

  const filteredTags = tagCollection?.tags.filter(tag => {
    const matchQuery = !searchQuery || tag.name.toLowerCase().includes(searchQuery.toLowerCase());
    const matchType = !selectedTagType || tag.tag_type === selectedTagType;
    return matchQuery && matchType;
  }) || [];

  return (
    <div className="border-b border-border bg-card">
      <div className="px-4 py-2">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <TrendingUp className="w-4 h-4 text-primary" />
            <span className="text-sm font-medium">角色成长</span>
          </div>
          <div className="flex gap-1">
            <button
              onClick={() => setActiveTab('growth')}
              className={`px-3 py-1.5 text-sm rounded-md transition-colors ${
                activeTab === 'growth'
                  ? 'bg-primary text-primary-foreground'
                  : 'text-muted-foreground hover:bg-muted'
              }`}
            >
              成长轨迹
            </button>
            <button
              onClick={() => setActiveTab('tags')}
              className={`px-3 py-1.5 text-sm rounded-md transition-colors ${
                activeTab === 'tags'
                  ? 'bg-primary text-primary-foreground'
                  : 'text-muted-foreground hover:bg-muted'
              }`}
            >
              标签管理
            </button>
          </div>
        </div>

        {loading && (
          <div className="mt-3 text-center text-sm text-muted-foreground">
            加载中...
          </div>
        )}

        {!loading && activeTab === 'growth' && timeline && (
          <div className="mt-3 space-y-4">
            <div className="grid grid-cols-4 gap-3 p-3 bg-muted rounded-md">
              <div>
                <div className="text-xs text-muted-foreground">总变化</div>
                <div className="text-lg font-semibold">{timeline.summary.total_changes}</div>
              </div>
              <div>
                <div className="text-xs text-muted-foreground">性格变化</div>
                <div className="text-lg font-semibold">{timeline.summary.personality_changes}</div>
              </div>
              <div>
                <div className="text-xs text-muted-foreground">状态变化</div>
                <div className="text-lg font-semibold">{timeline.summary.status_changes}</div>
              </div>
              <div>
                <div className="text-xs text-muted-foreground">重大变化</div>
                <div className="text-lg font-semibold">{timeline.summary.major_changes}</div>
              </div>
            </div>

            <div className="space-y-3">
              {timeline.timeline.length === 0 ? (
                <div className="p-4 text-center text-sm text-muted-foreground">
                  暂无成长记录
                </div>
              ) : (
                timeline.timeline.map((event, index) => (
                  <div key={index} className="p-3 border rounded-md hover:border-primary transition-colors">
                    <div className="flex items-center justify-between mb-2">
                      <div className="flex items-center gap-2">
                        <Calendar className="w-4 h-4 text-muted-foreground" />
                        <span className="font-medium">{event.chapter_title}</span>
                      </div>
                      <div className="text-xs text-muted-foreground">
                        {formatDate(event.timestamp)}
                      </div>
                    </div>
                    <div className="space-y-2">
                      {event.changes.map((change, changeIndex) => (
                        <div key={changeIndex} className={`p-2 rounded-md ${getSignificanceColor(change.significance)}`}>
                          <div className="flex items-center gap-2 mb-1">
                            {getSignificanceIcon(change.significance)}
                            <span className="text-sm font-medium">{change.category}</span>
                          </div>
                          <div className="text-xs text-muted-foreground mb-1">
                            {change.description}
                          </div>
                          {(change.before || change.after) && (
                            <div className="flex items-center gap-2 text-xs">
                              {change.before && (
                                <span className="text-red-600 line-through">{change.before}</span>
                              )}
                              {change.before && change.after && (
                                <span className="text-muted-foreground">→</span>
                              )}
                              {change.after && (
                                <span className="text-green-600">{change.after}</span>
                              )}
                            </div>
                          )}
                        </div>
                      ))}
                    </div>
                  </div>
                ))
              )}
            </div>
          </div>
        )}

        {!loading && activeTab === 'tags' && tagCollection && tagLibrary && (
          <div className="mt-3 space-y-4">
            <div className="flex items-center gap-2 mb-3">
              <div className="flex-1 relative">
                <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground" />
                <input
                  type="text"
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  placeholder="搜索标签..."
                  className="w-full pl-10 pr-3 py-1.5 border rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-primary"
                />
              </div>
              <select
                value={selectedTagType}
                onChange={(e) => setSelectedTagType(e.target.value)}
                className="px-3 py-1.5 border rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-primary"
              >
                <option value="">全部类型</option>
                {tagLibrary.categories.map((category) => (
                  <option key={category.id} value={category.tag_type}>
                    {category.name}
                  </option>
                ))}
              </select>
              <button
                onClick={() => setShowAddTagModal(true)}
                className="flex items-center gap-1.5 px-3 py-1.5 bg-primary text-primary-foreground rounded-md text-sm hover:bg-primary/90 transition-colors"
              >
                <Plus className="w-3 h-3" />
                添加标签
              </button>
            </div>

            <div className="grid grid-cols-2 gap-3">
              <div className="space-y-2">
                {tagCollection.tag_groups.personality_tags.map((tag) => (
                  <TagBadge key={tag.id} tag={tag} onDelete={handleDeleteTag} />
                ))}
              </div>
              <div className="space-y-2">
                {tagCollection.tag_groups.role_tags.map((tag) => (
                  <TagBadge key={tag.id} tag={tag} onDelete={handleDeleteTag} />
                ))}
              </div>
              <div className="space-y-2">
                {tagCollection.tag_groups.skill_tags.map((tag) => (
                  <TagBadge key={tag.id} tag={tag} onDelete={handleDeleteTag} />
                ))}
              </div>
              <div className="space-y-2">
                {tagCollection.tag_groups.relationship_tags.map((tag) => (
                  <TagBadge key={tag.id} tag={tag} onDelete={handleDeleteTag} />
                ))}
              </div>
              <div className="space-y-2">
                {tagCollection.tag_groups.trait_tags.map((tag) => (
                  <TagBadge key={tag.id} tag={tag} onDelete={handleDeleteTag} />
                ))}
              </div>
              <div className="space-y-2">
                {tagCollection.tag_groups.custom_tags.map((tag) => (
                  <TagBadge key={tag.id} tag={tag} onDelete={handleDeleteTag} />
                ))}
              </div>
            </div>

            {statistics && (
              <div className="mt-4 p-3 bg-muted rounded-md">
                <div className="flex items-center gap-2 mb-3">
                  <BarChart3 className="w-4 h-4 text-primary" />
                  <span className="text-sm font-medium">标签统计</span>
                </div>
                <div className="grid grid-cols-4 gap-3 text-xs">
                  <div>
                    <div className="text-muted-foreground">总标签数</div>
                    <div className="font-semibold">{statistics.total_tags}</div>
                  </div>
                  <div>
                    <div className="text-muted-foreground">角色数</div>
                    <div className="font-semibold">{statistics.characters_with_tags}</div>
                  </div>
                  <div className="col-span-2">
                    <div className="text-muted-foreground">常用标签</div>
                    <div className="flex flex-wrap gap-1 mt-1">
                      {statistics.most_used_tags.slice(0, 5).map(([name, count]) => (
                        <span key={name} className="px-1.5 py-0.5 bg-primary/10 text-primary text-xs rounded">
                          {name} ({count})
                        </span>
                      ))}
                    </div>
                  </div>
                </div>
              </div>
            )}
          </div>
        )}
      </div>

      {showAddTagModal && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="bg-card rounded-lg shadow-lg p-6 w-full max-w-md">
            <h3 className="text-lg font-semibold mb-4">添加标签</h3>
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium mb-1">标签类型</label>
                <select
                  value={selectedTagType}
                  onChange={(e) => setSelectedTagType(e.target.value)}
                  className="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-primary"
                >
                  {tagLibrary?.categories.map((category) => (
                    <option key={category.id} value={category.tag_type}>
                      {category.name}
                    </option>
                  ))}
                </select>
              </div>
              <div>
                <label className="block text-sm font-medium mb-1">标签名称</label>
                <input
                  type="text"
                  value={newTagName}
                  onChange={(e) => setNewTagName(e.target.value)}
                  placeholder="输入标签名称"
                  className="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-primary"
                />
              </div>
              <div>
                <label className="block text-sm font-medium mb-1">描述</label>
                <textarea
                  value={newTagDescription}
                  onChange={(e) => setNewTagDescription(e.target.value)}
                  placeholder="标签描述..."
                  rows={2}
                  className="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-primary resize-none"
                />
              </div>
              <div>
                <label className="block text-sm font-medium mb-1">颜色</label>
                <div className="flex gap-2">
                  {tagLibrary?.categories
                    .find(c => c.tag_type === selectedTagType)
                    ?.color_palette.map((color) => (
                      <button
                        key={color}
                        onClick={() => setNewTagColor(color)}
                        className={`w-8 h-8 rounded-md border-2 ${
                          newTagColor === color ? 'border-primary' : 'border-transparent'
                        }`}
                        style={{ backgroundColor: color }}
                      />
                    ))}
                </div>
              </div>
              <div className="flex justify-end gap-2">
                <button
                  onClick={() => setShowAddTagModal(false)}
                  className="px-4 py-2 text-sm text-muted-foreground hover:bg-muted rounded-md transition-colors"
                >
                  取消
                </button>
                <button
                  onClick={handleAddTag}
                  disabled={!newTagName.trim() || loading}
                  className="px-4 py-2 text-sm bg-primary text-primary-foreground rounded-md hover:bg-primary/90 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
                >
                  {loading ? '添加中...' : '添加'}
                </button>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

interface TagBadgeProps {
  tag: CharacterTag;
  onDelete: (tagId: string) => void;
}

const TagBadge: React.FC<TagBadgeProps> = ({ tag, onDelete }) => {
  return (
    <div
      className="inline-flex items-center gap-1 px-2 py-1 rounded-md text-sm"
      style={{ backgroundColor: `${tag.color}20`, borderLeft: `3px solid ${tag.color}` }}
    >
      <Tag className="w-3 h-3" style={{ color: tag.color }} />
      <span>{tag.name}</span>
      {tag.description && (
        <span className="text-xs text-muted-foreground max-w-32 truncate">
          {tag.description}
        </span>
      )}
      <button
        onClick={() => onDelete(tag.id)}
        className="p-0.5 text-muted-foreground hover:text-red-600 hover:bg-red-50 rounded transition-colors"
        title="删除标签"
      >
        <Trash2 className="w-3 h-3" />
      </button>
    </div>
  );
};
