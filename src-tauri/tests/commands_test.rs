#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    use tempfile::NamedTempFile;
    use std::fs;

    #[path = "../tests/ai_test.rs"]
    mod ai_test;

    fn create_test_db() -> Connection {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap().to_string();
        crate::database::init_database(std::path::Path::new(&db_path)).unwrap();
        Connection::open(db_path).unwrap()
    }

    #[tokio::test]
    async fn test_create_project() {
        let mut app = tauri::test::mock_app();
        
        let request = CreateProjectRequest {
            name: "Test Project".to_string(),
            description: Some("Test Description".to_string()),
            genre: Some("Fantasy".to_string()),
            template: None,
        };

        let result = create_project(app.handle(), request).await;
        assert!(result.is_ok());
        
        let project = result.unwrap();
        assert_eq!(project.name, "Test Project");
        assert_eq!(project.description, Some("Test Description".to_string()));
        assert_eq!(project.genre, Some("Fantasy".to_string()));
        assert_eq!(project.status, "draft");
    }

    #[tokio::test]
    async fn test_get_projects_empty() {
        let mut app = tauri::test::mock_app();
        
        let result = get_projects(app.handle()).await;
        assert!(result.is_ok());
        
        let projects = result.unwrap();
        assert!(projects.is_empty());
    }

    #[tokio::test]
    async fn test_create_and_get_projects() {
        let mut app = tauri::test::mock_app();
        
        let request = CreateProjectRequest {
            name: "Test Project".to_string(),
            description: None,
            genre: None,
            template: None,
        };

        let _ = create_project(app.handle(), request).await;
        
        let result = get_projects(app.handle()).await;
        assert!(result.is_ok());
        
        let projects = result.unwrap();
        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0].name, "Test Project");
    }

    #[tokio::test]
    async fn test_delete_project() {
        let mut app = tauri::test::mock_app();
        
        let request = CreateProjectRequest {
            name: "To Delete".to_string(),
            description: None,
            genre: None,
            template: None,
        };

        let project = create_project(app.handle(), request).await.unwrap();
        
        let result = delete_project(app.handle(), project.id).await;
        assert!(result.is_ok());
        
        let projects = get_projects(app.handle()).await.unwrap();
        assert!(projects.is_empty());
    }

    #[tokio::test]
    async fn test_update_project() {
        let mut app = tauri::test::mock_app();
        
        let request = CreateProjectRequest {
            name: "Original Name".to_string(),
            description: None,
            genre: None,
            template: None,
        };

        let project = create_project(app.handle(), request).await.unwrap();
        
        let result = update_project(
            app.handle(),
            project.id,
            Some("Updated Name".to_string()),
            None,
            None,
        ).await;
        
        assert!(result.is_ok());
        
        let updated = result.unwrap();
        assert_eq!(updated.name, "Updated Name");
    }

    #[tokio::test]
    async fn test_save_chapter() {
        let mut app = tauri::test::mock_app();
        
        let project_request = CreateProjectRequest {
            name: "Test Project".to_string(),
            description: None,
            genre: None,
            template: None,
        };

        let project = create_project(app.handle(), project_request).await.unwrap();
        
        let chapter_request = SaveChapterRequest {
            project_id: project.id.clone(),
            title: "Chapter 1".to_string(),
            content: "Test content".to_string(),
            sort_order: 0,
        };

        let result = save_chapter(app.handle(), chapter_request).await;
        assert!(result.is_ok());
        
        let chapter = result.unwrap();
        assert_eq!(chapter.title, "Chapter 1");
        assert_eq!(chapter.content, "Test content");
        assert_eq!(chapter.word_count, 12);
    }

    #[tokio::test]
    async fn test_create_character() {
        let mut app = tauri::test::mock_app();
        
        let project_request = CreateProjectRequest {
            name: "Test Project".to_string(),
            description: None,
            genre: None,
            template: None,
        };

        let project = create_project(app.handle(), project_request).await.unwrap();
        
        let character_request = CreateCharacterRequest {
            project_id: project.id.clone(),
            name: "Test Character".to_string(),
            age: Some(25),
            gender: Some("Male".to_string()),
            appearance: Some("Tall".to_string()),
            personality: Some("Brave".to_string()),
            background: Some("Hero".to_string()),
        };

        let result = create_character(app.handle(), character_request).await;
        assert!(result.is_ok());
        
        let character = result.unwrap();
        assert_eq!(character.name, "Test Character");
        assert_eq!(character.age, Some(25));
        assert_eq!(character.gender, Some("Male".to_string()));
    }

    #[tokio::test]
    async fn test_update_character() {
        let mut app = tauri::test::mock_app();
        
        let project_request = CreateProjectRequest {
            name: "Test Project".to_string(),
            description: None,
            genre: None,
            template: None,
        };

        let project = create_project(app.handle(), project_request).await.unwrap();
        
        let character_request = CreateCharacterRequest {
            project_id: project.id.clone(),
            name: "Original Name".to_string(),
            age: None,
            gender: None,
            appearance: None,
            personality: None,
            background: None,
        };

        let character = create_character(app.handle(), character_request).await.unwrap();
        
        let update_data = serde_json::json!({
            "name": "Updated Name"
        });

        let result = update_character(app.handle(), character.id, update_data).await;
        assert!(result.is_ok());
        
        let updated = result.unwrap();
        assert_eq!(updated.name, "Updated Name");
    }

    #[tokio::test]
    async fn test_delete_character() {
        let mut app = tauri::test::mock_app();
        
        let project_request = CreateProjectRequest {
            name: "Test Project".to_string(),
            description: None,
            genre: None,
            template: None,
        };

        let project = create_project(app.handle(), project_request).await.unwrap();
        
        let character_request = CreateCharacterRequest {
            project_id: project.id.clone(),
            name: "To Delete".to_string(),
            age: None,
            gender: None,
            appearance: None,
            personality: None,
            background: None,
        };

        let character = create_character(app.handle(), character_request).await.unwrap();
        
        let result = delete_character(app.handle(), character.id).await;
        assert!(result.is_ok());
        
        let characters = get_characters(app.handle(), project.id).await.unwrap();
        assert!(characters.is_empty());
    }
}
