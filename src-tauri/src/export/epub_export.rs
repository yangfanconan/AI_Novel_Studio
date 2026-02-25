use super::{ExportContent, ExportFormat};
use anyhow::{Context, Result};
use epub_builder::{EpubBuilder, EpubContent, ZipLibrary};
use std::fs::File;
use std::path::Path;

pub fn export_as_epub(
    content: &ExportContent,
    output_path: &Path,
) -> Result<()> {
    let zip_lib = ZipLibrary::new()
        .map_err(|e| anyhow::anyhow!("无法创建ZIP库: {}", e))?;
    let mut builder = EpubBuilder::new(zip_lib)
        .map_err(|e| anyhow::anyhow!("无法创建EPUB构建器: {}", e))?;
    
    builder.metadata("title", &content.metadata.title).map_err(|e| anyhow::anyhow!("无法设置标题: {}", e))?;
    builder.metadata("author", &content.metadata.author).map_err(|e| anyhow::anyhow!("无法设置作者: {}", e))?;
    
    if let Some(desc) = &content.metadata.description {
        builder.metadata("description", desc).map_err(|e| anyhow::anyhow!("无法设置描述: {}", e))?;
    }
    
    for (index, chapter) in content.chapters.iter().enumerate() {
        let mut chapter_html = String::new();
        chapter_html.push_str("<!DOCTYPE html>\n");
        chapter_html.push_str("<html>\n");
        chapter_html.push_str("<head>\n");
        chapter_html.push_str("<meta charset=\"utf-8\"/>\n");
        chapter_html.push_str("<style>\n");
        chapter_html.push_str("body { font-family: 'Georgia', serif; line-height: 1.6; margin: 0; padding: 20px; }\n");
        chapter_html.push_str("h1 { color: #333; border-bottom: 2px solid #eee; padding-bottom: 10px; }\n");
        chapter_html.push_str("p { text-indent: 2em; margin: 10px 0; }\n");
        chapter_html.push_str("</style>\n");
        chapter_html.push_str("</head>\n");
        chapter_html.push_str("<body>\n");
        chapter_html.push_str(&format!("<h1>第{}章 {}</h1>\n", chapter.number, chapter.title));
        chapter_html.push_str(&format!("<p><strong>字数:</strong> {}</p>\n", chapter.content.chars().count()));
        chapter_html.push_str("<div style='text-align: justify;'>\n");
        
        for paragraph in chapter.content.split('\n') {
            if !paragraph.trim().is_empty() {
                chapter_html.push_str(&format!("<p>{}</p>\n", paragraph));
            }
        }
        
        chapter_html.push_str("</div>\n");
        chapter_html.push_str("</body>\n");
        chapter_html.push_str("</html>");
        
        builder.add_content(
            EpubContent::new(&format!("chapter_{}.html", index), chapter_html.as_bytes())
                .title(&format!("第{}章 {}", chapter.number, chapter.title))
        ).map_err(|e| anyhow::anyhow!("无法添加章节: {}", e))?;
    }
    
    let mut file = File::create(output_path)
        .with_context(|| format!("无法创建 EPUB 文件: {:?}", output_path))?;
    
    builder.generate(&mut file)
        .map_err(|e| anyhow::anyhow!("无法生成 EPUB 文件: {}", e))?;
    
    Ok(())
}
