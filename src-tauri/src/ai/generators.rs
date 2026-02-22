use super::models::PromptTemplate;
use crate::logger::Logger;

/// AI 生成器提示词管理器
pub struct GeneratorPrompts {
    #[allow(dead_code)]
    logger: Logger,
}

impl GeneratorPrompts {
    pub fn new() -> Self {
        Self {
            logger: Logger::new().with_feature("ai-generators"),
        }
    }

    /// 获取所有生成器提示词模板
    pub fn get_templates() -> Vec<PromptTemplate> {
        vec![
            // 角色生成模板
            PromptTemplate {
                id: "ai-generate-character".to_string(),
                name: "AI生成角色".to_string(),
                category: "generator".to_string(),
                system_prompt: r#"你是一位专业的小说角色设计师，擅长创建立体、有深度的角色。

请根据用户提供的描述，生成一个完整的角色设定。你需要返回一个 JSON 格式的角色数据，包含以下字段：
- name: 角色姓名（必须有创意且符合设定）
- age: 年龄（整数）
- gender: 性别
- appearance: 外貌描写（100-200字的详细描写）
- personality: 性格特点（100-200字，包含优点和缺点）
- background: 背景故事（200-300字，包含成长经历和重要事件）

请确保角色具有：
1. 独特的性格魅力
2. 合理的成长弧线潜力
3. 与故事类型相符的特征
4. 令人印象深刻的标志性特点

只返回 JSON 对象，不要包含其他说明文字。"#.to_string(),
                user_prompt_template: r#"请为我的小说生成一个角色。

小说类型/题材：{genre}
角色类型：{character_type}
额外描述：{description}

请生成一个符合要求的角色设定。"#.to_string(),
                variables: vec!["genre".to_string(), "character_type".to_string(), "description".to_string()],
            },
            // 角色关系生成模板
            PromptTemplate {
                id: "ai-generate-character-relations".to_string(),
                name: "AI生成角色关系".to_string(),
                category: "generator".to_string(),
                system_prompt: r#"你是一位擅长构建人物关系的小说编剧，能够设计出复杂而合理的人物关系网络。

请根据给定的角色列表和故事背景，生成角色之间的关系。返回一个 JSON 数组，每个元素包含：
- from_character: 角色A的姓名
- to_character: 角色B的姓名
- relation_type: 关系类型（如：朋友、敌人、恋人、师徒、对手、亲人等）
- description: 关系描述（50-100字，包含关系起源和当前状态）

关系设计要点：
1. 关系要有戏剧张力和发展空间
2. 要考虑角色性格的契合与冲突
3. 关系网络要有层次感
4. 要为后续情节发展埋下伏笔

只返回 JSON 数组，不要包含其他说明文字。"#.to_string(),
                user_prompt_template: r#"请根据以下角色和背景生成角色关系：

角色列表：
{characters}

故事背景：
{story_context}

请生成3-5个有戏剧价值的人物关系。"#.to_string(),
                variables: vec!["characters".to_string(), "story_context".to_string()],
            },
            // 世界观生成模板
            PromptTemplate {
                id: "ai-generate-worldview".to_string(),
                name: "AI生成世界观".to_string(),
                category: "generator".to_string(),
                system_prompt: r#"你是一位世界构建专家，擅长创造独特、自洽的虚构世界。

请根据用户指定的类别，生成世界观设定。返回一个 JSON 对象，包含：
- title: 设定标题
- content: 详细内容（300-500字）
- tags: 相关标签（逗号分隔的字符串）

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

只返回 JSON 对象，不要包含其他说明文字。"#.to_string(),
                user_prompt_template: r#"请为我的小说生成世界观设定。

故事类型：{genre}
设定类别：{category}
已有设定：{existing_context}
额外要求：{description}

请生成一个详细的世界观设定。"#.to_string(),
                variables: vec!["genre".to_string(), "category".to_string(), "existing_context".to_string(), "description".to_string()],
            },
            // 情节点生成模板
            PromptTemplate {
                id: "ai-generate-plot-points".to_string(),
                name: "AI生成情节点".to_string(),
                category: "generator".to_string(),
                system_prompt: r#"你是一位资深的剧情设计师，擅长设计引人入胜的故事情节。

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

只返回 JSON 数组，不要包含其他说明文字。"#.to_string(),
                user_prompt_template: r#"请为我的小说生成情节点。

故事背景：
{context}

已有情节：
{existing_plots}

发展方向：
{direction}

请生成3-5个有情节点价值的故事情节。"#.to_string(),
                variables: vec!["context".to_string(), "existing_plots".to_string(), "direction".to_string()],
            },
            // 分镜提示词生成模板
            PromptTemplate {
                id: "ai-generate-storyboard".to_string(),
                name: "AI生成分镜提示词".to_string(),
                category: "generator".to_string(),
                system_prompt: r#"你是一位专业的影视分镜师和AI绘画提示词专家，能够将文字场景转化为精确的图像生成提示词。

请根据给定的场景描述，生成分镜提示词。返回一个 JSON 数组，每个元素包含：
- shot_number: 镜头编号
- shot_type: 镜头类型（特写、中景、远景、俯视、仰视等）
- duration: 建议时长（秒）
- scene_description: 场景描述（中文，50-100字）
- camera_movement: 镜头运动（推、拉、摇、移、跟等）
- visual_prompt: AI绘画提示词（英文，用于Midjourney/Stable Diffusion）
- negative_prompt: 负面提示词（可选，避免生成的内容）
- style_notes: 风格备注（色调、光影等）

分镜设计要点：
1. 镜头要有变化和节奏感
2. 要突出重点和情感
3. 要考虑画面构图和视觉冲击
4. AI提示词要具体、可执行

只返回 JSON 数组，不要包含其他说明文字。"#.to_string(),
                user_prompt_template: r#"请将以下内容转化为分镜提示词：

场景内容：
{content}

视觉风格要求：
{style_preference}

请生成3-8个分镜镜头的提示词。"#.to_string(),
                variables: vec!["content".to_string(), "style_preference".to_string()],
            },
            // 一键排版模板
            PromptTemplate {
                id: "ai-format-content".to_string(),
                name: "AI一键排版".to_string(),
                category: "generator".to_string(),
                system_prompt: r#"你是一位专业的文字排版编辑，擅长优化小说文本的格式和可读性。

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

返回处理后的完整文本，不要添加任何解释说明。"#.to_string(),
                user_prompt_template: r#"请对以下文本进行排版处理：

原文内容：
{content}

排版选项：
- 段落分隔方式：{paragraph_style}
- 对话格式：{dialogue_style}
- 场景分隔符：{scene_separator}
- 特殊要求：{special_requirements}

请返回排版后的文本。"#.to_string(),
                variables: vec![
                    "content".to_string(),
                    "paragraph_style".to_string(),
                    "dialogue_style".to_string(),
                    "scene_separator".to_string(),
                    "special_requirements".to_string(),
                ],
            },
        ]
    }

