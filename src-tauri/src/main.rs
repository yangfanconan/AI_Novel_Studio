// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod models;
mod commands;
mod logger;
mod ai;
mod export;
mod plugin_commands;
mod plugin_marketplace_commands;
mod cloud_sync_commands;
mod cloud_sync;
mod multimedia_generation;
mod multimedia_generation_commands;
mod collaboration;
mod collaboration_commands;
mod text_analysis;
mod text_analysis_commands;
mod writing_tools;
mod writing_tools_commands;
mod version_control;
mod version_control_commands;
mod character_growth;
mod character_tags;
mod character_growth_commands;
mod character_dialogue;
mod character_dialogue_commands;

use tauri::Manager;
use logger::Logger;
use ai::create_ai_service;
use plugin_commands::PluginManagerState;
use plugin_marketplace_commands::MarketplaceState;
use cloud_sync_commands::CloudSyncState;
use multimedia_generation_commands::MultimediaState;
use collaboration_commands::CollaborationState;
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

            let plugin_manager_state = PluginManagerState::new();
            plugin_manager_state.initialize()
                .expect("Failed to initialize plugin manager state");
            app.manage(plugin_manager_state);

            app_logger.info("Plugin manager initialized");

            let marketplace_state = MarketplaceState::new();
            app.manage(marketplace_state);
            app_logger.info("Plugin marketplace initialized");

            let cloud_sync_state = CloudSyncState::new();
            app.manage(cloud_sync_state);
            app_logger.info("Cloud sync initialized");

            let api_key = std::env::var("OPENAI_API_KEY").unwrap_or_default();
            let multimedia_state = MultimediaState::new(api_key);
            app.manage(multimedia_state);
            app_logger.info("Multimedia generation initialized");

            let collab_state = CollaborationState::new();
            app.manage(collab_state);
            app_logger.info("Collaboration initialized");

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
            // 导出命令
            commands::export_project,
            commands::export_chapter,
            commands::get_export_formats,
            // 插件系统命令
            plugin_commands::plugin_get_all,
            plugin_commands::plugin_get,
            plugin_commands::plugin_activate,
            plugin_commands::plugin_deactivate,
            plugin_commands::plugin_install,
            plugin_commands::plugin_uninstall,
            plugin_commands::plugin_get_permissions,
            plugin_commands::plugin_grant_permission,
            plugin_commands::plugin_revoke_permission,
            plugin_commands::plugin_get_settings,
            plugin_commands::plugin_update_settings,
            plugin_commands::plugin_get_commands,
            plugin_commands::plugin_search,
            plugin_commands::plugin_get_resource_usage,
            // 插件市场命令
            plugin_marketplace_commands::marketplace_search_plugins,
            plugin_marketplace_commands::marketplace_get_plugin,
            plugin_marketplace_commands::marketplace_get_plugin_manifest,
            plugin_marketplace_commands::marketplace_get_categories,
            plugin_marketplace_commands::marketplace_get_featured,
            plugin_marketplace_commands::marketplace_get_reviews,
            plugin_marketplace_commands::marketplace_submit_review,
            plugin_marketplace_commands::marketplace_report_plugin,
            // 云端同步命令
            cloud_sync_commands::cloud_sync_configure,
            cloud_sync_commands::cloud_sync_get_config,
            cloud_sync_commands::cloud_sync_authenticate,
            cloud_sync_commands::cloud_sync_start,
            cloud_sync_commands::cloud_sync_get_status,
            cloud_sync_commands::cloud_sync_start_auto,
            cloud_sync_commands::cloud_sync_stop_auto,
            cloud_sync_commands::cloud_sync_resolve_conflict,
            // 协作编辑命令
            collaboration_commands::collab_create_session,
            collaboration_commands::collab_join_session,
            collaboration_commands::collab_leave_session,
            collaboration_commands::collab_broadcast_operation,
            collaboration_commands::collab_update_cursor,
            collaboration_commands::collab_get_session,
            collaboration_commands::collab_get_user_cursors,
            collaboration_commands::collab_generate_user_id,
            collaboration_commands::collab_generate_color,
            // 文本分析命令
            text_analysis_commands::analyze_writing_style,
            text_analysis_commands::analyze_rhythm,
            text_analysis_commands::analyze_emotion,
            text_analysis_commands::analyze_readability,
            text_analysis_commands::detect_repetitions,
            text_analysis_commands::check_logic,
            text_analysis_commands::run_full_analysis,
            // 写作工具命令
            writing_tools_commands::detect_sensitive_words,
            writing_tools_commands::detect_typos,
            writing_tools_commands::check_grammar,
            writing_tools_commands::normalize_format,
            writing_tools_commands::run_full_writing_tools,
            // 版本控制命令
            version_control_commands::create_snapshot,
            version_control_commands::get_snapshots,
            version_control_commands::get_snapshot,
            version_control_commands::restore_snapshot,
            version_control_commands::delete_snapshot,
            version_control_commands::compare_snapshots,
            version_control_commands::get_version_config,
            version_control_commands::set_version_config,
            // 角色成长和标签命令
            character_growth_commands::create_growth_record,
            character_growth_commands::get_growth_timeline,
            character_growth_commands::compare_growth_positions,
            character_growth_commands::create_character_tag,
            character_growth_commands::get_character_tags,
            character_growth_commands::delete_character_tag,
            character_growth_commands::search_tags,
            character_growth_commands::get_tag_library,
            character_growth_commands::get_tag_statistics,
            // 角色对话命令
            character_dialogue_commands::create_dialogue_session,
            character_dialogue_commands::get_dialogue_sessions,
            character_dialogue_commands::get_dialogue_session,
            character_dialogue_commands::send_dialogue_message,
            character_dialogue_commands::update_dialogue_session,
            character_dialogue_commands::delete_dialogue_session,
            character_dialogue_commands::delete_dialogue_message,
            character_dialogue_commands::regenerate_ai_response,
            // 多媒体生成命令
            multimedia_generation_commands::mmg_extract_scenes,
            multimedia_generation_commands::mmg_generate_storyboard,
            multimedia_generation_commands::mmg_convert_to_script,
            multimedia_generation_commands::mmg_optimize_script,
            multimedia_generation_commands::mmg_generate_comic,
            multimedia_generation_commands::mmg_generate_scene_illustration,
            multimedia_generation_commands::mmg_generate_character_portrait,
            multimedia_generation_commands::mmg_generate_cover,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
