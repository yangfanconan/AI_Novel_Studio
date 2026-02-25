import React, { useState, useEffect } from "react";
import { MoyinService, WorkflowTemplate } from "../services/moyin.service";

const moyinService = MoyinService.getInstance();

export default function ComfyUIPanel() {
  const [connected, setConnected] = useState(false);
  const [connectionMessage, setConnectionMessage] = useState("");
  const [templates, setTemplates] = useState<WorkflowTemplate[]>([]);
  const [selectedTemplate, setSelectedTemplate] = useState<WorkflowTemplate | null>(null);
  const [categoryFilter, setCategoryFilter] = useState("all");
  const [keywordFilter, setKeywordFilter] = useState("");
  const [isGenerating, setIsGenerating] = useState(false);
  const [generationLog, setGenerationLog] = useState<string[]>([]);
  const [activeTab, setActiveTab] = useState<"connection" | "templates" | "generate">("connection");

  const [generateParams, setGenerateParams] = useState({
    positivePrompt: "",
    negativePrompt: "",
    width: 1024,
    height: 1024,
    seed: -1,
    steps: 20,
    cfg: 7.0,
  });

  useEffect(() => {
    loadTemplates();
  }, []);

  const loadTemplates = async () => {
    try {
      const data = await moyinService.getWorkflowTemplates();
      setTemplates(data);
    } catch (error) {
      console.error("Failed to load templates:", error);
    }
  };

  const checkConnection = async () => {
    try {
      const result = await moyinService.comfyuiCheckConnection();
      setConnected(result.connected);
      setConnectionMessage(result.message);
      addLog(result.connected ? "Connected to ComfyUI" : "Connection failed");
    } catch (error) {
      setConnected(false);
      setConnectionMessage("Error checking connection");
      addLog("Error: " + (error as Error).message);
    }
  };

  const generateImage = async () => {
    if (!connected) {
      addLog("Error: Not connected to ComfyUI");
      return;
    }

    setIsGenerating(true);
    addLog("Starting image generation...");

    try {
      const imageUrl = await moyinService.comfyuiGenerateImage(
        generateParams.positivePrompt,
        generateParams.negativePrompt,
        generateParams.width,
        generateParams.height,
        generateParams.seed,
        generateParams.steps,
        generateParams.cfg
      );

      addLog(`Generation complete: ${imageUrl}`);
    } catch (error) {
      addLog("Generation failed: " + (error as Error).message);
    } finally {
      setIsGenerating(false);
    }
  };

  const applyTemplate = async (template: WorkflowTemplate) => {
    const variables: Record<string, unknown> = {
      positive_prompt: generateParams.positivePrompt,
      negative_prompt: generateParams.negativePrompt,
      width: generateParams.width,
      height: generateParams.height,
      seed: generateParams.seed,
      steps: generateParams.steps,
      cfg: generateParams.cfg,
    };

    try {
      const result = await moyinService.applyWorkflowTemplate(template.id, variables);
      addLog(`Template "${template.name}" applied successfully`);
      console.log("Applied template result:", result);
    } catch (error) {
      addLog("Failed to apply template: " + (error as Error).message);
    }
  };

  const toggleFavorite = async (templateId: string) => {
    try {
      const updated = await moyinService.toggleFavoriteWorkflowTemplate(templateId);
      setTemplates(templates.map((t) => (t.id === templateId ? updated : t)));
    } catch (error) {
      console.error("Failed to toggle favorite:", error);
    }
  };

  const addLog = (message: string) => {
    const timestamp = new Date().toLocaleTimeString();
    setGenerationLog((prev) => [...prev, `[${timestamp}] ${message}`]);
  };

  const filteredTemplates = templates.filter((template) => {
    const matchesCategory = categoryFilter === "all" || template.category === categoryFilter;
    const matchesKeyword =
      !keywordFilter ||
      template.name.toLowerCase().includes(keywordFilter.toLowerCase()) ||
      template.description.toLowerCase().includes(keywordFilter.toLowerCase()) ||
      template.tags.some((tag) => tag.toLowerCase().includes(keywordFilter.toLowerCase()));
    return matchesCategory && matchesKeyword;
  });

  const categories = Array.from(new Set(templates.map((t) => t.category)));

  return (
    <div className="p-4 h-full flex flex-col">
      <h2 className="text-2xl font-bold mb-4">ComfyUI Integration</h2>

      <div className="flex gap-2 mb-4 border-b pb-2">
        <button
          onClick={() => setActiveTab("connection")}
          className={`px-4 py-2 rounded ${activeTab === "connection" ? "bg-blue-500 text-white" : "bg-gray-200"}`}
        >
          Connection
        </button>
        <button
          onClick={() => setActiveTab("templates")}
          className={`px-4 py-2 rounded ${activeTab === "templates" ? "bg-blue-500 text-white" : "bg-gray-200"}`}
        >
          Templates ({templates.length})
        </button>
        <button
          onClick={() => setActiveTab("generate")}
          className={`px-4 py-2 rounded ${activeTab === "generate" ? "bg-blue-500 text-white" : "bg-gray-200"}`}
        >
          Generate
        </button>
      </div>

      <div className="flex-1 overflow-auto">
        {activeTab === "connection" && (
          <div className="space-y-4">
            <div className="p-4 border rounded">
              <h3 className="text-lg font-semibold mb-2">Connection Status</h3>
              <div
                className={`p-3 rounded ${connected ? "bg-green-100 text-green-800" : "bg-red-100 text-red-800"}`}
              >
                <div className="font-medium">{connected ? "Connected" : "Disconnected"}</div>
                <div className="text-sm mt-1">{connectionMessage}</div>
              </div>
              <button
                onClick={checkConnection}
                className="mt-3 px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
              >
                Check Connection
              </button>
            </div>

            <div className="p-4 border rounded">
              <h3 className="text-lg font-semibold mb-2">Quick Actions</h3>
              <div className="grid grid-cols-2 gap-2">
                <button
                  onClick={() => moyinService.comfyuiGetQueueInfo()}
                  className="px-4 py-2 bg-gray-200 rounded hover:bg-gray-300"
                >
                  Get Queue Info
                </button>
                <button
                  onClick={() => moyinService.comfyuiGetSystemInfo()}
                  className="px-4 py-2 bg-gray-200 rounded hover:bg-gray-300"
                >
                  Get System Info
                </button>
                <button
                  onClick={() => moyinService.comfyuiEmbeddingsList()}
                  className="px-4 py-2 bg-gray-200 rounded hover:bg-gray-300"
                >
                  List Embeddings
                </button>
                <button
                  onClick={() => moyinService.comfyuiFreeMemory()}
                  className="px-4 py-2 bg-gray-200 rounded hover:bg-gray-300"
                >
                  Free Memory
                </button>
              </div>
            </div>
          </div>
        )}

        {activeTab === "templates" && (
          <div className="space-y-4">
            <div className="flex gap-2">
              <input
                type="text"
                placeholder="Search templates..."
                value={keywordFilter}
                onChange={(e) => setKeywordFilter(e.target.value)}
                className="flex-1 px-3 py-2 border rounded"
              />
              <select
                value={categoryFilter}
                onChange={(e) => setCategoryFilter(e.target.value)}
                className="px-3 py-2 border rounded"
              >
                <option value="all">All Categories</option>
                {categories.map((cat) => (
                  <option key={cat} value={cat}>
                    {cat}
                  </option>
                ))}
              </select>
              <button
                onClick={() => moyinService.getBuiltInWorkflowTemplates()}
                className="px-4 py-2 bg-gray-200 rounded hover:bg-gray-300"
              >
                Load Built-in
              </button>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
              {filteredTemplates.map((template) => (
                <div
                  key={template.id}
                  className="border rounded p-4 hover:shadow-lg transition-shadow cursor-pointer"
                  onClick={() => setSelectedTemplate(template)}
                >
                  <div className="flex justify-between items-start mb-2">
                    <h4 className="font-semibold">{template.name}</h4>
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        toggleFavorite(template.id);
                      }}
                      className="text-xl"
                    >
                      {template.is_favorite ? "★" : "☆"}
                    </button>
                  </div>
                  <div className="text-sm text-gray-600 mb-2">{template.category}</div>
                  <div className="text-sm mb-2 line-clamp-2">{template.description}</div>
                  <div className="flex flex-wrap gap-1">
                    {template.tags.map((tag) => (
                      <span key={tag} className="text-xs px-2 py-1 bg-gray-100 rounded">
                        {tag}
                      </span>
                    ))}
                  </div>
                  <div className="text-xs text-gray-500 mt-2">
                    Used {template.usage_count} times
                  </div>
                  <button
                    onClick={(e) => {
                      e.stopPropagation();
                      applyTemplate(template);
                    }}
                    className="mt-2 w-full px-3 py-1 bg-blue-500 text-white text-sm rounded hover:bg-blue-600"
                  >
                    Apply Template
                  </button>
                </div>
              ))}
            </div>
          </div>
        )}

        {activeTab === "generate" && (
          <div className="space-y-4">
            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="block text-sm font-medium mb-1">Positive Prompt</label>
                <textarea
                  value={generateParams.positivePrompt}
                  onChange={(e) =>
                    setGenerateParams({ ...generateParams, positivePrompt: e.target.value })
                  }
                  rows={4}
                  className="w-full px-3 py-2 border rounded"
                  placeholder="Describe the image you want to generate..."
                />
              </div>
              <div>
                <label className="block text-sm font-medium mb-1">Negative Prompt</label>
                <textarea
                  value={generateParams.negativePrompt}
                  onChange={(e) =>
                    setGenerateParams({ ...generateParams, negativePrompt: e.target.value })
                  }
                  rows={4}
                  className="w-full px-3 py-2 border rounded"
                  placeholder="Describe what you don't want..."
                />
              </div>
            </div>

            <div className="grid grid-cols-4 gap-4">
              <div>
                <label className="block text-sm font-medium mb-1">Width</label>
                <input
                  type="number"
                  value={generateParams.width}
                  onChange={(e) =>
                    setGenerateParams({ ...generateParams, width: Number(e.target.value) })
                  }
                  className="w-full px-3 py-2 border rounded"
                />
              </div>
              <div>
                <label className="block text-sm font-medium mb-1">Height</label>
                <input
                  type="number"
                  value={generateParams.height}
                  onChange={(e) =>
                    setGenerateParams({ ...generateParams, height: Number(e.target.value) })
                  }
                  className="w-full px-3 py-2 border rounded"
                />
              </div>
              <div>
                <label className="block text-sm font-medium mb-1">Steps</label>
                <input
                  type="number"
                  value={generateParams.steps}
                  onChange={(e) =>
                    setGenerateParams({ ...generateParams, steps: Number(e.target.value) })
                  }
                  className="w-full px-3 py-2 border rounded"
                />
              </div>
              <div>
                <label className="block text-sm font-medium mb-1">CFG Scale</label>
                <input
                  type="number"
                  step="0.5"
                  value={generateParams.cfg}
                  onChange={(e) =>
                    setGenerateParams({ ...generateParams, cfg: Number(e.target.value) })
                  }
                  className="w-full px-3 py-2 border rounded"
                />
              </div>
            </div>

            <div>
              <label className="block text-sm font-medium mb-1">Seed (-1 for random)</label>
              <input
                type="number"
                value={generateParams.seed}
                onChange={(e) =>
                  setGenerateParams({ ...generateParams, seed: Number(e.target.value) })
                }
                className="w-full px-3 py-2 border rounded"
              />
            </div>

            <button
              onClick={generateImage}
              disabled={isGenerating || !connected}
              className={`w-full py-3 text-white font-semibold rounded ${
                isGenerating || !connected
                  ? "bg-gray-400 cursor-not-allowed"
                  : "bg-blue-500 hover:bg-blue-600"
              }`}
            >
              {isGenerating ? "Generating..." : "Generate Image"}
            </button>

            {generationLog.length > 0 && (
              <div className="border rounded p-3">
                <h4 className="font-medium mb-2">Generation Log</h4>
                <div className="bg-gray-100 p-2 rounded text-sm font-mono max-h-48 overflow-auto">
                  {generationLog.map((log, i) => (
                    <div key={i}>{log}</div>
                  ))}
                </div>
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
}
