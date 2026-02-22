use tauri::{Manager, test::mock_context};
use ai_novel_studio::commands;
use ai_novel_studio::models::*;
use tests::common::*;

#[test]
fn test_create_project_success() {
    let mut suite = TestSuite::new("Project Management - Create Project");

    suite.add_test("create project with valid data", || {
        let app = mock_context();
        let request = CreateProjectRequest {
            name: "Test Project".to_string(),
            description: Some("A test project".to_string()),
            genre: "fantasy".to_string(),
            template: None,
            status: None,
        };

        let result = commands::create_project(app.handle(), request);
        
        match result {
            Ok(project) => {
                assert_eq!("create_project_name", project.name, "Test Project");
                assert_eq!("create_project_description", project.description, Some("A test project".to_string()));
                assert_eq!("create_project_genre", project.genre, "fantasy");
                assert_some!("create_project_id", project.id);
            }
            Err(e) => {
                return assert_eq!("create_project_error", 
                    "Expected success", 
                    format!("Error: {}", e));
            }
        }
        
        assert_true!("create_project_success", result.is_ok())
    });

    suite.add_test("create project with minimal data", || {
        let app = mock_context();
        let request = CreateProjectRequest {
            name: "Minimal Project".to_string(),
            description: None,
            genre: "scifi".to_string(),
            template: None,
            status: None,
        };

        let result = commands::create_project(app.handle(), request);
        
        assert_true!("minimal_project_success", result.is_ok())
    });

    suite.print_summary();
    assert!(suite.is_all_passed(), "Project creation tests failed");
}

#[test]
fn test_get_projects() {
    let mut suite = TestSuite::new("Project Management - Get Projects");

    suite.add_test("get projects list", || {
        let app = mock_context();
        let projects = commands::get_projects(app.handle());
        
        assert_true!("get_projects_success", projects.is_ok());
        assert_true!("get_projects_is_list", projects.unwrap().len() >= 0);
    });

    suite.print_summary();
    assert!(suite.is_all_passed(), "Get projects tests failed");
}

#[test]
fn test_delete_project() {
    let mut suite = TestSuite::new("Project Management - Delete Project");

    suite.add_test("delete project after creation", || {
        let app = mock_context();
        
        let create_request = CreateProjectRequest {
            name: "To Delete".to_string(),
            description: None,
            genre: "urban".to_string(),
            template: None,
            status: None,
        };

        let project = commands::create_project(app.handle(), create_request).unwrap();
        let result = commands::delete_project(app.handle(), project.id);
        
        assert_true!("delete_project_success", result.is_ok());
    });

    suite.print_summary();
    assert!(suite.is_all_passed(), "Delete project tests failed");
}

#[test]
fn test_update_project() {
    let mut suite = TestSuite::new("Project Management - Update Project");

    suite.add_test("update project name and description", || {
        let app = mock_context();
        
        let create_request = CreateProjectRequest {
            name: "Original Name".to_string(),
            description: Some("Original Description".to_string()),
            genre: "fantasy".to_string(),
            template: None,
            status: None,
        };

        let project = commands::create_project(app.handle(), create_request).unwrap();
        
        let update_request = UpdateProjectRequest {
            id: project.id,
            name: Some("Updated Name".to_string()),
            description: Some("Updated Description".to_string()),
            genre: None,
            template: None,
            status: None,
        };

        let result = commands::update_project(app.handle(), update_request);
        
        assert_true!("update_project_success", result.is_ok());
        
        let updated = result.unwrap();
        assert_eq!("updated_name", updated.name, "Updated Name");
        assert_eq!("updated_description", updated.description, Some("Updated Description".to_string()));
    });

    suite.print_summary();
    assert!(suite.is_all_passed(), "Update project tests failed");
}

#[test]
fn test_project_validation() {
    let mut suite = TestSuite::new("Project Management - Validation");

    suite.add_test("reject empty project name", || {
        let app = mock_context();
        let request = CreateProjectRequest {
            name: "".to_string(),
            description: None,
            genre: "fantasy".to_string(),
            template: None,
            status: None,
        };

        let result = commands::create_project(app.handle(), request);
        
        assert_false!("empty_name_rejected", result.is_ok());
    });

    suite.add_test("reject project name too long", || {
        let app = mock_context();
        let long_name = "A".repeat(1000);
        let request = CreateProjectRequest {
            name: long_name,
            description: None,
            genre: "fantasy".to_string(),
            template: None,
            status: None,
        };

        let result = commands::create_project(app.handle(), request);
        
        assert_false!("long_name_rejected", result.is_ok());
    });

    suite.print_summary();
    assert!(suite.is_all_passed(), "Project validation tests failed");
}
