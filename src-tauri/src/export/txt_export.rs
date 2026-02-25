use super::{ExportContent, ExportFormat};
use anyhow::{Context, Result};
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn export_as_txt(
    content: &ExportContent,
    output_path: &Path,
) -> Result<()> {
    let mut file = File::create(output_path)
        .with_context(|| format!("无法创建 TXT 文件: {:?}", output_path))?;
    
    writeln!(file, "══════════════════════════════════════════════════════════════")?;
    writeln!(file, "                    {}", content.metadata.title)?;
    writeln!(file, "════════════════════════════════════════════════════════════════")?;
    writeln!(file,)?;
    
    writeln!(file, "作者: {}", content.metadata.author)?;
    writeln!(file, "创建时间: {}", content.metadata.created_at)?;
    
    if let Some(desc) = &content.metadata.description {
        writeln!(file, "简介: {}", desc)?;
    }
    
    writeln!(file,)?;
    writeln!(file, "─────────────────────────────────────────────────────────────────────────────────")?;
    writeln!(file,)?;
    
    for chapter in &content.chapters {
        writeln!(file, "══════════════════════════════════════════════════════════════════")?;
        writeln!(file, "第{}章  {}", chapter.number, chapter.title)?;
        writeln!(file, "════════════════════════════════════════════════════════════════")?;
        writeln!(file,)?;
        writeln!(file, "字数: {}", chapter.content.chars().count())?;
        writeln!(file,)?;
        
        for line in chapter.content.lines() {
            writeln!(file, "{}", line)?;
        }
        
        writeln!(file)?;
        writeln!(file, "─────────────────────────────────────────────────────────────────────────────────")?;
        writeln!(file)?;
    }
    
    writeln!(file, "════════════════════════════════════════════════════════════════")?;
    writeln!(file, "                              完")?;
    writeln!(file, "════════════════════════════════════════════════════════════════")?;
    
    file.flush()
        .with_context(|| "无法刷新文件缓冲区")?;
    
    Ok(())
}
