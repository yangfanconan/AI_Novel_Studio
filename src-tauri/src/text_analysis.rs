use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WritingStyleAnalysis {
    pub avg_sentence_length: f32,
    pub avg_word_length: f32,
    pub vocabulary_richness: f32,
    pub sentence_variety: Vec<String>,
    pub tone: String,
    pub writing_style_tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RhythmAnalysis {
    pub pacing_score: f32,
    pub pacing_segments: Vec<PacingSegment>,
    pub action_vs_description_ratio: f32,
    pub dialogue_ratio: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacingSegment {
    pub start_position: usize,
    pub end_position: usize,
    pub intensity: f32,
    pub segment_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionAnalysis {
    pub overall_emotion: String,
    pub emotion_curve: Vec<EmotionPoint>,
    pub emotion_changes: usize,
    pub dominant_emotions: Vec<EmotionScore>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionPoint {
    pub position: usize,
    pub emotion: String,
    pub intensity: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionScore {
    pub emotion: String,
    pub score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadabilityAnalysis {
    pub flesch_score: f32,
    pub reading_level: String,
    pub avg_sentence_complexity: f32,
    pub syllable_count: usize,
    pub word_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepetitionDetection {
    pub repeated_words: Vec<RepeatedItem>,
    pub repeated_phrases: Vec<RepeatedItem>,
    pub repetition_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepeatedItem {
    pub text: String,
    pub count: usize,
    pub positions: Vec<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogicCheck {
    pub logical_issues: Vec<LogicIssue>,
    pub character_consistency_issues: Vec<ConsistencyIssue>,
    pub timeline_issues: Vec<TimelineIssue>,
    pub overall_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogicIssue {
    pub position: usize,
    pub issue_type: String,
    pub description: String,
    pub severity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsistencyIssue {
    pub character_name: String,
    pub issue_type: String,
    pub description: String,
    pub positions: Vec<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineIssue {
    pub position: usize,
    pub issue_type: String,
    pub description: String,
}

pub struct TextAnalyzer;

impl TextAnalyzer {
    pub fn analyze_writing_style(text: &str) -> WritingStyleAnalysis {
        let sentences: Vec<&str> = text.split_inclusive(&['.', '!', '?', '。', '！', '？'])
            .filter(|s| !s.trim().is_empty())
            .collect();

        let avg_sentence_length = if sentences.is_empty() {
            0.0
        } else {
            let total_chars: usize = sentences.iter().map(|s| s.chars().count()).sum();
            total_chars as f32 / sentences.len() as f32
        };

        let words: Vec<&str> = text.split_whitespace().collect();
        let avg_word_length = if words.is_empty() {
            0.0
        } else {
            let total_chars: usize = words.iter().map(|w| w.chars().count()).sum();
            total_chars as f32 / words.len() as f32
        };

        let unique_words: std::collections::HashSet<&str> = words.iter().cloned().collect();
        let vocabulary_richness = if words.is_empty() {
            0.0
        } else {
            (unique_words.len() as f32 / words.len() as f32) * 100.0
        };

        let tone = Self::detect_tone(text);
        let writing_style_tags = Self::detect_style_tags(text);

        WritingStyleAnalysis {
            avg_sentence_length,
            avg_word_length,
            vocabulary_richness,
            sentence_variety: vec![],
            tone,
            writing_style_tags,
        }
    }

    pub fn analyze_rhythm(text: &str) -> RhythmAnalysis {
        let paragraphs: Vec<&str> = text.split('\n').filter(|p| !p.trim().is_empty()).collect();
        let segment_size = std::cmp::max(1, paragraphs.len() / 10);
        
        let mut pacing_segments = Vec::new();
        let mut total_intensity = 0.0;
        let mut action_count = 0;
        let mut dialogue_count = 0;
        let mut description_count = 0;

        for (i, paragraph) in paragraphs.iter().enumerate() {
            let intensity = Self::calculate_paragraph_intensity(paragraph);
            total_intensity += intensity;

            let segment_type = if paragraph.contains('"') || paragraph.contains('"') {
                dialogue_count += 1;
                "dialogue".to_string()
            } else {
                description_count += 1;
                "description".to_string()
            };

            if i % segment_size == 0 || i == paragraphs.len() - 1 {
                pacing_segments.push(PacingSegment {
                    start_position: i.saturating_sub(segment_size),
                    end_position: i,
                    intensity,
                    segment_type,
                });
            }
        }

        let pacing_score = if pacing_segments.is_empty() {
            50.0
        } else {
            (total_intensity / pacing_segments.len() as f32).min(100.0)
        };

        let total_content = action_count + description_count + dialogue_count;
        let action_vs_description_ratio = if total_content == 0 {
            50.0
        } else {
            (action_count as f32 / total_content as f32) * 100.0
        };

        let dialogue_ratio = if total_content == 0 {
            0.0
        } else {
            (dialogue_count as f32 / total_content as f32) * 100.0
        };

        RhythmAnalysis {
            pacing_score,
            pacing_segments,
            action_vs_description_ratio,
            dialogue_ratio,
        }
    }

    pub fn analyze_emotion(text: &str) -> EmotionAnalysis {
        let emotion_keywords = [
            ("joy", vec!["开心", "快乐", "喜悦", "幸福", "愉快", "happy", "joy", "excited"]),
            ("sadness", vec!["悲伤", "难过", "痛苦", "哭泣", "流泪", "sad", "cry", "tears"]),
            ("anger", vec!["愤怒", "生气", "暴怒", "火大", "恼火", "angry", "furious", "rage"]),
            ("fear", vec!["恐惧", "害怕", "惊恐", "颤抖", "害怕", "fear", "scared", "terrified"]),
            ("surprise", vec!["惊讶", "吃惊", "震惊", "意外", "surprise", "shocked", "amazed"]),
            ("love", vec!["爱", "喜欢", "心动", "温暖", "love", "like", "affection"]),
        ];

        let mut emotion_scores: std::collections::HashMap<&str, f32> =
            emotion_keywords.iter().map(|(k, _)| (*k, 0.0)).collect();

        let paragraphs: Vec<&str> = text.split('\n').filter(|p| !p.trim().is_empty()).collect();
        let mut emotion_curve = Vec::new();
        let mut emotion_changes = 0;
        let mut last_emotion = None;

        for (i, paragraph) in paragraphs.iter().enumerate() {
            let mut paragraph_emotions: std::collections::HashMap<&str, usize> =
                emotion_keywords.iter().map(|(k, _)| (*k, 0)).collect();

            for (emotion, keywords) in &emotion_keywords {
                let mut count: usize = 0;
                for keyword in keywords {
                    if paragraph.contains(keyword) {
                        count += 1;
                    }
                }
                *paragraph_emotions.get_mut(emotion).unwrap() += count;
            }

            let dominant_emotion = paragraph_emotions.iter()
                .max_by(|a, b| a.1.cmp(&b.1))
                .map(|(k, v)| (k.to_string(), *v as f32));

            if let Some((emotion, score)) = dominant_emotion {
                if score > 0.0 {
                    if let Some(ref last) = last_emotion {
                        if last != &emotion {
                            emotion_changes += 1;
                        }
                    }
                    last_emotion = Some(emotion.clone());

                    emotion_curve.push(EmotionPoint {
                        position: i,
                        emotion: emotion.clone(),
                        intensity: score / paragraph.len().max(1) as f32 * 100.0,
                    });

                    *emotion_scores.get_mut(emotion.as_str()).unwrap() += score as f32;
                }
            }
        }

        let overall_emotion = emotion_scores.iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(k, _)| k.to_string())
            .unwrap_or_else(|| "neutral".to_string());

        let total_score: f32 = emotion_scores.values().sum();
        let dominant_emotions = emotion_scores.iter()
            .map(|(emotion, score)| EmotionScore {
                emotion: emotion.to_string(),
                score: if total_score > 0.0 { (score / total_score) * 100.0 } else { 0.0 },
            })
            .filter(|e| e.score > 5.0)
            .collect();

        EmotionAnalysis {
            overall_emotion,
            emotion_curve,
            emotion_changes,
            dominant_emotions,
        }
    }

    pub fn analyze_readability(text: &str) -> ReadabilityAnalysis {
        let sentences: Vec<&str> = text.split_inclusive(&['.', '!', '?', '。', '！', '？'])
            .filter(|s| !s.trim().is_empty())
            .collect();

        let words: Vec<&str> = text.split_whitespace().collect();
        let word_count = words.len();

        let syllable_count: usize = words.iter()
            .map(|w| Self::count_syllables(w))
            .sum();

        let avg_sentence_complexity = if sentences.is_empty() {
            0.0
        } else {
            let avg_words_per_sentence = word_count as f32 / sentences.len() as f32;
            let avg_syllables_per_word = syllable_count as f32 / word_count.max(1) as f32;
            (avg_words_per_sentence * 0.4 + avg_syllables_per_word * 0.6).min(100.0)
        };

        let flesch_score = if sentences.is_empty() || word_count == 0 {
            0.0
        } else {
            206.835 - 1.015 * (word_count as f32 / sentences.len() as f32)
                - 84.6 * (syllable_count as f32 / word_count as f32)
        };

        let reading_level = match flesch_score {
            s if s >= 90.0 => "小学低年级".to_string(),
            s if s >= 80.0 => "小学高年级".to_string(),
            s if s >= 70.0 => "初中".to_string(),
            s if s >= 60.0 => "高中".to_string(),
            s if s >= 50.0 => "大学".to_string(),
            s if s >= 30.0 => "专业".to_string(),
            _ => "学术".to_string(),
        };

        ReadabilityAnalysis {
            flesch_score,
            reading_level,
            avg_sentence_complexity,
            syllable_count,
            word_count,
        }
    }

    pub fn detect_repetitions(text: &str, min_repetitions: usize) -> RepetitionDetection {
        let words: Vec<&str> = text.split_whitespace()
            .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()))
            .filter(|w| w.len() > 1)
            .collect();

        let mut word_counts: std::collections::HashMap<&str, (usize, Vec<usize>)> = std::collections::HashMap::new();

        for (i, word) in words.iter().enumerate() {
            let entry = word_counts.entry(word).or_insert((0, Vec::new()));
            entry.0 += 1;
            entry.1.push(i);
        }

        let repeated_words: Vec<RepeatedItem> = word_counts.iter()
            .filter(|(_, (count, _))| *count >= min_repetitions)
            .map(|(text, (count, positions))| RepeatedItem {
                text: text.to_string(),
                count: *count,
                positions: positions.clone(),
            })
            .collect();

        let repeated_phrases = Self::detect_repeated_phrases(text, min_repetitions);

        let repetition_score = if words.is_empty() {
            0.0
        } else {
            (repeated_words.len() + repeated_phrases.len()) as f32 / words.len() as f32 * 100.0
        };

        RepetitionDetection {
            repeated_words,
            repeated_phrases,
            repetition_score,
        }
    }

    pub fn check_logic(
        text: &str,
        characters: &Vec<crate::models::Character>,
    ) -> LogicCheck {
        let mut logical_issues = Vec::new();
        let mut character_consistency_issues = Vec::new();
        let timeline_issues = Vec::new();

        let paragraphs: Vec<&str> = text.split('\n').filter(|p| !p.trim().is_empty()).collect();

        for (i, paragraph) in paragraphs.iter().enumerate() {
            if paragraph.contains("突然") && paragraph.contains("但是") {
                logical_issues.push(LogicIssue {
                    position: i,
                    issue_type: "abrupt_transition".to_string(),
                    description: "突然的转折可能缺乏铺垫".to_string(),
                    severity: "low".to_string(),
                });
            }

            if paragraph.contains("同时") && paragraph.contains("却") {
                logical_issues.push(LogicIssue {
                    position: i,
                    issue_type: "contradiction".to_string(),
                    description: "同时发生的矛盾表述".to_string(),
                    severity: "medium".to_string(),
                });
            }
        }

        for character in characters {
            let appearances: Vec<usize> = paragraphs.iter()
                .enumerate()
                .filter(|(_, p)| p.contains(&character.name))
                .map(|(i, _)| i)
                .collect();

            if appearances.len() > 1 {
                for window in appearances.windows(2) {
                    let distance = window[1] - window[0];
                    if distance > 50 {
                        character_consistency_issues.push(ConsistencyIssue {
                            character_name: character.name.clone(),
                            issue_type: "long_absence".to_string(),
                            description: format!("角色在{}段后重新出现", distance),
                            positions: vec![window[0], window[1]],
                        });
                    }
                }
            }
        }

        let overall_score = if logical_issues.is_empty()
            && character_consistency_issues.is_empty()
            && timeline_issues.is_empty() {
            100.0
        } else {
            (100.0 - (logical_issues.len() as f32 * 10.0)
                - (character_consistency_issues.len() as f32 * 15.0)
                - (timeline_issues.len() as f32 * 20.0))
                .max(0.0)
        };

        LogicCheck {
            logical_issues,
            character_consistency_issues,
            timeline_issues,
            overall_score,
        }
    }

    fn detect_tone(text: &str) -> String {
        let positive_keywords = ["开心", "快乐", "成功", "胜利", "希望", "美好", "温暖"];
        let negative_keywords = ["悲伤", "痛苦", "失败", "绝望", "黑暗", "寒冷"];
        
        let mut positive_count = 0;
        for k in &positive_keywords {
            if text.contains(k) {
                positive_count += 1;
            }
        }
        
        let mut negative_count = 0;
        for k in &negative_keywords {
            if text.contains(k) {
                negative_count += 1;
            }
        }

        if positive_count > negative_count * 2 {
            "positive".to_string()
        } else if negative_count > positive_count * 2 {
            "negative".to_string()
        } else {
            "neutral".to_string()
        }
    }

    fn detect_style_tags(text: &str) -> Vec<String> {
        let mut tags = Vec::new();
        
        if text.len() > 5000 {
            tags.push("verbose".to_string());
        }
        if text.split('"').count() > 10 {
            tags.push("dialogue_heavy".to_string());
        }
        if text.chars().filter(|c| c.is_alphanumeric()).count() as f32 / text.len() as f32 > 0.6 {
            tags.push("descriptive".to_string());
        }

        tags
    }

    fn calculate_paragraph_intensity(paragraph: &str) -> f32 {
        let action_words = ["跑", "跳", "打", "击", "攻击", "逃", "追", "fight", "run"];
        let emotion_words = ["爱", "恨", "恐惧", "愤怒", "悲伤", "love", "hate", "fear"];
        
        let mut action_count = 0;
        for w in &action_words {
            if paragraph.contains(w) {
                action_count += 1;
            }
        }
        let mut emotion_count = 0;
        for w in &emotion_words {
            if paragraph.contains(w) {
                emotion_count += 1;
            }
        }
        let word_count = paragraph.split_whitespace().count();

        if word_count == 0 {
            return 0.0;
        }

        let intensity = ((action_count + emotion_count) * 2) as f32 / word_count as f32 * 100.0;
        intensity.min(100.0)
    }

    fn count_syllables(word: &str) -> usize {
        word.chars()
            .filter(|c| c.is_alphanumeric() || matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'ā' | 'á' | 'ǎ' | 'à' | 'ē' | 'é' | 'ě' | 'è' | 'ī' | 'í' | 'ǐ' | 'ì' | 'ō' | 'ó' | 'ǒ' | 'ò' | 'ū' | 'ú' | 'ǔ' | 'ù' | 'ǖ' | 'ǘ' | 'ǚ'))
            .count()
            .max(1)
    }

    fn detect_repeated_phrases(text: &str, min_repetitions: usize) -> Vec<RepeatedItem> {
        let phrases: Vec<&str> = text.matches(&['.', '。'][..])
            .map(|s| s.trim())
            .filter(|s| s.len() > 5)
            .collect();

        let mut phrase_counts: std::collections::HashMap<&str, (usize, Vec<usize>)> = std::collections::HashMap::new();

        for (i, phrase) in phrases.iter().enumerate() {
            let entry = phrase_counts.entry(phrase).or_insert((0, Vec::new()));
            entry.0 += 1;
            entry.1.push(i);
        }

        phrase_counts.iter()
            .filter(|(_, (count, _))| *count >= min_repetitions)
            .map(|(text, (count, positions))| RepeatedItem {
                text: text.to_string(),
                count: *count,
                positions: positions.clone(),
            })
            .collect()
    }
}
