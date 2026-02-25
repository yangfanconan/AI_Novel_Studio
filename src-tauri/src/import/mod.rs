pub mod txt_import;
pub mod md_import;
pub mod docx_import;

pub use txt_import::import_from_txt;
pub use md_import::import_from_markdown;
pub use docx_import::import_from_docx;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImportFormat {
    Txt,
    Md,
    Docx,
}

impl ImportFormat {
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "txt" => Some(ImportFormat::Txt),
            "md" | "markdown" => Some(ImportFormat::Md),
            "docx" => Some(ImportFormat::Docx),
            _ => None,
        }
    }
    
    pub fn extension(&self) -> &str {
        match self {
            ImportFormat::Txt => "txt",
            ImportFormat::Md => "md",
            ImportFormat::Docx => "docx",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportResult {
    pub success: bool,
    pub title: String,
    pub content: String,
    pub chapter_count: usize,
    pub word_count: usize,
    pub chapters: Vec<ImportedChapter>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImportedChapter {
    pub title: String,
    pub content: String,
    pub word_count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportRequest {
    pub file_path: String,
    pub format: ImportFormat,
    pub project_id: Option<String>,
}
