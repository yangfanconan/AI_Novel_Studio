use crate::multimedia_generation::types::*;
use crate::multimedia_generation::scene_extractor::SceneExtractor;
use crate::multimedia_generation::storyboard::StoryboardGenerator;
use crate::multimedia_generation::script::ScriptGenerator;
use crate::multimedia_generation::comic::ComicGenerator;
use crate::multimedia_generation::illustration::IllustrationGenerator;
use crate::ai::OpenAIAdapter;
use std::sync::Arc;
use tauri::State;

#[derive(Clone)]
pub struct MultimediaState {
    storyboard_generator: Arc<StoryboardGenerator>,
    script_generator: Arc<ScriptGenerator>,
    comic_generator: Arc<ComicGenerator>,
    illustration_generator: Arc<IllustrationGenerator>,
}

impl MultimediaState {
    pub fn new(api_key: String) -> Self {
        let ai_model = Arc::new(OpenAIAdapter::new(api_key, "gpt-4".to_string()));

        Self {
            storyboard_generator: Arc::new(StoryboardGenerator::new(ai_model.clone())),
            script_generator: Arc::new(ScriptGenerator::new(ai_model.clone())),
            comic_generator: Arc::new(ComicGenerator::new(ai_model.clone())),
            illustration_generator: Arc::new(IllustrationGenerator::new(ai_model)),
        }
    }
}

