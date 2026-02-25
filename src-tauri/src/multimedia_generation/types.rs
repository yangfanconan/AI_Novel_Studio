use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeOfDay {
    Dawn,
    Morning,
    Noon,
    Afternoon,
    Dusk,
    Evening,
    Night,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmotionalTone {
    Happy,
    Sad,
    Tense,
    Romantic,
    Mysterious,
    Action,
    Peaceful,
    Dramatic,
    Horror,
    Comedy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShotType {
    ExtremeCloseUp,
    CloseUp,
    MediumCloseUp,
    MediumShot,
    MediumFullShot,
    FullShot,
    LongShot,
    ExtremeLongShot,
    OverTheShoulder,
    Pov,
    TwoShot,
    Establishing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterInScene {
    pub id: String,
    pub name: String,
    pub appearance: Option<String>,
    pub expression: Option<String>,
    pub action: Option<String>,
    pub dialogue: Option<Vec<Dialogue>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dialogue {
    pub character: String,
    pub text: String,
    pub emotion: Option<String>,
    pub direction: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub id: String,
    pub number: i32,
    pub title: String,
    pub location: String,
    pub time_of_day: TimeOfDay,
    pub characters: Vec<CharacterInScene>,
    pub description: String,
    pub action: String,
    pub emotional_tone: EmotionalTone,
    pub suggested_shots: Vec<ShotType>,
    pub original_text: String,
    pub duration: Option<f64>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StoryboardFormat {
    Film,
    Animation,
    Commercial,
    Documentary,
    MusicVideo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VisualStyle {
    Realistic,
    Cinematic,
    Anime,
    Cartoon,
    Noir,
    Fantasy,
    SciFi,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CameraMovementType {
    Static,
    Pan,
    Tilt,
    Dolly,
    Zoom,
    Tracking,
    Crane,
    Handheld,
    Steadicam,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Transition {
    Cut,
    Fade,
    Dissolve,
    Wipe,
    Iris,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraMovement {
    pub movement_type: CameraMovementType,
    pub direction: Option<String>,
    pub speed: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shot {
    pub shot_number: i32,
    pub shot_type: ShotType,
    pub description: String,
    pub camera: CameraMovement,
    pub characters: Vec<String>,
    pub action: String,
    pub dialogue: Option<Dialogue>,
    pub sound_effects: Option<Vec<String>>,
    pub duration: f64,
    pub transition: Option<Transition>,
    pub visual_notes: Option<String>,
    pub thumbnail: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryboardScene {
    pub scene_number: i32,
    pub title: String,
    pub location: String,
    pub time_of_day: TimeOfDay,
    pub shots: Vec<Shot>,
    pub estimated_duration: f64,
    pub notes: String,
    pub color_mood: ColorPalette,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    pub primary: String,
    pub secondary: String,
    pub accent: String,
    pub background: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryboardMetadata {
    pub generated_at: String,
    pub source_text: String,
    pub options: StoryboardOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryboardOptions {
    pub title: Option<String>,
    pub format: StoryboardFormat,
    pub style: VisualStyle,
    pub detail_level: String,
    pub include_dialogue: bool,
    pub include_camera_movement: bool,
    pub include_sound_effects: bool,
    pub target_duration: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Storyboard {
    pub title: String,
    pub format: StoryboardFormat,
    pub style: VisualStyle,
    pub scenes: Vec<StoryboardScene>,
    pub total_duration: f64,
    pub metadata: StoryboardMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScriptFormat {
    Hollywood,
    Bbc,
    Chinese,
    StagePlay,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptScene {
    pub heading: String,
    pub location: String,
    pub time_of_day: String,
    pub action: String,
    pub elements: Vec<ScriptElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ScriptElement {
    Action { content: String },
    Dialogue { character: String, parenthetical: Option<String>, dialogue: String },
    Transition { transition: String },
    Shot { shot_type: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptMetadata {
    pub generated_at: String,
    pub source_text: String,
    pub format: ScriptFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Script {
    pub title: String,
    pub format: ScriptFormat,
    pub scenes: Vec<ScriptScene>,
    pub characters: Vec<String>,
    pub locations: Vec<String>,
    pub metadata: ScriptMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComicStyle {
    Manga,
    American,
    Manhua,
    European,
    Webtoon,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LayoutType {
    OnePanel,
    TwoHorizontal,
    TwoVertical,
    ThreeEqual,
    ThreeVariable,
    FourGrid,
    FourVariable,
    FiveVariable,
    SixGrid,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BubbleType {
    Speech,
    Thought,
    Whisper,
    Shout,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelPosition {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeechBubble {
    pub id: String,
    pub character: String,
    pub text: String,
    pub position: (f64, f64),
    pub bubble_type: BubbleType,
    pub tail_direction: String,
    pub style: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundEffect {
    pub text: String,
    pub position: (f64, f64),
    pub style: String,
    pub rotation: Option<f64>,
    pub scale: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComicPanel {
    pub index: i32,
    pub position: PanelPosition,
    pub image: String,
    pub speech_bubbles: Vec<SpeechBubble>,
    pub sound_effects: Vec<SoundEffect>,
    pub border_style: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComicPage {
    pub page_number: i32,
    pub layout: LayoutType,
    pub panels: Vec<ComicPanel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComicMetadata {
    pub generated_at: String,
    pub total_pages: i32,
    pub total_panels: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comic {
    pub title: String,
    pub style: ComicStyle,
    pub pages: Vec<ComicPage>,
    pub metadata: ComicMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArtStyle {
    Realistic,
    Anime,
    Manga,
    Watercolor,
    OilPainting,
    DigitalArt,
    ConceptArt,
    Fantasy,
    Cyberpunk,
    Steampunk,
    Minimalist,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IllustrationOptions {
    pub style: ArtStyle,
    pub aspect_ratio: String,
    pub quality: String,
    pub variations: i32,
    pub color_palette: Option<ColorPalette>,
    pub mood: Option<String>,
    pub lighting: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedPrompt {
    pub positive: String,
    pub negative: String,
    pub parameters: ImageParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageParameters {
    pub steps: i32,
    pub cfg_scale: f64,
    pub sampler: String,
    pub seed: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IllustrationMetadata {
    pub generated_at: String,
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Illustration {
    pub scene_id: String,
    pub images: Vec<String>,
    pub prompt: EnhancedPrompt,
    pub style: ArtStyle,
    pub metadata: IllustrationMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterView {
    pub angle: String,
    pub image: String,
    pub embedding: Option<Vec<f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterExpression {
    pub expression: String,
    pub image: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterPortrait {
    pub character_id: String,
    pub views: Vec<CharacterView>,
    pub expressions: Vec<CharacterExpression>,
    pub turnaround: String,
}
