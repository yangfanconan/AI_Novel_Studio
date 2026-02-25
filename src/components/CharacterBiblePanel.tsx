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
} from "lucide-react";
import {
  moyinService,
  CharacterBible,
  CreateCharacterBibleRequest,
  CharacterBibleUpdate,
  ReferenceImage,
} from "../services/moyin.service";

interface CharacterBiblePanelProps {
  projectId: string;
  onSelectCharacter?: (character: CharacterBible) => void;
}

export const CharacterBiblePanel: React.FC<CharacterBiblePanelProps> = ({
  projectId,
  onSelectCharacter,
}) => {
  const [characters, setCharacters] = useState<CharacterBible[]>([]);
  const [selectedCharacter, setSelectedCharacter] = useState<CharacterBible | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [editMode, setEditMode] = useState(false);
  const [createDialogOpen, setCreateDialogOpen] = useState(false);
  const [expandedSections, setExpandedSections] = useState<Record<string, boolean>>({
    visual: true,
    style: true,
    colors: true,
    references: true,
  });

  const [editForm, setEditForm] = useState<CharacterBibleUpdate>({});
  const [createForm, setCreateForm] = useState<CreateCharacterBibleRequest>({
    project_id: projectId,
    name: "",
    type: "human",
    visual_traits: "",
    style_tokens: [],
    color_palette: [],
    personality: "",
  });

  const [newStyleToken, setNewStyleToken] = useState("");
  const [newColor, setNewColor] = useState("");

  useEffect(() => {
    loadCharacters();
  }, [projectId]);

  const loadCharacters = async () => {
    try {
      setLoading(true);
      const data = await moyinService.getCharacterBibles(projectId);
      setCharacters(data);
      setError(null);
    } catch (err) {
      setError("加载角色圣经失败");
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  const handleCreateCharacter = async () => {
    if (!createForm.name.trim()) {
      setError("请输入角色名称");
      return;
    }

    try {
      setLoading(true);
      const character = await moyinService.createCharacterBible(createForm);
      setCharacters([character, ...characters]);
      setSelectedCharacter(character);
      setCreateDialogOpen(false);
      setCreateForm({
        project_id: projectId,
        name: "",
        type: "human",
        visual_traits: "",
        style_tokens: [],
        color_palette: [],
        personality: "",
      });
      setError(null);
    } catch (err) {
      setError("创建角色失败");
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  const handleUpdateCharacter = async () => {
    if (!selectedCharacter) return;

    try {
      setLoading(true);
      const updated = await moyinService.updateCharacterBible(selectedCharacter.id, editForm);
      setCharacters(characters.map((c) => (c.id === updated.id ? updated : c)));
      setSelectedCharacter(updated);
      setEditMode(false);
      setEditForm({});
      setError(null);
    } catch (err) {
      setError("更新角色失败");
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  const handleDeleteCharacter = async (id: string) => {
    if (!confirm("确定要删除这个角色圣经吗？")) return;

    try {
      setLoading(true);
      await moyinService.deleteCharacterBible(id);
      setCharacters(characters.filter((c) => c.id !== id));
      if (selectedCharacter?.id === id) {
        setSelectedCharacter(null);
      }
      setError(null);
    } catch (err) {
      setError("删除角色失败");
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  const handleSelectCharacter = (character: CharacterBible) => {
    setSelectedCharacter(character);
    setEditMode(false);
    setEditForm({});
    onSelectCharacter?.(character);
  };

  const startEdit = () => {
    if (!selectedCharacter) return;
    setEditForm({
      name: selectedCharacter.name,
      char_type: selectedCharacter.type,
      visual_traits: selectedCharacter.visual_traits,
      style_tokens: [...selectedCharacter.style_tokens],
      color_palette: [...selectedCharacter.color_palette],
      personality: selectedCharacter.personality,
    });
    setEditMode(true);
  };

  const toggleSection = (section: string) => {
    setExpandedSections((prev) => ({
      ...prev,
      [section]: !prev[section],
    }));
  };

  const addStyleToken = () => {
    if (!newStyleToken.trim()) return;
    const currentTokens = editForm.style_tokens || [];
    setEditForm({
      ...editForm,
      style_tokens: [...currentTokens, newStyleToken.trim()],
    });
    setNewStyleToken("");
  };

  const removeStyleToken = (index: number) => {
    const tokens = editForm.style_tokens || [];
    setEditForm({
      ...editForm,
      style_tokens: tokens.filter((_, i) => i !== index),
    });
  };

  const addColor = () => {
    if (!newColor.trim()) return;
    const currentColors = editForm.color_palette || [];
    setEditForm({
      ...editForm,
      color_palette: [...currentColors, newColor.trim()],
    });
    setNewColor("");
  };

  const removeColor = (index: number) => {
    const colors = editForm.color_palette || [];
    setEditForm({
      ...editForm,
      color_palette: colors.filter((_, i) => i !== index),
    });
  };

  const renderCharacterList = () => (
    <div className="character-list flex-1 overflow-y-auto">
      {characters.length === 0 ? (
        <div className="empty-state p-4 text-center text-gray-500">
          <User className="w-12 h-12 mx-auto mb-2 opacity-50" />
          <p>暂无角色圣经</p>
          <p className="text-sm">点击上方按钮创建新角色</p>
        </div>
      ) : (
        characters.map((character) => (
          <div
            key={character.id}
            className={`character-item p-3 border-b cursor-pointer hover:bg-gray-50 ${
              selectedCharacter?.id === character.id
                ? "bg-blue-50 border-l-4 border-l-blue-500"
                : ""
            }`}
            onClick={() => handleSelectCharacter(character)}
          >
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2">
                <div className="w-10 h-10 rounded-full bg-gradient-to-br from-purple-400 to-pink-400 flex items-center justify-center text-white font-medium">
                  {character.name.charAt(0)}
                </div>
                <div>
                  <h4 className="font-medium">{character.name}</h4>
                  <p className="text-xs text-gray-500">{character.type}</p>
                </div>
              </div>
              <button
                onClick={(e) => {
                  e.stopPropagation();
                  handleDeleteCharacter(character.id);
                }}
                className="p-1 text-gray-400 hover:text-red-500"
              >
                <Trash2 className="w-4 h-4" />
              </button>
            </div>
            {character.style_tokens.length > 0 && (
              <div className="mt-2 flex flex-wrap gap-1">
                {character.style_tokens.slice(0, 3).map((token, idx) => (
                  <span
                    key={idx}
                    className="px-2 py-0.5 bg-purple-100 text-purple-700 text-xs rounded-full"
                  >
                    {token}
                  </span>
                ))}
                {character.style_tokens.length > 3 && (
                  <span className="text-xs text-gray-400">
                    +{character.style_tokens.length - 3}
                  </span>
                )}
              </div>
            )}
          </div>
        ))
      )}
    </div>
  );

  const renderCharacterDetail = () => {
    if (!selectedCharacter) {
      return (
        <div className="empty-detail flex-1 flex items-center justify-center text-gray-500">
          <div className="text-center">
            <User className="w-16 h-16 mx-auto mb-2 opacity-30" />
            <p>选择一个角色查看详情</p>
          </div>
        </div>
      );
    }

    const displayCharacter = editMode ? { ...selectedCharacter, ...editForm } : selectedCharacter;

    return (
      <div className="character-detail flex-1 overflow-y-auto p-4">
        <div className="flex items-center justify-between mb-4">
          <h3 className="text-lg font-medium">
            {editMode ? (
              <input
                type="text"
                value={editForm.name || ""}
                onChange={(e) => setEditForm({ ...editForm, name: e.target.value })}
                className="border rounded px-2 py-1 w-full"
              />
            ) : (
              displayCharacter.name
            )}
          </h3>
          <div className="flex gap-2">
            {editMode ? (
              <>
                <button
                  onClick={handleUpdateCharacter}
                  className="px-3 py-1 bg-blue-500 text-white rounded hover:bg-blue-600 flex items-center gap-1"
                  disabled={loading}
                >
                  <Save className="w-4 h-4" />
                  保存
                </button>
                <button
                  onClick={() => {
                    setEditMode(false);
                    setEditForm({});
                  }}
                  className="px-3 py-1 bg-gray-200 rounded hover:bg-gray-300"
                >
                  取消
                </button>
              </>
            ) : (
              <button
                onClick={startEdit}
                className="px-3 py-1 bg-gray-100 rounded hover:bg-gray-200 flex items-center gap-1"
              >
                <Edit className="w-4 h-4" />
                编辑
              </button>
            )}
          </div>
        </div>

        <div className="mb-4">
          <label className="block text-sm font-medium text-gray-600 mb-1">类型</label>
          {editMode ? (
            <select
              value={editForm.char_type || displayCharacter.type}
              onChange={(e) => setEditForm({ ...editForm, char_type: e.target.value })}
              className="border rounded px-2 py-1 w-full"
            >
              <option value="human">人类</option>
              <option value="animal">动物</option>
              <option value="fantasy">奇幻生物</option>
              <option value="robot">机器人</option>
              <option value="other">其他</option>
            </select>
          ) : (
            <span className="px-2 py-1 bg-gray-100 rounded text-sm">{displayCharacter.type}</span>
          )}
        </div>

        <div className="section mb-4">
          <button
            onClick={() => toggleSection("visual")}
            className="flex items-center justify-between w-full py-2 text-left font-medium border-b"
          >
            <span className="flex items-center gap-2">
              <User className="w-4 h-4" />
              视觉特征
            </span>
            {expandedSections.visual ? (
              <ChevronUp className="w-4 h-4" />
            ) : (
              <ChevronDown className="w-4 h-4" />
            )}
          </button>
          {expandedSections.visual && (
            <div className="mt-2">
              {editMode ? (
                <textarea
                  value={editForm.visual_traits || ""}
                  onChange={(e) => setEditForm({ ...editForm, visual_traits: e.target.value })}
                  className="border rounded px-2 py-1 w-full h-24 resize-none"
                  placeholder="描述角色的外观特征..."
                />
              ) : (
                <p className="text-gray-600 whitespace-pre-wrap">
                  {displayCharacter.visual_traits || "暂无描述"}
                </p>
              )}
            </div>
          )}
        </div>

        <div className="section mb-4">
          <button
            onClick={() => toggleSection("style")}
            className="flex items-center justify-between w-full py-2 text-left font-medium border-b"
          >
            <span className="flex items-center gap-2">
              <Sparkles className="w-4 h-4" />
              风格标签
            </span>
            {expandedSections.style ? (
              <ChevronUp className="w-4 h-4" />
            ) : (
              <ChevronDown className="w-4 h-4" />
            )}
          </button>
          {expandedSections.style && (
            <div className="mt-2">
              <div className="flex flex-wrap gap-2 mb-2">
                {(editMode ? editForm.style_tokens : displayCharacter.style_tokens)?.map(
                  (token, idx) => (
                    <span
                      key={idx}
                      className="px-2 py-1 bg-purple-100 text-purple-700 rounded-full text-sm flex items-center gap-1"
                    >
                      {token}
                      {editMode && (
                        <button
                          onClick={() => removeStyleToken(idx)}
                          className="hover:text-red-500"
                        >
                          <X className="w-3 h-3" />
                        </button>
                      )}
                    </span>
                  )
                )}
              </div>
              {editMode && (
                <div className="flex gap-2">
                  <input
                    type="text"
                    value={newStyleToken}
                    onChange={(e) => setNewStyleToken(e.target.value)}
                    onKeyPress={(e) => e.key === "Enter" && addStyleToken()}
                    className="border rounded px-2 py-1 flex-1"
                    placeholder="添加风格标签..."
                  />
                  <button
                    onClick={addStyleToken}
                    className="px-3 py-1 bg-purple-500 text-white rounded hover:bg-purple-600"
                  >
                    添加
                  </button>
                </div>
              )}
            </div>
          )}
        </div>

        <div className="section mb-4">
          <button
            onClick={() => toggleSection("colors")}
            className="flex items-center justify-between w-full py-2 text-left font-medium border-b"
          >
            <span className="flex items-center gap-2">
              <Palette className="w-4 h-4" />
              色彩方案
            </span>
            {expandedSections.colors ? (
              <ChevronUp className="w-4 h-4" />
            ) : (
              <ChevronDown className="w-4 h-4" />
            )}
          </button>
          {expandedSections.colors && (
            <div className="mt-2">
              <div className="flex flex-wrap gap-2 mb-2">
                {(editMode ? editForm.color_palette : displayCharacter.color_palette)?.map(
                  (color, idx) => (
                    <span
                      key={idx}
                      className="px-2 py-1 bg-gray-100 rounded-full text-sm flex items-center gap-1"
                    >
                      <span className="w-4 h-4 rounded-full" style={{ backgroundColor: color }} />
                      {color}
                      {editMode && (
                        <button onClick={() => removeColor(idx)} className="hover:text-red-500">
                          <X className="w-3 h-3" />
                        </button>
                      )}
                    </span>
                  )
                )}
              </div>
              {editMode && (
                <div className="flex gap-2">
                  <input
                    type="color"
                    value={newColor}
                    onChange={(e) => setNewColor(e.target.value)}
                    className="w-10 h-8 border rounded"
                  />
                  <input
                    type="text"
                    value={newColor}
                    onChange={(e) => setNewColor(e.target.value)}
                    className="border rounded px-2 py-1 flex-1"
                    placeholder="#hex 或颜色名"
                  />
                  <button
                    onClick={addColor}
                    className="px-3 py-1 bg-blue-500 text-white rounded hover:bg-blue-600"
                  >
                    添加
                  </button>
                </div>
              )}
            </div>
          )}
        </div>

        <div className="section mb-4">
          <button
            onClick={() => toggleSection("references")}
            className="flex items-center justify-between w-full py-2 text-left font-medium border-b"
          >
            <span className="flex items-center gap-2">
              <Image className="w-4 h-4" />
              参考图片
            </span>
            {expandedSections.references ? (
              <ChevronUp className="w-4 h-4" />
            ) : (
              <ChevronDown className="w-4 h-4" />
            )}
          </button>
          {expandedSections.references && (
            <div className="mt-2">
              {displayCharacter.reference_images?.length > 0 ? (
                <div className="grid grid-cols-3 gap-2">
                  {displayCharacter.reference_images.map((img, idx) => (
                    <div key={idx} className="relative group">
                      <img
                        src={img.url}
                        alt={`参考图 ${idx + 1}`}
                        className="w-full h-24 object-cover rounded"
                      />
                      {img.is_primary && (
                        <span className="absolute top-1 left-1 px-1 bg-blue-500 text-white text-xs rounded">
                          主图
                        </span>
                      )}
                    </div>
                  ))}
                </div>
              ) : (
                <p className="text-gray-500 text-sm">暂无参考图片</p>
              )}
            </div>
          )}
        </div>

        <div className="mb-4">
          <label className="block text-sm font-medium text-gray-600 mb-1">性格描述</label>
          {editMode ? (
            <textarea
              value={editForm.personality || ""}
              onChange={(e) => setEditForm({ ...editForm, personality: e.target.value })}
              className="border rounded px-2 py-1 w-full h-20 resize-none"
              placeholder="描述角色的性格特点..."
            />
          ) : (
            <p className="text-gray-600 whitespace-pre-wrap">
              {displayCharacter.personality || "暂无描述"}
            </p>
          )}
        </div>
      </div>
    );
  };

  return (
    <div className="character-bible-panel h-full flex flex-col">
      <div className="panel-header p-3 border-b flex items-center justify-between bg-gray-50">
        <h2 className="font-medium flex items-center gap-2">
          <User className="w-5 h-5" />
          角色圣经
        </h2>
        <button
          onClick={() => setCreateDialogOpen(true)}
          className="px-3 py-1 bg-blue-500 text-white rounded hover:bg-blue-600 flex items-center gap-1 text-sm"
        >
          <Plus className="w-4 h-4" />
          新建
        </button>
      </div>

      {error && (
        <div className="error-banner px-3 py-2 bg-red-100 text-red-700 text-sm">
          {error}
          <button onClick={() => setError(null)} className="ml-2 text-red-500">
            <X className="w-4 h-4 inline" />
          </button>
        </div>
      )}

      <div className="panel-content flex-1 flex overflow-hidden">
        <div className="character-list-container w-1/3 border-r flex flex-col">
          {renderCharacterList()}
        </div>
        <div className="character-detail-container w-2/3 flex flex-col">
          {renderCharacterDetail()}
        </div>
      </div>

      {createDialogOpen && (
        <div className="dialog-overlay fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="dialog bg-white rounded-lg shadow-lg w-full max-w-md p-4">
            <div className="dialog-header flex items-center justify-between mb-4">
              <h3 className="font-medium">创建角色圣经</h3>
              <button
                onClick={() => setCreateDialogOpen(false)}
                className="text-gray-400 hover:text-gray-600"
              >
                <X className="w-5 h-5" />
              </button>
            </div>
            <div className="dialog-body space-y-3">
              <div>
                <label className="block text-sm font-medium text-gray-600 mb-1">角色名称 *</label>
                <input
                  type="text"
                  value={createForm.name}
                  onChange={(e) => setCreateForm({ ...createForm, name: e.target.value })}
                  className="border rounded px-2 py-1 w-full"
                  placeholder="输入角色名称"
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-600 mb-1">类型</label>
                <select
                  value={createForm.type}
                  onChange={(e) => setCreateForm({ ...createForm, type: e.target.value })}
                  className="border rounded px-2 py-1 w-full"
                >
                  <option value="human">人类</option>
                  <option value="animal">动物</option>
                  <option value="fantasy">奇幻生物</option>
                  <option value="robot">机器人</option>
                  <option value="other">其他</option>
                </select>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-600 mb-1">视觉特征</label>
                <textarea
                  value={createForm.visual_traits}
                  onChange={(e) => setCreateForm({ ...createForm, visual_traits: e.target.value })}
                  className="border rounded px-2 py-1 w-full h-20 resize-none"
                  placeholder="描述角色的外观特征..."
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-600 mb-1">性格描述</label>
                <textarea
                  value={createForm.personality}
                  onChange={(e) => setCreateForm({ ...createForm, personality: e.target.value })}
                  className="border rounded px-2 py-1 w-full h-16 resize-none"
                  placeholder="描述角色的性格特点..."
                />
              </div>
            </div>
            <div className="dialog-footer flex justify-end gap-2 mt-4">
              <button
                onClick={() => setCreateDialogOpen(false)}
                className="px-4 py-2 bg-gray-100 rounded hover:bg-gray-200"
              >
                取消
              </button>
              <button
                onClick={handleCreateCharacter}
                className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
                disabled={loading || !createForm.name.trim()}
              >
                创建
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default CharacterBiblePanel;
