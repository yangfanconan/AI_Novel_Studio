use super::{ExportContent, ExportFormat};
use anyhow::{Context, Result};
use std::io::Write;
use std::path::Path;

pub fn export_as_md(
    content: &ExportContent,
    output_path: &Path,
) -> Result<()> {
    let mut md_content = String::new();
    
    md_content.push_str(&format!("# {}\n\n", content.metadata.title));
    md_content.push_str(&format!("**作者**: {}\n\n", content.metadata.author));
    
    if let Some(desc) = &content.metadata.description {
        md_content.push_str(&format!("**简介**: {}\n\n", desc));
    }
    
    md_content.push_str("---\n\n");
    md_content.push_str(&format!("**创建时间**: {}\n\n", content.metadata.created_at));
    md_content.push_str(&format!("**字数**: {}\n\n", content.metadata.word_count));
    md_content.push_str(&format!("**章节数**: {}\n\n", content.metadata.chapter_count));
    md_content.push_str("---\n\n");
    
    for chapter in &content.chapters {
        md_content.push_str(&format!("## 第{}章 {}\n\n", chapter.number, chapter.title));
        md_content.push_str(&format!("*字数: {}*\n\n", chapter.content.chars().count()));
        md_content.push_str(&chapter.content);
        md_content.push_str("\n\n");
    }
    
    let mut file = std::fs::File::create(output_path)
        .with_context(|| format!("无法创建导出文件: {:?}", output_path))?;
    
    file.write_all(md_content.as_bytes())
        .with_context(|| format!("无法保存文件: {:?}", output_path))?;
    
    Ok(())
}
