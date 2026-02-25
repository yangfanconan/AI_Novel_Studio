import React, { useState, useEffect } from "react";
import { MoyinService, WorkflowTemplate, CreateTemplateRequest } from "../services/moyin.service";

const moyinService = MoyinService.getInstance();

interface WorkflowTemplateEditorProps {
  templateId?: string;
  onSave?: (template: WorkflowTemplate) => void;
  onCancel?: () => void;
}

export default function WorkflowTemplateEditor({
  templateId,
  onSave,
  onCancel,
}: WorkflowTemplateEditorProps) {
  const [template, setTemplate] = useState<CreateTemplateRequest>({
    name: "",
    category: "General",
    description: "",
    workflow_json: "{}",
    preview_image: "",
    tags: [],
  });

  const [existingTemplate, setExistingTemplate] = useState<WorkflowTemplate | null>(null);
  const [tagInput, setTagInput] = useState("");
  const [isValidJson, setIsValidJson] = useState(true);
  const [isSaving, setIsSaving] = useState(false);
  const [error, setError] = useState("");
  const [jsonError, setJsonError] = useState("");

  useEffect(() => {
    if (templateId) {
      loadTemplate(templateId);
    }
  }, [templateId]);

  const loadTemplate = async (id: string) => {
    try {
      const data = await moyinService.getWorkflowTemplate(id);
      if (data) {
        setExistingTemplate(data);
        setTemplate({
          name: data.name,
          category: data.category,
          description: data.description,
          workflow_json: data.workflow_json,
          preview_image: data.preview_image || "",
          tags: data.tags,
        });
      }
    } catch (error) {
      setError("Failed to load template: " + (error as Error).message);
    }
  };

  const validateJson = (jsonStr: string): boolean => {
    try {
      JSON.parse(jsonStr);
      setJsonError("");
      return true;
    } catch (e) {
      setJsonError((e as Error).message);
      return false;
    }
  };

  const handleWorkflowJsonChange = (value: string) => {
    setTemplate({ ...template, workflow_json: value });
    setIsValidJson(validateJson(value));
  };

  const handleAddTag = () => {
    if (tagInput.trim() && !template.tags.includes(tagInput.trim())) {
      setTemplate({
        ...template,
        tags: [...template.tags, tagInput.trim()],
      });
      setTagInput("");
    }
  };

  const handleRemoveTag = (tag: string) => {
    setTemplate({
      ...template,
      tags: template.tags.filter((t) => t !== tag),
    });
  };

  const handleSave = async () => {
    if (!template.name.trim()) {
      setError("Template name is required");
      return;
    }

    if (!isValidJson) {
      setError("Invalid workflow JSON");
      return;
    }

    setIsSaving(true);
    setError("");

    try {
      let savedTemplate: WorkflowTemplate;

      if (existingTemplate) {
        savedTemplate = await moyinService.updateWorkflowTemplate(existingTemplate.id, template);
      } else {
        savedTemplate = await moyinService.createWorkflowTemplate(template);
      }

      if (onSave) {
        onSave(savedTemplate);
      }
    } catch (error) {
      setError("Failed to save template: " + (error as Error).message);
    } finally {
      setIsSaving(false);
    }
  };

  const formatJson = () => {
    try {
      const parsed = JSON.parse(template.workflow_json);
      const formatted = JSON.stringify(parsed, null, 2);
      setTemplate({ ...template, workflow_json: formatted });
      setIsValidJson(true);
      setJsonError("");
    } catch (e) {
      setJsonError((e as Error).message);
      setIsValidJson(false);
    }
  };

  const createTxt2ImgTemplate = () => {
    const defaultWorkflow = {
      "3": {
        inputs: {
          seed: 0,
          steps: 20,
          cfg: 8,
          sampler_name: "euler",
          scheduler: "normal",
          denoise: 1,
          model: ["4", 0],
          positive: ["6", 0],
          negative: ["7", 0],
          latent_image: ["5", 0],
        },
        class_type: "KSampler",
      },
      "4": {
        inputs: {
          ckpt_name: "v1-5-pruned-emaonly.ckpt",
        },
        class_type: "CheckpointLoaderSimple",
      },
      "5": {
        inputs: {
          width: 512,
          height: 512,
          batch_size: 1,
        },
        class_type: "EmptyLatentImage",
      },
      "6": {
        inputs: {
          text: "{{positive_prompt}}",
          clip: ["4", 1],
        },
        class_type: "CLIPTextEncode",
      },
      "7": {
        inputs: {
          text: "{{negative_prompt}}",
          clip: ["4", 1],
        },
        class_type: "CLIPTextEncode",
      },
      "8": {
        inputs: {
          samples: ["3", 0],
          vae: ["4", 2],
        },
        class_type: "VAEDecode",
      },
      "9": {
        inputs: {
          filename_prefix: "ComfyUI",
          images: ["8", 0],
        },
        class_type: "SaveImage",
      },
    };

    setTemplate({
      ...template,
      workflow_json: JSON.stringify(defaultWorkflow, null, 2),
      name: "Text to Image",
      category: "Image Generation",
      description: "Basic text-to-image workflow with sampling",
      tags: ["txt2img", "basic"],
    });
    setIsValidJson(true);
  };

  const createImg2ImgTemplate = () => {
    const defaultWorkflow = {
      "10": {
        inputs: {
          seed: 0,
          steps: 20,
          cfg: 8,
          sampler_name: "euler",
          scheduler: "normal",
          denoise: 0.75,
          model: ["11", 0],
          positive: ["13", 0],
          negative: ["14", 0],
          latent_image: ["15", 0],
        },
        class_type: "KSampler",
      },
      "11": {
        inputs: {
          ckpt_name: "v1-5-pruned-emaonly.ckpt",
        },
        class_type: "CheckpointLoaderSimple",
      },
      "12": {
        inputs: {
          image: "example.png",
          upload: "image",
        },
        class_type: "LoadImage",
      },
      "13": {
        inputs: {
          text: "{{positive_prompt}}",
          clip: ["11", 1],
        },
        class_type: "CLIPTextEncode",
      },
      "14": {
        inputs: {
          text: "{{negative_prompt}}",
          clip: ["11", 1],
        },
        class_type: "CLIPTextEncode",
      },
      "15": {
        inputs: {
          pixels: ["12", 0],
          vae: ["11", 2],
        },
        class_type: "VAEEncode",
      },
      "16": {
        inputs: {
          samples: ["10", 0],
          vae: ["11", 2],
        },
        class_type: "VAEDecode",
      },
      "17": {
        inputs: {
          filename_prefix: "ComfyUI",
          images: ["16", 0],
        },
        class_type: "SaveImage",
      },
    };

    setTemplate({
      ...template,
      workflow_json: JSON.stringify(defaultWorkflow, null, 2),
      name: "Image to Image",
      category: "Image Generation",
      description: "Image-to-image workflow with latent encoding",
      tags: ["img2img", "basic"],
    });
    setIsValidJson(true);
  };

  const commonCategories = [
    "Image Generation",
    "Video Generation",
    "Image Enhancement",
    "Style Transfer",
    "Face Processing",
    "Background Removal",
    "General",
  ];

  return (
    <div className="p-4 h-full flex flex-col">
      <h2 className="text-2xl font-bold mb-4">
        {existingTemplate ? "Edit Workflow Template" : "Create Workflow Template"}
      </h2>

      {error && <div className="p-3 bg-red-100 text-red-800 rounded mb-4">{error}</div>}

      <div className="flex-1 overflow-auto space-y-4">
        <div className="grid grid-cols-2 gap-4">
          <div>
            <label className="block text-sm font-medium mb-1">Template Name *</label>
            <input
              type="text"
              value={template.name}
              onChange={(e) => setTemplate({ ...template, name: e.target.value })}
              className="w-full px-3 py-2 border rounded"
              placeholder="Enter template name"
            />
          </div>

          <div>
            <label className="block text-sm font-medium mb-1">Category</label>
            <select
              value={template.category}
              onChange={(e) => setTemplate({ ...template, category: e.target.value })}
              className="w-full px-3 py-2 border rounded"
            >
              {commonCategories.map((cat) => (
                <option key={cat} value={cat}>
                  {cat}
                </option>
              ))}
            </select>
          </div>
        </div>

        <div>
          <label className="block text-sm font-medium mb-1">Description</label>
          <textarea
            value={template.description}
            onChange={(e) => setTemplate({ ...template, description: e.target.value })}
            rows={2}
            className="w-full px-3 py-2 border rounded"
            placeholder="Describe what this template does"
          />
        </div>

        <div>
          <label className="block text-sm font-medium mb-1">Tags</label>
          <div className="flex gap-2 mb-2">
            <input
              type="text"
              value={tagInput}
              onChange={(e) => setTagInput(e.target.value)}
              onKeyPress={(e) => e.key === "Enter" && handleAddTag()}
              className="flex-1 px-3 py-2 border rounded"
              placeholder="Add a tag..."
            />
            <button
              onClick={handleAddTag}
              className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
            >
              Add
            </button>
          </div>
          <div className="flex flex-wrap gap-2">
            {template.tags.map((tag) => (
              <span key={tag} className="px-3 py-1 bg-gray-200 rounded flex items-center gap-2">
                {tag}
                <button
                  onClick={() => handleRemoveTag(tag)}
                  className="text-red-500 hover:text-red-700"
                >
                  ×
                </button>
              </span>
            ))}
          </div>
        </div>

        <div>
          <div className="flex justify-between items-center mb-2">
            <label className="block text-sm font-medium mb-1">Workflow JSON *</label>
            <div className="flex gap-2">
              <button
                onClick={createTxt2ImgTemplate}
                className="px-3 py-1 bg-gray-200 rounded text-sm hover:bg-gray-300"
              >
                Load Txt2Img
              </button>
              <button
                onClick={createImg2ImgTemplate}
                className="px-3 py-1 bg-gray-200 rounded text-sm hover:bg-gray-300"
              >
                Load Img2Img
              </button>
              <button
                onClick={formatJson}
                className="px-3 py-1 bg-gray-200 rounded text-sm hover:bg-gray-300"
              >
                Format JSON
              </button>
            </div>
          </div>
          <textarea
            value={template.workflow_json}
            onChange={(e) => handleWorkflowJsonChange(e.target.value)}
            rows={20}
            className={`w-full px-3 py-2 border rounded font-mono text-sm ${
              !isValidJson ? "border-red-500" : ""
            }`}
            placeholder='{"3": {"class_type": "KSampler", "inputs": {...}}}'
          />
          {!isValidJson && (
            <div className="text-red-600 text-sm mt-1">Invalid JSON: {jsonError}</div>
          )}
        </div>

        <div>
          <label className="block text-sm font-medium mb-1">Preview Image URL</label>
          <input
            type="text"
            value={template.preview_image}
            onChange={(e) => setTemplate({ ...template, preview_image: e.target.value })}
            className="w-full px-3 py-2 border rounded"
            placeholder="https://example.com/preview.png"
          />
        </div>

        <div className="bg-yellow-50 border border-yellow-200 rounded p-3">
          <h4 className="font-medium text-yellow-800 mb-1">Variable Substitution</h4>
          <p className="text-sm text-yellow-700">
            Use {"{{variable_name}}"} syntax for dynamic values in your workflow. Common variables:{" "}
            {"{{positive_prompt}}"}, {"{{negative_prompt}}"}, {"{{width}}"}, {"{{height}}"},{" "}
            {"{{seed}}"}, {"{{steps}}"}, {"{{cfg}}"}
          </p>
        </div>
      </div>

      <div className="flex justify-end gap-2 mt-4 pt-4 border-t">
        <button onClick={onCancel} className="px-6 py-2 bg-gray-200 rounded hover:bg-gray-300">
          Cancel
        </button>
        <button
          onClick={handleSave}
          disabled={isSaving || !isValidJson || !template.name.trim()}
          className={`px-6 py-2 text-white rounded ${
            isSaving || !isValidJson || !template.name.trim()
              ? "bg-gray-400 cursor-not-allowed"
              : "bg-blue-500 hover:bg-blue-600"
          }`}
        >
          {isSaving ? "Saving..." : "Save Template"}
        </button>
      </div>
    </div>
  );
}
