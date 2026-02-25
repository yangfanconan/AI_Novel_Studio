import { invoke } from "@tauri-apps/api/core";

export interface AIScene {
  scene_id: number;
  narration: string;
  visual_content: string;
  action: string;
  camera: string;
  character_description: string;
}

export interface AICharacter {
  id: string;
  name: string;
  type: string;
  visual_traits: string;
  style_tokens: string[];
  color_palette: string[];
}

export interface ReferenceImage {
  id: string;
  url: string;
  analysis_result?: Record<string, unknown>;
  is_primary: boolean;
}

export interface ThreeViewImages {
  front?: string;
  side?: string;
  back?: string;
}

export interface CharacterBible {
  id: string;
  project_id: string;
  name: string;
  type: string;
  visual_traits: string;
  style_tokens: string[];
  color_palette: string[];
  personality: string;
  reference_images: ReferenceImage[];
  three_view_images?: ThreeViewImages;
  created_at: string;
  updated_at: string;
}

export interface CreateCharacterBibleRequest {
  project_id: string;
  name: string;
  type: string;
  visual_traits: string;
  style_tokens?: string[];
  color_palette?: string[];
  personality?: string;
}

export interface CharacterBibleUpdate {
  name?: string;
  char_type?: string;
  visual_traits?: string;
  style_tokens?: string[];
  color_palette?: string[];
  personality?: string;
  reference_images?: ReferenceImage[];
  three_view_images?: ThreeViewImages;
}

export interface AsyncTaskResult {
  task_id: string;
  status: "Pending" | "Processing" | "Completed" | "Failed";
  progress?: number;
  result_url?: string;
  error?: string;
  estimated_time?: number;
}

export type TaskType =
  | "ImageGeneration"
  | "VideoGeneration"
  | "AudioGeneration"
  | "ScriptGeneration"
  | "Custom";
export type TaskPriority = "Low" | "Normal" | "High" | "Urgent";
export type TaskState = "Pending" | "Running" | "Completed" | "Failed" | "Cancelled";

export interface QueuedTask {
  id: string;
  project_id: string;
  task_type: TaskType;
  priority: TaskPriority;
  state: TaskState;
  provider?: string;
  input_data: Record<string, unknown>;
  output_data?: Record<string, unknown>;
  error_message?: string;
  retry_count: number;
  max_retries: number;
  progress: number;
  created_at: string;
  updated_at: string;
  started_at?: string;
  completed_at?: string;
}

export interface CreateTaskRequest {
  project_id: string;
  task_type: TaskType;
  priority?: TaskPriority;
  provider?: string;
  input_data: Record<string, unknown>;
  max_retries?: number;
}

export interface TaskQueueStats {
  total: number;
  pending: number;
  running: number;
  completed: number;
  failed: number;
  cancelled: number;
}

export interface GenerationConfig {
  style_tokens: string[];
  quality_tokens: string[];
}

export interface ParsedScene {
  scene_id: number;
  narration: string;
  visual_content: string;
  action: string;
  camera: string;
  character_description: string;
  duration_seconds?: number;
  transition?: string;
}

export interface ParsedScreenplay {
  title: string;
  scenes: ParsedScene[];
  total_duration: number;
  character_references: string[];
}

export interface ScriptScene {
  id: string;
  project_id: string;
  chapter_id?: string;
  scene_index: number;
  narration: string;
  visual_content: string;
  action: string;
  camera: string;
  character_description: string;
  generated_image_url?: string;
  generated_video_url?: string;
  status: string;
  created_at: string;
  updated_at: string;
}

export interface CreateSceneRequest {
  project_id: string;
  chapter_id?: string;
  scene_index: number;
  narration: string;
  visual_content: string;
  action: string;
  camera: string;
  character_description: string;
}

export interface UpdateSceneRequest {
  id: string;
  narration?: string;
  visual_content?: string;
  action?: string;
  camera?: string;
  character_description?: string;
  generated_image_url?: string;
  generated_video_url?: string;
  status?: string;
}

export interface SceneBatchResult {
  created: number;
  updated: number;
  failed: number;
  scenes: ScriptScene[];
}

export interface SceneStatistics {
  total: number;
  pending: number;
  processing: number;
  image_ready: number;
  completed: number;
  failed: number;
}

