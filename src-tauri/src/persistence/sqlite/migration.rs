use std::path::Path;

use rusqlite::Connection;

use crate::core::{Error, Result};

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

    if !(path.extension() == Some("db".as_ref())) {
        return Err(Error::InfrastructureError(format!(
            "the file name is not correct.{}",
            path.to_string_lossy().to_string()
        )));
    }

    let conn = Connection::open(path)
        .map_err(|e| Error::InfrastructureError(format!("fail to open connection: {}", e)))?;

    let migrations = [include_str!("migrations/001_create_tables.sql")];

    for sql in migrations {
        conn.execute_batch(sql)
            .map_err(|e| Error::InfrastructureError(format!("migration failed: {}", e)))?;
    }

    Ok(())
}
