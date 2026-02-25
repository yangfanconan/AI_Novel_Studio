use rusqlite::{Connection, Result as SqlResult};
use std::path::Path;

pub fn init_database(db_path: &Path) -> SqlResult<()> {
    let conn = Connection::open(db_path)?;

    // 创建项目表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS projects (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT,
            genre TEXT,
            template TEXT,
            status TEXT DEFAULT 'draft',
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        [],
    )?;

    // 创建章节表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS chapters (
            id TEXT PRIMARY KEY,
            project_id TEXT NOT NULL,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            word_count INTEGER DEFAULT 0,
            sort_order INTEGER DEFAULT 0,
            status TEXT DEFAULT 'draft',
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // 创建角色表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS characters (
            id TEXT PRIMARY KEY,
            project_id TEXT NOT NULL,
            name TEXT NOT NULL,
            role_type TEXT,
            race TEXT,
            age INTEGER,
            gender TEXT,
            birth_date TEXT,
            appearance TEXT,
            personality TEXT,
            background TEXT,
            skills TEXT,
            status TEXT,
            bazi TEXT,
            ziwei TEXT,
            mbti TEXT,
            enneagram TEXT,
            items TEXT,
            avatar_url TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // 创建情节表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS plot_points (
            id TEXT PRIMARY KEY,
            project_id TEXT NOT NULL,
            parent_id TEXT,
            title TEXT NOT NULL,
            description TEXT,
            note TEXT,
            chapter_id TEXT,
            status TEXT DEFAULT 'draft',
            sort_order INTEGER DEFAULT 0,
            level INTEGER DEFAULT 0,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
            FOREIGN KEY (parent_id) REFERENCES plot_points(id) ON DELETE CASCADE,
            FOREIGN KEY (chapter_id) REFERENCES chapters(id) ON DELETE SET NULL
        )",
        [],
    )?;

    // 创建世界观表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS world_views (
            id TEXT PRIMARY KEY,
            project_id TEXT NOT NULL,
            category TEXT NOT NULL,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            tags TEXT,
            status TEXT DEFAULT 'draft',
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // 创建角色关系表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS character_relations (
            id TEXT PRIMARY KEY,
            project_id TEXT NOT NULL,
            from_character_id TEXT NOT NULL,
            to_character_id TEXT NOT NULL,
            relation_type TEXT NOT NULL,
            description TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
            FOREIGN KEY (from_character_id) REFERENCES characters(id) ON DELETE CASCADE,
            FOREIGN KEY (to_character_id) REFERENCES characters(id) ON DELETE CASCADE,
            UNIQUE(from_character_id, to_character_id, relation_type)
        )",
        [],
    )?;

    // 创建索引
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_chapters_project ON chapters(project_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_characters_project ON characters(project_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_plot_points_project ON plot_points(project_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_plot_points_parent ON plot_points(parent_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_plot_points_chapter ON plot_points(chapter_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_world_views_project ON world_views(project_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_world_views_category ON world_views(category)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_character_relations_project ON character_relations(project_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_character_relations_from ON character_relations(from_character_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_character_relations_to ON character_relations(to_character_id)",
        [],
    )?;

    // 创建应用配置表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS app_settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        [],
    )?;

    // 创建 API 密钥表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS api_keys (
            provider TEXT PRIMARY KEY,
            api_key TEXT NOT NULL,
            is_configured INTEGER DEFAULT 1,
            updated_at TEXT NOT NULL
        )",
        [],
    )?;

    // 创建角色时间线事件表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS character_timeline_events (
            id TEXT PRIMARY KEY,
            character_id TEXT NOT NULL,
            event_type TEXT NOT NULL,
            event_title TEXT NOT NULL,
            event_description TEXT,
            story_time TEXT,
            real_chapter_id TEXT,
            emotional_state TEXT,
            state_changes TEXT,
            sort_order INTEGER DEFAULT 0,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE,
            FOREIGN KEY (real_chapter_id) REFERENCES chapters(id) ON DELETE SET NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_character_timeline_character ON character_timeline_events(character_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_character_timeline_chapter ON character_timeline_events(real_chapter_id)",
        [],
    )?;

    // 创建世界观时间线事件表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS worldview_timeline_events (
            id TEXT PRIMARY KEY,
            worldview_id TEXT NOT NULL,
            event_type TEXT NOT NULL,
            event_title TEXT NOT NULL,
            event_description TEXT,
            story_time TEXT,
            impact_scope TEXT,
            related_characters TEXT,
            sort_order INTEGER DEFAULT 0,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (worldview_id) REFERENCES world_views(id) ON DELETE CASCADE
        )",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_worldview_timeline_worldview ON worldview_timeline_events(worldview_id)",
        [],
    )?;

    // 创建剧情节点表（用于Galgame风格的剧情树）
    conn.execute(
        "CREATE TABLE IF NOT EXISTS plot_nodes (
            id TEXT PRIMARY KEY,
            project_id TEXT NOT NULL,
            chapter_id TEXT,
            parent_node_id TEXT,
            title TEXT NOT NULL,
            summary TEXT,
            content TEXT,
            choice_made TEXT,
            characters_involved TEXT,
            location TEXT,
            emotional_tone TEXT,
            word_count INTEGER DEFAULT 0,
            is_main_path INTEGER DEFAULT 1,
            branch_name TEXT,
            sort_order INTEGER DEFAULT 0,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
            FOREIGN KEY (chapter_id) REFERENCES chapters(id) ON DELETE SET NULL,
            FOREIGN KEY (parent_node_id) REFERENCES plot_nodes(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // 创建剧情节点索引
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_plot_nodes_project ON plot_nodes(project_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_plot_nodes_chapter ON plot_nodes(chapter_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_plot_nodes_parent ON plot_nodes(parent_node_id)",
        [],
    )?;

    // 创建知识库条目表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS knowledge_entries (
            id TEXT PRIMARY KEY,
            project_id TEXT NOT NULL,
            entry_type TEXT NOT NULL,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            source_type TEXT NOT NULL,
            source_id TEXT,
            keywords TEXT,
            importance INTEGER DEFAULT 0,
            is_verified INTEGER DEFAULT 0,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
        )",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_knowledge_entries_project ON knowledge_entries(project_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_knowledge_entries_type ON knowledge_entries(entry_type)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_knowledge_entries_source ON knowledge_entries(source_type, source_id)",
        [],
    )?;

    // 创建知识库关系表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS knowledge_relations (
            id TEXT PRIMARY KEY,
            project_id TEXT NOT NULL,
            from_entry_id TEXT NOT NULL,
            to_entry_id TEXT NOT NULL,
            relation_type TEXT NOT NULL,
            description TEXT,
            strength INTEGER DEFAULT 1,
            created_at TEXT NOT NULL,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
            FOREIGN KEY (from_entry_id) REFERENCES knowledge_entries(id) ON DELETE CASCADE,
            FOREIGN KEY (to_entry_id) REFERENCES knowledge_entries(id) ON DELETE CASCADE
        )",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_knowledge_relations_project ON knowledge_relations(project_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_knowledge_relations_from ON knowledge_relations(from_entry_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_knowledge_relations_to ON knowledge_relations(to_entry_id)",
        [],
    )?;

    // 创建项目快照表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS project_snapshots (
            id TEXT PRIMARY KEY,
            project_id TEXT NOT NULL,
            version TEXT NOT NULL,
            timestamp INTEGER NOT NULL,
            description TEXT,
            chapters_json TEXT NOT NULL,
            characters_json TEXT NOT NULL,
            world_views_json TEXT NOT NULL,
            plot_points_json TEXT NOT NULL,
            metadata_json TEXT NOT NULL,
            auto_generated INTEGER DEFAULT 0,
            created_at TEXT NOT NULL,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
        )",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_project_snapshots_project ON project_snapshots(project_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_project_snapshots_timestamp ON project_snapshots(timestamp)",
        [],
    )?;

    // 创建版本差异表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS version_diffs (
            id TEXT PRIMARY KEY,
            project_id TEXT NOT NULL,
            from_version TEXT NOT NULL,
            to_version TEXT NOT NULL,
            timestamp INTEGER NOT NULL,
            chapter_changes_json TEXT NOT NULL,
            character_changes_json TEXT NOT NULL,
            world_view_changes_json TEXT NOT NULL,
            plot_point_changes_json TEXT NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
        )",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_version_diffs_project ON version_diffs(project_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_version_diffs_timestamp ON version_diffs(timestamp)",
        [],
    )?;

    // 创建角色成长轨迹表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS character_growth_records (
            id TEXT PRIMARY KEY,
            character_id TEXT NOT NULL,
            chapter_id TEXT NOT NULL,
            position INTEGER NOT NULL,
            changes_json TEXT NOT NULL,
            auto_detected INTEGER DEFAULT 0,
            notes TEXT,
            created_at TEXT NOT NULL,
            FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE,
            FOREIGN KEY (chapter_id) REFERENCES chapters(id) ON DELETE CASCADE
        )",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_character_growth_character ON character_growth_records(character_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_character_growth_chapter ON character_growth_records(chapter_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_character_growth_position ON character_growth_records(position)",
        [],
    )?;

    // 创建角色标签表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS character_tags (
            id TEXT PRIMARY KEY,
            character_id TEXT NOT NULL,
            tag_type TEXT NOT NULL,
            name TEXT NOT NULL,
            value TEXT,
            description TEXT,
            color TEXT NOT NULL,
            weight TEXT NOT NULL,
            auto_assigned INTEGER DEFAULT 0,
            source TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE
        )",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_character_tags_character ON character_tags(character_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_character_tags_type ON character_tags(tag_type)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_character_tags_name ON character_tags(name)",
        [],
    )?;

    // 创建版本控制配置表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS version_control_config (
            id TEXT PRIMARY KEY DEFAULT 'config',
            auto_save_enabled INTEGER DEFAULT 1,
            auto_save_interval_minutes INTEGER DEFAULT 30,
            max_snapshots_per_project INTEGER DEFAULT 50,
            compression_enabled INTEGER DEFAULT 1,
            updated_at TEXT NOT NULL
        )",
        [],
    )?;

    // 创建角色对话会话表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS character_dialogue_sessions (
            id TEXT PRIMARY KEY,
            character_id TEXT NOT NULL,
            chapter_id TEXT,
            session_name TEXT NOT NULL,
            system_prompt TEXT,
            context_summary TEXT,
            ai_model TEXT DEFAULT 'default',
            temperature REAL DEFAULT 0.7,
            max_tokens INTEGER DEFAULT 1000,
            is_active INTEGER DEFAULT 1,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE,
            FOREIGN KEY (chapter_id) REFERENCES chapters(id) ON DELETE SET NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_character_dialogue_sessions_character ON character_dialogue_sessions(character_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_character_dialogue_sessions_chapter ON character_dialogue_sessions(chapter_id)",
        [],
    )?;

    // 创建角色对话消息表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS character_dialogue_messages (
            id TEXT PRIMARY KEY,
            session_id TEXT NOT NULL,
            role TEXT NOT NULL,
            content TEXT NOT NULL,
            message_type TEXT NOT NULL,
            character_state_json TEXT,
            emotional_context TEXT,
            scene_context TEXT,
            tokens_used INTEGER DEFAULT 0,
            created_at TEXT NOT NULL,
            FOREIGN KEY (session_id) REFERENCES character_dialogue_sessions(id) ON DELETE CASCADE
        )",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_character_dialogue_messages_session ON character_dialogue_messages(session_id)",
        [],
    )?;

    // 提示词模板表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS prompt_templates (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            category TEXT NOT NULL,
            description TEXT,
            system_prompt TEXT NOT NULL,
            user_prompt_template TEXT NOT NULL,
            variables TEXT,
            is_default INTEGER DEFAULT 0,
            is_custom INTEGER DEFAULT 1,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_prompt_templates_category ON prompt_templates(category)",
        [],
    )?;

    // 角色圣经表 (Character Bible - 用于AI影视生成的角色一致性)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS character_bibles (
            id TEXT PRIMARY KEY,
            project_id TEXT NOT NULL,
            name TEXT NOT NULL,
            type TEXT NOT NULL,
            visual_traits TEXT,
            style_tokens TEXT,
            color_palette TEXT,
            personality TEXT,
            reference_images TEXT,
            three_view_images TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
        )",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_character_bibles_project ON character_bibles(project_id)",
        [],
    )?;

    // AI任务队列表 (用于批量生成任务管理)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS ai_task_queue (
            id TEXT PRIMARY KEY,
            project_id TEXT NOT NULL,
            task_type TEXT NOT NULL,
            priority INTEGER DEFAULT 5,
            state TEXT NOT NULL DEFAULT 'pending',
            provider TEXT,
            input_data TEXT NOT NULL,
            output_data TEXT,
            error_message TEXT,
            retry_count INTEGER DEFAULT 0,
            max_retries INTEGER DEFAULT 3,
            progress INTEGER DEFAULT 0,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            started_at TEXT,
            completed_at TEXT,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
        )",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_ai_task_queue_project ON ai_task_queue(project_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_ai_task_queue_state ON ai_task_queue(state)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_ai_task_queue_type ON ai_task_queue(task_type)",
        [],
    )?;

    // 剧本场景表 (用于AI影视场景解析)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS script_scenes (
            id TEXT PRIMARY KEY,
            project_id TEXT NOT NULL,
            chapter_id TEXT,
            scene_index INTEGER NOT NULL,
            narration TEXT,
            visual_content TEXT,
            action TEXT,
            camera TEXT,
            character_description TEXT,
            generated_image_url TEXT,
            generated_video_url TEXT,
            status TEXT DEFAULT 'pending',
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
            FOREIGN KEY (chapter_id) REFERENCES chapters(id) ON DELETE SET NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_script_scenes_project ON script_scenes(project_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_script_scenes_chapter ON script_scenes(chapter_id)",
        [],
    )?;

    // 数据库迁移：为 characters 表添加新列（如果不存在）
    let migrations = vec![
        "ALTER TABLE characters ADD COLUMN role_type TEXT",
        "ALTER TABLE characters ADD COLUMN race TEXT",
        "ALTER TABLE characters ADD COLUMN birth_date TEXT",
        "ALTER TABLE characters ADD COLUMN skills TEXT",
        "ALTER TABLE characters ADD COLUMN status TEXT",
        "ALTER TABLE characters ADD COLUMN bazi TEXT",
        "ALTER TABLE characters ADD COLUMN ziwei TEXT",
        "ALTER TABLE characters ADD COLUMN mbti TEXT",
        "ALTER TABLE characters ADD COLUMN enneagram TEXT",
        "ALTER TABLE characters ADD COLUMN items TEXT",
    ];

    for migration in migrations {
        let _ = conn.execute(migration, []);
    }

    Ok(())
}

pub fn get_connection(db_path: &Path) -> SqlResult<Connection> {
    Connection::open_with_flags(
        db_path,
        rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE | rusqlite::OpenFlags::SQLITE_OPEN_CREATE
    )
}
