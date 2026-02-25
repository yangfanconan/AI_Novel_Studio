pub mod docx_export;
pub mod pdf_export;
pub mod epub_export;
pub mod txt_export;
pub mod md_export;

pub use docx_export::export_as_docx;
pub use pdf_export::export_as_pdf;
pub use epub_export::export_as_epub;
pub use txt_export::export_as_txt;
pub use md_export::export_as_md;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportMetadata {
    pub title: String,
    pub author: String,
    pub description: Option<String>,
    pub created_at: String,
    pub word_count: usize,
    pub chapter_count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportContent {
    pub metadata: ExportMetadata,
    pub chapters: Vec<ChapterContent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChapterContent {
    pub id: String,
    pub title: String,
    pub number: usize,
    pub content: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Docx,
    Pdf,
    Epub,
    Txt,
    Md,
}

impl ExportFormat {
    pub fn extension(&self) -> &str {
        match self {
            ExportFormat::Docx => ".docx",
            ExportFormat::Pdf => ".pdf",
            ExportFormat::Epub => ".epub",
            ExportFormat::Txt => ".txt",
            ExportFormat::Md => ".md",
        }
    }

    pub fn mime_type(&self) -> &str {
        match self {
            ExportFormat::Docx => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            ExportFormat::Pdf => "application/pdf",
            ExportFormat::Epub => "application/epub+zip",
            ExportFormat::Txt => "text/plain",
            ExportFormat::Md => "text/markdown",
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            ExportFormat::Docx => "Word文档 (.docx)",
            ExportFormat::Pdf => "PDF文档 (.pdf)",
            ExportFormat::Epub => "EPUB电子书 (.epub)",
            ExportFormat::Txt => "纯文本 (.txt)",
            ExportFormat::Md => "Markdown文档 (.md)",
        }
    }
}