#[tauri::command]
pub async fn mmg_extract_scenes(
    text: String,
    state: State<'_, MultimediaState>,
) -> Result<String, String> {
    let scene_extractor = SceneExtractor::new(state.storyboard_generator.ai_model.clone());
    let scenes = scene_extractor.extract_scenes(&text).await?;
    serde_json::to_string(&scenes).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn mmg_generate_storyboard(
    text: String,
    title: Option<String>,
    format: String,
    style: String,
    state: State<'_, MultimediaState>,
) -> Result<String, String> {
    let storyboard_format = match format.as_str() {
        "film" => StoryboardFormat::Film,
        "animation" => StoryboardFormat::Animation,
        "commercial" => StoryboardFormat::Commercial,
        "documentary" => StoryboardFormat::Documentary,
        "music_video" => StoryboardFormat::MusicVideo,
        _ => return Err("无效的格式".to_string()),
    };

    let visual_style = match style.as_str() {
        "realistic" => VisualStyle::Realistic,
        "cinematic" => VisualStyle::Cinematic,
        "anime" => VisualStyle::Anime,
        "cartoon" => VisualStyle::Cartoon,
        "noir" => VisualStyle::Noir,
        "fantasy" => VisualStyle::Fantasy,
        "sci_fi" => VisualStyle::SciFi,
        _ => return Err("无效的风格".to_string()),
    };

    let options = StoryboardOptions {
        title,
        format: storyboard_format,
        style: visual_style,
        detail_level: "standard".to_string(),
        include_dialogue: true,
        include_camera_movement: true,
        include_sound_effects: true,
        target_duration: None,
    };

    let storyboard = state
        .storyboard_generator
        .generate_storyboard(&text, options)
        .await?;

    serde_json::to_string(&storyboard).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn mmg_convert_to_script(
    text: String,
    format: String,
    state: State<'_, MultimediaState>,
) -> Result<String, String> {
    let script_format = match format.as_str() {
        "hollywood" => ScriptFormat::Hollywood,
        "bbc" => ScriptFormat::Bbc,
        "chinese" => ScriptFormat::Chinese,
        "stage_play" => ScriptFormat::StagePlay,
        _ => return Err("无效的格式".to_string()),
    };

    let script = state
        .script_generator
        .convert_to_script(&text, script_format)
        .await?;

    serde_json::to_string(&script).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn mmg_optimize_script(
    script_json: String,
    state: State<'_, MultimediaState>,
) -> Result<String, String> {
    let script: Script =
        serde_json::from_str(&script_json).map_err(|e| format!("解析剧本失败: {}", e))?;

    let optimized = state.script_generator.optimize_for_screen(&script).await?;

    serde_json::to_string(&optimized).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn mmg_generate_comic(
    text: String,
    title: String,
    style: String,
    state: State<'_, MultimediaState>,
) -> Result<String, String> {
    let comic_style = match style.as_str() {
        "manga" => ComicStyle::Manga,
        "american" => ComicStyle::American,
        "manhua" => ComicStyle::Manhua,
        "european" => ComicStyle::European,
        "webtoon" => ComicStyle::Webtoon,
        _ => return Err("无效的风格".to_string()),
    };

    let comic = state
        .comic_generator
        .generate_comic(&text, &title, comic_style)
        .await?;

    serde_json::to_string(&comic).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn mmg_generate_scene_illustration(
    scene_json: String,
    style: String,
    aspect_ratio: String,
    quality: String,
    variations: i32,
    state: State<'_, MultimediaState>,
) -> Result<String, String> {
    let scene: Scene =
        serde_json::from_str(&scene_json).map_err(|e| format!("解析场景失败: {}", e))?;

    let art_style = match style.as_str() {
        "realistic" => ArtStyle::Realistic,
        "anime" => ArtStyle::Anime,
        "manga" => ArtStyle::Manga,
        "watercolor" => ArtStyle::Watercolor,
        "oil_painting" => ArtStyle::OilPainting,
        "digital_art" => ArtStyle::DigitalArt,
        "concept_art" => ArtStyle::ConceptArt,
        "fantasy" => ArtStyle::Fantasy,
        "cyberpunk" => ArtStyle::Cyberpunk,
        "steampunk" => ArtStyle::Steampunk,
        "minimalist" => ArtStyle::Minimalist,
        _ => return Err("无效的风格".to_string()),
    };

    let options = IllustrationOptions {
        style: art_style,
        aspect_ratio,
        quality,
        variations,
        color_palette: None,
        mood: None,
        lighting: None,
    };

    let illustration = state
        .illustration_generator
        .generate_scene_illustration(&scene, options)
        .await?;

    serde_json::to_string(&illustration).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn mmg_generate_character_portrait(
    character_id: String,
    character_name: String,
    appearance: String,
    style: String,
    state: State<'_, MultimediaState>,
) -> Result<String, String> {
    let art_style = match style.as_str() {
        "realistic" => ArtStyle::Realistic,
        "anime" => ArtStyle::Anime,
        "manga" => ArtStyle::Manga,
        "watercolor" => ArtStyle::Watercolor,
        "oil_painting" => ArtStyle::OilPainting,
        "digital_art" => ArtStyle::DigitalArt,
        "concept_art" => ArtStyle::ConceptArt,
        "fantasy" => ArtStyle::Fantasy,
        "cyberpunk" => ArtStyle::Cyberpunk,
        "steampunk" => ArtStyle::Steampunk,
        "minimalist" => ArtStyle::Minimalist,
        _ => return Err("无效的风格".to_string()),
    };

    let portrait = state
        .illustration_generator
        .generate_character_portrait(character_id, character_name, appearance, art_style)
        .await?;

    serde_json::to_string(&portrait).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn mmg_generate_cover(
    project_name: String,
    project_description: String,
    genre: String,
    style: String,
    state: State<'_, MultimediaState>,
) -> Result<String, String> {
    let art_style = match style.as_str() {
        "realistic" => ArtStyle::Realistic,
        "anime" => ArtStyle::Anime,
        "manga" => ArtStyle::Manga,
        "watercolor" => ArtStyle::Watercolor,
        "oil_painting" => ArtStyle::OilPainting,
        "digital_art" => ArtStyle::DigitalArt,
        "concept_art" => ArtStyle::ConceptArt,
        "fantasy" => ArtStyle::Fantasy,
        "cyberpunk" => ArtStyle::Cyberpunk,
        "steampunk" => ArtStyle::Steampunk,
        "minimalist" => ArtStyle::Minimalist,
        _ => return Err("无效的风格".to_string()),
    };

    let cover = state
        .illustration_generator
        .generate_cover(project_name, project_description, genre, art_style)
        .await?;

    Ok(cover)
}
