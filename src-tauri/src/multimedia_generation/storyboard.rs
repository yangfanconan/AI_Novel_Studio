use crate::multimedia_generation::types::*;
use crate::multimedia_generation::scene_extractor::SceneExtractor;
use crate::ai::traits::AIModel;
use crate::ai::models::{AIRequest, AIResponse, AIMessage};
use std::sync::Arc;

pub struct StoryboardGenerator {
    pub ai_model: Arc<dyn AIModel>,
    scene_extractor: SceneExtractor,
}

impl StoryboardGenerator {
    pub fn new(ai_model: Arc<dyn AIModel>) -> Self {
        let scene_extractor = SceneExtractor::new(ai_model.clone());
        Self {
            ai_model,
            scene_extractor,
        }
    }

    pub async fn generate_storyboard(
        &self,
        text: &str,
        options: StoryboardOptions,
    ) -> Result<Storyboard, String> {
        let scenes = self.scene_extractor.extract_scenes(text).await?;

        let mut storyboard_scenes = Vec::new();

        for scene in &scenes {
            let storyboard_scene = self.generate_scene_storyboard(scene, &options).await?;
            storyboard_scenes.push(storyboard_scene);
        }

        let total_duration = storyboard_scenes.iter().map(|s| s.estimated_duration).sum();

        let metadata = StoryboardMetadata {
            generated_at: chrono::Utc::now().to_rfc3339(),
            source_text: text.to_string(),
            options: options.clone(),
        };

        Ok(Storyboard {
            title: options.title.unwrap_or_else(|| "未命名分镜".to_string()),
            format: options.format.clone(),
            style: options.style.clone(),
            scenes: storyboard_scenes,
            total_duration,
            metadata,
        })
    }

    async fn generate_scene_storyboard(
        &self,
        scene: &Scene,
        options: &StoryboardOptions,
    ) -> Result<StoryboardScene, String> {
        let prompt = self.build_storyboard_prompt(scene, options);

        let request = AIRequest {
            model: self.ai_model.get_name(),
            messages: vec![AIMessage {
                role: "user".to_string(),
                content: prompt,
            }],
            temperature: Some(0.4),
            max_tokens: None,
            stream: Some(false),
        };

        let response = self
            .ai_model
            .complete(request)
            .await
            .map_err(|e| e.to_string())?;

        let shots = self.parse_shots(&response.content)?;

        let estimated_duration = self.estimate_scene_duration(&shots);
        let color_mood = self.suggest_color_mood(scene);

        Ok(StoryboardScene {
            scene_number: scene.number,
            title: scene.title.clone(),
            location: scene.location.clone(),
            time_of_day: scene.time_of_day.clone(),
            shots,
            estimated_duration,
            notes: self.generate_scene_notes(scene),
            color_mood,
        })
    }