    /// 构建角色生成的用户提示
    pub fn build_character_prompt(
        genre: &str,
        character_type: Option<&str>,
        description: Option<&str>,
    ) -> String {
        let char_type = character_type.unwrap_or("主要角色");
        let desc = description.unwrap_or("无特殊要求");

        format!(
            r#"请为我的小说生成一个角色。

小说类型/题材：{}
角色类型：{}
额外描述：{}

请生成一个符合要求的角色设定。"#,
            genre, char_type, desc
        )
    }

    /// 构建角色关系生成的用户提示
    pub fn build_character_relations_prompt(
        characters: &str,
        story_context: &str,
    ) -> String {
        format!(
            r#"请根据以下角色和背景生成角色关系：

角色列表：
{}

故事背景：
{}

请为这些角色构建一个丰富的人物关系网络。要求：
1. 每个角色至少参与1-2个关系
2. 关系要形成网络，而不是孤立的一对一关系
3. 包含不同类型的关系（朋友、敌人、恋人、师徒、亲人、对手等）
4. 关系之间要有交叉和层次，体现复杂的人际关系"#,
            characters, story_context
        )
    }

    /// 构建世界观生成的用户提示
    pub fn build_worldview_prompt(
        genre: &str,
        category: &str,
        existing_context: &str,
        description: Option<&str>,
    ) -> String {
        let desc = description.unwrap_or("无特殊要求");

        format!(
            r#"请为我的小说生成世界观设定。

故事类型：{}
设定类别：{}
已有设定：{}
额外要求：{}

请生成一个详细的世界观设定。"#,
            genre, category, existing_context, desc
        )
    }