export type BatchJobStatus =
  | "Pending"
  | "Running"
  | "Paused"
  | "Completed"
  | "Failed"
  | "Cancelled";
export type BatchSourceType = "NovelText" | "AiGenerated" | "ChapterContent" | "ExistingScenes";

export interface BatchProductionConfig {
  image_provider?: string;
  video_provider?: string;
  style_tokens: string[];
  quality_tokens: string[];
  max_concurrent_tasks: number;
  retry_failed_tasks: boolean;
}

export interface BatchProductionJob {
  id: string;
  project_id: string;
  name: string;
  status: BatchJobStatus;
  total_scenes: number;
  completed_scenes: number;
  failed_scenes: number;
  config: BatchProductionConfig;
  created_at: string;
  updated_at: string;
}

export interface ProductionProgress {
  job_id: string;
  current_scene: number;
  total_scenes: number;
  current_status: string;
  percentage: number;
  estimated_remaining_seconds?: number;
}

export interface CreateBatchJobRequest {
  project_id: string;
  name: string;
  source_type: BatchSourceType;
  source_content?: string;
  chapter_ids?: string[];
  scene_count?: number;
  config?: BatchProductionConfig;
}

export class MoyinService {
  private static instance: MoyinService;

  private constructor() {}

  static getInstance(): MoyinService {
    if (!MoyinService.instance) {
      MoyinService.instance = new MoyinService();
    }
    return MoyinService.instance;
  }

  async compileImagePrompt(
    scene: AIScene,
    characters: AICharacter[],
    styleTokens: string[],
    qualityTokens: string[]
  ): Promise<string> {
    return invoke<string>("compile_image_prompt", {
      sceneJson: JSON.stringify(scene),
      charactersJson: JSON.stringify(characters),
      styleTokens,
      qualityTokens,
    });
  }

  async compileVideoPrompt(scene: AIScene, characters: AICharacter[]): Promise<string> {
    return invoke<string>("compile_video_prompt", {
      sceneJson: JSON.stringify(scene),
      charactersJson: JSON.stringify(characters),
    });
  }

  async compileScreenplayPrompt(prompt: string, sceneCount: number): Promise<string> {
    return invoke<string>("compile_screenplay_prompt", {
      prompt,
      sceneCount,
    });
  }

  async getNegativePrompt(additionalTerms?: string[]): Promise<string> {
    return invoke<string>("get_negative_prompt", {
      additionalTerms,
    });
  }

  async createCharacterBible(request: CreateCharacterBibleRequest): Promise<CharacterBible> {
    return invoke<CharacterBible>("create_character_bible", { request });
  }

  async getCharacterBibles(projectId: string): Promise<CharacterBible[]> {
    return invoke<CharacterBible[]>("get_character_bibles", { projectId });
  }

  async updateCharacterBible(id: string, updates: CharacterBibleUpdate): Promise<CharacterBible> {
    return invoke<CharacterBible>("update_character_bible", { id, updates });
  }

  async deleteCharacterBible(id: string): Promise<boolean> {
    return invoke<boolean>("delete_character_bible", { id });
  }

  async buildConsistencyPrompt(characterIds: string[]): Promise<string> {
    return invoke<string>("build_consistency_prompt", { characterIds });
  }

  async getCharacterStyleTokens(characterIds: string[]): Promise<string[]> {
    return invoke<string[]>("get_character_style_tokens", { characterIds });
  }

  async pollTaskStatus(
    taskId: string,
    timeoutSeconds?: number,
    intervalSeconds?: number
  ): Promise<AsyncTaskResult> {
    return invoke<AsyncTaskResult>("poll_task_status", {
      taskId,
      timeoutSeconds,
      intervalSeconds,
    });
  }

  async createTask(request: CreateTaskRequest): Promise<QueuedTask> {
    return invoke<QueuedTask>("create_task", { request });
  }

  async getTask(id: string): Promise<QueuedTask | null> {
    return invoke<QueuedTask | null>("get_task", { id });
  }

  async getProjectTasks(projectId: string): Promise<QueuedTask[]> {
    return invoke<QueuedTask[]>("get_project_tasks", { projectId });
  }

  async cancelTask(id: string): Promise<QueuedTask | null> {
    return invoke<QueuedTask | null>("cancel_task", { id });
  }

