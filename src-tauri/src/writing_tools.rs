use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitiveWordDetection {
    pub sensitive_words: Vec<SensitiveWordMatch>,
    pub total_count: usize,
    pub severity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitiveWordMatch {
    pub word: String,
    pub position: usize,
    pub context: String,
    pub severity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypoDetection {
    pub typos: Vec<TypoMatch>,
    pub total_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypoMatch {
    pub original: String,
    pub correction: String,
    pub position: usize,
    pub context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrammarCheck {
    pub grammar_issues: Vec<GrammarIssue>,
    pub total_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrammarIssue {
    pub position: usize,
    pub issue_type: String,
    pub description: String,
    pub suggestion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatNormalization {
    pub original: String,
    pub normalized: String,
    pub changes: Vec<FormatChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatChange {
    pub change_type: String,
    pub position: usize,
    pub original: String,
    pub corrected: String,
}

pub struct WritingTools;

impl WritingTools {
    pub fn detect_sensitive_words(text: &str) -> SensitiveWordDetection {
        let sensitive_word_list = Self::get_sensitive_word_list();
        let mut matches = Vec::new();
        let mut severity = "low".to_string();

        let words: Vec<&str> = text.split_whitespace().collect();

        for (i, word) in words.iter().enumerate() {
            let trimmed_word = word.trim();
            if let Some(severity_level) = sensitive_word_list.get(trimmed_word) {
                let context_start = if i > 0 { i - 1 } else { 0 };
                let context_end = if i < words.len() - 1 { i + 2 } else { i + 1 };
                
                let context: String = words.iter()
                    .skip(context_start)
                    .take(context_end - context_start + 1)
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(" ");

                let severity_str = severity_level.to_string();
                matches.push(SensitiveWordMatch {
                    word: trimmed_word.to_string(),
                    position: i,
                    context,
                    severity: severity_str.clone(),
                });

                if *severity_level == "high" {
                    severity = "high".to_string();
                } else if *severity_level == "medium" && severity != "high" {
                    severity = "medium".to_string();
                }
            }
        }

        let total_count = matches.len();
        SensitiveWordDetection {
            sensitive_words: matches,
            total_count,
            severity,
        }
    }

    pub fn detect_typos(text: &str) -> TypoDetection {
        let common_typos = Self::get_common_typos();
        let mut typos = Vec::new();

        let words: Vec<&str> = text.split_whitespace().collect();

        for (position, word) in words.iter().enumerate() {
            let lower_word = word.trim().to_lowercase();
            if let Some(correction) = common_typos.get(lower_word.as_str()) {
                let context = Self::get_context(text, position, word.len());
                typos.push(TypoMatch {
                    original: word.trim().to_string(),
                    correction: correction.to_string(),
                    position,
                    context,
                });
            }
        }

        let total_count = typos.len();
        TypoDetection {
            typos,
            total_count,
        }
    }

    pub fn check_grammar(text: &str) -> GrammarCheck {
        let mut issues = Vec::new();

        let lines: Vec<&str> = text.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            if line.contains("的") && line.split("的").count() > 3 {
                issues.push(GrammarIssue {
                    position: i,
                    issue_type: "excessive_de".to_string(),
                    description: "过多使用'的'字".to_string(),
                    suggestion: "尝试减少'的'字的使用或合并句子".to_string(),
                });
            }

            if line.contains("了") && line.split("了").count() > 2 {
                issues.push(GrammarIssue {
                    position: i,
                    issue_type: "excessive_le".to_string(),
                    description: "过多使用'了'字".to_string(),
                    suggestion: "尝试减少'了'字的使用".to_string(),
                });
            }

            if line.contains("非常") && line.contains("很") {
                issues.push(GrammarIssue {
                    position: i,
                    issue_type: "redundant_modifier".to_string(),
                    description: "同时使用'非常'和'很'".to_string(),
                    suggestion: "选择一个程度副词".to_string(),
                });
            }

            if line.ends_with("。") && line.len() < 5 {
                issues.push(GrammarIssue {
                    position: i,
                    issue_type: "short_sentence".to_string(),
                    description: "句子过短".to_string(),
                    suggestion: "考虑扩展句子或与其他句子合并".to_string(),
                });
            }
        }

        let total_count = issues.len();
        GrammarCheck {
            grammar_issues: issues,
            total_count,
        }
    }

    pub fn normalize_format(text: &str) -> FormatNormalization {
        let mut changes = Vec::new();
        let mut normalized = text.to_string();

        let original = text.to_string();

        let mut position = 0;

        while let Some(idx) = normalized.find("。。") {
            let _context = Self::get_context(&normalized, idx, 2);
            changes.push(FormatChange {
                change_type: "multiple_period".to_string(),
                position,
                original: "。。".to_string(),
                corrected: "。".to_string(),
            });
            normalized = normalized.replacen("。。", "。", 1);
            position = idx + 1;
        }

        position = 0;
        while let Some(idx) = normalized.find("，，") {
            let _context = Self::get_context(&normalized, idx, 2);
            changes.push(FormatChange {
                change_type: "multiple_comma".to_string(),
                position,
                original: ",,".to_string(),
                corrected: ",".to_string(),
            });
            normalized = normalized.replacen(",,", ",", 1);
            position = idx + 1;
        }

        position = 0;
        while let Some(idx) = normalized.find("！！！") {
            let _context = Self::get_context(&normalized, idx, 3);
            changes.push(FormatChange {
                change_type: "excessive_exclamation".to_string(),
                position,
                original: "！！！".to_string(),
                corrected: "！".to_string(),
            });
            normalized = normalized.replacen("！！！", "！", 1);
            position = idx + 1;
        }

        position = 0;
        while let Some(idx) = normalized.find("？？？") {
            let _context = Self::get_context(&normalized, idx, 3);
            changes.push(FormatChange {
                change_type: "excessive_question".to_string(),
                position,
                original: "？？？".to_string(),
                corrected: "？".to_string(),
            });
            normalized = normalized.replacen("？？？", "？", 1);
            position = idx + 1;
        }

        let lines: Vec<&str> = normalized.lines().collect();
        let mut paragraph_count = 0;
        for (i, line) in lines.iter().enumerate() {
            if line.trim().is_empty() {
                paragraph_count += 1;
            } else if paragraph_count > 1 && line.starts_with("  ") {
                changes.push(FormatChange {
                    change_type: "indentation".to_string(),
                    position: i,
                    original: line.to_string(),
                    corrected: line.trim_start().to_string(),
                });
            }
        }

        FormatNormalization {
            original,
            normalized,
            changes,
        }
    }

    fn get_sensitive_word_list() -> HashMap<&'static str, &'static str> {
        let mut map = HashMap::new();
        
        map.insert("暴力", "high");
        map.insert("血腥", "high");
        map.insert("恐怖", "medium");
        map.insert("残忍", "high");
        map.insert("酷刑", "high");
        map.insert("谋杀", "high");
        map.insert("自杀", "high");
        map.insert("性暴力", "high");
        map.insert("性骚扰", "high");
        map.insert("歧视", "medium");
        map.insert("仇恨", "medium");
        map.insert("种族歧视", "high");
        map.insert("宗教歧视", "high");
        map.insert("性别歧视", "high");
        
        map
    }

    fn get_common_typos() -> HashMap<&'static str, &'static str> {
        let mut map = HashMap::new();
        
        map.insert("的地得", "的");
        map.insert("的地", "的");
        map.insert("得地", "得");
        map.insert("的得", "的");
        map.insert("再在", "在");
        map.insert("在再", "在");
        map.insert("像象", "像");
        map.insert("象像", "像");
        map.insert("坐座", "坐");
        map.insert("座坐", "坐");
        map.insert("作做", "做");
        map.insert("做作", "做");
        map.insert("既即", "既");
        map.insert("即既", "既");
        map.insert("帐账", "账");
        map.insert("账帐", "账");
        
        map
    }

    fn get_context(text: &str, position: usize, length: usize) -> String {
        let chars: Vec<char> = text.chars().collect();
        let start = if position >= 10 { position - 10 } else { 0 };
        let end = if position + length + 10 <= chars.len() {
            position + length + 10
        } else {
            chars.len()
        };
        
        chars[start..end].iter().collect()
    }
}
