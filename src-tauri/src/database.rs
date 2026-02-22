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
