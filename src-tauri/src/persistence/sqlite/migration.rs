use std::path::Path;

use rusqlite::Connection;

use crate::core::{Error, Result};

/// Migrates the SQLite database at the given path to the latest schema version.
pub fn migrate(db_path: impl AsRef<str>) -> Result<()> {
    let path = Path::new(db_path.as_ref());

    if path.exists() {
        if !path.is_file() {
            return Err(Error::InfrastructureError(format!(
                "the file is not a file."
            )));
        }

        let meta = path
            .metadata()
            .map_err(|e| Error::InfrastructureError(format!("fail to get the meta data: {}", e)))?;

        if meta.permissions().readonly() {
            return Err(Error::InfrastructureError(format!(
                "the file is not writable."
            )));
        }
    }

    // Allow only .db files
    if !(path.extension() == Some("db".as_ref())) {
        return Err(Error::InfrastructureError(format!(
            "the file name is not correct.{}",
            path.to_string_lossy().to_string()
        )));
    }

    let conn = Connection::open(path)
        .map_err(|e| Error::InfrastructureError(format!("fail to open connection: {}", e)))?;

    let version = conn
        .query_row("PRAGMA user_version", [], |row| row.get::<_, i64>(0))
        .map_err(|e| Error::InfrastructureError(format!("fail to get migration version: {}", e)))?;

    // Apply migrations
    // Note: Each migration should be idempotent.
    (|| -> rusqlite::Result<(), rusqlite::Error> {
        if version < 1 {
            conn.execute_batch(include_str!("migrations/001_create_tables.sql"))?;
        }

        if version < 2 {
            conn.execute_batch(include_str!("migrations/002_add_deleted_at_column.sql"))?;
        }

        Ok(())
    })()
    .map_err(|e| Error::InfrastructureError(format!("migration failed: {}", e)))?;

    Ok(())
}