  async getQueueStats(): Promise<TaskQueueStats> {
    return invoke<TaskQueueStats>("get_queue_stats");
  }

  async clearCompletedTasks(): Promise<void> {
    return invoke<void>("clear_completed_tasks");
  }

  async parseNovelToScreenplay(
    text: string,
    sceneCount?: number,
    language?: string
  ): Promise<ParsedScreenplay> {
    const result = await invoke<string>("parse_novel_to_screenplay", {
      request: { text, scene_count: sceneCount, language },
    });
    return JSON.parse(result);
  }

  async parseAiScreenplayResponse(jsonResponse: string): Promise<ParsedScreenplay> {
    const result = await invoke<string>("parse_ai_screenplay_response", {
      jsonResponse,
    });
    return JSON.parse(result);
  }

  async mergeScreenplayScenes(
    scenes: ParsedScene[],
    targetCount: number
  ): Promise<ParsedScreenplay> {
    const result = await invoke<string>("merge_screenplay_scenes", {
      scenesJson: JSON.stringify(scenes),
      targetCount,
    });
    return JSON.parse(result);
  }

  async createScriptScene(request: CreateSceneRequest, dbPath: string): Promise<ScriptScene> {
    return invoke<ScriptScene>("create_script_scene", { request, dbPath });
  }

  async getScriptScene(id: string, dbPath: string): Promise<ScriptScene | null> {
    return invoke<ScriptScene | null>("get_script_scene", { id, dbPath });
  }

  async getProjectScriptScenes(projectId: string, dbPath: string): Promise<ScriptScene[]> {
    return invoke<ScriptScene[]>("get_project_script_scenes", { projectId, dbPath });
  }

  async getChapterScriptScenes(chapterId: string, dbPath: string): Promise<ScriptScene[]> {
    return invoke<ScriptScene[]>("get_chapter_script_scenes", { chapterId, dbPath });
  }

  async updateScriptScene(
    request: UpdateSceneRequest,
    dbPath: string
  ): Promise<ScriptScene | null> {
    return invoke<ScriptScene | null>("update_script_scene", { request, dbPath });
  }

  async deleteScriptScene(id: string, dbPath: string): Promise<boolean> {
    return invoke<boolean>("delete_script_scene", { id, dbPath });
  }

  async batchCreateScriptScenes(
    requests: CreateSceneRequest[],
    dbPath: string
  ): Promise<SceneBatchResult> {
    return invoke<SceneBatchResult>("batch_create_script_scenes", { requests, dbPath });
  }

  async updateSceneGenerationStatus(
    id: string,
    status: string,
    dbPath: string
  ): Promise<ScriptScene | null> {
    return invoke<ScriptScene | null>("update_scene_generation_status", { id, status, dbPath });
  }

  async setSceneGeneratedImage(
    id: string,
    imageUrl: string,
    dbPath: string
  ): Promise<ScriptScene | null> {
    return invoke<ScriptScene | null>("set_scene_generated_image", { id, imageUrl, dbPath });
  }

  async setSceneGeneratedVideo(
    id: string,
    videoUrl: string,
    dbPath: string
  ): Promise<ScriptScene | null> {
    return invoke<ScriptScene | null>("set_scene_generated_video", { id, videoUrl, dbPath });
  }

  async getSceneStatistics(projectId: string, dbPath: string): Promise<SceneStatistics> {
    return invoke<SceneStatistics>("get_scene_statistics_cmd", { projectId, dbPath });
  }

  async createBatchProductionJob(request: CreateBatchJobRequest): Promise<BatchProductionJob> {
    return invoke<BatchProductionJob>("create_batch_production_job", { request });
  }

  async getBatchProductionJob(id: string): Promise<BatchProductionJob | null> {
    return invoke<BatchProductionJob | null>("get_batch_production_job", { id });
  }

  async getProjectBatchJobs(projectId: string): Promise<BatchProductionJob[]> {
    return invoke<BatchProductionJob[]>("get_project_batch_jobs", { projectId });
  }

  async cancelBatchJob(id: string): Promise<BatchProductionJob | null> {
    return invoke<BatchProductionJob | null>("cancel_batch_job", { id });
  }

  async pauseBatchJob(id: string): Promise<BatchProductionJob | null> {
    return invoke<BatchProductionJob | null>("pause_batch_job", { id });
  }

