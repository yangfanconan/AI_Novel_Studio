use super::{ExportContent, ExportFormat};
use anyhow::{Context, Result};
use genpdf::{elements, style, Element};
use std::path::Path;

pub fn export_as_pdf(
    content: &ExportContent,
    output_path: &Path,
) -> Result<()> {
    let font_family = genpdf::fonts::from_files(
        "/System/Library/Fonts",
        "Helvetica",
        None
    ).map_err(|e| anyhow::anyhow!("无法加载字体: {:?}", e))?;
    
    let mut doc = genpdf::Document::new(font_family);
    doc.set_title(&content.metadata.title);
    
    let title_style = style::Style::new()
        .with_font_size(24)
        .bold();
    
    let header_style = style::Style::new()
        .with_font_size(12);
    
    let chapter_title_style = style::Style::new()
        .with_font_size(16)
        .bold();
    
    let text_style = style::Style::new()
        .with_font_size(10);
    
    doc.push(elements::Paragraph::new(&content.metadata.title)
        .styled(title_style));
    doc.push(elements::Break::new(1));
    doc.push(elements::Paragraph::new(&format!("作者: {}", content.metadata.author))
        .styled(header_style));
    doc.push(elements::Paragraph::new(&format!("创建时间: {}", content.metadata.created_at))
        .styled(style::Style::new().with_font_size(10)));
    doc.push(elements::Break::new(2));
    
    for chapter in &content.chapters {
        doc.push(elements::Paragraph::new(&format!("第{}章 {}", chapter.number, chapter.title))
            .styled(chapter_title_style));
        doc.push(elements::Paragraph::new(&format!("字数: {}", chapter.content.chars().count()))
            .styled(text_style));
        doc.push(elements::Break::new(1));
        
        for paragraph in chapter.content.split('\n') {
            if !paragraph.trim().is_empty() {
                doc.push(elements::Paragraph::new(paragraph).styled(text_style));
            }
        }
        
        doc.push(elements::Break::new(1));
    }
    
    doc.render_to_file(output_path)
        .map_err(|e| anyhow::anyhow!("无法生成 PDF: {:?}", e))?;
    
    Ok(())
}
