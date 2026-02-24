// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod models;
mod commands;
mod logger;
mod ai;

use tauri::Manager;
use logger::Logger;
use ai::create_ai_service;
use rusqlite::params;

fn load_api_key_from_db(db_path: &std::path::PathBuf, provider: &str) -> Option<String> {
    let conn = database::get_connection(db_path).ok()?;
    let mut stmt = conn.prepare("SELECT api_key FROM api_keys WHERE provider = ?1 AND is_configured = 1").ok()?;
    let key: Result<String, _> = stmt.query_row(params![provider], |row| row.get(0));
    key.ok()
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let app_logger = Logger::new().with_feature("main");
            app_logger.info("Initializing application");

            let db_path = if cfg!(debug_assertions) {
                let mut project_dir = std::env::current_dir().expect("Failed to get current directory");
                project_dir.push("novel_studio_dev.db");
                app_logger.debug(&format!("Using development database: {:?}", project_dir));
                std::fs::canonicalize(&project_dir).unwrap_or(project_dir)
            } else {
                let app_data_dir = app.path().app_data_dir().expect("Failed to get app data directory");
                app_logger.debug(&format!("App data directory: {:?}", app_data_dir));
                std::fs::create_dir_all(&app_data_dir).expect("Failed to create app data directory");
                app_logger.debug("App data directory created");
                app_data_dir.join("novel_studio.db")
            };

            app_logger.info(&format!("Database path: {:?}", db_path));
            database::init_database(&db_path).expect("Failed to initialize database");
            app_logger.info("Database initialized successfully");

            // 从数据库加载已保存的 API 密钥
            if let Some(saved_key) = load_api_key_from_db(&db_path, "bigmodel") {
                app_logger.info("Found saved BigModel API key, setting environment variable");
                std::env::set_var("BIGMODEL_API_KEY", &saved_key);
            }

            let ai_service = create_ai_service();

            let ai_service_clone = ai_service.clone();
            tauri::async_runtime::spawn(async move {
                let service = ai_service_clone.read().await;
                service.get_registry().initialize_default_bigmodel_models().await;
            });

            app.manage(ai_service);
            app_logger.info("AI service initialized");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::create_project,
            commands::get_projects,
            commands::delete_project,
            commands::update_project,
            commands::save_chapter,
            commands::get_chapters,
            commands::delete_chapter,
            commands::update_chapter,
            commands::create_character,
            commands::get_characters,
            commands::update_character,
            commands::delete_character,
            commands::create_plot_point,
            commands::get_plot_points,
            commands::update_plot_point,
            commands::delete_plot_point,
            commands::create_world_view,
            commands::get_world_views,
            commands::update_world_view,
            commands::delete_world_view,
            commands::create_character_relation,
            commands::get_character_graph,
            commands::update_character_relation,
            commands::delete_character_relation,
            commands::register_openai_model,
            commands::register_ollama_model,
            commands::get_models,
            commands::ai_continue_novel,
            commands::ai_rewrite_content,
            commands::get_prompt_templates,
            commands::save_debug_log,
            commands::save_debug_log_file,
            commands::set_bigmodel_api_key,
            commands::get_bigmodel_api_key,
            commands::get_all_debug_logs,
            commands::save_ui_logs,
            // AI 生成命令
            commands::ai_generate_character,
            commands::ai_generate_character_relations,
            commands::ai_generate_worldview,
            commands::ai_generate_plot_points,
            commands::ai_generate_storyboard,
            commands::ai_format_content,
            // 智能写作助手命令
            commands::generate_writing_choices,
            commands::validate_writing,
            commands::create_plot_node,
            commands::get_plot_tree,
            commands::delete_plot_node,
            // 角色时间线事件命令
            commands::create_character_timeline_event,
            commands::get_character_timeline,
            commands::update_character_timeline_event,
            commands::delete_character_timeline_event,
            // 世界观时间线事件命令
            commands::create_worldview_timeline_event,
            commands::get_worldview_timeline,
            commands::update_worldview_timeline_event,
            commands::delete_worldview_timeline_event,
            // 知识库命令
            commands::create_knowledge_entry,
            commands::get_knowledge_entries,
            commands::get_knowledge_entries_by_type,
            commands::update_knowledge_entry,
            commands::delete_knowledge_entry,
            commands::search_knowledge,
            commands::create_knowledge_relation,
            commands::get_knowledge_relations,
            commands::delete_knowledge_relation,
            commands::build_knowledge_context,
            commands::sync_character_to_knowledge,
            commands::sync_worldview_to_knowledge,
            // 系统设置命令
            commands::get_default_model,
            commands::set_default_model,
            commands::get_ai_params,
            commands::set_ai_params,
            commands::get_api_keys,
            commands::set_api_key,
            commands::get_models_with_default,
            // 多媒体生成命令
            commands::multimedia_generate_storyboard,
            commands::multimedia_generate_script,
            commands::multimedia_generate_comic,
            commands::multimedia_generate_illustration,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
