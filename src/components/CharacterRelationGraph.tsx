import React, { useCallback, useState, useEffect, useMemo } from "react";
import { Plus, X, Link2, RotateCw, Sparkles, Loader2 } from "lucide-react";
import ReactFlow, {
  Node,
  Edge,
  addEdge,
  useNodesState,
  useEdgesState,
  Connection,
  EdgeLabelRenderer,
  getBezierPath,
  Position,
} from "reactflow";
import "reactflow/dist/style.css";
import {
  CharacterGraph,
  CharacterNode as CharNode,
  CharacterEdge as CharEdge,
  Character,
  GeneratedRelation,
} from "../types";
import { invoke } from "@tauri-apps/api/core";
import { CharacterNode as CustomCharacterNode } from "./CharacterNode";
import { aiGeneratorService, relationService } from "../services/api";

const nodeTypes = {
  custom: CustomCharacterNode,
};

interface CharacterRelationGraphProps {
  projectId: string;
  characters: Character[];
}

const RELATION_TYPES = [
  { id: "friend", label: "朋友", color: "#3b82f6" },
  { id: "enemy", label: "敌对", color: "#ef4444" },
  { id: "family", label: "家人", color: "#22c55e" },
  { id: "lover", label: "恋人", color: "#ec4899" },
  { id: "mentor", label: "师徒", color: "#8b5cf6" },
  { id: "ally", label: "盟友", color: "#14b8a6" },
  { id: "rival", label: "对手", color: "#f97316" },
  { id: "other", label: "其他", color: "#6b7280" },
];

