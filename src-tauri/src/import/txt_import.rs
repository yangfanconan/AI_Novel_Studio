use super::{ImportFormat, ImportResult, ImportedChapter};
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use regex::Regex;

pub fn import_from_txt(file_path: &Path) -> Result<ImportResult> {
    let content = fs::read_to_string(file_path)
        .with_context(|| format!("无法读取 TXT 文件: {:?}", file_path))?;
    
    let filename = file_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("未命名")
        .to_string();
    
    let chapters = parse_txt_chapters(&content);
    let chapter_count = chapters.len();
    let word_count: usize = chapters.iter().map(|c| c.word_count).sum();
    
    Ok(ImportResult {
        success: true,
        title: filename,
        content: content.clone(),
        chapter_count,
        word_count,
        chapters,
        message: if chapter_count > 0 {
            Some(format!("成功解析 {} 个章节", chapter_count))
        } else {
            Some("文件内容将作为单章节导入".to_string())
        },
    })
}

fn parse_txt_chapters(content: &str) -> Vec<ImportedChapter> {
    let mut chapters = Vec::new();
    
    let chapter_patterns = vec![
        Regex::new(r"^第([零一二三四五六七八九十百千万\d]+)章[\s:：]*(.*)$").unwrap(),
        Regex::new(r"^Chapter\s*(\d+)[\s:：]*(.*)$").unwrap(),
        Regex::new(r"^(\d+)[\.\s]+(.*)$").unwrap(),
    ];
    
    let lines: Vec<&str> = content.lines().collect();
    let mut current_title = String::new();
    let mut current_content = String::new();
    let mut found_chapters = false;
    
    for line in &lines {
        let trimmed = line.trim();
        let mut is_chapter_start = false;
        let mut chapter_title = String::new();
        
        for pattern in &chapter_patterns {
            if let Some(caps) = pattern.captures(trimmed) {
                is_chapter_start = true;
                if caps.len() > 2 {
                    chapter_title = if caps[2].is_empty() {
                        format!("第{}章", &caps[1])
                    } else {
                        format!("第{}章 {}", &caps[1], caps[2].trim())
                    };
                } else {
                    chapter_title = caps[0].to_string();
                }
                break;
            }
        }
        
        if is_chapter_start {
            if !current_content.trim().is_empty() || !current_title.is_empty() {
                let word_count = current_content.chars().count();
                if word_count > 0 {
                    chapters.push(ImportedChapter {
                        title: if current_title.is_empty() {
                            "序章".to_string()
                        } else {
                            current_title.clone()
                        },
                        content: current_content.trim().to_string(),
                        word_count,
                    });
                }
            }
            current_title = chapter_title;
            current_content = String::new();
            found_chapters = true;
        } else if found_chapters || !trimmed.is_empty() {
            if !current_content.is_empty() {
                current_content.push('\n');
            }
            current_content.push_str(line);
        }
    }
    
    if !current_content.trim().is_empty() {
        let word_count = current_content.chars().count();
        chapters.push(ImportedChapter {
            title: if current_title.is_empty() {
                if chapters.is_empty() {
                    "正文".to_string()
                } else {
                    "尾声".to_string()
                }
            } else {
                current_title
            },
            content: current_content.trim().to_string(),
            word_count,
        });
    }
    
    if chapters.is_empty() {
        let word_count = content.chars().count();
        if word_count > 0 {
            chapters.push(ImportedChapter {
                title: "正文".to_string(),
                content: content.trim().to_string(),
                word_count,
            });
        }
    }
    
    chapters
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_chinese_chapters() {
        let content = "第一章 开始\n这是第一章的内容。\n\n第二章 继续\n这是第二章的内容。";
        let chapters = parse_txt_chapters(content);
        assert_eq!(chapters.len(), 2);
        assert_eq!(chapters[0].title, "第一章 开始");
        assert_eq!(chapters[1].title, "第二章 继续");
    }
    
    #[test]
    fn test_parse_no_chapters() {
        let content = "这是一段没有章节标记的文本。";
        let chapters = parse_txt_chapters(content);
        assert_eq!(chapters.len(), 1);
        assert_eq!(chapters[0].title, "正文");
    }
}
