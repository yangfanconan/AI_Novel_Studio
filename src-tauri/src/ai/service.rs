use super::models::{
    AICompletionRequest, AIRewriteRequest, AIMessage, AIRequest,
    AIGenerateCharacterRequest, AIGenerateCharacterRelationsRequest,
    AIGenerateWorldViewRequest, AIGeneratePlotPointsRequest,
    AIGenerateStoryboardRequest, AIFormatContentRequest,
};
use super::{
    ModelRegistry, PromptManager, BigModelAdapter,
    GeneratorPrompts, FormatOptions,
    GeneratedCharacter, GeneratedCharacterRelation,
    GeneratedWorldView, GeneratedPlotPoint, GeneratedStoryboard,
};
use crate::logger::Logger;
use futures::StreamExt;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionReviewResult {
    pub consistency_score: f32,
    pub creativity_score: f32,
    pub completeness_score: f32,
    pub rhythm_score: f32,
    pub overall_score: f32,
    pub feedback: String,
    pub recommended_version: u32,
}

pub struct AIService {
    model_registry: ModelRegistry,
    prompt_manager: PromptManager,
    logger: Logger,
}

impl AIService {
    fn clean_json_string(input: &str) -> String {
        let cleaned = input
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();
        
        cleaned
            .chars()
            .filter(|c| (*c as u32) >= 0x20)
            .collect()
    }

    pub fn new() -> Self {
        Self {
            model_registry: ModelRegistry::new(),
            prompt_manager: PromptManager::new(),
            logger: Logger::new().with_feature("ai-service"),
        }
    }

    pub async fn initialize_default_models(&mut self) {
        let default_api_key = std::env::var("BIGMODEL_API_KEY")
            .unwrap_or_else(|_| "45913d02a609452b916a1706b8dc9702".to_string());

        self.logger.info("Initializing default BigModel models");

        let glm4 = Arc::new(BigModelAdapter::new(default_api_key.clone(), "glm-4".to_string()));
        let glm4_plus = Arc::new(BigModelAdapter::new(default_api_key.clone(), "glm-4-plus".to_string()));
        let glm4_air = Arc::new(BigModelAdapter::new(default_api_key.clone(), "glm-4-air".to_string()));
        let glm4_flash = Arc::new(BigModelAdapter::new(default_api_key.clone(), "glm-4-flash".to_string()));
        let glm4_flashx = Arc::new(BigModelAdapter::new(default_api_key.clone(), "glm-4-flashx".to_string()));
        let glm47_flash = Arc::new(BigModelAdapter::new(default_api_key.clone(), "glm-4.7-flash".to_string()));

        self.model_registry.register_model("glm-4".to_string(), glm4).await;
        self.model_registry.register_model("glm-4-plus".to_string(), glm4_plus).await;
        self.model_registry.register_model("glm-4-air".to_string(), glm4_air).await;
        self.model_registry.register_model("glm-4-flash".to_string(), glm4_flash).await;
        self.model_registry.register_model("glm-4-flashx".to_string(), glm4_flashx).await;
        self.model_registry.register_model("glm-4.7-flash".to_string(), glm47_flash).await;

        self.logger.info("Default BigModel models initialized successfully (including glm-4.7-flash with 128K output)");
    }

    pub fn get_registry(&self) -> &ModelRegistry {
        &self.model_registry
    }

    pub fn get_prompt_manager(&self) -> &PromptManager {
        &self.prompt_manager
    }

    fn clean_json_response(&self, response: &str) -> String {
        let cleaned = response
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();
        
        cleaned
            .chars()
            .filter(|c| (*c as u32) >= 0x20)
            .collect()
    }

