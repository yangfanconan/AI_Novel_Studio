#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use crate::models::ChapterMission;
use crate::logger::Logger;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViolationType {
    ForbiddenCharacter(String),
    ForbiddenTopic(String),
    ForbiddenEmoji(String),
    LengthExceeded(i32),
    LengthInsufficient(i32),
    InfoLeak(String),
    POVInconsistency(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardrailsCheckResult {
    pub passed: bool,
    pub violations: Vec<ViolationType>,
    pub suggestions: Vec<String>,
    pub auto_fix_available: bool,
}

pub struct GuardrailsService {
    logger: Logger,
}

impl GuardrailsService {
    pub fn new() -> Self {
        Self {
            logger: Logger::new().with_feature("guardrails_service"),
        }
    }

    pub async fn check_chapter(
        &self,
        content: &str,
        mission: Option<&ChapterMission>,
    ) -> Result<GuardrailsCheckResult, String> {
        self.logger.info("Checking chapter against guardrails");

        let mut violations = Vec::new();
        let mut suggestions = Vec::new();
        let _content_len = content.chars().count() as i32;

        if let Some(m) = mission {
            for forbidden_char in &m.forbidden_characters {
                if content.contains(forbidden_char) {
                    violations.push(ViolationType::ForbiddenCharacter(forbidden_char.clone()));
                    suggestions.push(format!("请移除角色 \"{}\" 的相关内容", forbidden_char));
                }
            }

            for forbidden_topic in &m.forbidden_characters {
                let topic_keywords: Vec<&str> = forbidden_topic.split(',').collect();
                for keyword in topic_keywords {
                    if content.contains(keyword.trim()) {
                        violations.push(ViolationType::ForbiddenTopic(forbidden_topic.clone()));
                        suggestions.push(format!("请修改涉及 \"{}\" 的内容", forbidden_topic));
                        break;
                    }
                }
            }
        }

        let emoji_pattern = regex::Regex::new(r"[\x{1F300}-\x{1F9FF}]").unwrap();
        if let Some(emoji) = emoji_pattern.find(content) {
            violations.push(ViolationType::ForbiddenEmoji(emoji.as_str().to_string()));
            suggestions.push("请移除表情符号".to_string());
        }

        let passed = violations.is_empty();
        let auto_fix_available = !passed && violations.len() <= 3;

        Ok(GuardrailsCheckResult {
            passed,
            violations,
            suggestions,
            auto_fix_available,
        })
    }

    pub async fn auto_fix(
        &self,
        content: &str,
        violations: &[ViolationType],
    ) -> Result<String, String> {
        self.logger.info(&format!("Auto-fixing {} violations", violations.len()));

        let mut fixed_content = content.to_string();

        for violation in violations {
            match violation {
                ViolationType::ForbiddenEmoji(emoji) => {
                    fixed_content = fixed_content.replace(emoji, "");
                }
                _ => {}
            }
        }

        Ok(fixed_content)
    }

    pub async fn check_length(
        &self,
        content: &str,
        min_length: i32,
        max_length: i32,
    ) -> GuardrailsCheckResult {
        let content_len = content.chars().count() as i32;
        let mut violations = Vec::new();
        let mut suggestions = Vec::new();

        if content_len < min_length {
            violations.push(ViolationType::LengthInsufficient(min_length - content_len));
            suggestions.push(format!("内容长度不足，还需增加约 {} 字", min_length - content_len));
        }

        if content_len > max_length {
            violations.push(ViolationType::LengthExceeded(content_len - max_length));
            suggestions.push(format!("内容超出限制，建议删减约 {} 字", content_len - max_length));
        }

        GuardrailsCheckResult {
            passed: violations.is_empty(),
            violations,
            suggestions,
            auto_fix_available: false,
        }
    }
}

impl Default for GuardrailsService {
    fn default() -> Self {
        Self::new()
    }
}
