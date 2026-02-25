use crate::multimedia_generation::types::*;
use crate::ai::traits::AIModel;
use crate::ai::models::{AIRequest, AIMessage};
use serde::{Serialize, Deserialize};
use std::sync::Arc;

pub struct AnimationGenerator {
    ai_model: Arc<dyn AIModel>,
}

impl AnimationGenerator {
    pub fn new(ai_model: Arc<dyn AIModel>) -> Self {
        Self { ai_model }
    }

    pub async fn generate_keyframes(
        &self,
        scene: &Scene,
        shot_count: usize,
    ) -> Result<Vec<Keyframe>, String> {
        let prompt = self.build_keyframe_prompt(scene, shot_count);

        let request = AIRequest {
            model: self.ai_model.get_name(),
            messages: vec![AIMessage {
                role: "user".to_string(),
                content: prompt,
            }],
            temperature: Some(0.5),
            max_tokens: None,
            stream: Some(false),
        };

        let response = self.ai_model.complete(request).await.map_err(|e| e.to_string())?;

        let keyframes = self.parse_keyframes(&response.content)?;
        Ok(keyframes)
    }

    fn build_keyframe_prompt(&self, scene: &Scene, shot_count: usize) -> String {
        format!(
            r#"Based on the following scene description, generate detailed descriptions for {} keyframes.

Scene Information:
- Title: {}
- Location: {}
- Time: {}
- Description: {}
- Emotional Tone: {}
- Characters: {}

For each keyframe, please provide:
1. Timestamp (seconds)
2. Frame description (detailed visual description for AI image generation)
3. Character actions (character actions and expressions)
4. Camera information (angle, movement, focus)
5. Sound/music cues

Return format (JSON array):
[
  {{
    "timestamp": 0.0,
    "description": "frame description",
    "character_actions": ["action1", "action2"],
    "camera": {{ "angle": "medium shot", "movement": "fixed", "focus": "character" }},
    "audio": {{ "sound": "footsteps", "music": "tense" }}
  }}
]"#,
            shot_count,
            scene.title,
            scene.location,
            format!("{:?}", scene.time_of_day),
            scene.description,
            format!("{:?}", scene.emotional_tone),
            scene.characters.iter().map(|c| c.name.as_str()).collect::<Vec<_>>().join(", ")
        )
    }

    fn parse_keyframes(&self, content: &str) -> Result<Vec<Keyframe>, String> {
        let json_start = content.find('[').ok_or("Array start not found")?;
        let json_str = &content[json_start..];

        let keyframes: Vec<Keyframe> = serde_json::from_str(json_str)
            .map_err(|e| format!("Failed to parse keyframes: {}", e))?;

        Ok(keyframes)
    }

    pub async fn generate_animation_sequence(
        &self,
        keyframes: &[Keyframe],
        fps: u32,
        duration: f32,
    ) -> Result<AnimationSequence, String> {
        let total_frames = (fps as f32 * duration) as u32;

        let frames = self.interpolate_frames(keyframes, total_frames, fps)?;

        Ok(AnimationSequence {
            id: format!("anim_{}", uuid::Uuid::new_v4()),
            keyframes: keyframes.to_vec(),
            frames,
            fps,
            duration,
            total_frames,
        })
    }

    fn interpolate_frames(
        &self,
        keyframes: &[Keyframe],
        total_frames: u32,
        fps: u32,
    ) -> Result<Vec<Frame>, String> {
        let mut frames = Vec::new();

        for keyframe_idx in 0..keyframes.len() {
            let current_keyframe = &keyframes[keyframe_idx];
            let next_keyframe = keyframes.get(keyframe_idx + 1);

            let start_frame = (current_keyframe.timestamp * fps as f32) as u32;
            let end_frame = match next_keyframe {
                Some(kf) => (kf.timestamp * fps as f32) as u32,
                None => total_frames,
            };

            for frame_idx in start_frame..end_frame.min(total_frames) {
                let progress = if end_frame > start_frame {
                    (frame_idx - start_frame) as f32 / (end_frame - start_frame) as f32
                } else {
                    0.0
                };

                let description = self.interpolate_description(
                    &current_keyframe.description,
                    next_keyframe.map(|kf| kf.description.as_str()),
                    progress,
                );

                frames.push(Frame {
                    number: frame_idx,
                    timestamp: frame_idx as f32 / fps as f32,
                    description,
                    keyframe_id: current_keyframe.id.clone(),
                });
            }
        }

        Ok(frames)
    }