  async resumeBatchJob(id: string): Promise<BatchProductionJob | null> {
    return invoke<BatchProductionJob | null>("resume_batch_job", { id });
  }

  async getBatchJobProgress(id: string): Promise<ProductionProgress | null> {
    return invoke<ProductionProgress | null>("get_batch_job_progress", { id });
  }

  async prepareScenesFromNovel(text: string, sceneCount: number): Promise<CreateSceneRequest[]> {
    return invoke<CreateSceneRequest[]>("prepare_scenes_from_novel", { text, sceneCount });
  }

  async prepareScenesFromAi(jsonResponse: string): Promise<CreateSceneRequest[]> {
    return invoke<CreateSceneRequest[]>("prepare_scenes_from_ai", { jsonResponse });
  }

  async getBatchJobStatistics(): Promise<Record<string, number>> {
    return invoke<Record<string, number>>("get_batch_job_statistics");
  }

  createDefaultStyleTokens(): string[] {
    return ["cinematic", "dramatic lighting", "high detail", "8k", "professional"];
  }

  createDefaultQualityTokens(): string[] {
    return ["masterpiece", "best quality", "ultra detailed", "sharp focus"];
  }

  createDefaultBatchConfig(): BatchProductionConfig {
    return {
      image_provider: "openai",
      style_tokens: this.createDefaultStyleTokens(),
      quality_tokens: this.createDefaultQualityTokens(),
      max_concurrent_tasks: 3,
      retry_failed_tasks: true,
    };
  }

  mapPriorityToEnum(priority: string): TaskPriority {
    const map: Record<string, TaskPriority> = {
      low: "Low",
      normal: "Normal",
      high: "High",
      urgent: "Urgent",
    };
    return map[priority.toLowerCase()] || "Normal";
  }

  mapTaskTypeToEnum(taskType: string): TaskType {
    const map: Record<string, TaskType> = {
      image: "ImageGeneration",
      video: "VideoGeneration",
      audio: "AudioGeneration",
      script: "ScriptGeneration",
      custom: "Custom",
    };
    return map[taskType.toLowerCase()] || "Custom";
  }

  mapBatchStatusToEnum(status: string): BatchJobStatus {
    const map: Record<string, BatchJobStatus> = {
      pending: "Pending",
      running: "Running",
      paused: "Paused",
      completed: "Completed",
      failed: "Failed",
      cancelled: "Cancelled",
    };
    return map[status.toLowerCase()] || "Pending";
  }

  async comfyuiCheckConnection(): Promise<{ connected: boolean; message: string }> {
    return invoke<{ connected: boolean; message: string }>("comfyui_check_connection");
  }

  async comfyuiQueuePrompt(workflowJson: string, clientId?: string): Promise<string> {
    return invoke<string>("comfyui_queue_prompt", { workflowJson, clientId });
  }

  async comfyuiGenerateImage(
    positivePrompt: string,
    negativePrompt?: string,
    width?: number,
    height?: number,
    seed?: number,
    steps?: number,
    cfg?: number
  ): Promise<string> {
    return invoke<string>("comfyui_generate_image", {
      positivePrompt,
      negativePrompt,
      width,
      height,
      seed,
      steps,
      cfg,
    });
  }

  async comfyuiUploadImage(filePath: string, overwrite?: boolean): Promise<string> {
    return invoke<string>("comfyui_upload_image", { filePath, overwrite });
  }

  async comfyuiGetHistory(promptId: string): Promise<Record<string, unknown>> {
    return invoke<Record<string, unknown>>("comfyui_get_history", { promptId });
  }

  async comfyuiGetImages(promptId: string): Promise<string[]> {
    return invoke<string[]>("comfyui_get_images", { promptId });
  }

  async comfyuiGetQueueInfo(): Promise<Record<string, unknown>> {
    return invoke<Record<string, unknown>>("comfyui_get_queue_info");
  }

  async comfyuiGetObjectInfo(nodeType: string): Promise<Record<string, unknown>> {
    return invoke<Record<string, unknown>>("comfyui_get_object_info", { nodeType });
  }

  async comfyuiGetSystemInfo(): Promise<Record<string, unknown>> {
    return invoke<Record<string, unknown>>("comfyui_get_system_info");
  }

  async comfyuiEmbeddingsList(): Promise<string[]> {
    return invoke<string[]>("comfyui_embeddings_list");
  }

