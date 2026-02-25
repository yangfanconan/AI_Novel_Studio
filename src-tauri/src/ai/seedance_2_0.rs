use serde::{Deserialize, Serialize};
use tauri::command;

use super::storyboard_system::{Storyboard, StoryboardScene, StoryboardShot};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeedanceConstraints {
    pub max_images: usize,
    pub max_videos: usize,
    pub max_audio: usize,
    pub max_prompt_length: usize,
}

impl Default for SeedanceConstraints {
    fn default() -> Self {
        Self {
            max_images: 9,
            max_videos: 3,
            max_audio: 3,
            max_prompt_length: 5000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultimodalReference {
    #[serde(rename = "type")]
    pub ref_type: String,
    pub id: String,
    pub url: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptLayer {
    pub action: String,
    pub cinematography: String,
    pub dialogue: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeedanceRequest {
    pub prompt: String,
    pub images: Vec<MultimodalReference>,
    pub videos: Vec<MultimodalReference>,
    pub audio: Vec<MultimodalReference>,
    pub first_frame_grid: Option<FirstFrameGrid>,
    pub duration: Option<f32>,
    pub aspect_ratio: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirstFrameGrid {
    pub rows: usize,
    pub cols: usize,
    pub images: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeedanceResponse {
    pub video_url: String,
    pub generation_id: String,
    pub duration: f32,
    pub resolution: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

pub struct SeedanceEngine {
    constraints: SeedanceConstraints,
}

impl SeedanceEngine {
    pub fn new() -> Self {
        Self {
            constraints: SeedanceConstraints::default(),
        }
    }

    pub fn with_constraints(mut self, constraints: SeedanceConstraints) -> Self {
        self.constraints = constraints;
        self
    }

    pub fn validate_request(&self, request: &SeedanceRequest) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        if request.prompt.len() > self.constraints.max_prompt_length {
            errors.push(format!(
                "Prompt length {} exceeds maximum of {} characters",
                request.prompt.len(),
                self.constraints.max_prompt_length
            ));
        }

        if request.images.len() > self.constraints.max_images {
            errors.push(format!(
                "Image count {} exceeds maximum of {}",
                request.images.len(),
                self.constraints.max_images
            ));
        }

        if request.videos.len() > self.constraints.max_videos {
            errors.push(format!(
                "Video count {} exceeds maximum of {}",
                request.videos.len(),
                self.constraints.max_videos
            ));
        }

        if request.audio.len() > self.constraints.max_audio {
            errors.push(format!(
                "Audio count {} exceeds maximum of {}",
                request.audio.len(),
                self.constraints.max_audio
            ));
        }

        if request.images.is_empty() && request.videos.is_empty() && request.audio.is_empty() {
            warnings.push("No multimodal references provided. Consider adding character or scene references for better results.".to_string());
        }

        if let Some(grid) = &request.first_frame_grid {
            let total_images = grid.rows * grid.cols;
            if total_images > self.constraints.max_images {
                errors.push(format!(
                    "Grid image count {} exceeds maximum of {}",
                    total_images,
                    self.constraints.max_images
                ));
            }
            if grid.images.len() != total_images {
                errors.push(format!(
                    "Grid expects {} images but {} provided",
                    total_images,
                    grid.images.len()
                ));
            }
        }

        ValidationResult {
            valid: errors.is_empty(),
            errors,
            warnings,
        }
    }

    pub fn build_smart_prompt(&self, layers: PromptLayer) -> String {
        let mut parts = Vec::new();

        if !layers.action.is_empty() {
            parts.push(format!("Action: {}", layers.action));
        }

        if !layers.cinematography.is_empty() {
            parts.push(format!("Cinematography: {}", layers.cinematography));
        }

        if !layers.dialogue.is_empty() {
            parts.push(format!("Dialogue with lip sync: {}", layers.dialogue));
        }

        if parts.is_empty() {
            String::new()
        } else {
            parts.join(", ")
        }
    }

    pub fn create_first_frame_grid(&self, storyboard: &Storyboard, rows: usize, cols: usize) -> Option<FirstFrameGrid> {
        let all_shots: Vec<&StoryboardShot> = storyboard
            .scenes
            .iter()
            .flat_map(|s| &s.shots)
            .collect();
        
        let total_needed = rows * cols;

        if all_shots.len() < total_needed {
            return None;
        }

        let images: Vec<String> = all_shots
            .iter()
            .take(total_needed)
            .filter_map(|shot| shot.visual_reference.clone())
            .collect();

        if images.len() != total_needed {
            return None;
        }

        Some(FirstFrameGrid {
            rows,
            cols,
            images,
        })
    }

    pub fn collect_references(
        &self,
        storyboard: &Storyboard,
    ) -> (Vec<MultimodalReference>, Vec<MultimodalReference>, Vec<MultimodalReference>) {
        let mut images: Vec<MultimodalReference> = Vec::new();
        let mut videos: Vec<MultimodalReference> = Vec::new();
        let mut audio: Vec<MultimodalReference> = Vec::new();

        for scene in &storyboard.scenes {
            for shot in &scene.shots {
                if let Some(ref img) = shot.visual_reference {
                    if !img.is_empty() {
                        images.push(MultimodalReference {
                            ref_type: "image".to_string(),
                            id: shot.id.clone(),
                            url: img.clone(),
                            description: Some(shot.description.clone()),
                        });
                    }
                }

                if let Some(ref vid) = shot.video_reference {
                    if !vid.is_empty() {
                        videos.push(MultimodalReference {
                            ref_type: "video".to_string(),
                            id: shot.id.clone(),
                            url: vid.clone(),
                            description: Some(shot.description.clone()),
                        });
                    }
                }

                if let Some(ref aud) = shot.audio_reference {
                    if !aud.is_empty() {
                        audio.push(MultimodalReference {
                            ref_type: "audio".to_string(),
                            id: shot.id.clone(),
                            url: aud.clone(),
                            description: Some(format!("Audio for {}", shot.description)),
                        });
                    }
                }
            }
        }

        (images, videos, audio)
    }

    pub fn create_narrative_video_request(
        &self,
        storyboard: &Storyboard,
        prompt: String,
        duration: Option<f32>,
    ) -> SeedanceRequest {
        let (images, videos, audio) = self.collect_references(storyboard);

        SeedanceRequest {
            prompt,
            images,
            videos,
            audio,
            first_frame_grid: self.create_first_frame_grid(storyboard, 3, 3),
            duration,
            aspect_ratio: Some("16:9".to_string()),
        }
    }
}

impl Default for SeedanceEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[command]
pub fn seedance_validate_request(request: SeedanceRequest) -> ValidationResult {
    let engine = SeedanceEngine::new();
    engine.validate_request(&request)
}

#[command]
pub fn seedance_build_prompt(action: String, cinematography: String, dialogue: String) -> String {
    let engine = SeedanceEngine::new();
    let layers = PromptLayer {
        action,
        cinematography,
        dialogue,
    };
    engine.build_smart_prompt(layers)
}

#[command]
pub fn seedance_get_constraints() -> SeedanceConstraints {
    SeedanceConstraints::default()
}

#[command]
pub fn seedance_create_grid(
    images: Vec<String>,
    rows: usize,
    cols: usize,
) -> Result<FirstFrameGrid, String> {
    let total_expected = rows * cols;
    if images.len() != total_expected {
        return Err(format!(
            "Expected {} images for {}x{} grid, got {}",
            total_expected, rows, cols, images.len()
        ));
    }

    Ok(FirstFrameGrid {
        rows,
        cols,
        images,
    })
}

#[command]
pub fn seedance_validate_grid(rows: usize, cols: usize, image_count: usize) -> ValidationResult {
    let constraints = SeedanceConstraints::default();
    let total = rows * cols;
    let mut errors: Vec<String> = Vec::new();

    if total > constraints.max_images {
        errors.push(format!(
            "Grid {}x{} ({} images) exceeds maximum of {}",
            rows, cols, total, constraints.max_images
        ));
    }

    if image_count != total {
        errors.push(format!(
            "Grid expects {} images but {} provided",
            total, image_count
        ));
    }

    ValidationResult {
        valid: errors.is_empty(),
        errors,
        warnings: Vec::new(),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeVideoConfig {
    pub storyboard_id: String,
    pub custom_prompt: Option<String>,
    pub duration: Option<f32>,
    pub aspect_ratio: Option<String>,
    pub include_audio: bool,
    pub include_references: bool,
}

impl Default for NarrativeVideoConfig {
    fn default() -> Self {
        Self {
            storyboard_id: String::new(),
            custom_prompt: None,
            duration: Some(5.0),
            aspect_ratio: Some("16:9".to_string()),
            include_audio: true,
            include_references: true,
        }
    }
}

#[command]
pub fn seedance_prepare_narrative_video(
    storyboard: Storyboard,
    config: NarrativeVideoConfig,
) -> Result<SeedanceRequest, String> {
    let engine = SeedanceEngine::new();

    let prompt = match config.custom_prompt {
        Some(p) => p,
        None => {
            let layers = PromptLayer {
                action: storyboard.scenes
                    .first()
                    .and_then(|s| s.shots.first())
                    .map(|s| s.action.clone())
                    .unwrap_or_default(),
                cinematography: storyboard
                    .scenes
                    .first()
                    .and_then(|s| s.shots.first())
                    .map(|s| format!("{:?}", s.camera_movement))
                    .unwrap_or_default(),
                dialogue: storyboard
                    .scenes
                    .iter()
                    .flat_map(|s| &s.shots)
                    .filter_map(|s| s.dialogue.clone())
                    .collect::<Vec<_>>()
                    .join("; "),
            };
            engine.build_smart_prompt(layers)
        }
    };

    let mut request = engine.create_narrative_video_request(&storyboard, prompt, config.duration);

    if let Some(ratio) = config.aspect_ratio {
        request.aspect_ratio = Some(ratio);
    }

    if !config.include_audio {
        request.audio.clear();
    }

    if !config.include_references {
        request.images.clear();
        request.videos.clear();
    }

    Ok(request)
}
