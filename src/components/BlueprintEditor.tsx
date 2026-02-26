import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Globe, Users, Network, Map, Plus, Trash2, Save, Sparkles, Loader2, RefreshCw } from "lucide-react";
import type { Blueprint, BlueprintCharacter, BlueprintRelationship, BlueprintSetting } from "../types";

interface BlueprintEditorProps {
  projectId: string;
  onClose?: () => void;
}

export const BlueprintEditor: React.FC<BlueprintEditorProps> = ({ projectId, onClose }) => {
  const [blueprint, setBlueprint] = useState<Blueprint | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [isSaving, setIsSaving] = useState(false);
  const [activeTab, setActiveTab] = useState<"overview" | "characters" | "relationships" | "settings">("overview");
  const [isCreating, setIsCreating] = useState(false);

  const loadBlueprint = async () => {
    setIsLoading(true);
    try {
      const result = await invoke<Blueprint | null>("get_blueprint", { projectId });
      setBlueprint(result);
    } catch (error) {
      console.error("加载蓝图失败:", error);
      alert("加载蓝图失败: " + (error as Error).message);
    } finally {
      setIsLoading(false);
    }
  };

  const createBlueprint = async () => {
    setIsCreating(true);
    try {
      const title = prompt("请输入故事标题:");
      if (!title) return;

      const result = await invoke<Blueprint>("create_blueprint", {
        request: {
          project_id: projectId,
          title,
          genre: undefined,
          target_length: undefined,
        },
      });

      setBlueprint(result);
    } catch (error) {
      console.error("创建蓝图失败:", error);
      alert("创建蓝图失败: " + (error as Error).message);
    } finally {
      setIsCreating(false);
    }
  };

  const saveBlueprint = async () => {
    if (!blueprint) return;

    setIsSaving(true);
    try {
      const result = await invoke<Blueprint>("update_blueprint", {
        request: {
          blueprint_id: blueprint.id,
          title: blueprint.title,
          genre: blueprint.genre,
          target_length: blueprint.target_length,
          characters: blueprint.characters,
          relationships: blueprint.relationships,
          settings: blueprint.settings,
        },
      });
      setBlueprint(result);
      alert("蓝图保存成功!");
    } catch (error) {
      console.error("保存蓝图失败:", error);
      alert("保存蓝图失败: " + (error as Error).message);
    } finally {
      setIsSaving(false);
    }
  };

  useEffect(() => {
    loadBlueprint();
  }, [projectId]);

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-full">
        <Loader2 className="w-8 h-8 animate-spin" />
      </div>
    );
  }

  if (!blueprint) {
    return (
      <div className="flex flex-col items-center justify-center h-full gap-4">
        <div className="text-center space-y-2">
          <Globe className="w-16 h-16 text-muted-foreground mx-auto" />
          <h3 className="text-lg font-medium">尚未创建项目蓝图</h3>
          <p className="text-sm text-muted-foreground max-w-md">
            项目蓝图是AI创作的基础，它整合了角色、关系和世界观设定，
            为后续的章节生成提供全知视角的上下文。
          </p>
        </div>
        <button
          onClick={createBlueprint}
          disabled={isCreating}
          className="px-6 py-3 bg-primary text-primary-foreground rounded-md hover:bg-primary/90 disabled:opacity-50 transition-colors flex items-center gap-2"
        >
          {isCreating ? (
            <>
              <Loader2 className="w-4 h-4 animate-spin" />
              创建中...
            </>
          ) : (
            <>
              <Sparkles className="w-4 h-4" />
              创建蓝图
            </>
          )}
        </button>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full">
      <div className="p-4 border-b border-border bg-muted/30">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <Globe className="w-5 h-5 text-primary" />
            <h2 className="text-lg font-semibold">项目蓝图</h2>
            <span className="text-xs text-muted-foreground">L1规划层</span>
          </div>
          <div className="flex items-center gap-2">
            <button
              onClick={loadBlueprint}
              className="p-2 hover:bg-accent rounded-md transition-colors"
              title="刷新"
            >
              <RefreshCw className="w-4 h-4" />
            </button>
            <button
              onClick={saveBlueprint}
              disabled={isSaving}
              className="px-4 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90 disabled:opacity-50 transition-colors flex items-center gap-2"
            >
              {isSaving ? (
                <>
                  <Loader2 className="w-4 h-4 animate-spin" />
                  保存中...
                </>
              ) : (
                <>
                  <Save className="w-4 h-4" />
                  保存
                </>
              )}
            </button>
            {onClose && (
              <button
                onClick={onClose}
                className="px-4 py-2 bg-muted text-muted-foreground rounded-md hover:bg-muted/80 transition-colors"
              >
                关闭
              </button>
            )}
          </div>
        </div>
      </div>

      <div className="flex-1 flex overflow-hidden">
        <div className="w-64 border-r border-border bg-muted/30">
          <nav className="p-2 space-y-1">
            <button
              onClick={() => setActiveTab("overview")}
              className={`w-full text-left px-3 py-2 rounded-md transition-colors flex items-center gap-2 ${
                activeTab === "overview"
                  ? "bg-primary text-primary-foreground"
                  : "hover:bg-accent"
              }`}
            >
              <Globe className="w-4 h-4" />
              概览
            </button>
            <button
              onClick={() => setActiveTab("characters")}
              className={`w-full text-left px-3 py-2 rounded-md transition-colors flex items-center gap-2 ${
                activeTab === "characters"
                  ? "bg-primary text-primary-foreground"
                  : "hover:bg-accent"
              }`}
            >
              <Users className="w-4 h-4" />
              角色蓝图
              {blueprint.characters.length > 0 && (
                <span className="ml-auto text-xs bg-primary-foreground/20 px-2 rounded-full">
                  {blueprint.characters.length}
                </span>
              )}
            </button>
            <button
              onClick={() => setActiveTab("relationships")}
              className={`w-full text-left px-3 py-2 rounded-md transition-colors flex items-center gap-2 ${
                activeTab === "relationships"
                  ? "bg-primary text-primary-foreground"
                  : "hover:bg-accent"
              }`}
            >
              <Network className="w-4 h-4" />
              关系蓝图
              {blueprint.relationships.length > 0 && (
                <span className="ml-auto text-xs bg-primary-foreground/20 px-2 rounded-full">
                  {blueprint.relationships.length}
                </span>
              )}
            </button>
            <button
              onClick={() => setActiveTab("settings")}
              className={`w-full text-left px-3 py-2 rounded-md transition-colors flex items-center gap-2 ${
                activeTab === "settings"
                  ? "bg-primary text-primary-foreground"
                  : "hover:bg-accent"
              }`}
            >
              <Map className="w-4 h-4" />
              设定蓝图
              {blueprint.settings.length > 0 && (
                <span className="ml-auto text-xs bg-primary-foreground/20 px-2 rounded-full">
                  {blueprint.settings.length}
                </span>
              )}
            </button>
          </nav>
        </div>

        <div className="flex-1 overflow-y-auto p-6">
          {activeTab === "overview" && (
            <div className="space-y-6">
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-sm font-medium mb-2">故事标题</label>
                  <input
                    type="text"
                    value={blueprint.title}
                    onChange={(e) =>
                      setBlueprint({ ...blueprint, title: e.target.value })
                    }
                    className="w-full px-3 py-2 border border-input rounded-md bg-background"
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium mb-2">类型</label>
                  <input
                    type="text"
                    value={blueprint.genre || ""}
                    onChange={(e) =>
                      setBlueprint({ ...blueprint, genre: e.target.value })
                    }
                    className="w-full px-3 py-2 border border-input rounded-md bg-background"
                    placeholder="例如：玄幻、都市、科幻..."
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium mb-2">目标字数</label>
                  <input
                    type="number"
                    value={blueprint.target_length || ""}
                    onChange={(e) =>
                      setBlueprint({
                        ...blueprint,
                        target_length: e.target.value
                          ? parseInt(e.target.value)
                          : undefined,
                      })
                    }
                    className="w-full px-3 py-2 border border-input rounded-md bg-background"
                    placeholder="例如：100000"
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium mb-2">创建时间</label>
                  <div className="px-3 py-2 border border-input rounded-md bg-muted/50 text-sm">
                    {new Date(blueprint.created_at).toLocaleString()}
                  </div>
                </div>
              </div>

              <div className="grid grid-cols-3 gap-4 pt-4 border-t border-border">
                <div className="p-4 bg-blue-50 dark:bg-blue-950/30 rounded-lg">
                  <div className="flex items-center gap-2 mb-2">
                    <Users className="w-5 h-5 text-blue-500" />
                    <span className="text-sm font-medium">角色数</span>
                  </div>
                  <div className="text-2xl font-bold text-blue-600 dark:text-blue-400">
                    {blueprint.characters.length}
                  </div>
                </div>
                <div className="p-4 bg-green-50 dark:bg-green-950/30 rounded-lg">
                  <div className="flex items-center gap-2 mb-2">
                    <Network className="w-5 h-5 text-green-500" />
                    <span className="text-sm font-medium">关系数</span>
                  </div>
                  <div className="text-2xl font-bold text-green-600 dark:text-green-400">
                    {blueprint.relationships.length}
                  </div>
                </div>
                <div className="p-4 bg-purple-50 dark:bg-purple-950/30 rounded-lg">
                  <div className="flex items-center gap-2 mb-2">
                    <Map className="w-5 h-5 text-purple-500" />
                    <span className="text-sm font-medium">设定数</span>
                  </div>
                  <div className="text-2xl font-bold text-purple-600 dark:text-purple-400">
                    {blueprint.settings.length}
                  </div>
                </div>
              </div>
            </div>
          )}

          {activeTab === "characters" && (
            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <h3 className="text-lg font-semibold">角色蓝图</h3>
                <button
                  onClick={() => {
                    const name = prompt("请输入角色名:");
                    if (name) {
                      setBlueprint({
                        ...blueprint,
                        characters: [
                          ...blueprint.characters,
                          {
                            name,
                            is_main_character: false,
                          },
                        ],
                      });
                    }
                  }}
                  className="px-3 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90 transition-colors flex items-center gap-2"
                >
                  <Plus className="w-4 h-4" />
                  添加角色
                </button>
              </div>

              <div className="space-y-3">
                {blueprint.characters.map((character, index) => (
                  <div
                    key={index}
                    className={`p-4 rounded-lg border-2 ${
                      character.is_main_character
                        ? "border-yellow-400 bg-yellow-50 dark:bg-yellow-950/30"
                        : "border-border bg-card"
                    }`}
                  >
                    <div className="flex items-start justify-between mb-3">
                      <div className="flex items-center gap-2">
                        {character.is_main_character && (
                          <span className="px-2 py-0.5 bg-yellow-400 text-yellow-900 text-xs font-medium rounded-full">
                            主角
                          </span>
                        )}
                        <h4 className="font-semibold text-lg">{character.name}</h4>
                      </div>
                      <button
                        onClick={() => {
                          if (confirm(`确定删除角色 ${character.name}?`)) {
                            setBlueprint({
                              ...blueprint,
                              characters: blueprint.characters.filter(
                                (_, i) => i !== index
                              ),
                            });
                          }
                        }}
                        className="p-1 text-red-500 hover:bg-red-50 dark:hover:bg-red-950/30 rounded"
                      >
                        <Trash2 className="w-4 h-4" />
                      </button>
                    </div>

                    <div className="grid grid-cols-2 gap-3">
                      <div>
                        <label className="block text-xs font-medium mb-1 text-muted-foreground">
                          角色定位
                        </label>
                        <input
                          type="text"
                          value={character.role || ""}
                          onChange={(e) => {
                            const newCharacters = [...blueprint.characters];
                            newCharacters[index].role = e.target.value;
                            setBlueprint({
                              ...blueprint,
                              characters: newCharacters,
                            });
                          }}
                          className="w-full px-2 py-1.5 text-sm border border-input rounded-md bg-background"
                          placeholder="主角/反派/配角..."
                        />
                      </div>
                      <div>
                        <label className="block text-xs font-medium mb-1 text-muted-foreground">
                          角色弧类型
                        </label>
                        <input
                          type="text"
                          value={character.arc_type || ""}
                          onChange={(e) => {
                            const newCharacters = [...blueprint.characters];
                            newCharacters[index].arc_type = e.target.value;
                            setBlueprint({
                              ...blueprint,
                              characters: newCharacters,
                            });
                          }}
                          className="w-full px-2 py-1.5 text-sm border border-input rounded-md bg-background"
                          placeholder="成长型/救赎型/悲剧型..."
                        />
                      </div>
                    </div>

                    <div className="mt-3">
                      <label className="block text-xs font-medium mb-1 text-muted-foreground">
                        性格描述
                      </label>
                      <textarea
                        value={character.personality || ""}
                        onChange={(e) => {
                          const newCharacters = [...blueprint.characters];
                          newCharacters[index].personality = e.target.value;
                          setBlueprint({
                            ...blueprint,
                            characters: newCharacters,
                          });
                        }}
                        className="w-full px-2 py-1.5 text-sm border border-input rounded-md bg-background resize-none"
                        rows={2}
                        placeholder="描述角色的性格特点..."
                      />
                    </div>

                    <div className="mt-3">
                      <label className="block text-xs font-medium mb-1 text-muted-foreground">
                        背景故事
                      </label>
                      <textarea
                        value={character.background || ""}
                        onChange={(e) => {
                          const newCharacters = [...blueprint.characters];
                          newCharacters[index].background = e.target.value;
                          setBlueprint({
                            ...blueprint,
                            characters: newCharacters,
                          });
                        }}
                        className="w-full px-2 py-1.5 text-sm border border-input rounded-md bg-background resize-none"
                        rows={2}
                        placeholder="描述角色的背景故事..."
                      />
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )}

          {activeTab === "relationships" && (
            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <h3 className="text-lg font-semibold">关系蓝图</h3>
                <button
                  onClick={() => {
                    const from = prompt("角色A:");
                    const to = prompt("角色B:");
                    const type = prompt("关系类型:");
                    if (from && to && type) {
                      setBlueprint({
                        ...blueprint,
                        relationships: [
                          ...blueprint.relationships,
                          { from, to, relationship_type: type },
                        ],
                      });
                    }
                  }}
                  className="px-3 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90 transition-colors flex items-center gap-2"
                >
                  <Plus className="w-4 h-4" />
                  添加关系
                </button>
              </div>

              <div className="space-y-3">
                {blueprint.relationships.map((rel, index) => (
                  <div
                    key={index}
                    className="p-4 rounded-lg border border-border bg-card"
                  >
                    <div className="flex items-start justify-between mb-3">
                      <div className="flex items-center gap-3">
                        <div className="px-3 py-1.5 bg-blue-100 dark:bg-blue-900 rounded-md text-blue-900 dark:text-blue-100 font-medium">
                          {rel.from}
                        </div>
                        <Network className="w-4 h-4 text-muted-foreground" />
                        <div className="px-3 py-1.5 bg-purple-100 dark:bg-purple-900 rounded-md text-purple-900 dark:text-purple-100 font-medium">
                          {rel.to}
                        </div>
                      </div>
                      <button
                        onClick={() => {
                          if (confirm("确定删除此关系?")) {
                            setBlueprint({
                              ...blueprint,
                              relationships: blueprint.relationships.filter(
                                (_, i) => i !== index
                              ),
                            });
                          }
                        }}
                        className="p-1 text-red-500 hover:bg-red-50 dark:hover:bg-red-950/30 rounded"
                      >
                        <Trash2 className="w-4 h-4" />
                      </button>
                    </div>

                    <div>
                      <label className="block text-xs font-medium mb-1 text-muted-foreground">
                        关系类型
                      </label>
                      <input
                        type="text"
                        value={rel.relationship_type}
                        onChange={(e) => {
                          const newRelationships = [...blueprint.relationships];
                          newRelationships[index].relationship_type =
                            e.target.value;
                          setBlueprint({
                            ...blueprint,
                            relationships: newRelationships,
                          });
                        }}
                        className="w-full px-2 py-1.5 text-sm border border-input rounded-md bg-background"
                      />
                    </div>

                    <div className="mt-2">
                      <label className="block text-xs font-medium mb-1 text-muted-foreground">
                        关系描述
                      </label>
                      <textarea
                        value={rel.description || ""}
                        onChange={(e) => {
                          const newRelationships = [...blueprint.relationships];
                          newRelationships[index].description =
                            e.target.value;
                          setBlueprint({
                            ...blueprint,
                            relationships: newRelationships,
                          });
                        }}
                        className="w-full px-2 py-1.5 text-sm border border-input rounded-md bg-background resize-none"
                        rows={2}
                        placeholder="描述这个关系的细节..."
                      />
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )}

          {activeTab === "settings" && (
            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <h3 className="text-lg font-semibold">设定蓝图</h3>
                <button
                  onClick={() => {
                    const category = prompt("类别:");
                    const name = prompt("设定名称:");
                    if (category && name) {
                      setBlueprint({
                        ...blueprint,
                        settings: [
                          ...blueprint.settings,
                          { category, name },
                        ],
                      });
                    }
                  }}
                  className="px-3 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90 transition-colors flex items-center gap-2"
                >
                  <Plus className="w-4 h-4" />
                  添加设定
                </button>
              </div>

              <div className="space-y-3">
                {blueprint.settings.map((setting, index) => (
                  <div
                    key={index}
                    className="p-4 rounded-lg border border-border bg-card"
                  >
                    <div className="flex items-start justify-between mb-3">
                      <div className="flex items-center gap-3">
                        <div className="px-3 py-1.5 bg-orange-100 dark:bg-orange-900 rounded-md text-orange-900 dark:text-orange-100 text-xs font-medium">
                          {setting.category}
                        </div>
                        <h4 className="font-semibold">{setting.name}</h4>
                      </div>
                      <button
                        onClick={() => {
                          if (confirm(`确定删除设定 ${setting.name}?`)) {
                            setBlueprint({
                              ...blueprint,
                              settings: blueprint.settings.filter(
                                (_, i) => i !== index
                              ),
                            });
                          }
                        }}
                        className="p-1 text-red-500 hover:bg-red-50 dark:hover:bg-red-950/30 rounded"
                      >
                        <Trash2 className="w-4 h-4" />
                      </button>
                    </div>

                    <div className="mt-3">
                      <label className="block text-xs font-medium mb-1 text-muted-foreground">
                        描述
                      </label>
                      <textarea
                        value={setting.description || ""}
                        onChange={(e) => {
                          const newSettings = [...blueprint.settings];
                          newSettings[index].description = e.target.value;
                          setBlueprint({
                            ...blueprint,
                            settings: newSettings,
                          });
                        }}
                        className="w-full px-2 py-1.5 text-sm border border-input rounded-md bg-background resize-none"
                        rows={2}
                        placeholder="简要描述这个设定..."
                      />
                    </div>

                    <div className="mt-2">
                      <label className="block text-xs font-medium mb-1 text-muted-foreground">
                        详细说明
                      </label>
                      <textarea
                        value={setting.details || ""}
                        onChange={(e) => {
                          const newSettings = [...blueprint.settings];
                          newSettings[index].details = e.target.value;
                          setBlueprint({
                            ...blueprint,
                            settings: newSettings,
                          });
                        }}
                        className="w-full px-2 py-1.5 text-sm border border-input rounded-md bg-background resize-none"
                        rows={3}
                        placeholder="详细说明这个设定的各个方面..."
                      />
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