  async comfyuiLoadLora(name: string): Promise<Record<string, unknown>> {
    return invoke<Record<string, unknown>>("comfyui_load_lora", { name });
  }

  async comfyuiUnloadLora(name: string): Promise<boolean> {
    return invoke<boolean>("comfyui_unload_lora", { name });
  }

  async comfyuiInterrupt(): Promise<boolean> {
    return invoke<boolean>("comfyui_interrupt");
  }

  async comfyuiFreeMemory(): Promise<boolean> {
    return invoke<boolean>("comfyui_free_memory");
  }

  async createWorkflowTemplate(request: CreateTemplateRequest): Promise<WorkflowTemplate> {
    return invoke<WorkflowTemplate>("create_workflow_template", { request });
  }

  async getWorkflowTemplates(): Promise<WorkflowTemplate[]> {
    return invoke<WorkflowTemplate[]>("get_workflow_templates");
  }

  async getWorkflowTemplate(id: string): Promise<WorkflowTemplate | null> {
    return invoke<WorkflowTemplate | null>("get_workflow_template", { id });
  }

  async updateWorkflowTemplate(
    id: string,
    updates: WorkflowTemplateUpdate
  ): Promise<WorkflowTemplate> {
    return invoke<WorkflowTemplate>("update_workflow_template", { id, updates });
  }

  async deleteWorkflowTemplate(id: string): Promise<boolean> {
    return invoke<boolean>("delete_workflow_template", { id });
  }

  async getWorkflowTemplatesByCategory(category: string): Promise<WorkflowTemplate[]> {
    return invoke<WorkflowTemplate[]>("get_workflow_templates_by_category", { category });
  }

  async toggleFavoriteWorkflowTemplate(id: string): Promise<WorkflowTemplate> {
    return invoke<WorkflowTemplate>("toggle_favorite_workflow_template", { id });
  }

  async searchWorkflowTemplates(keyword: string): Promise<WorkflowTemplate[]> {
    return invoke<WorkflowTemplate[]>("search_workflow_templates", { keyword });
  }

  async applyWorkflowTemplate(
    templateId: string,
    variables: Record<string, unknown>
  ): Promise<Record<string, unknown>> {
    return invoke<Record<string, unknown>>("apply_workflow_template", { templateId, variables });
  }

  async getBuiltInWorkflowTemplates(): Promise<WorkflowTemplate[]> {
    return invoke<WorkflowTemplate[]>("get_builtin_workflow_templates");
  }

  async seedanceValidateRequest(request: SeedanceRequest): Promise<ValidationResult> {
    return invoke<ValidationResult>("seedance_validate_request", { request });
  }

  async seedanceBuildPrompt(
    action: string,
    cinematography: string,
    dialogue: string
  ): Promise<string> {
    return invoke<string>("seedance_build_prompt", { action, cinematography, dialogue });
  }

  async seedanceGetConstraints(): Promise<SeedanceConstraints> {
    return invoke<SeedanceConstraints>("seedance_get_constraints");
  }

  async seedanceCreateGrid(images: string[], rows: number, cols: number): Promise<FirstFrameGrid> {
    return invoke<FirstFrameGrid>("seedance_create_grid", { images, rows, cols });
  }

  async seedanceValidateGrid(
    rows: number,
    cols: number,
    imageCount: number
  ): Promise<ValidationResult> {
    return invoke<ValidationResult>("seedance_validate_grid", { rows, cols, imageCount });
  }

  async seedancePrepareNarrativeVideo(
    storyboard: Storyboard,
    config: NarrativeVideoConfig
  ): Promise<SeedanceRequest> {
    return invoke<SeedanceRequest>("seedance_prepare_narrative_video", { storyboard, config });
  }
}

export interface SeedanceConstraints {
  max_images: number;
  max_videos: number;
  max_audio: number;
  max_prompt_length: number;
}

export interface SeedanceRequest {
  prompt: string;
  images: MultimodalReference[];
  videos: MultimodalReference[];
  audio: MultimodalReference[];
  first_frame_grid?: FirstFrameGrid;
  duration?: number;
  aspect_ratio?: string;
}

export interface MultimodalReference {
  type: string;
  id: string;
  url: string;
  description?: string;
}

export interface FirstFrameGrid {
  rows: number;
  cols: number;
  images: string[];
}

