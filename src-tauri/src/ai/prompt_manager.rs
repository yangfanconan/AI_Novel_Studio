use super::models::PromptTemplate;
use crate::logger::Logger;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct PromptManager {
    templates: Arc<RwLock<HashMap<String, PromptTemplate>>>,
    logger: Logger,
}

impl PromptManager {
    pub fn new() -> Self {
        let manager = Self {
            templates: Arc::new(RwLock::new(HashMap::new())),
            logger: Logger::new().with_feature("prompt-manager"),
        };

        manager.load_default_templates();
        manager
    }

    fn load_default_templates(&self) {
        let templates = vec![
            PromptTemplate {
                id: "novel-continuation".to_string(),
                name: "小说续写".to_string(),
                category: "writing".to_string(),
                system_prompt: r#"你是一位专业的小说作家，擅长各种文学流派的创作。

在续写时，你必须严格遵守以下规则：

1. **角色名称一致性**：
   - 必须使用【角色信息】中提供的角色名称，绝对不能自行创造新名字
   - 如果文中提到的角色在【角色信息】中找不到，保持原文中的称呼方式
   - 不要随意更改角色的姓氏、名字或昵称

2. **角色性格一致性**：
   - 角色的言行必须符合其性格设定
   - 对话风格要符合角色的身份和背景

3. **世界观一致性**：
   - 遵守【世界观设定】中的规则和设定
   - 不要引入与世界观数据相矛盾的元素

4. **情节连贯性**：
   - 续写内容要与前文自然衔接
   - 保持文风、节奏的一致性

请根据给定的上下文继续创作，续写内容应当自然流畅，符合故事发展逻辑。"#.to_string(),
                user_prompt_template: r#"请根据以下内容续写小说：

【世界观设定】
{worldview_context}

【角色信息】
{character_context}

【前文内容】
{context}

【续写要求】
{instruction}

请直接续写内容，不需要重复原文。记住：必须使用上述角色信息中的准确名称！"#.to_string(),
                variables: vec!["context".to_string(), "instruction".to_string(), "character_context".to_string(), "worldview_context".to_string()],
            },
            PromptTemplate {
                id: "novel-rewrite".to_string(),
                name: "小说重写".to_string(),
                category: "writing".to_string(),
                system_prompt: "你是一位专业的编辑和作家，擅长修改和优化文学作品。请根据指令对给定的文本进行重写，在保持原意的基础上提升文采、调整语调或优化表达。".to_string(),
                user_prompt_template: "请根据以下要求重写文本：\n\n原文：\n{content}\n\n重写要求：{instruction}\n\n请直接输出重写后的内容。".to_string(),
                variables: vec!["content".to_string(), "instruction".to_string()],
            },
            PromptTemplate {
                id: "character-dialogue".to_string(),
                name: "角色对话生成".to_string(),
                category: "dialogue".to_string(),
                system_prompt: "你是一位擅长塑造角色的作家，能够根据角色设定创作符合其性格、语气和背景的对话。请确保对话自然生动，能够体现角色的个性特征。".to_string(),
                user_prompt_template: "请为以下角色创作对话：\n\n角色信息：\n{character_info}\n\n场景描述：\n{scene}\n\n对话要求：{instruction}\n\n请直接输出对话内容。".to_string(),
                variables: vec!["character_info".to_string(), "scene".to_string(), "instruction".to_string()],
            },
            PromptTemplate {
                id: "scene-description".to_string(),
                name: "场景描写".to_string(),
                category: "description".to_string(),
                system_prompt: "你是一位擅长环境描写的作家，能够用生动细腻的笔触描绘各种场景，营造沉浸式的阅读体验。请注重感官细节，调动读者的视觉、听觉、嗅觉等多种感官。".to_string(),
                user_prompt_template: "请为以下场景进行描写：\n\n场景信息：\n{scene}\n\n描写要求：{instruction}\n\n请直接输出场景描写。".to_string(),
                variables: vec!["scene".to_string(), "instruction".to_string()],
            },
            PromptTemplate {
                id: "plot-suggestion".to_string(),
                name: "情节建议".to_string(),
                category: "plot".to_string(),
                system_prompt: "你是一位资深的故事架构师，擅长设计引人入胜的情节和转折。请根据当前情节提供合理且富有创意的发展建议，注意保持故事的连贯性和逻辑性。".to_string(),
                user_prompt_template: "请为以下情节提供发展建议：\n\n当前情节：\n{context}\n\n要求：{instruction}\n\n请提供3-5个情节发展建议，每个建议简要说明理由。".to_string(),
                variables: vec!["context".to_string(), "instruction".to_string()],
            },
        ];

        let rt = tokio::runtime::Handle::try_current();
        if let Ok(handle) = rt {
            let templates_clone = self.templates.clone();
            handle.block_on(async {
                let mut templates_map = templates_clone.write().await;
                for template in templates {
                    templates_map.insert(template.id.clone(), template);
                }
            });
        } else {
            self.logger.warn("No tokio runtime found, loading templates synchronously");
            let templates_clone = self.templates.clone();
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let mut templates_map = templates_clone.write().await;
                for template in templates {
                    templates_map.insert(template.id.clone(), template);
                }
            });
        }
    }

    pub async fn get_template(&self, id: &str) -> Option<PromptTemplate> {
        let templates = self.templates.read().await;
        templates.get(id).cloned()
    }

    pub async fn list_templates(&self, category: Option<String>) -> Vec<PromptTemplate> {
        let templates = self.templates.read().await;
        if let Some(cat) = category {
            templates
                .values()
                .filter(|t| t.category == cat)
                .cloned()
                .collect()
        } else {
            templates.values().cloned().collect()
        }
    }

    pub async fn build_prompt(
        &self,
        template_id: &str,
        variables: &HashMap<String, String>,
    ) -> Result<(String, String), String> {
        let template = self
            .get_template(template_id)
            .await
            .ok_or_else(|| format!("Template not found: {}", template_id))?;

        let mut user_prompt = template.user_prompt_template.clone();

        for var_name in &template.variables {
            if let Some(value) = variables.get(var_name) {
                user_prompt = user_prompt.replace(&format!("{{{}}}", var_name), value);
            } else {
                self.logger.warn(&format!("Missing variable: {}", var_name));
            }
        }

        self.logger.debug(&format!(
            "Built prompt from template: {}",
            template.name
        ));

        Ok((template.system_prompt, user_prompt))
    }

    pub async fn add_template(&self, template: PromptTemplate) {
        let template_name = template.name.clone();
        let mut templates = self.templates.write().await;
        templates.insert(template.id.clone(), template);
        self.logger.info(&format!("Added template: {}", template_name));
    }

    pub async fn remove_template(&self, id: &str) -> bool {
        let mut templates = self.templates.write().await;
        let removed = templates.remove(id).is_some();
        if removed {
            self.logger.info(&format!("Removed template: {}", id));
        }
        removed
    }
}

impl Default for PromptManager {
    fn default() -> Self {
        Self::new()
    }
}
