import React, { useState, useEffect } from "react";
import {
  User,
  Plus,
  Trash2,
  Edit,
  Image,
  Palette,
  Sparkles,
  Save,
  X,
  ChevronDown,
  ChevronUp,
  Search,
  Filter,
  Tag,
  Heart,
  Shield,
  Zap,
  BookOpen,
  MapPin,
  Briefcase,
  MoreVertical,
  Copy,
} from "lucide-react";
import type { Character } from "../types";

interface CharacterCardProps {
  character: Character;
  isSelected: boolean;
  onSelect: () => void;
  onEdit: () => void;
  onDelete: () => void;
  onDuplicate?: () => void;
}

const CharacterCard: React.FC<CharacterCardProps> = ({
  character,
  isSelected,
  onSelect,
  onEdit,
  onDelete,
  onDuplicate,
}) => {
  const [showMenu, setShowMenu] = useState(false);

  const getAvatarColor = (name: string) => {
    const colors = [
      "from-purple-400 to-pink-400",
      "from-blue-400 to-cyan-400",
      "from-green-400 to-emerald-400",
      "from-orange-400 to-red-400",
      "from-indigo-400 to-purple-400",
    ];
    const index = name.charCodeAt(0) % colors.length;
    return colors[index];
  };

  return (
    <div
      className={`group relative p-4 border rounded-lg cursor-pointer transition-all ${
        isSelected
          ? "border-blue-500 bg-blue-50 shadow-md"
          : "border-border hover:border-blue-300 hover:shadow-sm"
      }`}
      onClick={onSelect}
    >
      <div className="flex items-start gap-3">
        <div
          className={`w-12 h-12 rounded-full bg-gradient-to-br ${getAvatarColor(
            character.name
          )} flex items-center justify-center text-white font-semibold text-lg shrink-0`}
        >
          {character.name.charAt(0)}
        </div>
        <div className="flex-1 min-w-0">
          <h4 className="font-medium text-sm truncate">{character.name}</h4>
          <div className="flex items-center gap-2 mt-1">
            {character.age && (
              <span className="text-xs text-muted-foreground">{character.age}岁</span>
            )}
            {character.gender && (
              <span className="text-xs text-muted-foreground">· {character.gender}</span>
            )}
          </div>
          {character.description && (
            <p className="text-xs text-muted-foreground mt-1 line-clamp-2">
              {character.description}
            </p>
          )}
        </div>
        <div className="relative">
          <button
            onClick={(e) => {
              e.stopPropagation();
              setShowMenu(!showMenu);
            }}
            className="p-1 hover:bg-gray-100 rounded opacity-0 group-hover:opacity-100 transition-opacity"
          >
            <MoreVertical className="w-4 h-4 text-muted-foreground" />
          </button>
          {showMenu && (
            <div className="absolute right-0 top-8 bg-white border border-border rounded-lg shadow-lg py-1 z-10 min-w-[120px]">
              <button
                onClick={(e) => {
                  e.stopPropagation();
                  onEdit();
                  setShowMenu(false);
                }}
                className="w-full px-3 py-2 text-left text-sm hover:bg-gray-100 flex items-center gap-2"
              >
                <Edit className="w-3 h-3" />
                编辑
              </button>
              {onDuplicate && (
                <button
                  onClick={(e) => {
                    e.stopPropagation();
                    onDuplicate();
                    setShowMenu(false);
                  }}
                  className="w-full px-3 py-2 text-left text-sm hover:bg-gray-100 flex items-center gap-2"
                >
                  <Copy className="w-3 h-3" />
                  复制
                </button>
              )}
              <button
                onClick={(e) => {
                  e.stopPropagation();
                  onDelete();
                  setShowMenu(false);
                }}
                className="w-full px-3 py-2 text-left text-sm hover:bg-red-50 text-red-500 flex items-center gap-2"
              >
                <Trash2 className="w-3 h-3" />
                删除
              </button>
            </div>
          )}
        </div>
      </div>
      {character.tags && character.tags.length > 0 && (
        <div className="flex flex-wrap gap-1 mt-2">
          {character.tags.slice(0, 3).map((tag, idx) => (
            <span
              key={idx}
              className="px-2 py-0.5 bg-blue-100 text-blue-700 text-xs rounded-full"
            >
              {tag}
            </span>
          ))}
          {character.tags.length > 3 && (
            <span className="text-xs text-muted-foreground">
              +{character.tags.length - 3}
            </span>
          )}
        </div>
      )}
    </div>
  );
};

