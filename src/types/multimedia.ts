export enum TimeOfDay {
  DAWN = "dawn",
  MORNING = "morning",
  NOON = "noon",
  AFTERNOON = "afternoon",
  DUSK = "dusk",
  EVENING = "evening",
  NIGHT = "night",
  UNKNOWN = "unknown",
}

export enum EmotionalTone {
  HAPPY = "happy",
  SAD = "sad",
  TENSE = "tense",
  ROMANTIC = "romantic",
  MYSTERIOUS = "mysterious",
  ACTION = "action",
  PEACEFUL = "peaceful",
  DRAMATIC = "dramatic",
  HORROR = "horror",
  COMEDY = "comedy",
}

export enum ShotType {
  EXTREME_CLOSE_UP = "extreme_close_up",
  CLOSE_UP = "close_up",
  MEDIUM_CLOSE_UP = "medium_close_up",
  MEDIUM_SHOT = "medium_shot",
  MEDIUM_FULL_SHOT = "medium_full_shot",
  FULL_SHOT = "full_shot",
  LONG_SHOT = "long_shot",
  EXTREME_LONG_SHOT = "extreme_long_shot",
  OVER_THE_SHOULDER = "over_the_shoulder",
  POV = "pov",
  TWO_SHOT = "two_shot",
  ESTABLISHING = "establishing",
}

export enum StoryboardFormat {
  FILM = "film",
  ANIMATION = "animation",
  COMMERCIAL = "commercial",
  DOCUMENTARY = "documentary",
  MUSIC_VIDEO = "music_video",
}

export enum VisualStyle {
  REALISTIC = "realistic",
  CINEMATIC = "cinematic",
  ANIME = "anime",
  CARTOON = "cartoon",
  NOIR = "noir",
  FANTASY = "fantasy",
  SCI_FI = "sci_fi",
  WATERCOLOR = "watercolor",
  OIL_PAINTING = "oil_painting",
  SKETCH = "sketch",
}

export enum ScriptFormat {
  STANDARD = "standard",
  HOLLYWOOD = "hollywood",
  BROADCAST = "broadcast",
  STAGE_PLAY = "stage_play",
  MANGA = "manga",
  NOVEL = "novel",
}

export enum ComicPanelLayout {
  SINGLE = "single",
  TWO_HORIZONTAL = "two_horizontal",
  TWO_VERTICAL = "two_vertical",
  THREE_HORIZONTAL = "three_horizontal",
  FOUR_GRID = "four_grid",
  SIX_GRID = "six_grid",
  CUSTOM = "custom",
}

export enum ComicPanelShape {
  RECTANGLE = "rectangle",
  SQUARE = "square",
  CIRCLE = "circle",
  DIAGONAL = "diagonal",
  IRREGULAR = "irregular",
  SPLASH = "splash",
}

export interface Dialogue {
  character: string;
  text: string;
  emotion?: string;
  direction?: string;
}

export interface CharacterInScene {
  id: string;
  name: string;
  appearance?: string;
  expression?: string;
  action?: string;
  dialogue?: Dialogue[];
}

export interface CharacterAppearance {
  age?: string;
  gender?: string;
  height?: string;
  build?: string;
  hairColor?: string;
  hairStyle?: string;
  eyeColor?: string;
  skinTone?: string;
  facialFeatures?: string;
  clothing?: string;
  accessories?: string[];
  distinctiveFeatures?: string[];
}

export interface ColorPalette {
  primary: string;
  secondary: string;
  accent: string;
  background: string;
  mood: string;
}

export interface Scene {
  id: string;
  number: number;
  title: string;
  location: string;
  timeOfDay: TimeOfDay;
  characters: CharacterInScene[];
  description: string;
  action: string;
  emotionalTone: EmotionalTone;
  suggestedShots: ShotType[];
  originalText: string;
  duration?: number;
  notes?: string;
}

export interface CameraMovement {
  type: "static" | "pan" | "tilt" | "dolly" | "zoom" | "crane" | "handheld" | "tracking";
  direction?: "left" | "right" | "up" | "down" | "in" | "out";
  speed?: "slow" | "normal" | "fast";
}

export interface Transition {
  type: "cut" | "fade" | "dissolve" | "wipe" | "iris" | "zoom";
  duration?: number;
}

export interface Shot {
  shotNumber: number;
  shotType: ShotType;
  description: string;
  camera: CameraMovement;
  characters: string[];
  action: string;
  dialogue?: Dialogue;
  soundEffects?: string[];
  duration: number;
  transition?: Transition;
  visualNotes?: string;
  thumbnail?: string;
  visualPrompt?: string;
  negativePrompt?: string;
}

