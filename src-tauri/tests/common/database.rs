use std::path::PathBuf;
use tempfile::TempDir;

pub struct TestDatabase {
    pub temp_dir: TempDir,
    pub db_path: PathBuf,
}

impl TestDatabase {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let db_path = temp_dir.path().join("test.db");
        
        let conn = rusqlite::Connection::open(&db_path)?;
        crate::database::initialize_database(&conn)?;
        
        Ok(TestDatabase {
            temp_dir,
            db_path,
        })
    }

    pub fn get_connection(&self) -> Result<rusqlite::Connection, Box<dyn std::error::Error>> {
        Ok(rusqlite::Connection::open(&self.db_path)?)
    }
}

impl Drop for TestDatabase {
    fn drop(&mut self) {
        let _ = self.temp_dir.close();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_creation() {
        let db = TestDatabase::new().unwrap();
        assert!(db.db_path.exists());
    }

    #[test]
    fn test_database_connection() {
        let db = TestDatabase::new().unwrap();
        let conn = db.get_connection().unwrap();
        
        conn.execute("SELECT 1", []).unwrap();
    }
}
