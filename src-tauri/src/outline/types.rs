use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutlineNode {
    pub id: String,
    pub project_id: String,
    pub parent_id: Option<String>,
    pub title: String,
    pub content: String,
    pub node_type: OutlineNodeType,
    pub sort_order: i32,
    pub status: OutlineNodeStatus,
    pub word_count_target: Option<i32>,
    pub word_count_actual: i32,
    pub metadata: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OutlineNodeType {
    Arc,
    Chapter,
    Scene,
    Beat,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OutlineNodeStatus {
    Planned,
    InProgress,
    Completed,
    Skipped,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOutlineNodeRequest {
    pub project_id: String,
    pub parent_id: Option<String>,
    pub title: String,
    pub content: Option<String>,
    pub node_type: OutlineNodeType,
    pub sort_order: Option<i32>,
    pub word_count_target: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateOutlineNodeRequest {
    pub id: String,
    pub title: Option<String>,
    pub content: Option<String>,
    pub status: Option<OutlineNodeStatus>,
    pub sort_order: Option<i32>,
    pub word_count_target: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateOutlineRequest {
    pub project_id: String,
    pub genre: String,
    pub theme: Option<String>,
    pub main_characters: Vec<String>,
    pub target_chapters: i32,
    pub target_words_per_chapter: i32,
    pub style: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutlineGenerationResult {
    pub arcs: Vec<GeneratedArc>,
    pub total_chapters: i32,
    pub estimated_words: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratedArc {
    pub title: String,
    pub description: String,
    pub chapters: Vec<GeneratedChapter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratedChapter {
    pub title: String,
    pub summary: String,
    pub key_events: Vec<String>,
    pub estimated_words: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutlineTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub structure: Vec<TemplateNode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateNode {
    pub title: String,
    pub node_type: OutlineNodeType,
    pub description: String,
    pub children: Vec<TemplateNode>,
}

pub fn get_default_templates() -> Vec<OutlineTemplate> {
    vec![
        OutlineTemplate {
            id: "three-act".to_string(),
            name: "三幕式结构".to_string(),
            description: "经典的三幕式故事结构，适合大多数小说".to_string(),
            structure: vec![
                TemplateNode {
                    title: "第一幕：铺垫".to_string(),
                    node_type: OutlineNodeType::Arc,
                    description: "介绍背景、人物、建立冲突".to_string(),
                    children: vec![
                        TemplateNode {
                            title: "开篇".to_string(),
                            node_type: OutlineNodeType::Scene,
                            description: "故事开场，吸引读者".to_string(),
                            children: vec![],
                        },
                        TemplateNode {
                            title: "人物介绍".to_string(),
                            node_type: OutlineNodeType::Scene,
                            description: "展示主要角色".to_string(),
                            children: vec![],
                        },
                        TemplateNode {
                            title: "激励事件".to_string(),
                            node_type: OutlineNodeType::Scene,
                            description: "打破平衡的事件".to_string(),
                            children: vec![],
                        },
                    ],
                },
                TemplateNode {
                    title: "第二幕：对抗".to_string(),
                    node_type: OutlineNodeType::Arc,
                    description: "冲突升级，角色成长".to_string(),
                    children: vec![
                        TemplateNode {
                            title: "中点".to_string(),
                            node_type: OutlineNodeType::Scene,
                            description: "故事的转折点".to_string(),
                            children: vec![],
                        },
                        TemplateNode {
                            title: "低谷".to_string(),
                            node_type: OutlineNodeType::Scene,
                            description: "主角遭遇最大挫折".to_string(),
                            children: vec![],
                        },
                    ],
                },
                TemplateNode {
                    title: "第三幕：解决".to_string(),
                    node_type: OutlineNodeType::Arc,
                    description: "高潮与结局".to_string(),
                    children: vec![
                        TemplateNode {
                            title: "高潮".to_string(),
                            node_type: OutlineNodeType::Scene,
                            description: "最终对决".to_string(),
                            children: vec![],
                        },
                        TemplateNode {
                            title: "结局".to_string(),
                            node_type: OutlineNodeType::Scene,
                            description: "故事的收尾".to_string(),
                            children: vec![],
                        },
                    ],
                },
            ],
        },
        OutlineTemplate {
            id: "heros-journey".to_string(),
            name: "英雄之旅".to_string(),
            description: "约瑟夫·坎贝尔的经典英雄旅程结构".to_string(),
            structure: vec![
                TemplateNode {
                    title: "出发".to_string(),
                    node_type: OutlineNodeType::Arc,
                    description: "英雄接受召唤".to_string(),
                    children: vec![
                        TemplateNode { title: "平凡世界".to_string(), node_type: OutlineNodeType::Scene, description: "英雄的日常".to_string(), children: vec![] },
                        TemplateNode { title: "冒险召唤".to_string(), node_type: OutlineNodeType::Scene, description: "英雄面临挑战".to_string(), children: vec![] },
                        TemplateNode { title: "拒绝召唤".to_string(), node_type: OutlineNodeType::Scene, description: "英雄的犹豫".to_string(), children: vec![] },
                        TemplateNode { title: "遇见导师".to_string(), node_type: OutlineNodeType::Scene, description: "获得指引".to_string(), children: vec![] },
                    ],
                },
                TemplateNode {
                    title: "启蒙".to_string(),
                    node_type: OutlineNodeType::Arc,
                    description: "英雄的试炼与成长".to_string(),
                    children: vec![
                        TemplateNode { title: "跨越门槛".to_string(), node_type: OutlineNodeType::Scene, description: "进入特殊世界".to_string(), children: vec![] },
                        TemplateNode { title: "试炼之路".to_string(), node_type: OutlineNodeType::Scene, description: "面对挑战".to_string(), children: vec![] },
                        TemplateNode { title: "最深的洞穴".to_string(), node_type: OutlineNodeType::Scene, description: "面对最大的恐惧".to_string(), children: vec![] },
                        TemplateNode { title: "磨难".to_string(), node_type: OutlineNodeType::Scene, description: "生死考验".to_string(), children: vec![] },
                    ],
                },
                TemplateNode {
                    title: "归来".to_string(),
                    node_type: OutlineNodeType::Arc,
                    description: "英雄回归".to_string(),
                    children: vec![
                        TemplateNode { title: "归途".to_string(), node_type: OutlineNodeType::Scene, description: "返回平凡世界".to_string(), children: vec![] },
                        TemplateNode { title: "复活".to_string(), node_type: OutlineNodeType::Scene, description: "最后的考验".to_string(), children: vec![] },
                        TemplateNode { title: "带着灵药归来".to_string(), node_type: OutlineNodeType::Scene, description: "英雄改变世界".to_string(), children: vec![] },
                    ],
                },
            ],
        },
        OutlineTemplate {
            id: "multi-pov".to_string(),
            name: "多视角叙事".to_string(),
            description: "适合多主角、多线叙事的小说".to_string(),
            structure: vec![
                TemplateNode { title: "A线：主线剧情".to_string(), node_type: OutlineNodeType::Arc, description: "主要故事线".to_string(), children: vec![] },
                TemplateNode { title: "B线：副线剧情".to_string(), node_type: OutlineNodeType::Arc, description: "次要故事线".to_string(), children: vec![] },
                TemplateNode { title: "C线：背景线索".to_string(), node_type: OutlineNodeType::Arc, description: "隐藏的故事线".to_string(), children: vec![] },
            ],
        },
    ]
}
