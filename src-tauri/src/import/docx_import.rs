use super::{ImportFormat, ImportResult, ImportedChapter};
use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use zip::ZipArchive;
use quick_xml::Reader;
use quick_xml::events::Event;
use regex::Regex;

pub fn import_from_docx(file_path: &Path) -> Result<ImportResult> {
    let file = File::open(file_path)
        .with_context(|| format!("无法打开 DOCX 文件: {:?}", file_path))?;
    
    let mut archive = ZipArchive::new(file)
        .with_context(|| "无法解压 DOCX 文件，请确保文件格式正确")?;
    
    let document_xml = archive.by_name("word/document.xml")
        .with_context(|| "DOCX 文件中未找到 document.xml")?;
    
    let mut content = String::new();
    parse_docx_content(document_xml, &mut content)?;
    
    let filename = file_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("未命名")
        .to_string();
    
    let chapters = parse_txt_style_chapters(&content);
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

fn parse_docx_content<R: Read>(reader: R, output: &mut String) -> Result<()> {
    let mut xml_reader = Reader::from_reader(BufReader::new(reader));
    xml_reader.config_mut().trim_text(true);
    
    let mut in_paragraph = false;
    let mut in_text = false;
    let mut current_paragraph = String::new();
    let mut buf = Vec::new();
    
    loop {
        match xml_reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                match e.local_name().as_ref() {
                    b"w:p" => {
                        in_paragraph = true;
                        current_paragraph.clear();
                    }
                    b"w:t" => {
                        in_text = true;
                    }
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                match e.local_name().as_ref() {
                    b"w:p" => {
                        if !current_paragraph.is_empty() {
                            if !output.is_empty() {
                                output.push('\n');
                            }
                            output.push_str(&current_paragraph);
                        }
                        in_paragraph = false;
                    }
                    b"w:t" => {
                        in_text = false;
                    }
                    _ => {}
                }
            }
            Ok(Event::Text(ref e)) => {
                if in_text && in_paragraph {
                    if let Ok(text) = e.unescape() {
                        current_paragraph.push_str(&text);
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                return Err(anyhow::anyhow!("解析 DOCX XML 时出错: {:?}", e));
            }
            _ => {}
        }
        buf.clear();
    }
    
    Ok(())
}

fn parse_txt_style_chapters(content: &str) -> Vec<ImportedChapter> {
    let mut chapters = Vec::new();
    
    let chapter_patterns = vec![
        Regex::new(r"^第([零一二三四五六七八九十百千万\d]+)章[\s:：]*(.*)$").unwrap(),
        Regex::new(r"^Chapter\s*(\d+)[\s:：]*(.*)$").unwrap(),
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
                if chapters.is_empty() { "正文".to_string() } else { "尾声".to_string() }
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