    /// 构建情节点生成的用户提示
    pub fn build_plot_points_prompt(
        context: &str,
        existing_plots: &str,
        direction: Option<&str>,
    ) -> String {
        let dir = direction.unwrap_or("自然发展，注重情感深度");

        format!(
            r#"请为我的小说生成情节点。

故事背景：
{}

已有情节：
{}

发展方向：
{}

请生成3-5个有情节点价值的故事情节。"#,
            context, existing_plots, dir
        )
    }

    /// 构建分镜提示词生成的用户提示
    pub fn build_storyboard_prompt(
        content: &str,
        style_preference: Option<&str>,
    ) -> String {
        let style = style_preference.unwrap_or("写实风格，电影质感");

        format!(
            r#"请将以下内容转化为分镜提示词：

场景内容：
{}

视觉风格要求：
{}

请生成3-8个分镜镜头的提示词。"#,
            content, style
        )
    }

    /// 构建排版生成的用户提示
    pub fn build_format_prompt(
        content: &str,
        options: &FormatOptions,
    ) -> String {
        format!(
            r#"请对以下文本进行排版处理：

原文内容：
{}

排版选项：
- 段落分隔方式：{}
- 对话格式：{}
- 场景分隔符：{}
- 特殊要求：{}

请返回排版后的文本。"#,
            content,
            options.paragraph_style,
            options.dialogue_style,
            options.scene_separator,
            options.special_requirements
        )
    }
}

impl Default for GeneratorPrompts {
    fn default() -> Self {
        Self::new()
    }
}

/// 排版选项
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FormatOptions {
    pub paragraph_style: String,
    pub dialogue_style: String,
    pub scene_separator: String,
    pub special_requirements: String,
}

impl Default for FormatOptions {
    fn default() -> Self {
        Self {
            paragraph_style: "空行分隔".to_string(),
            dialogue_style: "中文引号".to_string(),
            scene_separator: "***".to_string(),
            special_requirements: "无".to_string(),
        }
    }
}

/// AI生成的角色数据
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GeneratedCharacter {
    pub name: String,
    pub role_type: Option<String>,
    pub race: Option<String>,
    pub age: Option<i32>,
    pub gender: Option<String>,
    pub birth_date: Option<String>,
    pub appearance: Option<String>,
    pub personality: Option<String>,
    pub background: Option<String>,
    pub mbti: Option<String>,
    pub enneagram: Option<String>,
    pub bazi: Option<String>,
    pub ziwei: Option<String>,
    pub skills: Option<String>,
    pub status: Option<String>,
    pub items: Option<String>,
}

/// AI生成的角色关系数据
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GeneratedCharacterRelation {
    pub from_character_name: String,
    pub to_character_name: String,
    pub relation_type: String,
    pub description: Option<String>,
}

/// AI生成的世界观数据
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GeneratedWorldView {
    pub category: String,
    pub title: String,
    pub content: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// AI生成的情节点数据
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GeneratedPlotPoint {
    pub title: String,
    pub description: Option<String>,
    pub note: Option<String>,
    pub emotional_tone: Option<String>,
}

/// AI生成的分镜数据
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GeneratedStoryboard {
    pub shot_number: i32,
    pub shot_type: Option<String>,
    pub duration: Option<i32>,
    pub scene_description: Option<String>,
    pub camera_movement: Option<String>,
    pub visual_prompt: Option<String>,
    pub negative_prompt: Option<String>,
    pub style_notes: Option<String>,
}