interface CharacterDetailSectionProps {
  icon: React.ReactNode;
  title: string;
  isExpanded: boolean;
  onToggle: () => void;
  children: React.ReactNode;
}

const CharacterDetailSection: React.FC<CharacterDetailSectionProps> = ({
  icon,
  title,
  isExpanded,
  onToggle,
  children,
}) => {
  return (
    <div className="mb-4">
      <button
        onClick={onToggle}
        className="flex items-center justify-between w-full py-2 text-left font-medium border-b border-border hover:bg-gray-50 transition-colors"
      >
        <span className="flex items-center gap-2 text-sm">{icon} {title}</span>
        {isExpanded ? (
          <ChevronUp className="w-4 h-4 text-muted-foreground" />
        ) : (
          <ChevronDown className="w-4 h-4 text-muted-foreground" />
        )}
      </button>
      {isExpanded && <div className="mt-3">{children}</div>}
    </div>
  );
};

interface EnhancedCharacterPanelProps {
  projectId?: string;
  characters: Character[];
  onCreateCharacter: () => void;
  onEditCharacter: (character: Character) => void;
  onDeleteCharacter: (characterId: string) => void;
  onUpdateCharacter?: (character: Character) => void;
}

export const EnhancedCharacterPanel: React.FC<EnhancedCharacterPanelProps> = ({
  projectId,
  characters,
  onCreateCharacter,
  onEditCharacter,
  onDeleteCharacter,
  onUpdateCharacter,
}) => {
  const [selectedCharacter, setSelectedCharacter] = useState<Character | null>(null);
  const [editMode, setEditMode] = useState(false);
  const [editForm, setEditForm] = useState<Partial<Character>>({});
  const [searchQuery, setSearchQuery] = useState("");
  const [filterTag, setFilterTag] = useState<string>("");
  const [expandedSections, setExpandedSections] = useState<Record<string, boolean>>({
    basic: true,
    personality: true,
    background: false,
    relationships: false,
    tags: true,
  });

  useEffect(() => {
    if (selectedCharacter && !editMode) {
      setEditForm(selectedCharacter);
    }
  }, [selectedCharacter, editMode]);

  const handleSelectCharacter = (character: Character) => {
    setSelectedCharacter(character);
    setEditMode(false);
  };

  const handleStartEdit = () => {
    if (!selectedCharacter) return;
    setEditForm({ ...selectedCharacter });
    setEditMode(true);
  };

  const handleSaveEdit = () => {
    if (!editForm.id || !onUpdateCharacter) return;
    onUpdateCharacter(editForm as Character);
    setEditMode(false);
    setSelectedCharacter(editForm as Character);
  };

  const handleCancelEdit = () => {
    setEditMode(false);
    if (selectedCharacter) {
      setEditForm(selectedCharacter);
    }
  };

  const handleDeleteCharacter = (characterId: string) => {
    if (confirm("确定要删除这个角色吗？")) {
      onDeleteCharacter(characterId);
      if (selectedCharacter?.id === characterId) {
        setSelectedCharacter(null);
      }
    }
  };

  const toggleSection = (section: string) => {
    setExpandedSections((prev) => ({
      ...prev,
      [section]: !prev[section],
    }));
  };

  const allTags = characters
    .flatMap((c) => c.tags || [])
    .filter((tag, index, self) => self.indexOf(tag) === index);

  const filteredCharacters = characters.filter((character) => {
    const matchesSearch =
      !searchQuery ||
      character.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
      character.description?.toLowerCase().includes(searchQuery.toLowerCase());
    const matchesTag = !filterTag || character.tags?.includes(filterTag);
    return matchesSearch && matchesTag;
  });

  const renderCharacterList = () => (
    <div className="flex flex-col h-full">
      <div className="p-4 border-b border-border">
        <div className="flex items-center justify-between mb-3">
          <h2 className="text-sm font-semibold">角色列表</h2>
          <div className="flex items-center gap-2">
            <span className="text-xs text-muted-foreground">{characters.length} 个</span>
            <button
              onClick={onCreateCharacter}
              className="px-2 py-1 bg-primary text-primary-foreground text-xs rounded hover:bg-primary/90 transition-colors flex items-center gap-1"
            >
              <Plus className="w-3 h-3" />
              新建
            </button>
          </div>
        </div>
        <div className="flex gap-2">
          <div className="flex-1 relative">
            <Search className="w-4 h-4 absolute left-2 top-1/2 -translate-y-1/2 text-muted-foreground" />
            <input
              type="text"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              placeholder="搜索角色..."
              className="w-full pl-8 pr-3 py-1.5 border border-border rounded-lg text-xs focus:outline-none focus:ring-2 focus:ring-primary/50"
            />
          </div>
          <select
            value={filterTag}
            onChange={(e) => setFilterTag(e.target.value)}
            className="px-3 py-1.5 border border-border rounded-lg text-xs focus:outline-none focus:ring-2 focus:ring-primary/50 bg-white"
          >
            <option value="">全部标签</option>
            {allTags.map((tag) => (
              <option key={tag} value={tag}>
                {tag}
              </option>
            ))}
          </select>
        </div>
      </div>
      <div className="flex-1 overflow-y-auto p-4 space-y-3">
        {filteredCharacters.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-full text-muted-foreground">
            <User className="w-12 h-12 mb-2 opacity-50" />
            <p className="text-sm">暂无角色</p>
          </div>
        ) : (
          filteredCharacters.map((character) => (
            <CharacterCard
              key={character.id}
              character={character}
              isSelected={selectedCharacter?.id === character.id}
              onSelect={() => handleSelectCharacter(character)}
              onEdit={() => {
                handleSelectCharacter(character);
                handleStartEdit();
              }}
              onDelete={() => handleDeleteCharacter(character.id)}
              onDuplicate={
                onUpdateCharacter
                  ? () => {
                      const newCharacter = {
                        ...character,
                        id: `${character.id}-copy-${Date.now()}`,
                        name: `${character.name} (副本)`,
                      };
                      onUpdateCharacter(newCharacter);
                    }
                  : undefined
              }
            />
          ))
        )}
      </div>
    </div>
  );

  const renderCharacterDetail = () => {
    if (!selectedCharacter) {
      return (
        <div className="flex flex-col items-center justify-center h-full text-muted-foreground">
          <User className="w-16 h-16 mb-2 opacity-30" />
          <p className="text-sm">选择一个角色查看详情</p>
        </div>
      );
    }

    const displayCharacter = editMode ? { ...selectedCharacter, ...editForm } : selectedCharacter;

    return (
      <div className="flex flex-col h-full">
        <div className="p-4 border-b border-border">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-3">
              <div
                className={`w-14 h-14 rounded-full bg-gradient-to-br from-purple-400 to-pink-400 flex items-center justify-center text-white font-bold text-xl`}
              >
                {displayCharacter.name.charAt(0)}
              </div>
              {editMode ? (
                <input
                  type="text"
                  value={editForm.name || ""}
                  onChange={(e) => setEditForm({ ...editForm, name: e.target.value })}
                  className="text-lg font-medium border-b-2 border-primary focus:outline-none flex-1"
                  autoFocus
                />
              ) : (
                <h2 className="text-lg font-medium">{displayCharacter.name}</h2>
              )}
            </div>
            <div className="flex gap-2">
              {editMode ? (
                <>
                  <button
                    onClick={handleSaveEdit}
                    className="px-3 py-1.5 bg-primary text-primary-foreground text-sm rounded hover:bg-primary/90 flex items-center gap-1"
                  >
                    <Save className="w-4 h-4" />
                    保存
                  </button>
                  <button
                    onClick={handleCancelEdit}
                    className="px-3 py-1.5 bg-gray-100 text-sm rounded hover:bg-gray-200"
                  >
                    取消
                  </button>
                </>
              ) : (
                <button
                  onClick={handleStartEdit}
                  className="px-3 py-1.5 bg-gray-100 text-sm rounded hover:bg-gray-200 flex items-center gap-1"
                >
                  <Edit className="w-4 h-4" />
                  编辑
                </button>
              )}
            </div>
          </div>
        </div>

        <div className="flex-1 overflow-y-auto p-4">
          <CharacterDetailSection
            icon={<User className="w-4 h-4" />}
            title="基本信息"
            isExpanded={expandedSections.basic}
            onToggle={() => toggleSection("basic")}
          >
            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="block text-xs font-medium text-muted-foreground mb-1">
                  年龄
                </label>
                {editMode ? (
                  <input
                    type="number"
                    value={editForm.age || ""}
                    onChange={(e) => setEditForm({ ...editForm, age: e.target.value ? parseInt(e.target.value) : undefined })}
                    className="w-full px-2 py-1.5 border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50"
                  />
                ) : (
                  <span className="text-sm">{displayCharacter.age || "-"}</span>
                )}
              </div>
              <div>
                <label className="block text-xs font-medium text-muted-foreground mb-1">
                  性别
                </label>
                {editMode ? (
                  <select
                    value={editForm.gender || ""}
                    onChange={(e) => setEditForm({ ...editForm, gender: e.target.value })}
                    className="w-full px-2 py-1.5 border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50 bg-white"
                  >
                    <option value="">未设置</option>
                    <option value="男">男</option>
                    <option value="女">女</option>
                    <option value="其他">其他</option>
                  </select>
                ) : (
                  <span className="text-sm">{displayCharacter.gender || "-"}</span>
                )}
              </div>
            </div>
            <div className="mt-4">
              <label className="block text-xs font-medium text-muted-foreground mb-1">
                描述
              </label>
              {editMode ? (
                <textarea
                  value={editForm.description || ""}
                  onChange={(e) => setEditForm({ ...editForm, description: e.target.value })}
                  className="w-full px-2 py-1.5 border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50 resize-none"
                  rows={4}
                />
              ) : (
                <p className="text-sm text-muted-foreground whitespace-pre-wrap">
                  {displayCharacter.description || "暂无描述"}
                </p>
              )}
            </div>
          </CharacterDetailSection>

          <CharacterDetailSection
            icon={<Heart className="w-4 h-4" />}
            title="性格特征"
            isExpanded={expandedSections.personality}
            onToggle={() => toggleSection("personality")}
          >
            <div className="space-y-3">
              {displayCharacter.personality?.split("\n").map((trait, idx) => (
                <div key={idx} className="flex items-center gap-2">
                  <div className="w-1.5 h-1.5 rounded-full bg-blue-500" />
                  <span className="text-sm">{trait}</span>
                </div>
              )) || <span className="text-sm text-muted-foreground">暂无性格特征</span>}
            </div>
          </CharacterDetailSection>

          <CharacterDetailSection
            icon={<BookOpen className="w-4 h-4" />}
            title="背景故事"
            isExpanded={expandedSections.background}
            onToggle={() => toggleSection("background")}
          >
            <p className="text-sm text-muted-foreground whitespace-pre-wrap">
              {displayCharacter.background || "暂无背景故事"}
            </p>
          </CharacterDetailSection>

          <CharacterDetailSection
            icon={<Shield className="w-4 h-4" />}
            title="关系网络"
            isExpanded={expandedSections.relationships}
            onToggle={() => toggleSection("relationships")}
          >
            <div className="flex flex-wrap gap-2">
              {displayCharacter.relationships?.map((rel, idx) => (
                <span
                  key={idx}
                  className="px-2 py-1 bg-purple-100 text-purple-700 text-xs rounded-full"
                >
                  {rel.relation_type}{rel.description ? `: ${rel.description}` : ''}
                </span>
              )) || <span className="text-sm text-muted-foreground">暂无关系记录</span>}
            </div>
          </CharacterDetailSection>

          <CharacterDetailSection
            icon={<Tag className="w-4 h-4" />}
            title="标签"
            isExpanded={expandedSections.tags}
            onToggle={() => toggleSection("tags")}
          >
            <div className="flex flex-wrap gap-2">
              {displayCharacter.tags?.map((tag, idx) => (
                <span
                  key={idx}
                  className="px-2 py-1 bg-green-100 text-green-700 text-xs rounded-full flex items-center gap-1"
                >
                  {tag}
                  {editMode && (
                    <button
                      onClick={() => {
                        const newTags = displayCharacter.tags?.filter((_, i) => i !== idx) || [];
                        setEditForm({ ...editForm, tags: newTags });
                      }}
                      className="hover:text-red-500"
                    >
                      <X className="w-3 h-3" />
                    </button>
                  )}
                </span>
              ))}
              {editMode && (
                <button
                  onClick={() => {
                    const newTag = prompt("添加新标签:");
                    if (newTag) {
                      setEditForm({
                        ...editForm,
                        tags: [...(displayCharacter.tags || []), newTag],
                      });
                    }
                  }}
                  className="px-2 py-1 border border-dashed border-border text-xs rounded hover:border-primary transition-colors"
                >
                  + 添加
                </button>
              )}
            </div>
          </CharacterDetailSection>
        </div>
      </div>
    );
  };

  return (
    <div className="flex h-full">
      <div className="w-80 border-r border-border">{renderCharacterList()}</div>
      <div className="flex-1">{renderCharacterDetail()}</div>
    </div>
  );
};

export default EnhancedCharacterPanel;
