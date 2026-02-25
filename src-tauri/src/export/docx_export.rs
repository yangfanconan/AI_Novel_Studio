use super::{ExportContent, ExportFormat};
use anyhow::{Context, Result};
use std::io::Write;
use std::path::Path;

pub fn export_as_docx(
    content: &ExportContent,
    output_path: &Path,
) -> Result<()> {
    let mut docx_content = String::new();
    
    docx_content.push_str(&format!("# {}\n\n", content.metadata.title));
    docx_content.push_str(&format!("**作者**: {}\n\n", content.metadata.author));
    
    if let Some(desc) = &content.metadata.description {
        docx_content.push_str(&format!("**简介**: {}\n\n", desc));
    }
    
    docx_content.push_str("---\n\n");
    
    for chapter in &content.chapters {
        docx_content.push_str(&format!("## 第{}章 {}\n\n", chapter.number, chapter.title));
        docx_content.push_str(&format!("*字数: {}*\n\n", chapter.content.chars().count()));
        docx_content.push_str(&chapter.content);
        docx_content.push_str("\n\n");
    }
    
    let mut file = std::fs::File::create(output_path)
        .with_context(|| format!("无法创建导出文件: {:?}", output_path))?;
    
    file.write_all(docx_content.as_bytes())
        .with_context(|| format!("无法保存文件: {:?}", output_path))?;
    
    Ok(())
}