    pub async fn complete(
        &self,
        model_id: &str,
        system_prompt: &str,
        user_content: &str,
        max_tokens: Option<u32>,
    ) -> Result<String, String> {
        let model = self
            .model_registry
            .get_model(model_id)
            .await
            .ok_or_else(|| format!("Model not found: {}", model_id))?;

        let request = AIRequest {
            model: model.get_name(),
            messages: vec![
                AIMessage {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                AIMessage {
                    role: "user".to_string(),
                    content: user_content.to_string(),
                },
            ],
            temperature: Some(0.7),
            max_tokens: Some(max_tokens.unwrap_or(8192)),
            stream: Some(false),
        };

        let response = model.complete(request).await?;
        Ok(response.content)
    }

    pub async fn review_versions(
        &self,
        versions: &[&str],
    ) -> Result<VersionReviewResult, String> {
        let system_prompt = r#"你是一个专业的小说编辑。请评审以下多个版本的内容，从以下维度打分（1-10分）：
1. 一致性 - 与上下文的连贯程度
2. 创意性 - 内容的新颖和创意程度
3. 完整性 - 情节的完整程度
4. 节奏感 - 叙事节奏的把控

请以JSON格式返回评审结果：
{
  "consistency_score": 8.0,
  "creativity_score": 7.5,
  "completeness_score": 8.5,
  "rhythm_score": 7.0,
  "overall_score": 7.8,
  "feedback": "简要评价...",
  "recommended_version": 2
}"#;

        let versions_text = versions
            .iter()
            .enumerate()
            .map(|(i, v)| format!("【版本{}】\n{}", i + 1, v))
            .collect::<Vec<_>>()
            .join("\n\n---\n\n");

        let response = self.complete("default", system_prompt, &versions_text, None).await?;
        
        let cleaned = Self::clean_json_string(&response);
        
        serde_json::from_str(&cleaned)
            .map_err(|e| format!("解析评审结果失败: {} - 原始响应: {}", e, cleaned))
    }

    pub async fn complete_stream(
        &self,
        model_id: &str,
        system_prompt: &str,
        user_content: &str,
        on_chunk: Box<dyn Fn(String) + Send + Sync>,
    ) -> Result<(), String> {
        let model = self
            .model_registry
            .get_model(model_id)
            .await
            .ok_or_else(|| format!("Model not found: {}", model_id))?;

        let request = AIRequest {
            model: model.get_name(),
            messages: vec![
                AIMessage {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                AIMessage {
                    role: "user".to_string(),
                    content: user_content.to_string(),
                },
            ],
            temperature: Some(0.7),
            max_tokens: Some(8192),
            stream: Some(true),
        };

        let mut stream = model.complete_stream(request).await?;

        while let Some(chunk_result) = stream.next().await {
            match chunk_result {
                Ok(chunk) => {
                    if !chunk.content.is_empty() {
                        on_chunk(chunk.content);
                    }
                    if chunk.done {
                        break;
                    }
                }
                Err(e) => {
                    self.logger.error(&format!("Stream error: {}", e));
                    return Err(e);
                }
            }
        }

        Ok(())
    }

    pub async fn continue_novel(
        &self,
        request: AICompletionRequest,
        on_chunk: Option<Box<dyn Fn(String) + Send + Sync>>,
    ) -> Result<String, String> {
        self.logger.info(&format!("Starting novel continuation with model: {}", request.model_id));

        let character_context = request.character_context.clone().unwrap_or_else(|| "暂无角色信息".to_string());
        let worldview_context = request.worldview_context.clone().unwrap_or_else(|| "暂无世界观设定".to_string());

        let (system_prompt, user_prompt) = self
            .prompt_manager
            .build_prompt(
                "novel-continuation",
                &HashMap::from([
                    ("context".to_string(), request.context),
                    ("instruction".to_string(), request.instruction),
                    ("character_context".to_string(), character_context),
                    ("worldview_context".to_string(), worldview_context),
                ]),
            )
            .await?;

        if let Some(on_chunk) = on_chunk {
            self.complete_stream(&request.model_id, &system_prompt, &user_prompt, on_chunk)
                .await?;
            Ok(String::new())
        } else {
            self.complete(&request.model_id, &system_prompt, &user_prompt, request.max_tokens)
                .await
        }
    }

    pub async fn rewrite_content(
        &self,
        request: AIRewriteRequest,
    ) -> Result<String, String> {
        self.logger.info(&format!("Starting content rewrite with model: {}", request.model_id));

        let (system_prompt, user_prompt) = self
            .prompt_manager
            .build_prompt(
                "novel-rewrite",
                &HashMap::from([
                    ("content".to_string(), request.content),
                    ("instruction".to_string(), request.instruction),
                ]),
            )
            .await?;

        self.complete(&request.model_id, &system_prompt, &user_prompt, None)
            .await
    }

    pub async fn generate_dialogue(
        &self,
        model_id: &str,
        character_info: &str,
        scene: &str,
        instruction: &str,
    ) -> Result<String, String> {
        let (system_prompt, user_prompt) = self
            .prompt_manager
            .build_prompt(
                "character-dialogue",
                &HashMap::from([
                    ("character_info".to_string(), character_info.to_string()),
                    ("scene".to_string(), scene.to_string()),
                    ("instruction".to_string(), instruction.to_string()),
                ]),
            )
            .await?;

        self.complete(model_id, &system_prompt, &user_prompt, None)
            .await
    }

    pub async fn describe_scene(
        &self,
        model_id: &str,
        scene: &str,
        instruction: &str,
    ) -> Result<String, String> {
        let (system_prompt, user_prompt) = self
            .prompt_manager
            .build_prompt(
                "scene-description",
                &HashMap::from([
                    ("scene".to_string(), scene.to_string()),
                    ("instruction".to_string(), instruction.to_string()),
                ]),
            )
            .await?;

        self.complete(model_id, &system_prompt, &user_prompt, None)
            .await
    }

    pub async fn suggest_plot(
        &self,
        model_id: &str,
        context: &str,
        instruction: &str,
    ) -> Result<String, String> {
        let (system_prompt, user_prompt) = self
            .prompt_manager
            .build_prompt(
                "plot-suggestion",
                &HashMap::from([
                    ("context".to_string(), context.to_string()),
                    ("instruction".to_string(), instruction.to_string()),
                ]),
            )
            .await?;

        self.complete(model_id, &system_prompt, &user_prompt, None)
            .await
    }

    /// AI生成角色（带上下文）
    pub async fn generate_character_with_context(
        &self,
        request: AIGenerateCharacterRequest,
        worldviews_context: &str,
        existing_characters_context: &str,
    ) -> Result<GeneratedCharacter, String> {
        self.logger.info(&format!("Starting character generation with context for project: {}", request.project_id));

        let model_id = request.model_id.clone().unwrap_or_else(|| "glm-4-flash".to_string());
        let genre = request.genre.clone().unwrap_or_else(|| "小说".to_string());

        let system_prompt = r#"你是一位专业的小说角色设计师，擅长创建立体、有深度的角色。

请根据用户提供的描述和项目上下文，生成一个完整的角色设定。你需要返回一个 JSON 格式的角色数据，包含以下字段：

必填字段：
- name: 角色姓名（必须有创意且符合设定）

可选字段（根据故事需要填写）：
- role_type: 角色身份（protagonist主角/deuteragonist第二主角/antagonist反派/supporting配角/minor小角色）
- race: 种族（如人类、精灵、兽人等，符合世界观设定）
- age: 年龄（整数）
- gender: 性别
- birth_date: 出生日期（如"龙历三千年三月初三"这种故事内的时间）
- appearance: 外貌描写（100-200字的详细描写）
- personality: 性格特点（100-200字，包含优点和缺点）
- background: 背景故事（200-300字，包含成长经历和重要事件）
- mbti: MBTI人格类型（如INTJ、ENFP等，仅返回4个字母）
- enneagram: 九型人格（如"3号-成就型"）
- bazi: 八字（如果是中式玄幻/武侠设定）
- ziwei: 紫微斗数主要星曜配置（如果是中式设定）
- skills: 技能列表（用顿号分隔）
- status: 当前状态（健康、情绪、位置等）
- items: 随身重要物品（用顿号分隔）

请确保角色具有：
1. 独特的性格魅力和缺点
2. 合理的成长弧线潜力
3. 与故事类型和世界观高度契合
4. 令人印象深刻的标志性特点
5. 与已有角色形成互补或冲突关系

只返回 JSON 对象，不要包含markdown代码块标记或其他说明文字。"#;

        let user_prompt = format!(
            r#"请为我的小说生成一个角色。

故事类型：{}
角色类型：{}
额外描述：{}

=== 项目上下文 ===

【世界观设定】
{}

【已有角色】
{}

请基于以上世界观和已有角色，生成一个能融入这个世界的新角色。新角色应该：
1. 符合世界观设定，种族、能力等要与世界一致
2. 与已有角色有潜在的互动可能
3. 有独特的定位，不与已有角色重复
4. 尽量填写所有可填写的字段，让角色更加立体"#,
            genre,
            request.character_type.as_deref().unwrap_or("配角"),
            request.description.as_deref().unwrap_or("无特殊要求"),
            worldviews_context,
            existing_characters_context
        );

        let response = self.complete(&model_id, system_prompt, &user_prompt, None).await?;
        
        let cleaned_response = self.clean_json_response(&response);

        let character: GeneratedCharacter = serde_json::from_str(&cleaned_response)
            .map_err(|e| format!("Failed to parse generated character: {}. Response: {}", e, cleaned_response))?;

        self.logger.info(&format!("Character generated successfully: {}", character.name));
        Ok(character)
    }

    /// AI生成角色
    pub async fn generate_character(
        &self,
        request: AIGenerateCharacterRequest,
    ) -> Result<GeneratedCharacter, String> {
        self.logger.info(&format!("Starting character generation for project: {}", request.project_id));

        let model_id = request.model_id.clone().unwrap_or_else(|| "glm-4-flash".to_string());
        let genre = request.genre.clone().unwrap_or_else(|| "小说".to_string());

        let system_prompt = r#"你是一位专业的小说角色设计师，擅长创建立体、有深度的角色。

请根据用户提供的描述，生成一个完整的角色设定。你需要返回一个 JSON 格式的角色数据，包含以下字段：

必填字段：
- name: 角色姓名（必须有创意且符合设定）

可选字段（根据故事需要填写）：
- role_type: 角色身份（protagonist主角/deuteragonist第二主角/antagonist反派/supporting配角/minor小角色）
- race: 种族（如人类、精灵、兽人等）
- age: 年龄（整数）
- gender: 性别
- birth_date: 出生日期
- appearance: 外貌描写（100-200字的详细描写）
- personality: 性格特点（100-200字，包含优点和缺点）
- background: 背景故事（200-300字）
- mbti: MBTI人格类型
- enneagram: 九型人格
- skills: 技能列表
- status: 当前状态
- items: 随身物品

请确保角色具有：
1. 独特的性格魅力
2. 合理的成长弧线潜力
3. 与故事类型相符的特征
4. 令人印象深刻的标志性特点

只返回 JSON 对象，不要包含markdown代码块标记或其他说明文字。"#;

        let user_prompt = GeneratorPrompts::build_character_prompt(
            &genre,
            request.character_type.as_deref(),
            request.description.as_deref(),
        );

        let response = self.complete(&model_id, system_prompt, &user_prompt, None).await?;
        
        let cleaned_response = self.clean_json_response(&response);

        let character: GeneratedCharacter = serde_json::from_str(&cleaned_response)
            .map_err(|e| format!("Failed to parse generated character: {}. Response: {}", e, cleaned_response))?;

        self.logger.info(&format!("Character generated successfully: {}", character.name));
        Ok(character)
    }

    /// AI生成角色关系
    pub async fn generate_character_relations(
        &self,
        request: AIGenerateCharacterRelationsRequest,
        project_characters: &[crate::models::Character],
        project_context: &str,
    ) -> Result<Vec<GeneratedCharacterRelation>, String> {
        self.logger.info(&format!("Starting character relations generation for project: {}", request.project_id));

        let model_id = request.model_id.clone().unwrap_or_else(|| "glm-4-flash".to_string());

        // 构建角色列表字符串
        let characters_str = project_characters
            .iter()
            .map(|c| format!("- {} ({}, {}岁): {} - {}", 
                c.name, 
                c.gender.as_deref().unwrap_or("未知"), 
                c.age.unwrap_or(0),
                c.personality.as_deref().unwrap_or("无性格描述"),
                c.background.as_deref().unwrap_or("无背景")
            ))
            .collect::<Vec<_>>()
            .join("\n");

        let system_prompt = r#"你是一位擅长构建人物关系的小说编剧，能够设计出复杂而合理的人物关系网络。

请根据给定的角色列表和故事背景，生成角色之间的关系网络。返回一个 JSON 数组，每个元素包含：
- from_character_name: 角色A的姓名
- to_character_name: 角色B的姓名
- relation_type: 关系类型（如：朋友、敌人、恋人、师徒、对手、亲人等）
- description: 关系描述（50-100字，包含关系起源和当前状态）

关系网络设计要点：
1. 每个角色应该与多个其他角色有关系，形成真正的网络
2. 关系要有戏剧张力和发展空间
3. 要考虑角色性格的契合与冲突
4. 关系网络要有层次感和交叉点
5. 要为后续情节发展埋下伏笔
6. 同一个角色可以有多种不同类型的关系

只返回 JSON 数组，不要包含markdown代码块标记或其他说明文字。"#;

        let user_prompt = GeneratorPrompts::build_character_relations_prompt(&characters_str, project_context);

        let response = self.complete(&model_id, system_prompt, &user_prompt, None).await?;
        
        let cleaned_response = self.clean_json_response(&response);

        let relations: Vec<GeneratedCharacterRelation> = serde_json::from_str(&cleaned_response)
            .map_err(|e| format!("Failed to parse generated relations: {}. Response: {}", e, cleaned_response))?;

        self.logger.info(&format!("Generated {} character relations", relations.len()));
        Ok(relations)
    }

    /// AI生成世界观
    pub async fn generate_worldview(
        &self,
        request: AIGenerateWorldViewRequest,
        project_genre: &str,
        existing_worldviews: &[crate::models::WorldView],
    ) -> Result<GeneratedWorldView, String> {
        self.logger.info(&format!("Starting worldview generation for category: {}", request.category));

        let model_id = request.model_id.clone().unwrap_or_else(|| "glm-4-flash".to_string());

        // 构建已有设定字符串
        let existing_context = if existing_worldviews.is_empty() {
            "暂无已有设定".to_string()
        } else {
            existing_worldviews
                .iter()
                .map(|w| format!("- [{}] {}: {}", w.category, w.title, w.content.chars().take(100).collect::<String>()))
                .collect::<Vec<_>>()
                .join("\n")
        };

        let system_prompt = r#"你是一位世界构建专家，擅长创造独特、自洽的虚构世界。

请根据用户指定的类别，生成世界观设定。返回一个 JSON 对象，包含：
- category: 世界观类别（与用户指定的类别一致）
- title: 设定标题
- content: 详细内容（300-500字）
- tags: 相关标签数组（如 ["玄幻", "历史", "星辰之力"]）

世界观类别说明：
- geography: 地理环境 - 地形地貌、气候特点、自然资源
- history: 历史背景 - 重要事件、朝代更迭、历史人物
- culture: 文化习俗 - 风俗习惯、节日庆典、艺术形式
- politics: 政治体制 - 权力结构、法律法规、政治派系
- economy: 经济系统 - 货币体系、贸易往来、产业分布
- religion: 宗教信仰 - 神祇体系、祭祀仪式、信仰冲突
- technology: 科技水平 - 技术特点、发明创造、发展趋势
- magic: 魔法体系 - 魔法原理、施法方式、限制代价
- races: 种族设定 - 种族特点、种族关系、种族分布
- organizations: 组织势力 - 组织目标、组织结构、组织活动

设计要点：
1. 要有独特性和辨识度
2. 内部逻辑要自洽
3. 要为故事提供发展空间
4. 要有细节支撑，避免空洞

只返回 JSON 对象，不要包含markdown代码块标记或其他说明文字。"#;

        let user_prompt = GeneratorPrompts::build_worldview_prompt(
            project_genre,
            &request.category,
            &existing_context,
            request.description.as_deref(),
        );

        let response = self.complete(&model_id, system_prompt, &user_prompt, None).await?;
        
        let cleaned_response = self.clean_json_response(&response);

        let worldview: GeneratedWorldView = serde_json::from_str(&cleaned_response)
            .map_err(|e| format!("Failed to parse generated worldview: {}. Response: {}", e, cleaned_response))?;

        self.logger.info(&format!("Worldview generated successfully: {}", worldview.title));
        Ok(worldview)
    }

    /// AI生成世界观（带上下文）
    pub async fn generate_worldview_with_context(
        &self,
        request: AIGenerateWorldViewRequest,
        project_genre: &str,
        existing_worldviews: &[crate::models::WorldView],
        characters_context: &str,
        plot_context: &str,
    ) -> Result<GeneratedWorldView, String> {
        self.logger.info(&format!("Starting worldview generation with context for category: {}", request.category));

        let model_id = request.model_id.clone().unwrap_or_else(|| "glm-4-flash".to_string());

        // 构建已有设定字符串
        let existing_context = if existing_worldviews.is_empty() {
            "暂无已有设定".to_string()
        } else {
            existing_worldviews
                .iter()
                .map(|w| format!("- [{}] {}: {}", w.category, w.title, w.content.chars().take(100).collect::<String>()))
                .collect::<Vec<_>>()
                .join("\n")
        };

        let system_prompt = r#"你是一位世界构建专家，擅长创造独特、自洽的虚构世界。

请根据用户指定的类别和项目上下文，生成世界观设定。返回一个 JSON 对象，包含：
- category: 世界观类别（与用户指定的类别一致）
- title: 设定标题
- content: 详细内容（300-500字）
- tags: 相关标签数组（如 ["玄幻", "历史", "星辰之力"]）

世界观类别说明：
- geography: 地理环境 - 地形地貌、气候特点、自然资源
- history: 历史背景 - 重要事件、朝代更迭、历史人物
- culture: 文化习俗 - 风俗习惯、节日庆典、艺术形式
- politics: 政治体制 - 权力结构、法律法规、政治派系
- economy: 经济系统 - 货币体系、贸易往来、产业分布
- religion: 宗教信仰 - 神祇体系、祭祀仪式、信仰冲突
- technology: 科技水平 - 技术特点、发明创造、发展趋势
- magic: 魔法体系 - 魔法原理、施法方式、限制代价
- races: 种族设定 - 种族特点、种族关系、种族分布
- organizations: 组织势力 - 组织目标、组织结构、组织活动

设计要点：
1. 要有独特性和辨识度
2. 内部逻辑要自洽
3. 要为故事和角色提供发展空间
4. 要有细节支撑，避免空洞
5. 要与已有角色和情节相呼应

只返回 JSON 对象，不要包含markdown代码块标记或其他说明文字。"#;

        let user_prompt = format!(
            r#"请为我的小说生成世界观设定。

故事类型：{}
设定类别：{}
额外要求：{}

=== 项目上下文 ===

【已有世界观设定】
{}

【已有角色】
{}

【已有情节】
{}

请基于以上角色和情节，生成能支撑故事发展的世界观设定。设定应该：
1. 为角色提供合适的活动舞台
2. 为情节发展提供合理的背景
3. 与已有世界观设定保持一致
4. 具有独特性和吸引力"#,
            project_genre,
            request.category,
            request.description.as_deref().unwrap_or("无特殊要求"),
            existing_context,
            characters_context,
            plot_context
        );

        let response = self.complete(&model_id, system_prompt, &user_prompt, None).await?;
        
        let cleaned_response = self.clean_json_response(&response);

        let worldview: GeneratedWorldView = serde_json::from_str(&cleaned_response)
            .map_err(|e| format!("Failed to parse generated worldview: {}. Response: {}", e, cleaned_response))?;

        self.logger.info(&format!("Worldview generated successfully: {}", worldview.title));
        Ok(worldview)
    }

    /// AI生成情节点（带上下文）
    pub async fn generate_plot_points_with_context(
        &self,
        request: AIGeneratePlotPointsRequest,
        project_info: &str,
        existing_plots: &[crate::models::PlotPoint],
        characters_context: &str,
        worldviews_context: &str,
    ) -> Result<Vec<GeneratedPlotPoint>, String> {
        self.logger.info("Starting plot points generation with context");

        let model_id = request.model_id.clone().unwrap_or_else(|| "glm-4-flash".to_string());

        // 构建已有情节字符串
        let existing_plots_str = if existing_plots.is_empty() {
            "暂无已有情节".to_string()
        } else {
            existing_plots
                .iter()
                .map(|p| format!("- {}: {}", p.title, p.description.as_deref().unwrap_or("无描述")))
                .collect::<Vec<_>>()
                .join("\n")
        };

        let system_prompt = r#"你是一位资深的剧情设计师，擅长设计引人入胜的故事情节。

请根据给定的故事背景、角色和世界观，生成情节点。返回一个 JSON 数组，每个元素包含：
- title: 情节点标题（简短有力）
- description: 情节描述（100-200字，要具体涉及角色）
- note: 创作提示（可选，50字内的注意事项）
- emotional_tone: 情感基调（如：紧张、温馨、悲伤、欢快等）

情节设计要点：
1. 要有明确的因果关系
2. 要推动角色成长和关系变化
3. 要有意外性和合理性
4. 要为后续发展埋下伏笔
5. 要有情感共鸣点
6. 要充分利用世界观设定

只返回 JSON 数组，不要包含markdown代码块标记或其他说明文字。"#;

        let context = request.context.as_deref().unwrap_or(project_info);
        let user_prompt = format!(
            r#"请为我的小说生成情节点。

项目信息：{}

【已有情节】
{}

【角色信息】
{}

【世界观设定】
{}

【发展方向】
{}

请基于以上角色和世界观，生成能与角色产生互动、符合世界观的情节。情节应该：
1. 让角色在故事中发挥重要作用
2. 符合世界观设定
3. 推动角色关系发展
4. 与已有情节形成连贯的故事线"#,
            context,
            existing_plots_str,
            characters_context,
            worldviews_context,
            request.direction.as_deref().unwrap_or("自然发展，注重情感深度")
        );

        let response = self.complete(&model_id, system_prompt, &user_prompt, None).await?;
        
        let cleaned_response = self.clean_json_response(&response);

        let plot_points: Vec<GeneratedPlotPoint> = serde_json::from_str(&cleaned_response)
            .map_err(|e| format!("Failed to parse generated plot points: {}. Response: {}", e, cleaned_response))?;

        self.logger.info(&format!("Generated {} plot points", plot_points.len()));
        Ok(plot_points)
    }

    /// AI生成情节点
    pub async fn generate_plot_points(
        &self,
        request: AIGeneratePlotPointsRequest,
        project_info: &str,
        existing_plots: &[crate::models::PlotPoint],
    ) -> Result<Vec<GeneratedPlotPoint>, String> {
        self.logger.info(&format!("Starting plot points generation for project"));

        let model_id = request.model_id.clone().unwrap_or_else(|| "glm-4-flash".to_string());

        // 构建已有情节字符串
        let existing_plots_str = if existing_plots.is_empty() {
            "暂无已有情节".to_string()
        } else {
            existing_plots
                .iter()
                .map(|p| format!("- {}: {}", p.title, p.description.as_deref().unwrap_or("无描述")))
                .collect::<Vec<_>>()
                .join("\n")
        };

        let system_prompt = r#"你是一位资深的剧情设计师，擅长设计引人入胜的故事情节。

请根据给定的故事背景和发展方向，生成情节点。返回一个 JSON 数组，每个元素包含：
- title: 情节点标题（简短有力）
- description: 情节描述（100-200字）
- note: 创作提示（可选，50字内的注意事项）
- emotional_tone: 情感基调（如：紧张、温馨、悲伤、欢快等）

情节设计要点：
1. 要有明确的因果关系
2. 要推动角色成长
3. 要有意外性和合理性
4. 要为后续发展埋下伏笔
5. 要有情感共鸣点

只返回 JSON 数组，不要包含markdown代码块标记或其他说明文字。"#;

        let context = request.context.as_deref().unwrap_or(project_info);
        let user_prompt = GeneratorPrompts::build_plot_points_prompt(
            context,
            &existing_plots_str,
            request.direction.as_deref(),
        );

        let response = self.complete(&model_id, system_prompt, &user_prompt, None).await?;
        
        let cleaned_response = self.clean_json_response(&response);

        let plot_points: Vec<GeneratedPlotPoint> = serde_json::from_str(&cleaned_response)
            .map_err(|e| format!("Failed to parse generated plot points: {}. Response: {}", e, cleaned_response))?;

        self.logger.info(&format!("Generated {} plot points", plot_points.len()));
        Ok(plot_points)
    }

    /// AI生成分镜提示词
    pub async fn generate_storyboard(
        &self,
        request: AIGenerateStoryboardRequest,
        content: &str,
    ) -> Result<Vec<GeneratedStoryboard>, String> {
        self.logger.info("Starting storyboard generation");

        let model_id = request.model_id.clone().unwrap_or_else(|| "glm-4-flash".to_string());

        let system_prompt = r#"你是一位专业的影视分镜师和AI绘画提示词专家，能够将文字场景转化为精确的图像生成提示词。

请根据给定的场景描述，生成分镜提示词。返回一个 JSON 数组，每个元素包含：
- shot_number: 镜头编号（整数，从1开始）
- shot_type: 镜头类型（特写、中景、远景、俯视、仰视等）
- duration: 建议时长（秒，整数）
- scene_description: 场景描述（中文，50-100字）
- camera_movement: 镜头运动（推、拉、摇、移、跟等）
- visual_prompt: AI绘画提示词（英文，用于Midjourney/Stable Diffusion，包含主体、环境、光线、风格等）
- negative_prompt: 负面提示词（可选，避免生成的内容）
- style_notes: 风格备注（色调、光影等）

分镜设计要点：
1. 镜头要有变化和节奏感
2. 要突出重点和情感
3. 要考虑画面构图和视觉冲击
4. AI提示词要具体、可执行

只返回 JSON 数组，不要包含markdown代码块标记或其他说明文字。"#;

        let user_prompt = GeneratorPrompts::build_storyboard_prompt(
            content,
            request.style_preference.as_deref(),
        );

        let response = self.complete(&model_id, system_prompt, &user_prompt, None).await?;
        
        let cleaned_response = self.clean_json_response(&response);

        let storyboard: Vec<GeneratedStoryboard> = serde_json::from_str(&cleaned_response)
            .map_err(|e| format!("Failed to parse generated storyboard: {}. Response: {}", e, cleaned_response))?;

        self.logger.info(&format!("Generated {} storyboard shots", storyboard.len()));
        Ok(storyboard)
    }

    /// AI一键排版
    pub async fn format_content(
        &self,
        request: AIFormatContentRequest,
    ) -> Result<String, String> {
        self.logger.info("Starting content formatting");

        let model_id = request.model_id.clone().unwrap_or_else(|| "glm-4-flash".to_string());

        let options = FormatOptions {
            paragraph_style: request.paragraph_style.unwrap_or_else(|| "空行分隔".to_string()),
            dialogue_style: request.dialogue_style.unwrap_or_else(|| "中文引号".to_string()),
            scene_separator: request.scene_separator.unwrap_or_else(|| "***".to_string()),
            special_requirements: request.special_requirements.unwrap_or_else(|| "无".to_string()),
        };

        let system_prompt = r#"你是一位专业的文字排版编辑，擅长优化小说文本的格式和可读性。

请根据用户的要求对文本进行排版处理。你需要：
1. 修正段落格式
2. 优化对话排版
3. 调整标点符号
4. 处理场景转换
5. 统一格式风格

排版规则：
- 段落之间空一行
- 对话使用正确的引号格式
- 场景转换使用分隔符
- 心理活动用斜体或特定符号标注
- 动作描写独立成段

只返回排版后的纯文本内容，不要添加任何解释说明、引号包裹或markdown代码块标记。"#;

        let user_prompt = GeneratorPrompts::build_format_prompt(&request.content, &options);

        let response = self.complete(&model_id, system_prompt, &user_prompt, None).await?;
        
        // 清理响应，移除可能的引号包裹
        let cleaned_response = response
            .trim()
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim()
            .trim_matches('"');

        self.logger.info("Content formatted successfully");
        Ok(cleaned_response.to_string())
    }

    /// 生成续写选项
    pub async fn generate_writing_choices(
        &self,
        request: crate::models::GenerateWritingChoicesRequest,
        characters: &[crate::models::Character],
        worldviews: &[crate::models::WorldView],
        plot_points: &[crate::models::PlotPoint],
    ) -> Result<crate::models::WritingSuggestion, String> {
        self.logger.info(&format!("Generating writing choices for chapter: {}", request.chapter_id));

        let model_id = request.model_id.clone().unwrap_or_else(|| "glm-4-flash".to_string());

        // 构建角色上下文
        let characters_context = characters
            .iter()
            .map(|c| format!("- {} ({}, {}岁): {}", 
                c.name, 
                c.gender.as_deref().unwrap_or("未知"), 
                c.age.unwrap_or(0),
                c.personality.as_deref().unwrap_or("无描述")))
            .collect::<Vec<_>>()
            .join("\n");

        // 构建世界观上下文
        let worldview_context = worldviews
            .iter()
            .take(5)
            .map(|w| format!("- [{}] {}", w.category, w.title))
            .collect::<Vec<_>>()
            .join("\n");

        // 构建情节点上下文
        let plot_context = plot_points
            .iter()
            .take(5)
            .map(|p| format!("- {}", p.title))
            .collect::<Vec<_>>()
            .join("\n");

        // 获取当前内容的最后部分作为上下文
        let content_preview = if request.current_content.chars().count() > 500 {
            request.current_content.chars().rev().take(500).collect::<String>().chars().rev().collect::<String>()
        } else {
            request.current_content.clone()
        };

        let system_prompt = r#"你是一位专业的小说创作顾问，擅长分析剧情走向并提供多种续写方向。

请根据当前的写作内容，返回一个 JSON 对象，包含以下字段：
- choices: 一个数组，包含3-5个不同的续写方向选项，每个选项包含：
  - id: 唯一标识（如 "choice_1"）
  - direction: 方向类型（如：冲突升级、情感深化、剧情反转、平稳过渡、紧张悬疑、奇遇机缘等）
  - direction_icon: 方向图标（如：🔥、💔、🎭、🌊、⚡、✨等emoji）
  - preview: 100-150字的续写预览
  - hint: 这个选择可能带来的影响提示（50字以内）
  - characters: 将涉及的角色名字数组
  - emotional_tone: 情感基调（如：紧张、温馨、悲伤、欢快等）

- detected_characters: 当前内容中出现的角色名字数组
- new_characters: 当前内容中出现但不在已有角色列表中的名字
- consistency_warnings: 一致性警告数组，每个包含：
  - warning_type: 警告类型（如：character_personality、character_relation、world_setting等）
  - character_name: 相关角色名（如适用）
  - expected: 设定中的描述
  - actual: 当前内容中的描述
  - severity: 严重程度（low、medium、high）
- new_settings: 检测到的新设定/名词

确保每个选项都有明显的差异，给作者提供真正的选择空间。只返回 JSON 对象，不要包含markdown代码块标记。"#;

        let user_prompt = format!(
            r#"请为我的小说生成续写选项。

【已有角色】
{}

【世界观设定】
{}

【剧情规划】
{}

【当前内容（末尾部分）】
{}

请分析当前内容，检测角色一致性，并提供多个不同方向的续写选项。"#,
            characters_context,
            worldview_context,
            plot_context,
            content_preview
        );

        let response = self.complete(&model_id, system_prompt, &user_prompt, None).await?;
        
        let cleaned_response = self.clean_json_response(&response);

        let suggestion: crate::models::WritingSuggestion = serde_json::from_str(&cleaned_response)
            .map_err(|e| format!("Failed to parse writing suggestion: {}. Response: {}", e, cleaned_response))?;

        self.logger.info(&format!("Generated {} writing choices", suggestion.choices.len()));
        Ok(suggestion)
    }

    /// 验证写作内容的一致性
    pub async fn validate_writing(
        &self,
        request: crate::models::ValidateWritingRequest,
        characters: &[crate::models::Character],
        worldviews: &[crate::models::WorldView],
        relations: &[crate::models::CharacterRelation],
    ) -> Result<crate::models::ValidationResult, String> {
        self.logger.info("Validating writing content");

        let model_id = "glm-4-flash".to_string();

        // 构建角色信息
        let characters_info = characters
            .iter()
            .map(|c| {
                let relations_str = relations
                    .iter()
                    .filter(|r| r.from_character_id == c.id || r.to_character_id == c.id)
                    .map(|r| {
                        let other_name = if r.from_character_id == c.id {
                            characters.iter().find(|ch| ch.id == r.to_character_id).map(|ch| ch.name.as_str()).unwrap_or("?")
                        } else {
                            characters.iter().find(|ch| ch.id == r.from_character_id).map(|ch| ch.name.as_str()).unwrap_or("?")
                        };
                        format!("{}（{}）", other_name, r.relation_type)
                    })
                    .collect::<Vec<_>>()
                    .join("、");

                format!("- {} ({}, {}岁) | 性格: {} | 关系: {}", 
                    c.name, 
                    c.gender.as_deref().unwrap_or("未知"), 
                    c.age.unwrap_or(0),
                    c.personality.as_deref().unwrap_or("无"),
                    if relations_str.is_empty() { "无" } else { &relations_str }
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        // 构建世界观关键词
        let settings_keywords = worldviews
            .iter()
            .flat_map(|w| {
                let mut keywords = vec![w.title.clone()];
                keywords.extend(w.tags.clone().unwrap_or_default().split(',').map(|s| s.trim().to_string()));
                keywords
            })
            .collect::<Vec<_>>()
            .join("、");

        // 获取内容的最后1000字符进行分析
        let content_to_check = if request.content.chars().count() > 1000 {
            request.content.chars().rev().take(1000).collect::<String>().chars().rev().collect::<String>()
        } else {
            request.content.clone()
        };

        let system_prompt = r#"你是一位专业的小说编辑，擅长检查文本的一致性和设定冲突。

请分析给定的文本，返回一个 JSON 对象，包含：
- detected_characters: 检测到的角色数组，每个包含：
  - name: 角色名
  - character_id: 如果匹配已有角色，填入ID，否则null
  - is_new: 是否是新角色
  - actions: 角色在文本中的行为描述（简要）
- new_characters: 未在已有角色列表中的角色名数组
- consistency_warnings: 一致性问题数组，每个包含：
  - warning_type: 问题类型
  - character_name: 相关角色
  - expected: 设定情况
  - actual: 文本中的情况
  - severity: 严重程度（low/medium/high）
- detected_settings: 文本中涉及的世界观设定
- new_settings: 不在已有设定中的新名词/设定

只返回 JSON 对象，不要包含markdown代码块标记。"#;

        let user_prompt = format!(
            r#"请检查以下小说片段的一致性。

【已有角色及设定】
{}

【世界观关键词】
{}

【待检查的文本】
{}

请检测角色出场、性格一致性、关系表现，以及世界观设定的使用情况。"#,
            characters_info,
            settings_keywords,
            content_to_check
        );

        let response = self.complete(&model_id, system_prompt, &user_prompt, None).await?;
        
        let cleaned_response = self.clean_json_response(&response);

        let result: crate::models::ValidationResult = serde_json::from_str(&cleaned_response)
            .map_err(|e| format!("Failed to parse validation result: {}. Response: {}", e, cleaned_response))?;

        self.logger.info(&format!("Validation complete: {} characters detected, {} warnings", 
            result.detected_characters.len(), result.consistency_warnings.len()));
        Ok(result)
    }
}

impl Default for AIService {
    fn default() -> Self {
        Self::new()
    }
}

pub type AIServiceArc = Arc<RwLock<AIService>>;

pub fn create_ai_service() -> AIServiceArc {
    Arc::new(RwLock::new(AIService::new()))
}
