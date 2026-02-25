use crate::writing_tools::WritingTools;
use crate::logger::Logger;
use serde_json;

#[tauri::command]
pub async fn detect_sensitive_words(
    text: String,
) -> Result<String, String> {
    let logger = Logger::new().with_feature("writing_tools");
    logger.info("Detecting sensitive words");

    let detection = WritingTools::detect_sensitive_words(&text);
    serde_json::to_string(&detection).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn detect_typos(
    text: String,
) -> Result<String, String> {
    let logger = Logger::new().with_feature("writing_tools");
    logger.info("Detecting typos");

    let detection = WritingTools::detect_typos(&text);
    serde_json::to_string(&detection).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn check_grammar(
    text: String,
) -> Result<String, String> {
    let logger = Logger::new().with_feature("writing_tools");
    logger.info("Checking grammar");

    let check = WritingTools::check_grammar(&text);
    serde_json::to_string(&check).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn normalize_format(
    text: String,
) -> Result<String, String> {
    let logger = Logger::new().with_feature("writing_tools");
    logger.info("Normalizing format");

    let normalized = WritingTools::normalize_format(&text);
    serde_json::to_string(&normalized).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn run_full_writing_tools(
    text: String,
) -> Result<String, String> {
    let logger = Logger::new().with_feature("writing_tools");
    logger.info("Running full writing tools analysis");

    let sensitive_words = WritingTools::detect_sensitive_words(&text);
    let typos = WritingTools::detect_typos(&text);
    let grammar = WritingTools::check_grammar(&text);
    let format = WritingTools::normalize_format(&text);

    let full_analysis = serde_json::json!({
        "sensitive_words": sensitive_words,
        "typos": typos,
        "grammar": grammar,
        "format": format,
    });

    serde_json::to_string(&full_analysis).map_err(|e| e.to_string())
}
