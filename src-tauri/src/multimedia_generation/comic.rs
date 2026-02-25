use crate::multimedia_generation::types::*;
use crate::ai::traits::AIModel;
use crate::ai::models::{AIRequest, AIResponse, AIMessage};
use std::sync::Arc;

pub struct ComicGenerator {
    ai_model: Arc<dyn AIModel>,
}

impl ComicGenerator {
    pub fn new(ai_model: Arc<dyn AIModel>) -> Self {
        Self { ai_model }
    }

    pub async fn generate_comic(
        &self,
        text: &str,
        title: &str,
        style: ComicStyle,
    ) -> Result<Comic, String> {
        let scenes = self.extract_comic_scenes(text).await?;

        let mut pages = Vec::new();
        for (i, scene) in scenes.iter().enumerate() {
            let page = self.generate_comic_page(scene, i, &style, &title).await?;
            pages.push(page);
        }

        let total_panels: i32 = pages.iter().map(|p| p.panels.len() as i32).sum();

        let metadata = ComicMetadata {
            generated_at: chrono::Utc::now().to_rfc3339(),
            total_pages: pages.len() as i32,
            total_panels,
        };

        Ok(Comic {
            title: title.to_string(),
            style,
            pages,
            metadata,
        })
    }

    async fn extract_comic_scenes(&self, text: &str) -> Result<Vec<Scene>, String> {
        let prompt = format!(
            "请从以下文本中提取适合漫画表现的关键场景：

文本：
{}

请为每个场景提供：
1. id - 场景ID
2. number - 场景编号
3. title - 场景标题
4. location - 地点
5. time_of_day - 时间
6. characters - 角色列表
7. description - 场景描述
8. action - 主要动作
9. emotional_tone - 情感基调
10. suggested_shots - 建议镜头类型
11. original_text - 原文片段

以JSON数组格式输出。",
            text
        );

        let request = AIRequest {
            model: self.ai_model.get_name(),
            messages: vec![AIMessage {
                role: "user".to_string(),
                content: prompt,
            }],
            temperature: Some(0.3),
            max_tokens: None,
            stream: Some(false),
        };

        let response = self
            .ai_model
            .complete(request)
            .await
            .map_err(|e| e.to_string())?;

        serde_json::from_str(&response.content).map_err(|e| format!("解析场景失败: {}", e))
    }

    async fn generate_comic_page(
        &self,
        scene: &Scene,
        page_index: usize,
        style: &ComicStyle,
        title: &str,
    ) -> Result<ComicPage, String> {
        let layout = self.calculate_panel_layout(scene, page_index).await?;

        let mut panels = Vec::new();
        for (i, position) in layout.positions.iter().enumerate() {
            let panel = self
                .generate_panel(scene, i, position, style, title)
                .await?;
            panels.push(panel);
        }

        Ok(ComicPage {
            page_number: page_index as i32 + 1,
            layout: layout.layout_type,
            panels,
        })
    }

    async fn calculate_panel_layout(&self, scene: &Scene, _page_index: usize) -> Result<ComicLayout, String> {
        let panel_count = if scene.action.len() > 100 {
            6
        } else if scene.action.len() > 50 {
            4
        } else if scene.action.len() > 20 {
            3
        } else {
            2
        };

        let layout_type = match panel_count {
            1 => LayoutType::OnePanel,
            2 => LayoutType::TwoVertical,
            3 => LayoutType::ThreeEqual,
            4 => LayoutType::FourGrid,
            5 => LayoutType::FiveVariable,
            6 => LayoutType::SixGrid,
            _ => LayoutType::FourGrid,
        };

        let positions = self.calculate_positions(layout_type.clone(), panel_count);

        Ok(ComicLayout {
            layout_type,
            panel_count,
            positions,
        })
    }

    fn calculate_positions(&self, layout_type: LayoutType, panel_count: usize) -> Vec<PanelPosition> {
        let page_width = 800.0;
        let page_height = 1200.0;

        match layout_type {
            LayoutType::OnePanel => vec![PanelPosition {
                x: 0.0,
                y: 0.0,
                width: page_width,
                height: page_height,
            }],
            LayoutType::TwoVertical => vec![
                PanelPosition {
                    x: 0.0,
                    y: 0.0,
                    width: page_width,
                    height: page_height / 2.0,
                },
                PanelPosition {
                    x: 0.0,
                    y: page_height / 2.0,
                    width: page_width,
                    height: page_height / 2.0,
                },
            ],
            LayoutType::TwoHorizontal => vec![
                PanelPosition {
                    x: 0.0,
                    y: 0.0,
                    width: page_width / 2.0,
                    height: page_height,
                },
                PanelPosition {
                    x: page_width / 2.0,
                    y: 0.0,
                    width: page_width / 2.0,
                    height: page_height,
                },
            ],
            LayoutType::ThreeEqual => {
                let panel_height = page_height / 3.0;
                (0..panel_count)
                    .map(|i| PanelPosition {
                        x: 0.0,
                        y: i as f64 * panel_height,
                        width: page_width,
                        height: panel_height,
                    })
                    .collect()
            }
            LayoutType::FourGrid => {
                let panel_width = page_width / 2.0;
                let panel_height = page_height / 2.0;
                vec![
                    PanelPosition {
                        x: 0.0,
                        y: 0.0,
                        width: panel_width,
                        height: panel_height,
                    },
                    PanelPosition {
                        x: panel_width,
                        y: 0.0,
                        width: panel_width,
                        height: panel_height,
                    },
                    PanelPosition {
                        x: 0.0,
                        y: panel_height,
                        width: panel_width,
                        height: panel_height,
                    },
                    PanelPosition {
                        x: panel_width,
                        y: panel_height,
                        width: panel_width,
                        height: panel_height,
                    },
                ]
            }
            LayoutType::SixGrid => {
                let panel_width = page_width / 2.0;
                let panel_height = page_height / 3.0;
                (0..panel_count)
                    .map(|i| PanelPosition {
                        x: (i % 2) as f64 * panel_width,
                        y: (i / 2) as f64 * panel_height,
                        width: panel_width,
                        height: panel_height,
                    })
                    .collect()
            }
            _ => vec![PanelPosition {
                x: 0.0,
                y: 0.0,
                width: page_width,
                height: page_height,
            }],
        }
    }

