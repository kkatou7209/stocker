use std::path::Path;

use rusqlite::named_params;
use rusqlite::params_from_iter;
use rusqlite::Connection;
use rusqlite::OptionalExtension;

use crate::core::domain::entities::stock::*;
use crate::core::domain::values::stock::*;
use crate::core::required_ports;
use crate::core::required_ports::*;
use crate::core::Error;
use crate::core::Result;

pub struct SqliteSupplierRepository {
    db_path: String,
}

impl SqliteSupplierRepository {
    pub fn new(db_path: impl AsRef<str>) -> Self {
        Self {
            db_path: Path::new(db_path.as_ref()).to_string_lossy().to_string(),
        }
    }
}

impl ForSupplierPersistence for SqliteSupplierRepository {
    fn next_id(&self) -> Result<SupplierId> {
        let conn = Connection::open(&self.db_path)
            .map_err(|e| Error::InfrastructureError(format!("fail to open connection: {}", e)))?;

        let next_id = conn
            .query_row(
                r"
                UPDATE suppliers_id_sequence
                SET value = value + 1
                WHERE name = 'supplier_id'
                RETURNING value
                ",
                [],
                |r| r.get::<_, i64>(0),
            )
            .map_err(|e| Error::InfrastructureError(format!("fail to query: {}", e)))?;

        let next_id = SupplierId::new(next_id.to_string())?;

        Ok(next_id)
    }

    fn has(&self, supplier_ids: &[SupplierId]) -> Result<bool> {
        let conn = Connection::open(&self.db_path)
            .map_err(|e| Error::InfrastructureError(format!("fail to open connection: {}", e)))?;

        let params = vec!["?"; supplier_ids.len()].join(", ");

        let mut statement = conn
            .prepare(&format!(
                "SELECT COUNT(id) FROM suppliers WHERE id IN ({})",
                params
            ))
            .map_err(|e| {
                Error::InfrastructureError(format!("failed to prepare statement: {}", e))
            })?;

        let count = statement
            .query_one(
                params_from_iter(supplier_ids.iter().map(|id| id.as_str())),
                |r| r.get::<_, i64>(0),
            )
            .map_err(|e| Error::InfrastructureError(format!("failed to query: {}", e)))?;

        Ok(count == supplier_ids.len() as i64)
    }

    fn list(&self) -> Result<Vec<Supplier>> {
        let conn = Connection::open(&self.db_path)
            .map_err(|e| Error::InfrastructureError(format!("fail to open connection: {}", e)))?;

        let mut statement = conn
            .prepare(
                r"
                SELECT
                    id,
                    name
                FROM suppliers
                ",
            )
            .map_err(|e| {
                Error::InfrastructureError(format!("failed to prepare statement: {}", e))
            })?;

        let supplier_results = statement
            .query_map([], |row| {
                let supplier = Supplier::restore(
                    SupplierId::new(row.get::<_, i64>(0)?.to_string())
                        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
                    SupplierName::new(row.get::<_, String>(1)?)
                        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
                );

                Ok(supplier)
            })
            .map_err(|e| Error::InfrastructureError(format!("failed to query: {}", e)))?;

        let suppliers = supplier_results
            .map(|supplier| {
                supplier.map_err(|e| {
                    Error::InfrastructureError(format!("failed to convert rows: {}", e))
                })
            })
            .collect::<Result<Vec<Supplier>>>()?;

        Ok(suppliers)
    }

    fn get(&self, query: required_ports::GetSupplierQuery) -> Result<Option<Supplier>> {
        let conn = Connection::open(&self.db_path)
            .map_err(|e| Error::InfrastructureError(format!("fail to open connection: {}", e)))?;

        let mut statement = conn
            .prepare(
                r"
                SELECT
                    id,
                    name
                FROM suppliers
                WHERE id = :id
                ",
            )
            .map_err(|e| {
                Error::InfrastructureError(format!("failed to prepare statement: {}", e))
            })?;

        let supplier = statement
            .query_row(
                named_params! {
                    ":id": query.supplier_id.as_str(),
                },
                |row| {
                    let supplier = Supplier::restore(
                        SupplierId::new(row.get::<_, i64>(0)?.to_string())
                            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
                        SupplierName::new(row.get::<_, String>(1)?)
                            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
                    );

                    Ok(supplier)
                },
            )
            .optional();

        let supplier =
            supplier.map_err(|e| Error::InfrastructureError(format!("failed to query: {}", e)))?;

        Ok(supplier)
    }