export interface ValidationResult {
  valid: boolean;
  errors: string[];
  warnings: string[];
}

export interface NarrativeVideoConfig {
  storyboard_id: string;
  custom_prompt?: string;
  duration?: number;
  aspect_ratio?: string;
  include_audio: boolean;
  include_references: boolean;
}

export interface Storyboard {
  id: string;
  project_id: string;
  name: string;
  description: string;
  scenes: StoryboardScene[];
  visual_style: VisualStyle;
  created_at: string;
  updated_at: string;
}

export interface StoryboardScene {
  id: string;
  storyboard_id: string;
  scene_number: number;
  location: string;
  time_of_day: TimeOfDay;
  shots: StoryboardShot[];
}

export interface StoryboardShot {
  id: string;
  scene_id: string;
  shot_number: number;
  shot_type: ShotType;
  camera_angle: CameraAngle;
  camera_movement: CameraMovement;
  subject: string;
  action: string;
  dialogue?: string;
  duration: number;
  description: string;
  visual_reference?: string;
  video_reference?: string;
  audio_reference?: string;
  notes?: string;
}

export type VisualStyle =
  | "Realistic"
  | "Anime2D"
  | "Anime3D"
  | "StopMotion"
  | "Watercolor"
  | "OilPainting"
  | "Sketch"
  | string;

export type TimeOfDay =
  | "Dawn"
  | "Morning"
  | "Noon"
  | "Afternoon"
  | "Evening"
  | "Night"
  | "Midnight";

export type ShotType =
  | "ExtremeCloseUp"
  | "CloseUp"
  | "MediumCloseUp"
  | "MediumShot"
  | "MediumFullShot"
  | "FullShot"
  | "WideShot"
  | "ExtremeWideShot"
  | "TwoShot"
  | "OverTheShoulder"
  | "PointOfView"
  | "Establishing";

export type CameraAngle =
  | "EyeLevel"
  | "LowAngle"
  | "HighAngle"
  | "DutchAngle"
  | "BirdEye"
  | "WormEye";

export type CameraMovement =
  | "Static"
  | "PanLeft"
  | "PanRight"
  | "TiltUp"
  | "TiltDown"
  | "ZoomIn"
  | "ZoomOut"
  | "DollyIn"
  | "DollyOut"
  | "TruckLeft"
  | "TruckRight"
  | "PedestalUp"
  | "PedestalDown"
  | "Arc"
  | "Crane"
  | "Handheld"
  | "Steadicam";

export interface CreateStoryboardRequest {
  project_id: string;
  name: string;
  description?: string;
  visual_style?: VisualStyle;
}

export interface CreateStoryboardSceneRequest {
  storyboard_id: string;
  scene_number: number;
  location: string;
  time_of_day: TimeOfDay;
  narration?: string;
}

export interface CreateStoryboardShotRequest {
  scene_id: string;
  shot_number: number;
  shot_type: ShotType;
  camera_angle: CameraAngle;
  camera_movement: CameraMovement;
  subject: string;
  action: string;
  dialogue?: string;
  duration: number;
  description: string;
  visual_reference?: string;
  video_reference?: string;
  audio_reference?: string;
  notes?: string;
}

export interface UpdateStoryboardShotRequest {
  id: string;
  shot_type?: ShotType;
  camera_angle?: CameraAngle;
  camera_movement?: CameraMovement;
  subject?: string;
  action?: string;
  dialogue?: string;
  duration?: number;
  description?: string;
  visual_reference?: string;
  video_reference?: string;
  audio_reference?: string;
  notes?: string;
}

export interface WorkflowTemplate {
  id: string;
  name: string;
  category: string;
  description: string;
  workflow_json: string;
  preview_image?: string;
  tags: string[];
  is_builtin: boolean;
  is_favorite: boolean;
  usage_count: number;
  created_at: string;
  updated_at: string;
}

export interface CreateTemplateRequest {
  name: string;
  category: string;
  description: string;
  workflow_json: string;
  preview_image?: string;
  tags?: string[];
}

export interface WorkflowTemplateUpdate {
  name?: string;
  category?: string;
  description?: string;
  workflow_json?: string;
  preview_image?: string;
  tags?: string[];
  is_favorite?: boolean;
}

export const moyinService = MoyinService.getInstance();