    async fn generate_panel(
        &self,
        scene: &Scene,
        panel_index: usize,
        position: &PanelPosition,
        style: &ComicStyle,
        _title: &str,
    ) -> Result<ComicPanel, String> {
        let visual_description = self
            .generate_visual_description(scene, panel_index, style)
            .await?;

        let image = self
            .generate_placeholder_image(&visual_description, position)
            .await;

        let speech_bubbles = self.generate_speech_bubbles(scene, panel_index).await?;
        let sound_effects = self.generate_sound_effects(scene, panel_index).await?;

        Ok(ComicPanel {
            index: panel_index as i32,
            position: position.clone(),
            image,
            speech_bubbles,
            sound_effects,
            border_style: "solid".to_string(),
        })
    }

    async fn generate_visual_description(
        &self,
        scene: &Scene,
        panel_index: usize,
        style: &ComicStyle,
    ) -> Result<String, String> {
        let prompt = format!(
            "请为漫画第{}格生成详细的画面描述：

场景：{}
角色：{}
动作：{}
情感：{:?}

风格要求：{:?}

请提供：
1. 构图描述
2. 角色姿势和表情
3. 背景细节
4. 光线效果
5. 视角

返回一个简洁的画面描述字符串。",
            panel_index + 1,
            scene.description,
            scene.characters.iter().map(|c| c.name.as_str()).collect::<Vec<_>>().join("、"),
            scene.action,
            scene.emotional_tone,
            style
        );

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

        Ok(response.content.trim().to_string())
    }

    async fn generate_placeholder_image(
        &self,
        description: &str,
        position: &PanelPosition,
    ) -> String {
        format!(
            "placeholder_{}x{}.png?text={}",
            position.width as i32,
            position.height as i32,
            urlencoding::encode(description)
        )
    }

    async fn generate_speech_bubbles(
        &self,
        scene: &Scene,
        _panel_index: usize,
    ) -> Result<Vec<SpeechBubble>, String> {
        let mut bubbles = Vec::new();

        for (i, character) in scene.characters.iter().enumerate() {
            if let Some(dialogues) = &character.dialogue {
                for (j, dialogue) in dialogues.iter().enumerate() {
                    let x = 100.0 + (i * 50) as f64;
                    let y = 100.0 + (j * 80) as f64;

                    bubbles.push(SpeechBubble {
                        id: format!("bubble_{}_{}", i, j),
                        character: character.name.clone(),
                        text: dialogue.text.clone(),
                        position: (x, y),
                        bubble_type: BubbleType::Speech,
                        tail_direction: "down".to_string(),
                        style: "rounded".to_string(),
                    });
                }
            }
        }

        Ok(bubbles)
    }

    async fn generate_sound_effects(&self, scene: &Scene, _panel_index: usize) -> Result<Vec<SoundEffect>, String> {
        let prompt = format!(
            "请为以下场景生成合适的音效描述：

场景：{}
动作：{}

请列出2-3个音效，每个音效用文字描述（如\"砰！\"、\"唰！\"等）。",
            scene.description, scene.action
        );

        let request = AIRequest {
            model: self.ai_model.get_name(),
            messages: vec![AIMessage {
                role: "user".to_string(),
                content: prompt,
            }],
            temperature: Some(0.6),
            max_tokens: None,
            stream: Some(false),
        };

        let response = self
            .ai_model
            .complete(request)
            .await
            .map_err(|e| e.to_string())?;

        let lines: Vec<&str> = response.content.lines().collect();
        let mut sound_effects = Vec::new();

        for (i, line) in lines.iter().take(3).enumerate() {
            let text = line.trim().to_string();
            if !text.is_empty() {
                sound_effects.push(SoundEffect {
                    text,
                    position: (400.0, 200.0 + i as f64 * 100.0),
                    style: "impact".to_string(),
                    rotation: Some(-10.0),
                    scale: Some(1.5),
                });
            }
        }

        Ok(sound_effects)
    }
}

#[derive(Debug, Clone)]
struct ComicLayout {
    layout_type: LayoutType,
    panel_count: usize,
    positions: Vec<PanelPosition>,
}
