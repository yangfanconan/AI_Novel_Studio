import React, { useState, useEffect, useCallback } from 'react';
import { PlotNode, PlotTree } from '../types/writingAssistant';
import { writingAssistantService } from '../services/writingAssistant.service';

interface PlotTreeViewProps {
  projectId: string;
  onNodeSelect?: (node: PlotNode) => void;
  onBranchFromNode?: (node: PlotNode) => void;
}

const PlotTreeView: React.FC<PlotTreeViewProps> = ({
  projectId,
  onNodeSelect,
  onBranchFromNode,
}) => {
  const [plotTree, setPlotTree] = useState<PlotTree | null>(null);
  const [loading, setLoading] = useState(true);
  const [selectedNodeId, setSelectedNodeId] = useState<string | null>(null);
  const [expandedNodes, setExpandedNodes] = useState<Set<string>>(new Set());

  useEffect(() => {
    loadPlotTree();
  }, [projectId]);

  const loadPlotTree = async () => {
    setLoading(true);
    try {
      const tree = await writingAssistantService.getPlotTree(projectId);
      setPlotTree(tree);
      // 默认展开所有根节点
      setExpandedNodes(new Set(tree.root_nodes));
    } catch (error) {
      console.error('Failed to load plot tree:', error);
    } finally {
      setLoading(false);
    }
  };

  const toggleNode = (nodeId: string) => {
    const newExpanded = new Set(expandedNodes);
    if (newExpanded.has(nodeId)) {
      newExpanded.delete(nodeId);
    } else {
      newExpanded.add(nodeId);
    }
    setExpandedNodes(newExpanded);
  };

  const handleNodeClick = (node: PlotNode) => {
    setSelectedNodeId(node.id);
    if (onNodeSelect) {
      onNodeSelect(node);
    }
  };

  const getChildNodes = (parentId: string): PlotNode[] => {
    if (!plotTree) return [];
    return plotTree.nodes.filter((n) => n.parent_node_id === parentId);
  };

  const renderNode = (node: PlotNode, depth: number = 0): React.ReactNode => {
    const children = getChildNodes(node.id);
    const isExpanded = expandedNodes.has(node.id);
    const isSelected = selectedNodeId === node.id;
    const hasChildren = children.length > 0;

    const getBranchColor = (branchName: string | null, isMainPath: boolean) => {
      if (isMainPath) return 'border-blue-400 bg-blue-50';
      if (!branchName) return 'border-gray-300 bg-gray-50';

      const colors = [
        'border-purple-400 bg-purple-50',
        'border-green-400 bg-green-50',
        'border-orange-400 bg-orange-50',
        'border-pink-400 bg-pink-50',
        'border-teal-400 bg-teal-50',
      ];
      const hash = branchName.split('').reduce((a, b) => a + b.charCodeAt(0), 0);
      return colors[hash % colors.length];
    };

    const getEmotionalIcon = (tone: string | null) => {
      if (!tone) return '📖';
      const toneMap: Record<string, string> = {
        紧张: '😰',
        温馨: '😊',
        悲伤: '😢',
        欢快: '😄',
        悬疑: '🤔',
        激烈: '🔥',
        平静: '😌',
        感动: '🥹',
      };
      return toneMap[tone] || '📖';
    };

    return (
      <div key={node.id} className="plot-node-container">
        <div
          className={`
            relative flex items-center gap-2 p-3 rounded-lg border-2 cursor-pointer
            transition-all duration-200 hover:shadow-md
            ${isSelected ? 'ring-2 ring-blue-500 ring-offset-2' : ''}
            ${getBranchColor(node.branch_name, node.is_main_path)}
          `}
          style={{ marginLeft: `${depth * 40}px` }}
          onClick={() => handleNodeClick(node)}
        >
          {hasChildren && (
            <button
              onClick={(e) => {
                e.stopPropagation();
                toggleNode(node.id);
              }}
              className="w-6 h-6 flex items-center justify-center rounded-full bg-white border border-gray-300 text-gray-500 hover:bg-gray-100"
            >
              {isExpanded ? '−' : '+'}
            </button>
          )}

          {!hasChildren && <div className="w-6" />}

          <span className="text-lg">{getEmotionalIcon(node.emotional_tone)}</span>

          <div className="flex-1 min-w-0">
            <div className="flex items-center gap-2">
              <span className="font-medium text-gray-800 truncate">{node.title}</span>
              {!node.is_main_path && node.branch_name && (
                <span className="text-xs px-2 py-0.5 bg-white/50 rounded text-gray-600">
                  {node.branch_name}
                </span>
              )}
            </div>
            <p className="text-xs text-gray-500 truncate mt-0.5">{node.summary}</p>
          </div>

          <div className="flex items-center gap-2 text-xs text-gray-400">
            <span>{node.word_count}字</span>
            {node.characters_involved.length > 0 && (
              <span className="flex items-center gap-1">
                👥 {node.characters_involved.length}
              </span>
            )}
          </div>

          {onBranchFromNode && (
            <button
              onClick={(e) => {
                e.stopPropagation();
                onBranchFromNode(node);
              }}
              className="px-2 py-1 text-xs bg-blue-100 text-blue-600 rounded hover:bg-blue-200 transition-colors"
            >
              分支
            </button>
          )}
        </div>

        {hasChildren && isExpanded && (
          <div className="relative">
            <div
              className="absolute left-4 top-0 w-0.5 h-full bg-gray-200"
              style={{ marginLeft: `${depth * 40 + 12}px` }}
            />
            <div className="mt-2 space-y-2">
              {children.map((child) => renderNode(child, depth + 1))}
            </div>
          </div>
        )}
      </div>
    );
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500" />
      </div>
    );
  }

  if (!plotTree || plotTree.nodes.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center h-64 text-gray-500">
        <span className="text-4xl mb-4">🌳</span>
        <p>暂无剧情节点</p>
        <p className="text-sm mt-2">写作时选择不同的续写方向将自动创建分支</p>
      </div>
    );
  }

  return (
    <div className="plot-tree-view p-4">
      <div className="flex items-center justify-between mb-4">
        <h3 className="text-lg font-semibold text-gray-800 flex items-center gap-2">
          <span>🌳</span> 剧情发展树
        </h3>
        <div className="flex items-center gap-2 text-sm text-gray-500">
          <span className="flex items-center gap-1">
            <div className="w-3 h-3 bg-blue-100 border border-blue-400 rounded" />
            主线
          </span>
          <span className="flex items-center gap-1">
            <div className="w-3 h-3 bg-purple-100 border border-purple-400 rounded" />
            分支
          </span>
        </div>
      </div>

      <div className="space-y-3">
        {plotTree.root_nodes.map((rootId) => {
          const rootNode = plotTree.nodes.find((n) => n.id === rootId);
          if (!rootNode) return null;
          return renderNode(rootNode);
        })}
      </div>

      <div className="mt-4 pt-4 border-t border-gray-200">
        <div className="flex items-center justify-between text-sm text-gray-500">
          <span>共 {plotTree.nodes.length} 个节点</span>
          <span>
            {plotTree.root_nodes.length} 条{' '}
            {plotTree.root_nodes.length > 1 ? '故事线' : '故事线'}
          </span>
        </div>
      </div>
    </div>
  );
};

export default PlotTreeView;