export interface StoryboardScene {
  sceneNumber: number;
  title: string;
  location: string;
  timeOfDay: TimeOfDay;
  shots: Shot[];
  estimatedDuration: number;
  notes: string;
  colorMood: ColorPalette;
}

export interface StoryboardMetadata {
  generatedAt: string;
  sourceText?: string;
  chapterId?: string;
  options: StoryboardOptions;
}

export interface Storyboard {
  id: string;
  title: string;
  format: StoryboardFormat;
  style: VisualStyle;
  scenes: StoryboardScene[];
  totalDuration: number;
  metadata: StoryboardMetadata;
}

export interface StoryboardOptions {
  title?: string;
  format: StoryboardFormat;
  style: VisualStyle;
  detailLevel: "basic" | "standard" | "detailed";
  includeDialogue: boolean;
  includeCameraMovement: boolean;
  includeSoundEffects: boolean;
  includeVisualPrompts: boolean;
  targetDuration?: number;
}

export interface ScriptCharacter {
  name: string;
  description?: string;
  voice?: string;
}

export interface ScriptScene {
  sceneNumber: number;
  heading: string;
  action: string;
  characters: ScriptCharacter[];
  dialogue: ScriptDialogue[];
  notes?: string;
}

export interface ScriptDialogue {
  character: string;
  parenthetical?: string;
  text: string;
  extension?: string;
}

export interface Script {
  id: string;
  title: string;
  format: ScriptFormat;
  scenes: ScriptScene[];
  characters: ScriptCharacter[];
  metadata: ScriptMetadata;
}

export interface ScriptMetadata {
  generatedAt: string;
  sourceChapterId?: string;
  sourceText?: string;
  options: ScriptConversionOptions;
}

export interface ScriptConversionOptions {
  targetFormat: ScriptFormat;
  includeSceneNumbers: boolean;
  includeCharacterDescriptions: boolean;
  dialogueStyle: "standard" | "naturalistic" | "stylized";
  includeCameraDirections: boolean;
  customTemplate?: string;
}

export interface ComicPanel {
  panelNumber: number;
  shape: ComicPanelShape;
  shot: Shot;
  dialogue: ComicDialogue[];
  caption?: string;
  soundEffects?: string[];
  visualPrompt?: string;
  negativePrompt?: string;
  imageData?: string;
}

export interface ComicDialogue {
  character: string;
  text: string;
  balloonType: "speech" | "thought" | "whisper" | "shout" | "electronic";
  tailDirection?: "left" | "right" | "up" | "down";
}

export interface ComicPage {
  pageNumber: number;
  layout: ComicPanelLayout;
  panels: ComicPanel[];
  notes?: string;
}

export interface Comic {
  id: string;
  title: string;
  style: VisualStyle;
  pages: ComicPage[];
  characters: ComicCharacter[];
  metadata: ComicMetadata;
}

export interface ComicCharacter {
  name: string;
  appearance: CharacterAppearance;
  visualReference?: string;
  consistencyTags?: string[];
}

export interface ComicMetadata {
  generatedAt: string;
  sourceChapterId?: string;
  sourceText?: string;
  options: ComicGenerationOptions;
}

export interface ComicGenerationOptions {
  style: VisualStyle;
  pageLayout: ComicPanelLayout;
  panelsPerPage: number;
  includeCaptions: boolean;
  includeSoundEffects: boolean;
  generateImages: boolean;
  imageProvider?: "stability" | "dalle" | "comfyui" | "midjourney";
}

export interface Illustration {
  id: string;
  title: string;
  description: string;
  style: VisualStyle;
  prompt: string;
  negativePrompt?: string;
  aspectRatio: "1:1" | "16:9" | "9:16" | "4:3" | "3:4" | "21:9";
  imageData?: string;
  metadata: IllustrationMetadata;
}

export interface IllustrationMetadata {
  generatedAt: string;
  sourceSceneId?: string;
  sourceText?: string;
  characterIds?: string[];
  options: IllustrationOptions;
}

export interface IllustrationOptions {
  style: VisualStyle;
  aspectRatio: "1:1" | "16:9" | "9:16" | "4:3" | "3:4" | "21:9";
  quality: "standard" | "high" | "ultra";
  includeCharacters: boolean;
  characterConsistency: boolean;
  customPrompt?: string;
  negativePrompt?: string;
  imageProvider?: "stability" | "dalle" | "comfyui" | "midjourney";
}

export interface ImageGenerationRequest {
  prompt: string;
  negativePrompt?: string;
  style: VisualStyle;
  aspectRatio: string;
  quality: string;
  numberOfImages?: number;
  seed?: number;
}

