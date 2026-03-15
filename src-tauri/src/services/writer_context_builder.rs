#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use crate::models::{ChapterMission, Character, PlotPoint};
use crate::logger::Logger;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriterContext {
    pub system_prompt: String,
    pub user_prompt: String,
    pub visible_characters: Vec<Character>,
    pub visible_plot_points: Vec<PlotPoint>,
    pub mission_context: Option<String>,
}

#[derive(Debug, Clone)]
pub struct VisibilityFilter {
    pub current_chapter: i32,
    pub appeared_characters: Vec<String>,
    pub allowed_new_characters: Vec<String>,
}

pub struct WriterContextBuilder {
    logger: Logger,
}

impl WriterContextBuilder {
    pub fn new() -> Self {
        Self {
            logger: Logger::new().with_feature("writer_context_builder"),
        }
    }

    pub async fn build_visibility_context(
        &self,
        _project_id: &str,
        chapter_id: &str,
        mission: Option<ChapterMission>,
    ) -> Result<WriterContext, String> {
        self.logger.info(&format!("Building visibility context for chapter: {}", chapter_id));

        let mut system_prompt = "你是一个专业的小说作家。请根据以下信息继续写作。".to_string();
        let mut user_prompt = String::new();
        let mut mission_context = None;

        if let Some(ref m) = mission {
            let mut context_parts = Vec::new();
            
            if !m.macro_beat.is_empty() {
                context_parts.push(format!("本章节拍目标: {}", m.macro_beat));
            }
            
            if !m.micro_beats.is_empty() {
                context_parts.push(format!("微观节拍: {}", m.micro_beats.join(", ")));
            }
            
            if let Some(ref pov) = m.pov {
                if !pov.is_empty() {
                    context_parts.push(format!("视角: {}", pov));
                }
            }
            
            if let Some(ref tone) = m.tone {
                if !tone.is_empty() {
                    context_parts.push(format!("基调: {}", tone));
                }
            }
            
            if let Some(ref pacing) = m.pacing {
                if !pacing.is_empty() {
                    context_parts.push(format!("节奏: {}", pacing));
                }
            }
            
            if !m.allowed_new_characters.is_empty() {
                context_parts.push(format!("允许新登场角色: {}", m.allowed_new_characters.join(", ")));
            }
            
            if !m.forbidden_characters.is_empty() {
                context_parts.push(format!("禁止出现角色: {}", m.forbidden_characters.join(", ")));
            }
            
            if !context_parts.is_empty() {
                mission_context = Some(context_parts.join("\n"));
                system_prompt = format!("{}\n\n章节导演指令:\n{}", system_prompt, context_parts.join("\n"));
            }
        }

        user_prompt = "请继续写作以下章节内容，保持风格一致性，遵循导演脚本的指导。".to_string();

        Ok(WriterContext {
            system_prompt,
            user_prompt,
            visible_characters: Vec::new(),
            visible_plot_points: Vec::new(),
            mission_context,
        })
    }

    pub fn filter_characters(
        &self,
        characters: Vec<Character>,
        filter: &VisibilityFilter,
    ) -> Vec<Character> {
        characters
            .into_iter()
            .filter(|c| {
                filter.appeared_characters.contains(&c.id)
                    || filter.allowed_new_characters.contains(&c.id)
            })
            .collect()
    }

    pub fn build_chapter_summary_prompt(&self, content: &str) -> String {
        format!(
            "请为以下章节内容生成一个简洁的摘要（200字以内），突出主要事件和情节发展：\n\n{}",
            content
        )
    }
}

impl Default for WriterContextBuilder {
    fn default() -> Self {
        Self::new()
    }
}
