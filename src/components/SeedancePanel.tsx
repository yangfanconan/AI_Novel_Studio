import React, { useState, useEffect } from "react";
import {
  MoyinService,
  SeedanceConstraints,
  SeedanceRequest,
  ValidationResult,
  FirstFrameGrid,
  Storyboard,
  StoryboardScene,
  StoryboardShot,
  VisualStyle,
  ShotType,
  CameraAngle,
  CameraMovement,
  NarrativeVideoConfig,
} from "../services/moyin.service";

const moyinService = MoyinService.getInstance();

export default function SeedancePanel() {
  const [constraints, setConstraints] = useState<SeedanceConstraints | null>(null);
  const [currentTab, setCurrentTab] = useState<"validate" | "prompt" | "grid" | "storyboard">(
    "validate"
  );

  const [request, setRequest] = useState<SeedanceRequest>({
    prompt: "",
    images: [],
    videos: [],
    audio: [],
    first_frame_grid: undefined,
    duration: 5.0,
    aspect_ratio: "16:9",
  });

  const [promptLayers, setPromptLayers] = useState({
    action: "",
    cinematography: "",
    dialogue: "",
  });

  const [grid, setGrid] = useState<FirstFrameGrid | null>(null);
  const [gridImages, setGridImages] = useState<string[]>(["", "", "", "", "", "", "", "", "", ""]);

  const [validationResult, setValidationResult] = useState<ValidationResult | null>(null);

  const [storyboard, setStoryboard] = useState<Storyboard | null>(null);

  const [narrativeConfig, setNarrativeConfig] = useState<NarrativeVideoConfig>({
    storyboard_id: "",
    custom_prompt: undefined,
    duration: 5.0,
    aspect_ratio: "16:9",
    include_audio: true,
    include_references: true,
  });

  useEffect(() => {
    loadConstraints();
  }, []);

  const loadConstraints = async () => {
    try {
      const data = await moyinService.seedanceGetConstraints();
      setConstraints(data);
    } catch (error) {
      console.error("Failed to load constraints:", error);
    }
  };

  const validateRequest = async () => {
    try {
      const result = await moyinService.seedanceValidateRequest(request);
      setValidationResult(result);
    } catch (error) {
      console.error("Validation failed:", error);
    }
  };

  const buildSmartPrompt = async () => {
    try {
      const prompt = await moyinService.seedanceBuildPrompt(
        promptLayers.action,
        promptLayers.cinematography,
        promptLayers.dialogue
      );
      setRequest({ ...request, prompt });
    } catch (error) {
      console.error("Failed to build prompt:", error);
    }
  };

  const createGrid = async () => {
    try {
      const validImages = gridImages.filter((img) => img.trim() !== "");
      const result = await moyinService.seedanceCreateGrid(validImages, 3, 3);
      setGrid(result);
      setRequest({ ...request, first_frame_grid: result });
    } catch (error) {
      alert("Failed to create grid: " + (error as Error).message);
    }
  };

  const prepareNarrativeVideo = async () => {
    if (!storyboard) {
      alert("Please select a storyboard first");
      return;
    }

    try {
      const prepared = await moyinService.seedancePrepareNarrativeVideo(
        storyboard,
        narrativeConfig
      );
      setRequest(prepared);
    } catch (error) {
      console.error("Failed to prepare narrative video:", error);
    }
  };

  const addReference = (type: "image" | "video" | "audio") => {
    const url = prompt(`Enter ${type} URL:`);
    if (url) {
      const newRef = {
        type,
        id: Date.now().toString(),
        url,
        description: "",
      };

      if (type === "image") {
        setRequest({ ...request, images: [...request.images, newRef] });
      } else if (type === "video") {
        setRequest({ ...request, videos: [...request.videos, newRef] });
      } else {
        setRequest({ ...request, audio: [...request.audio, newRef] });
      }
    }
  };

  const removeReference = (type: "image" | "video" | "audio", index: number) => {
    if (type === "image") {
      setRequest({
        ...request,
        images: request.images.filter((_, i) => i !== index),
      });
    } else if (type === "video") {
      setRequest({
        ...request,
        videos: request.videos.filter((_, i) => i !== index),
      });
    } else {
      setRequest({
        ...request,
        audio: request.audio.filter((_, i) => i !== index),
      });
    }
  };

  return (
    <div className="p-4 h-full flex flex-col">
      <h2 className="text-2xl font-bold mb-4">Seedance 2.0 - 多模态创作</h2>

      <div className="flex gap-2 mb-4 border-b pb-2">
        <button
          onClick={() => setCurrentTab("validate")}
          className={`px-4 py-2 rounded ${currentTab === "validate" ? "bg-blue-500 text-white" : "bg-gray-200"}`}
        >
          验证
        </button>
        <button
          onClick={() => setCurrentTab("prompt")}
          className={`px-4 py-2 rounded ${currentTab === "prompt" ? "bg-blue-500 text-white" : "bg-gray-200"}`}
        >
          智能提示词
        </button>
        <button
          onClick={() => setCurrentTab("grid")}
          className={`px-4 py-2 rounded ${currentTab === "grid" ? "bg-blue-500 text-white" : "bg-gray-200"}`}
        >
          首帧网格
        </button>
        <button
          onClick={() => setCurrentTab("storyboard")}
          className={`px-4 py-2 rounded ${currentTab === "storyboard" ? "bg-blue-500 text-white" : "bg-gray-200"}`}
        >
          叙事视频
        </button>
      </div>

      <div className="flex-1 overflow-auto">
        {currentTab === "validate" && (
          <div className="space-y-4">
            {constraints && (
              <div className="p-4 border rounded bg-blue-50">
                <h3 className="text-lg font-semibold mb-2">Seedance 2.0 约束条件</h3>
                <ul className="list-disc list-inside space-y-1 text-sm">
                  <li>
                    最大图片数: <strong>{constraints.max_images}</strong>
                  </li>
                  <li>
                    最大视频数: <strong>{constraints.max_videos}</strong>
                  </li>
                  <li>
                    最大音频数: <strong>{constraints.max_audio}</strong>
                  </li>
                  <li>
                    最大提示词长度: <strong>{constraints.max_prompt_length}</strong> 字符
                  </li>
                </ul>
              </div>
            )}

            <div className="p-4 border rounded">
              <h3 className="text-lg font-semibold mb-2">验证请求</h3>
              <div className="space-y-2">
                <div>
                  <label className="block text-sm font-medium mb-1">
                    提示词 ({request.prompt.length}/{constraints?.max_prompt_length || 0})
                  </label>
                  <textarea
                    value={request.prompt}
                    onChange={(e) => setRequest({ ...request, prompt: e.target.value })}
                    rows={4}
                    className="w-full px-3 py-2 border rounded"
                    placeholder="描述要生成的视频内容..."
                  />
                </div>

                <div className="grid grid-cols-3 gap-4">
                  <div>
                    <label className="block text-sm font-medium mb-1">
                      图片 ({request.images.length}/{constraints?.max_images || 0})
                    </label>
                    <div className="text-sm text-gray-600">
                      {request.images.length} / {constraints?.max_images || 0}
                    </div>
                  </div>
                  <div>
                    <label className="block text-sm font-medium mb-1">
                      视频 ({request.videos.length}/{constraints?.max_videos || 0})
                    </label>
                    <div className="text-sm text-gray-600">
                      {request.videos.length} / {constraints?.max_videos || 0}
                    </div>
                  </div>
                  <div>
                    <label className="block text-sm font-medium mb-1">
                      音频 ({request.audio.length}/{constraints?.max_audio || 0})
                    </label>
                    <div className="text-sm text-gray-600">
                      {request.audio.length} / {constraints?.max_audio || 0}
                    </div>
                  </div>
                </div>

                <div className="flex gap-2">
                  <button
                    onClick={validateRequest}
                    className="flex-1 px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
                  >
                    验证请求
                  </button>
                </div>

                {validationResult && (
                  <div
                    className={`p-3 rounded ${validationResult.valid ? "bg-green-100 text-green-800" : "bg-red-100 text-red-800"}`}
                  >
                    <div className="font-medium">
                      {validationResult.valid ? "验证通过" : "验证失败"}
                    </div>
                    {validationResult.errors.length > 0 && (
                      <ul className="list-disc list-inside mt-2 text-sm">
                        {validationResult.errors.map((err, i) => (
                          <li key={i} className="text-red-700">
                            {err}
                          </li>
                        ))}
                      </ul>
                    )}
                    {validationResult.warnings.length > 0 && (
                      <ul className="list-disc list-inside mt-2 text-sm">
                        {validationResult.warnings.map((warn, i) => (
                          <li key={i} className="text-yellow-700">
                            {warn}
                          </li>
                        ))}
                      </ul>
                    )}
                  </div>
                )}
              </div>
            </div>
          </div>
        )}

        {currentTab === "prompt" && (
          <div className="space-y-4">
            <div className="p-4 border rounded bg-yellow-50">
              <h3 className="text-lg font-semibold mb-2">智能提示词构建</h3>
              <p className="text-sm text-gray-700">三层融合: 动作描述 + 镜头语言 + 对白唇形同步</p>
            </div>

            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium mb-1">动作描述</label>
                <textarea
                  value={promptLayers.action}
                  onChange={(e) => setPromptLayers({ ...promptLayers, action: e.target.value })}
                  rows={3}
                  className="w-full px-3 py-2 border rounded"
                  placeholder="描述场景中的动作，例如：主角从左向右行走"
                />
              </div>

              <div>
                <label className="block text-sm font-medium mb-1">镜头语言</label>
                <textarea
                  value={promptLayers.cinematography}
                  onChange={(e) =>
                    setPromptLayers({ ...promptLayers, cinematography: e.target.value })
                  }
                  rows={3}
                  className="w-full px-3 py-2 border rounded"
                  placeholder="描述镜头运动，例如：慢速推进，从平视角度"
                />
              </div>

              <div>
                <label className="block text-sm font-medium mb-1">对白（用于唇形同步）</label>
                <textarea
                  value={promptLayers.dialogue}
                  onChange={(e) => setPromptLayers({ ...promptLayers, dialogue: e.target.value })}
                  rows={3}
                  className="w-full px-3 py-2 border rounded"
                  placeholder="角色对白，将用于视频中的唇形同步"
                />
              </div>

              <button
                onClick={buildSmartPrompt}
                className="w-full px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
              >
                构建提示词
              </button>

              {request.prompt && (
                <div className="p-4 border rounded bg-green-50">
                  <h4 className="font-medium mb-2">生成的提示词:</h4>
                  <p className="text-sm whitespace-pre-wrap">{request.prompt}</p>
                </div>
              )}
            </div>
          </div>
        )}

        {currentTab === "grid" && (
          <div className="space-y-4">
            <div className="p-4 border rounded bg-purple-50">
              <h3 className="text-lg font-semibold mb-2">首帧网格 (N×N 策略)</h3>
              <p className="text-sm text-gray-700">
                将多个分镜的首帧图拼接成网格，用于多镜头合并叙事视频生成
              </p>
            </div>

            <div className="grid grid-cols-3 gap-2">
              {gridImages.map((img, i) => (
                <div key={i}>
                  <label className="block text-xs text-gray-600 mb-1">位置 {i + 1}</label>
                  <input
                    type="text"
                    value={img}
                    onChange={(e) => {
                      const newImages = [...gridImages];
                      newImages[i] = e.target.value;
                      setGridImages(newImages);
                    }}
                    className="w-full px-2 py-1 border rounded text-sm"
                    placeholder="图片 URL"
                  />
                </div>
              ))}
            </div>

            <div className="flex gap-2">
              <button
                onClick={createGrid}
                className="flex-1 px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
              >
                创建 3×3 网格
              </button>
            </div>

            {grid && (
              <div className="p-4 border rounded bg-green-50">
                <h4 className="font-medium mb-2">网格配置:</h4>
                <div className="text-sm">
                  <p>
                    尺寸: {grid.rows} × {grid.cols}
                  </p>
                  <p>图片数: {grid.images.length}</p>
                </div>
              </div>
            )}
          </div>
        )}

        {currentTab === "storyboard" && (
          <div className="space-y-4">
            <div className="p-4 border rounded bg-indigo-50">
              <h3 className="text-lg font-semibold mb-2">叙事视频生成</h3>
              <p className="text-sm text-gray-700">
                从分镜故事板生成连贯的叙事视频，支持多模态引用
              </p>
            </div>

            <div className="space-y-4">
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-sm font-medium mb-1">持续时间 (秒)</label>
                  <input
                    type="number"
                    value={narrativeConfig.duration}
                    onChange={(e) =>
                      setNarrativeConfig({ ...narrativeConfig, duration: Number(e.target.value) })
                    }
                    className="w-full px-3 py-2 border rounded"
                    min="1"
                    max="60"
                    step="0.5"
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium mb-1">宽高比</label>
                  <select
                    value={narrativeConfig.aspect_ratio}
                    onChange={(e) =>
                      setNarrativeConfig({ ...narrativeConfig, aspect_ratio: e.target.value })
                    }
                    className="w-full px-3 py-2 border rounded"
                  >
                    <option value="16:9">16:9 (横屏)</option>
                    <option value="9:16">9:16 (竖屏)</option>
                    <option value="4:3">4:3</option>
                    <option value="1:1">1:1 (正方形)</option>
                  </select>
                </div>
              </div>

              <div className="space-y-2">
                <label className="flex items-center gap-2">
                  <input
                    type="checkbox"
                    checked={narrativeConfig.include_audio}
                    onChange={(e) =>
                      setNarrativeConfig({ ...narrativeConfig, include_audio: e.target.checked })
                    }
                  />
                  <span className="text-sm">包含音频引用</span>
                </label>
                <label className="flex items-center gap-2">
                  <input
                    type="checkbox"
                    checked={narrativeConfig.include_references}
                    onChange={(e) =>
                      setNarrativeConfig({
                        ...narrativeConfig,
                        include_references: e.target.checked,
                      })
                    }
                  />
                  <span className="text-sm">包含视觉/视频引用</span>
                </label>
              </div>

              <div>
                <label className="block text-sm font-medium mb-1">自定义提示词 (可选)</label>
                <textarea
                  value={narrativeConfig.custom_prompt || ""}
                  onChange={(e) =>
                    setNarrativeConfig({ ...narrativeConfig, custom_prompt: e.target.value })
                  }
                  rows={3}
                  className="w-full px-3 py-2 border rounded"
                  placeholder="留空则自动从分镜生成..."
                />
              </div>

              <button
                onClick={prepareNarrativeVideo}
                className="w-full px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
              >
                准备叙事视频请求
              </button>

              {request.prompt && (
                <div className="p-4 border rounded bg-green-50">
                  <h4 className="font-medium mb-2">准备好的请求:</h4>
                  <div className="text-sm space-y-2">
                    <p>
                      <strong>提示词:</strong> {request.prompt.substring(0, 200)}...
                    </p>
                    <p>
                      <strong>图片引用:</strong> {request.images.length}
                    </p>
                    <p>
                      <strong>视频引用:</strong> {request.videos.length}
                    </p>
                    <p>
                      <strong>音频引用:</strong> {request.audio.length}
                    </p>
                  </div>
                </div>
              )}
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