export function CharacterRelationGraph({ projectId, characters }: CharacterRelationGraphProps) {
  const [nodes, setNodes, onNodesChange] = useNodesState([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState([]);
  const [graphData, setGraphData] = useState<CharacterGraph | null>(null);
  const [selectedEdge, setSelectedEdge] = useState<Edge | null>(null);
  const [showRelationDialog, setShowRelationDialog] = useState(false);
  const [editingRelation, setEditingRelation] = useState<CharEdge | null>(null);
  const [fromCharacter, setFromCharacter] = useState<string>("");
  const [toCharacter, setToCharacter] = useState<string>("");
  const [relationType, setRelationType] = useState("friend");
  const [description, setDescription] = useState("");
  const [loading, setLoading] = useState(false);

  // AI 生成关系状态
  const [isAIGenerating, setIsAIGenerating] = useState(false);
  const [showAIPreview, setShowAIPreview] = useState(false);
  const [generatedRelations, setGeneratedRelations] = useState<GeneratedRelation[]>([]);
  const [savingRelations, setSavingRelations] = useState(false);

  const onConnect = useCallback((params: Connection) => {
    if (!params.source || !params.target) return;

    setFromCharacter(params.source);
    setToCharacter(params.target);
    setRelationType("friend");
    setDescription("");
    setEditingRelation(null);
    setShowRelationDialog(true);
  }, []);

  const loadGraphData = async () => {
    console.log("loadGraphData called, projectId:", projectId);
    setLoading(true);
    try {
      const graph = await invoke<CharacterGraph>("get_character_graph", { projectId });
      console.log("Graph data loaded:", graph);
      setGraphData(graph);
      updateFlowGraph(graph, characters);
    } catch (error) {
      console.error("Failed to load graph data:", error);
    } finally {
      setLoading(false);
    }
  };

  const updateFlowGraph = useCallback(
    (graph: CharacterGraph, charList: Character[]) => {
      const flowNodes: Node[] = graph.nodes.map((node) => {
        const character = charList.find((c) => c.id === node.id);
        return {
          id: node.id,
          type: "custom",
          position: {
            x: Math.random() * 800,
            y: Math.random() * 600,
          },
          data: {
            label: node.name,
            avatar: node.avatar_url,
            character: character,
          },
        };
      });

      const flowEdges: Edge[] = graph.edges.map((edge) => {
        const typeInfo = RELATION_TYPES.find((t) => t.id === edge.label) || RELATION_TYPES[0];
        return {
          id: edge.id,
          source: edge.from,
          target: edge.to,
          label: typeInfo.label,
          labelStyle: {
            fontSize: 12,
            fill: "#374151",
          },
          style: {
            stroke: typeInfo.color,
            strokeWidth: 2,
          },
          data: edge,
        };
      });

      setNodes(flowNodes);
      setEdges(flowEdges);
    },
    [setNodes, setEdges]
  );

  useEffect(() => {
    loadGraphData();
  }, [projectId, characters]);

  const handleSaveRelation = async () => {
    if (!fromCharacter || !toCharacter) return;

    try {
      if (editingRelation) {
        await invoke("update_character_relation", {
          request: {
            id: editingRelation.id,
            relation_type: relationType,
            description: description || undefined,
          },
        });
      } else {
        await invoke("create_character_relation", {
          request: {
            project_id: projectId,
            from_character_id: fromCharacter,
            to_character_id: toCharacter,
            relation_type: relationType,
            description: description || undefined,
          },
        });
      }

      setShowRelationDialog(false);
      setEditingRelation(null);
      await loadGraphData();
    } catch (error) {
      console.error("Failed to save relation:", error);
      alert("保存失败");
    }
  };

  const handleDeleteRelation = async (edge: Edge) => {
    if (!edge.id) return;

    if (!confirm("确定要删除这个关系吗？")) {
      return;
    }

    try {
      await invoke("delete_character_relation", { id: edge.id });
      await loadGraphData();
    } catch (error) {
      console.error("Failed to delete relation:", error);
      alert("删除失败");
    }
  };

  const handleEditRelation = (edge: Edge) => {
    setSelectedEdge(edge);
    const edgeData = edge.data as CharEdge;
    setEditingRelation(edgeData);
    setFromCharacter(edgeData.from);
    setToCharacter(edgeData.to);
    setRelationType(edgeData.label);
    setDescription(edgeData.description || "");
    setShowRelationDialog(true);
  };

  const edgeLabelRenderer = useCallback(() => {
    return (
      <EdgeLabelRenderer>
        {edges.map((edge) => {
          const typeInfo = RELATION_TYPES.find((t) => t.id === edge.label);
          return (
            <div
              key={edge.id}
              className="px-2 py-1 bg-white dark:bg-slate-800 rounded shadow text-xs font-medium cursor-pointer hover:shadow-lg transition-shadow"
              onClick={(e) => {
                e.stopPropagation();
                handleEditRelation(edge);
              }}
            >
              <span
                className="inline-block w-2 h-2 rounded-full mr-1"
                style={{ backgroundColor: typeInfo?.color || "#6b7280" }}
              />
              {edge.label}
            </div>
          );
        })}
      </EdgeLabelRenderer>
    );
  }, [edges, handleEditRelation]);

  // AI 生成关系
  const handleAIGenerateRelations = async () => {
    if (characters.length < 2) {
      alert("需要至少 2 个角色才能生成关系");
      return;
    }

    setIsAIGenerating(true);
    try {
      const relations = await aiGeneratorService.generateCharacterRelations(projectId);
      setGeneratedRelations(relations);
      setShowAIPreview(true);
    } catch (error) {
      console.error("Failed to generate relations:", error);
      alert("生成关系失败");
    } finally {
      setIsAIGenerating(false);
    }
  };

  // 确认保存 AI 生成的关系
  const handleConfirmAIRelations = async () => {
    setSavingRelations(true);
    try {
      // 创建角色名称到ID的映射
      const characterMap = new Map(characters.map((c) => [c.name, c.id]));
      console.log("Character map:", Object.fromEntries(characterMap));
      console.log("Generated relations to save:", generatedRelations);

      // 获取现有关系，用于去重
      const existingRelations = graphData?.edges || [];
      const existingKeys = new Set(existingRelations.map((r) => `${r.from}-${r.to}-${r.label}`));

      let savedCount = 0;
      let skippedCount = 0;

      for (const relation of generatedRelations) {
        const fromId = characterMap.get(relation.from_character_name);
        const toId = characterMap.get(relation.to_character_name);

        if (fromId && toId) {
          const key = `${fromId}-${toId}-${relation.relation_type}`;
          if (existingKeys.has(key)) {
            console.log(
              `Skipping duplicate relation: ${relation.from_character_name} -> ${relation.to_character_name} (${relation.relation_type})`
            );
            skippedCount++;
            continue;
          }

          console.log(
            `Saving relation: ${relation.from_character_name} (${fromId}) -> ${relation.to_character_name} (${toId})`
          );
          await relationService.createRelation({
            project_id: projectId,
            from_character_id: fromId,
            to_character_id: toId,
            relation_type: relation.relation_type,
            description: relation.description,
          });
          savedCount++;
          console.log("Relation saved successfully");
        } else {
          console.warn(
            `Skipping relation: character not found - from: ${relation.from_character_name}, to: ${relation.to_character_name}`
          );
        }
      }

      console.log(`Total relations saved: ${savedCount}, skipped (duplicates): ${skippedCount}`);
      setShowAIPreview(false);
      setGeneratedRelations([]);
      await loadGraphData();
      console.log("Graph data reloaded");
    } catch (error) {
      console.error("Failed to save relations:", error);
      alert("保存关系失败: " + (error instanceof Error ? error.message : String(error)));
    } finally {
      setSavingRelations(false);
    }
  };

  return (
    <div className="h-full flex flex-col bg-slate-50 dark:bg-slate-900">
      <div className="flex items-center justify-between p-4 border-b border-slate-200 dark:border-slate-700 bg-white dark:bg-slate-800">
        <h3 className="font-semibold text-slate-900 dark:text-slate-100">角色关系图谱</h3>
        <div className="flex items-center gap-2">
          <button
            onClick={loadGraphData}
            disabled={loading}
            className="flex items-center gap-2 px-3 py-1.5 bg-slate-100 dark:bg-slate-700 hover:bg-slate-200 dark:hover:bg-slate-600 rounded text-sm font-medium transition-colors disabled:opacity-50"
          >
            <RotateCw className={`w-4 h-4 ${loading ? "animate-spin" : ""}`} />
            刷新
          </button>
          <button
            onClick={handleAIGenerateRelations}
            disabled={isAIGenerating || characters.length < 2}
            className="flex items-center gap-2 px-3 py-1.5 bg-purple-500 hover:bg-purple-600 text-white rounded text-sm font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {isAIGenerating ? (
              <>
                <Loader2 className="w-4 h-4 animate-spin" />
                生成中...
              </>
            ) : (
              <>
                <Sparkles className="w-4 h-4" />
                AI 生成关系
              </>
            )}
          </button>
          <button
            onClick={() => {
              setFromCharacter("");
              setToCharacter("");
              setRelationType("friend");
              setDescription("");
              setEditingRelation(null);
              setShowRelationDialog(true);
            }}
            className="flex items-center gap-2 px-3 py-1.5 bg-blue-500 hover:bg-blue-600 text-white rounded text-sm font-medium transition-colors"
          >
            <Plus className="w-4 h-4" />
            新建关系
          </button>
        </div>
      </div>

      <div className="flex-1 relative">
        <ReactFlow
          nodes={nodes}
          edges={edges}
          nodeTypes={nodeTypes}
          onNodesChange={onNodesChange}
          onEdgesChange={onEdgesChange}
          onConnect={onConnect}
          fitView
          className="bg-slate-50 dark:bg-slate-900"
        >
          {edgeLabelRenderer()}
        </ReactFlow>

        {showRelationDialog && (
          <div className="absolute top-4 right-4 w-80 bg-white dark:bg-slate-800 rounded-lg shadow-xl border border-slate-200 dark:border-slate-700">
            <div className="flex items-center justify-between p-4 border-b border-slate-200 dark:border-slate-700">
              <h4 className="font-semibold text-slate-900 dark:text-slate-100">
                {editingRelation ? "编辑关系" : "新建关系"}
              </h4>
              <button
                onClick={() => setShowRelationDialog(false)}
                className="p-1 hover:bg-slate-100 dark:hover:bg-slate-700 rounded"
              >
                <X className="w-5 h-5 text-slate-500" />
              </button>
            </div>

            <div className="p-4 space-y-4">
              <div>
                <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">
                  从角色
                </label>
                <select
                  value={fromCharacter}
                  onChange={(e) => setFromCharacter(e.target.value)}
                  className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-blue-500"
                >
                  <option value="">选择角色</option>
                  {characters.map((char) => (
                    <option key={char.id} value={char.id}>
                      {char.name}
                    </option>
                  ))}
                </select>
              </div>

              <div>
                <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">
                  到角色
                </label>
                <select
                  value={toCharacter}
                  onChange={(e) => setToCharacter(e.target.value)}
                  className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-blue-500"
                >
                  <option value="">选择角色</option>
                  {characters.map((char) => (
                    <option key={char.id} value={char.id}>
                      {char.name}
                    </option>
                  ))}
                </select>
              </div>

              <div>
                <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">
                  关系类型
                </label>
                <div className="grid grid-cols-2 gap-2">
                  {RELATION_TYPES.map((type) => (
                    <button
                      key={type.id}
                      type="button"
                      onClick={() => setRelationType(type.id)}
                      className={`p-2 rounded-lg text-left transition-colors ${
                        relationType === type.id
                          ? "border-2 border-blue-500 bg-blue-50 dark:bg-blue-900/30"
                          : "border-2 border-transparent bg-slate-50 dark:bg-slate-700 hover:bg-slate-100 dark:hover:bg-slate-600"
                      }`}
                    >
                      <div className="flex items-center gap-2">
                        <span
                          className="w-3 h-3 rounded-full"
                          style={{ backgroundColor: type.color }}
                        />
                        <span className="text-sm font-medium">{type.label}</span>
                      </div>
                    </button>
                  ))}
                </div>
              </div>

              <div>
                <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">
                  描述
                </label>
                <textarea
                  value={description}
                  onChange={(e) => setDescription(e.target.value)}
                  rows={3}
                  className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-blue-500 resize-none"
                  placeholder="描述这个关系"
                />
              </div>

              <div className="flex gap-2">
                <button
                  type="button"
                  onClick={() => setShowRelationDialog(false)}
                  className="flex-1 px-4 py-2 text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-700 rounded font-medium transition-colors"
                >
                  取消
                </button>
                <button
                  type="button"
                  onClick={handleSaveRelation}
                  disabled={!fromCharacter || !toCharacter || !relationType}
                  className="flex-1 flex items-center justify-center gap-2 px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  <Link2 className="w-4 h-4" />
                  {editingRelation ? "更新" : "创建"}
                </button>
              </div>
            </div>
          </div>
        )}

        {/* AI 生成关系预览对话框 */}
        {showAIPreview && (
          <div className="absolute top-4 right-4 w-96 bg-white dark:bg-slate-800 rounded-lg shadow-xl border border-slate-200 dark:border-slate-700 max-h-[80vh] overflow-hidden flex flex-col">
            <div className="flex items-center justify-between p-4 border-b border-slate-200 dark:border-slate-700">
              <div className="flex items-center gap-2">
                <Sparkles className="w-5 h-5 text-purple-500" />
                <h4 className="font-semibold text-slate-900 dark:text-slate-100">AI 生成的关系</h4>
              </div>
              <button
                onClick={() => {
                  setShowAIPreview(false);
                  setGeneratedRelations([]);
                }}
                className="p-1 hover:bg-slate-100 dark:hover:bg-slate-700 rounded"
              >
                <X className="w-5 h-5 text-slate-500" />
              </button>
            </div>

            <div className="flex-1 overflow-y-auto p-4 space-y-3">
              {generatedRelations.length === 0 ? (
                <p className="text-sm text-slate-500 text-center py-4">没有生成任何关系建议</p>
              ) : (
                generatedRelations.map((relation, index) => (
                  <div key={index} className="p-3 bg-slate-50 dark:bg-slate-700/50 rounded-lg">
                    <div className="flex items-center gap-2 text-sm">
                      <span className="font-medium text-slate-900 dark:text-slate-100">
                        {relation.from_character_name}
                      </span>
                      <span className="text-slate-400">→</span>
                      <span className="font-medium text-slate-900 dark:text-slate-100">
                        {relation.to_character_name}
                      </span>
                    </div>
                    <div className="mt-2">
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
                ))
              )}
            </div>

            <div className="p-4 border-t border-slate-200 dark:border-slate-700 flex gap-2">
              <button
                onClick={() => {
                  setShowAIPreview(false);
                  setGeneratedRelations([]);
                }}
                className="flex-1 px-4 py-2 text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-700 rounded font-medium transition-colors"
              >
                取消
              </button>
              <button
                onClick={handleConfirmAIRelations}
                disabled={savingRelations || generatedRelations.length === 0}
                className="flex-1 flex items-center justify-center gap-2 px-4 py-2 bg-purple-500 hover:bg-purple-600 text-white rounded font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {savingRelations ? (
                  <>
                    <Loader2 className="w-4 h-4 animate-spin" />
                    保存中...
                  </>
                ) : (
                  <>
                    <Link2 className="w-4 h-4" />
                    保存全部 ({generatedRelations.length})
                  </>
                )}
              </button>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
