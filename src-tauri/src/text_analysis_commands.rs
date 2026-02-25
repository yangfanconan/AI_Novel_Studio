use crate::text_analysis::TextAnalyzer;
use crate::models::Character;
use crate::logger::Logger;
use serde_json;

#[tauri::command]
pub async fn analyze_writing_style(
    text: String,
) -> Result<String, String> {
    let logger = Logger::new().with_feature("text_analysis");
    logger.info("Analyzing writing style");

    let analysis = TextAnalyzer::analyze_writing_style(&text);
    serde_json::to_string(&analysis).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn analyze_rhythm(
    text: String,
) -> Result<String, String> {
    let logger = Logger::new().with_feature("text_analysis");
    logger.info("Analyzing text rhythm");

    let analysis = TextAnalyzer::analyze_rhythm(&text);
    serde_json::to_string(&analysis).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn analyze_emotion(
    text: String,
) -> Result<String, String> {
    let logger = Logger::new().with_feature("text_analysis");
    logger.info("Analyzing emotion");

    let analysis = TextAnalyzer::analyze_emotion(&text);
    serde_json::to_string(&analysis).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn analyze_readability(
    text: String,
) -> Result<String, String> {
    let logger = Logger::new().with_feature("text_analysis");
    logger.info("Analyzing readability");

    let analysis = TextAnalyzer::analyze_readability(&text);
    serde_json::to_string(&analysis).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn detect_repetitions(
    text: String,
    min_repetitions: i32,
) -> Result<String, String> {
    let logger = Logger::new().with_feature("text_analysis");
    logger.info("Detecting repetitions");

    let analysis = TextAnalyzer::detect_repetitions(&text, min_repetitions as usize);
    serde_json::to_string(&analysis).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn check_logic(
    text: String,
    characters_json: String,
) -> Result<String, String> {
    let logger = Logger::new().with_feature("text_analysis");
    logger.info("Checking logic");

    let characters: Vec<Character> = serde_json::from_str(&characters_json)
        .map_err(|e| format!("Failed to parse characters: {}", e))?;

    let analysis = TextAnalyzer::check_logic(&text, &characters);
    serde_json::to_string(&analysis).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn run_full_analysis(
    text: String,
    characters_json: Option<String>,
) -> Result<String, String> {
    let logger = Logger::new().with_feature("text_analysis");
    logger.info("Running full text analysis");

    let characters = if let Some(json) = characters_json {
        serde_json::from_str(&json)
            .map_err(|e| format!("Failed to parse characters: {}", e))?
    } else {
        Vec::new()
    };

    let writing_style = TextAnalyzer::analyze_writing_style(&text);
    let rhythm = TextAnalyzer::analyze_rhythm(&text);
    let emotion = TextAnalyzer::analyze_emotion(&text);
    let readability = TextAnalyzer::analyze_readability(&text);
    let repetitions = TextAnalyzer::detect_repetitions(&text, 3);
    let logic = TextAnalyzer::check_logic(&text, &characters);

    let full_analysis = serde_json::json!({
        "writing_style": writing_style,
        "rhythm": rhythm,
        "emotion": emotion,
        "readability": readability,
        "repetitions": repetitions,
        "logic": logic,
    });

    serde_json::to_string(&full_analysis).map_err(|e| e.to_string())
}