    fn find(&self, query: FindSupplierQuery) -> Result<Vec<Supplier>> {
        let conn = Connection::open(&self.db_path)
            .map_err(|e| Error::InfrastructureError(format!("fail to open connection: {}", e)))?;

        let mut statement = conn
            .prepare(
                r"
                SELECT
                    id,
                    name
                FROM suppliers
                WHERE
                    (:name IS NULL OR name LIKE :name)
                ",
            )
            .map_err(|e| {
                Error::InfrastructureError(format!("failed to prepare statement: {}", e))
            })?;

        let supplier_results = statement
            .query_map(
                named_params! {
                    ":name": query.supplier_name.and_then(|name| Some(format!("%{}%", name.to_string()))),
                },
                |row| {
                    let supplier = Supplier::restore(
                        SupplierId::new(row.get::<_, i64>(0)?.to_string())
                            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
                        SupplierName::new(row.get::<_, String>(1)?)
                            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
                    );

                    Ok(supplier)
                },
            )
            .map_err(|e| Error::InfrastructureError(format!("failed to query: {}", e)))?;

        let suppliers = supplier_results
            .map(|supplier| {
                supplier.map_err(|e| {
                    Error::InfrastructureError(format!("failed to convert rows: {}", e))
                })
            })
            .collect::<Result<Vec<Supplier>>>()?;

        Ok(suppliers)
    }

    fn add(&self, supplier: Supplier) -> Result<()> {
        let mut conn = Connection::open(&self.db_path)
            .map_err(|e| Error::InfrastructureError(format!("fail to open connection: {}", e)))?;

        let tran = conn.transaction().map_err(|e| {
            Error::InfrastructureError(format!("failed to start transacrion: {}", e))
        })?;

        let result = tran
            .execute(
                r"
                INSERT INTO suppliers (
                    id,
                    name
                ) VALUES (
                    ?,
                    ?
                );
                ",
                [supplier.id().as_str(), supplier.name().as_str()],
            )
            .map_err(|e| {
                Error::InfrastructureError(format!("failed to insert new supplier: {}", e))
            });

        if let Err(e) = result {
            tran.rollback()
                .map_err(|e| Error::InfrastructureError(format!("failed to rollback: {}", e)))?;

            return Err(e);
        }

        tran.commit()
            .map_err(|e| Error::InfrastructureError(format!("failed to commit: {}", e)))?;

        Ok(())
    }

    fn save(&self, supplier: Supplier) -> Result<()> {
        let mut conn = Connection::open(&self.db_path)
            .map_err(|e| Error::InfrastructureError(format!("fail to open connection: {}", e)))?;

        let tran = conn.transaction().map_err(|e| {
            Error::InfrastructureError(format!("failed to start transacrion: {}", e))
        })?;

        let result = tran
            .execute(
                r"
                UPDATE suppliers
                SET name = :name
                WHERE id = :id
                ",
                named_params! {
                    ":id": supplier.id().as_str(),
                    ":name": supplier.name().as_str(),
                },
            )
            .map_err(|e| {
                Error::InfrastructureError(format!("failed to execute transaction: {}", e))
            });

        if result.is_err() {
            tran.rollback()
                .map_err(|e| Error::InfrastructureError(format!("failed to rollback: {}", e)))?;

            return Err(result.err().unwrap());
        }

        tran.commit()
            .map_err(|e| Error::InfrastructureError(format!("failed to commit: {}", e)))?;

        Ok(())
    }
}
