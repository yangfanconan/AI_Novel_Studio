use super::{ImportFormat, ImportResult, ImportedChapter};
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use regex::Regex;

pub fn import_from_markdown(file_path: &Path) -> Result<ImportResult> {
    let content = fs::read_to_string(file_path)
        .with_context(|| format!("无法读取 Markdown 文件: {:?}", file_path))?;
    
    let filename = file_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("未命名")
        .to_string();
    
    let (title, clean_content) = extract_frontmatter(&content, &filename);
    let chapters = parse_md_chapters(&clean_content);
    let chapter_count = chapters.len();
    let word_count: usize = chapters.iter().map(|c| c.word_count).sum();
    
    Ok(ImportResult {
        success: true,
        title,
        content: clean_content.clone(),
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

fn extract_frontmatter(content: &str, default_title: &str) -> (String, String) {
    let mut title = default_title.to_string();
    let mut clean_content = content.to_string();
    
    let yaml_frontmatter = Regex::new(r"^---\s*\n([\s\S]*?)\n---\s*\n").unwrap();
    if let Some(caps) = yaml_frontmatter.captures(content) {
        let frontmatter = &caps[1];
        let title_re = Regex::new(r"(?m)^title:\s*(.+)$").unwrap();
        if let Some(title_caps) = title_re.captures(frontmatter) {
            title = title_caps[1].trim().trim_matches('"').to_string();
        }
        clean_content = content[caps[0].len()..].to_string();
    }
    
    let h1_re = Regex::new(r"^#\s+(.+)\s*$").unwrap();
    if let Some(caps) = h1_re.captures(&clean_content) {
        if title == default_title {
            title = caps[1].trim().to_string();
        }
    }
    
    (title, clean_content)
}

fn parse_md_chapters(content: &str) -> Vec<ImportedChapter> {
    let mut chapters = Vec::new();
    
    let heading_re = Regex::new(r"^(#{1,3})\s+(.+)$").unwrap();
    let chapter_re = Regex::new(r"^第([零一二三四五六七八九十百千万\d]+)章[\s:：]*(.*)$").unwrap();
    
    let lines: Vec<&str> = content.lines().collect();
    let mut current_title = String::new();
    let mut current_content = String::new();
    let mut found_chapters = false;
    let mut first_h1_skipped = false;
    
    for line in &lines {
        let trimmed = line.trim();
        
        if let Some(caps) = heading_re.captures(trimmed) {
            let level = caps[1].len();
            let heading_text = caps[2].trim();
            
            if level == 1 && !first_h1_skipped {
                first_h1_skipped = true;
                continue;
            }
            
            if level <= 2 || chapter_re.is_match(heading_text) {
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
                current_title = heading_text.to_string();
                current_content = String::new();
                found_chapters = true;
                continue;
            }
        }
        
        if found_chapters || !trimmed.is_empty() {
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
    fn test_parse_md_headings() {
        let content = "# 小说标题\n\n## 第一章 开始\n这是第一章。\n\n## 第二章 继续\n这是第二章。";
        let chapters = parse_md_chapters(content);
        assert!(chapters.len() >= 2);
    }
    
    #[test]
    fn test_extract_frontmatter() {
        let content = "---\ntitle: 我的小说\n---\n\n# 标题\n内容";
        let (title, _) = extract_frontmatter(content, "默认");
        assert_eq!(title, "我的小说");
    }
}