    fn build_storyboard_prompt(&self, scene: &Scene, options: &StoryboardOptions) -> String {
        format!(
            "请将以下场景转换为专业的分镜脚本：

场景信息：
- 标题：{}
- 地点：{}
- 时间：{:?}
- 出场角色：{}
- 场景描述：{}
- 主要动作：{}
- 情感基调：{:?}

原文片段：
{}

格式要求：{:?}
风格要求：{:?}

请为每个镜头提供以下信息，以JSON数组格式输出：
1. shot_number - 镜头编号
2. shot_type - 景别（ExtremeCloseUp/CloseUp/MediumCloseUp/MediumShot/MediumFullShot/FullShot/LongShot/ExtremeLongShot/OverTheShoulder/Pov/TwoShot/Establishing）
3. description - 画面描述
4. camera - 镜头运动（movement_type, direction, speed, description）
5. characters - 角色列表
6. action - 角色动作
7. dialogue - 对白（可选）
8. sound_effects - 音效列表（可选）
9. duration - 时长（秒）
10. transition - 转场（可选）
11. visual_notes - 视觉备注（可选）

输出格式示例：
[
  {{
    \"shot_number\": 1,
    \"shot_type\": \"LongShot\",
    \"description\": \"广阔的森林景观\",
    \"camera\": {{
      \"movement_type\": \"Static\",
      \"direction\": null,
      \"speed\": null,
      \"description\": \"固定镜头\"
    }},
    \"characters\": [],
    \"action\": \"\",
    \"dialogue\": null,
    \"sound_effects\": [\"鸟鸣声\"],
    \"duration\": 3.0,
    \"transition\": null,
    \"visual_notes\": null
  }}]",
            scene.title,
            scene.location,
            scene.time_of_day,
            scene.characters.iter().map(|c| c.name.as_str()).collect::<Vec<_>>().join("、"),
            scene.description,
            scene.action,
            scene.emotional_tone,
            scene.original_text,
            options.format,
            options.style
        )
    }

    fn parse_shots(&self, json_text: &str) -> Result<Vec<Shot>, String> {
        serde_json::from_str(json_text).map_err(|e| format!("解析镜头失败: {}", e))
    }

    fn estimate_scene_duration(&self, shots: &[Shot]) -> f64 {
        shots.iter().map(|s| s.duration).sum()
    }

    fn suggest_color_mood(&self, scene: &Scene) -> ColorPalette {
        match scene.emotional_tone {
            EmotionalTone::Happy => ColorPalette {
                primary: "#FFD700".to_string(),
                secondary: "#FFA500".to_string(),
                accent: "#FF6347".to_string(),
                background: "#FFF8DC".to_string(),
            },
            EmotionalTone::Sad => ColorPalette {
                primary: "#708090".to_string(),
                secondary: "#4169E1".to_string(),
                accent: "#6495ED".to_string(),
                background: "#E6E6FA".to_string(),
            },
            EmotionalTone::Tense => ColorPalette {
                primary: "#DC143C".to_string(),
                secondary: "#B22222".to_string(),
                accent: "#FF4500".to_string(),
                background: "#2F4F4F".to_string(),
            },
            EmotionalTone::Romantic => ColorPalette {
                primary: "#FF69B4".to_string(),
                secondary: "#FFB6C1".to_string(),
                accent: "#FF1493".to_string(),
                background: "#FFF0F5".to_string(),
            },
            EmotionalTone::Mysterious => ColorPalette {
                primary: "#4B0082".to_string(),
                secondary: "#483D8B".to_string(),
                accent: "#6A5ACD".to_string(),
                background: "#191970".to_string(),
            },
            EmotionalTone::Action => ColorPalette {
                primary: "#FF4500".to_string(),
                secondary: "#FF6347".to_string(),
                accent: "#FFD700".to_string(),
                background: "#2C3E50".to_string(),
            },
            EmotionalTone::Peaceful => ColorPalette {
                primary: "#90EE90".to_string(),
                secondary: "#98FB98".to_string(),
                accent: "#00FA9A".to_string(),
                background: "#F0FFF0".to_string(),
            },
            EmotionalTone::Dramatic => ColorPalette {
                primary: "#8B0000".to_string(),
                secondary: "#A52A2A".to_string(),
                accent: "#CD5C5C".to_string(),
                background: "#1C1C1C".to_string(),
            },
            EmotionalTone::Horror => ColorPalette {
                primary: "#2F4F4F".to_string(),
                secondary: "#006400".to_string(),
                accent: "#800000".to_string(),
                background: "#000000".to_string(),
            },
            EmotionalTone::Comedy => ColorPalette {
                primary: "#FFFF00".to_string(),
                secondary: "#FFD700".to_string(),
                accent: "#FFA500".to_string(),
                background: "#FFFFE0".to_string(),
            },
        }
    }

    fn generate_scene_notes(&self, scene: &Scene) -> String {
        format!(
            "场景{} - {} - {:?}",
            scene.number, scene.location, scene.time_of_day
        )
    }
}