export interface ImageGenerationResult {
  images: string[];
  seeds: number[];
  provider: string;
  metadata: Record<string, unknown>;
}

export interface GenerateStoryboardRequest {
  chapterId?: string;
  plotPointId?: string;
  content?: string;
  options?: Partial<StoryboardOptions>;
}

export interface GenerateScriptRequest {
  chapterId?: string;
  content?: string;
  options: ScriptConversionOptions;
}

export interface GenerateComicRequest {
  chapterId?: string;
  content?: string;
  options: ComicGenerationOptions;
}

export interface GenerateIllustrationRequest {
  sceneId?: string;
  content?: string;
  characterIds?: string[];
  options: IllustrationOptions;
}

export const SHOT_TYPE_LABELS: Record<ShotType, string> = {
  [ShotType.EXTREME_CLOSE_UP]: "极特写",
  [ShotType.CLOSE_UP]: "特写",
  [ShotType.MEDIUM_CLOSE_UP]: "中近景",
  [ShotType.MEDIUM_SHOT]: "中景",
  [ShotType.MEDIUM_FULL_SHOT]: "中全景",
  [ShotType.FULL_SHOT]: "全景",
  [ShotType.LONG_SHOT]: "远景",
  [ShotType.EXTREME_LONG_SHOT]: "极远景",
  [ShotType.OVER_THE_SHOULDER]: "过肩镜头",
  [ShotType.POV]: "主观视角",
  [ShotType.TWO_SHOT]: "双人镜头",
  [ShotType.ESTABLISHING]: "建立镜头",
};

export const EMOTIONAL_TONE_LABELS: Record<EmotionalTone, string> = {
  [EmotionalTone.HAPPY]: "欢快",
  [EmotionalTone.SAD]: "悲伤",
  [EmotionalTone.TENSE]: "紧张",
  [EmotionalTone.ROMANTIC]: "浪漫",
  [EmotionalTone.MYSTERIOUS]: "神秘",
  [EmotionalTone.ACTION]: "动作",
  [EmotionalTone.PEACEFUL]: "平静",
  [EmotionalTone.DRAMATIC]: "戏剧性",
  [EmotionalTone.HORROR]: "恐怖",
  [EmotionalTone.COMEDY]: "喜剧",
};

export const VISUAL_STYLE_LABELS: Record<VisualStyle, string> = {
  [VisualStyle.REALISTIC]: "写实风格",
  [VisualStyle.CINEMATIC]: "电影质感",
  [VisualStyle.ANIME]: "日式动漫",
  [VisualStyle.CARTOON]: "卡通风格",
  [VisualStyle.NOIR]: "黑色电影",
  [VisualStyle.FANTASY]: "奇幻风格",
  [VisualStyle.SCI_FI]: "科幻风格",
  [VisualStyle.WATERCOLOR]: "水彩风格",
  [VisualStyle.OIL_PAINTING]: "油画风格",
  [VisualStyle.SKETCH]: "素描风格",
};

export const TIME_OF_DAY_LABELS: Record<TimeOfDay, string> = {
  [TimeOfDay.DAWN]: "黎明",
  [TimeOfDay.MORNING]: "上午",
  [TimeOfDay.NOON]: "正午",
  [TimeOfDay.AFTERNOON]: "下午",
  [TimeOfDay.DUSK]: "黄昏",
  [TimeOfDay.EVENING]: "傍晚",
  [TimeOfDay.NIGHT]: "夜晚",
  [TimeOfDay.UNKNOWN]: "未知",
};

export const SCRIPT_FORMAT_LABELS: Record<ScriptFormat, string> = {
  [ScriptFormat.STANDARD]: "标准剧本",
  [ScriptFormat.HOLLYWOOD]: "好莱坞格式",
  [ScriptFormat.BROADCAST]: "广播剧",
  [ScriptFormat.STAGE_PLAY]: "舞台剧",
  [ScriptFormat.MANGA]: "漫画脚本",
  [ScriptFormat.NOVEL]: "小说格式",
};

export const COMIC_PANEL_LAYOUT_LABELS: Record<ComicPanelLayout, string> = {
  [ComicPanelLayout.SINGLE]: "单格",
  [ComicPanelLayout.TWO_HORIZONTAL]: "二格横排",
  [ComicPanelLayout.TWO_VERTICAL]: "二格竖排",
  [ComicPanelLayout.THREE_HORIZONTAL]: "三格横排",
  [ComicPanelLayout.FOUR_GRID]: "四格漫画",
  [ComicPanelLayout.SIX_GRID]: "六格漫画",
  [ComicPanelLayout.CUSTOM]: "自定义",
};
