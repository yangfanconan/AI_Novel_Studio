pub mod database;
pub mod logger;
pub mod assertions;

pub use database::*;
pub use logger::*;
pub use assertions::*;

use tauri::test::mock_context;
use ai_novel_studio::commands;
use ai_novel_studio::models::*;

pub fn create_test_project(app: &tauri::AppHandle) -> Project {
    let request = CreateProjectRequest {
        name: "Test Project".to_string(),
        description: Some("Test description".to_string()),
        genre: Some("fantasy".to_string()),
        template: None,
    };
    commands::create_project(app.clone(), request).unwrap()
}