    fn interpolate_description(
        &self,
        current: &str,
        next: Option<&str>,
        progress: f32,
    ) -> String {
        match next {
            Some(next_desc) => {
                format!(
                    "{} (transition {:.0}%)",
                    current,
                    progress * 100.0
                )
            }
            None => current.to_string(),
        }
    }

    pub async fn generate_motion_data(
        &self,
        keyframes: &[Keyframe],
    ) -> Result<MotionData, String> {
        let mut camera_movements = Vec::new();
        let mut character_animations = Vec::new();

        for (i, keyframe) in keyframes.iter().enumerate() {
            let next_keyframe = keyframes.get(i + 1);

            if let Some(next_kf) = next_keyframe {
                let movement = CameraMovement {
                    start_time: keyframe.timestamp,
                    end_time: next_kf.timestamp,
                    from_angle: keyframe.camera.angle.clone(),
                    to_angle: next_kf.camera.angle.clone(),
                    movement_type: keyframe.camera.movement.clone(),
                };
                camera_movements.push(movement);
            }

            for (char_idx, action) in keyframe.character_actions.iter().enumerate() {
                let animation = CharacterAnimation {
                    character_id: format!("char_{}", char_idx),
                    start_time: keyframe.timestamp,
                    end_time: next_keyframe
                        .map(|kf| kf.timestamp)
                        .unwrap_or(keyframe.timestamp),
                    action: action.clone(),
                    intensity: 0.5,
                };
                character_animations.push(animation);
            }
        }

        Ok(MotionData {
            camera_movements,
            character_animations,
        })
    }

    pub async fn export_animation(
        &self,
        sequence: &AnimationSequence,
        motion_data: &MotionData,
        format: AnimationFormat,
    ) -> Result<String, String> {
        let output_path = format!("animation_{}.{}", sequence.id, format.extension());

        match format {
            AnimationFormat::JSON => {
                let export_data = serde_json::to_string(sequence)
                    .map_err(|e| format!("Failed to export JSON: {}", e))?;
                std::fs::write(&output_path, export_data)
                    .map_err(|e| format!("Failed to write file: {}", e))?;
            }
            AnimationFormat::CSV => {
                let mut csv = String::from("frame_number,timestamp,description\n");
                for frame in &sequence.frames {
                    csv.push_str(&format!(
                        "{},{},{}\n",
                        frame.number, frame.timestamp, frame.description
                    ));
                }
                std::fs::write(&output_path, csv)
                    .map_err(|e| format!("Failed to write CSV: {}", e))?;
            }
        }

        Ok(output_path)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyframe {
    pub id: String,
    pub timestamp: f32,
    pub description: String,
    pub character_actions: Vec<String>,
    pub camera: CameraInfo,
    pub audio: AudioInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraInfo {
    pub angle: String,
    pub movement: String,
    pub focus: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioInfo {
    pub sound: Option<String>,
    pub music: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationSequence {
    pub id: String,
    pub keyframes: Vec<Keyframe>,
    pub frames: Vec<Frame>,
    pub fps: u32,
    pub duration: f32,
    pub total_frames: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frame {
    pub number: u32,
    pub timestamp: f32,
    pub description: String,
    pub keyframe_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionData {
    pub camera_movements: Vec<CameraMovement>,
    pub character_animations: Vec<CharacterAnimation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraMovement {
    pub start_time: f32,
    pub end_time: f32,
    pub from_angle: String,
    pub to_angle: String,
    pub movement_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterAnimation {
    pub character_id: String,
    pub start_time: f32,
    pub end_time: f32,
    pub action: String,
    pub intensity: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnimationFormat {
    JSON,
    CSV,
}

impl AnimationFormat {
    pub fn extension(&self) -> &str {
        match self {
            AnimationFormat::JSON => "json",
            AnimationFormat::CSV => "csv",
        }
    }
}
