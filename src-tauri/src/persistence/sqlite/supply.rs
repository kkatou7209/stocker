use std::path::Path;

use rusqlite::named_params;
use rusqlite::params_from_iter;
use rusqlite::Connection;
use rusqlite::OptionalExtension;

use crate::core::domain::entities::stock::Supply;
use crate::core::domain::values::stock::*;
use crate::core::required_ports::*;
use crate::core::Error;
use crate::core::Result;

pub struct SqliteSupplyRepository {
    db_path: String,
}

impl SqliteSupplyRepository {
    pub fn new(db_path: impl AsRef<str>) -> Self {
        Self {
            db_path: Path::new(db_path.as_ref()).to_string_lossy().to_string(),
        }
    }
}

impl ForSupplyPersistence for SqliteSupplyRepository {
    fn next_id(&self) -> Result<SupplyId> {
        let conn = Connection::open(&self.db_path)
            .map_err(|e| Error::InfrastructureError(format!("failed to open connection: {}", e)))?;

        let next_id = conn
            .query_row(
                r"
                UPDATE supplies_id_sequence
                SET value = value + 1
                WHERE name = 'supply_id'
                RETURNING value
                ",
                [],
                |r| r.get::<_, i64>(0),
            )
            .map_err(|e| Error::InfrastructureError(format!("fail to query: {}", e)))?;

        let next_id = SupplyId::new(next_id.to_string())?;

        Ok(next_id)
    }

    fn has(&self, supply_ids: &[SupplyId]) -> Result<bool> {
        let conn = Connection::open(&self.db_path)
            .map_err(|e| Error::InfrastructureError(format!("failed to open connection: {}", e)))?;

        let params = vec!["?"; supply_ids.len()].join(", ");

        let mut statement = conn
            .prepare(&format!(
                "SELECT COUNT(id) FROM supplies WHERE id IN ({})",
                params
            ))
            .map_err(|e| {
                Error::InfrastructureError(format!("failed to prepare statement: {}", e))
            })?;

        let count = statement
            .query_one(
                params_from_iter(supply_ids.iter().map(|id| id.as_str())),
                |r| r.get::<_, i64>(0),
            )
            .map_err(|e| Error::InfrastructureError(format!("failed to query: {}", e)))?;

        Ok(count == supply_ids.len() as i64)
    }

    fn list(&self) -> Result<Vec<crate::core::domain::entities::stock::Supply>> {
        let conn = Connection::open(&self.db_path)
            .map_err(|e| Error::InfrastructureError(format!("failed to open connection: {}", e)))?;

        let mut statement = conn
            .prepare(
                r"
                SELECT
                    id,
                    name,
                    unit_name,
                    supplier_id
                FROM supplies
                ",
            )
            .map_err(|e| {
                Error::InfrastructureError(format!("failed to prepare statement: {}", e))
            })?;

        let supply_results = statement
            .query_map([], |row| {
                let supply = Supply::new(
                    SupplyId::new(row.get::<_, i64>(0)?.to_string())?,
                    SupplyName::new(row.get::<_, String>(1)?)?,
                    UnitName::new(row.get::<_, String>(2)?)?,
                    SupplierId::new(row.get::<_, i64>(3)?.to_string())?,
                );

                Ok(supply)
            })
            .map_err(|e| Error::InfrastructureError(format!("failed to query: {}", e)))?;

        let supplies = supply_results
            .map(|supply| {
                supply.map_err(|e| {
                    Error::InfrastructureError(format!("failed to convert rows: {}", e))
                })
            })
            .collect::<Result<Vec<Supply>>>()?;

        Ok(supplies)
    }

    fn get(
        &self,
        query: GetSupplyQuery,
    ) -> Result<Option<crate::core::domain::entities::stock::Supply>> {
        let conn = Connection::open(&self.db_path)
            .map_err(|e| Error::InfrastructureError(format!("failed to open connection: {}", e)))?;

        let mut statement = conn
            .prepare(
                r"
                SELECT
                    id,
                    name,
                    unit_name,
                    supplier_id
                FROM supplies
                WHERE id = :id
                ",
            )
            .map_err(|e| {
                Error::InfrastructureError(format!("failed to prepare statement: {}", e))
            })?;

        let supply = statement
            .query_row(
                named_params! {
                    ":id": query.supply_id.as_str()
                },
                |row| {
                    let supply = Supply::new(
                        SupplyId::new(row.get::<_, i64>(0)?.to_string())?,
                        SupplyName::new(row.get::<_, String>(1)?)?,
                        UnitName::new(row.get::<_, String>(2)?)?,
                        SupplierId::new(row.get::<_, i64>(3)?.to_string())?,
                    );

                    Ok(supply)
                },
            )
            .optional()
            .map_err(|e| Error::InfrastructureError(format!("failed to query: {}", e)))?;

        Ok(supply)
    }

    fn find(&self, query: FindSuppliesQuery) -> Result<Vec<Supply>> {
        let conn = Connection::open(&self.db_path)
            .map_err(|e| Error::InfrastructureError(format!("failed to open connection: {}", e)))?;

        let mut statement = conn
            .prepare(
                r"
                SELECT
                    supplies.id,
                    supplies.name,
                    supplies.unit_name,
                    supplies.supplier_id
                FROM supplies
                INNER JOIN suppliers
                    ON suppliers.id = supplies.supplier_id
                WHERE
                    (:supply_name IS NULL OR supplies.name LIKE :supply_name)
                    AND
                    (:supplier_name IS NULL OR suppliers.name LIKE :supplier_name)
                ",
            )
            .map_err(|e| {
                Error::InfrastructureError(format!("failed to prepare statement: {}", e))
            })?;

        let supply_results = statement
            .query_map(
                named_params! {
                    ":supply_name": query.supply_name.and_then(|name| Some(format!("%{}%", name.as_str()))),
                    ":supplier_name": query.supplier_name.and_then(|name| Some(format!("%{}%", name.as_str()))),
                },
                |row| {
                    let supply = Supply::new(
                        SupplyId::new(row.get::<_, i64>(0)?.to_string())?,
                        SupplyName::new(row.get::<_, String>(1)?)?,
                        UnitName::new(row.get::<_, String>(2)?)?,
                        SupplierId::new(row.get::<_, i64>(1)?.to_string())?,
                    );

                    Ok(supply)
                },
            )
            .map_err(|e| Error::InfrastructureError(format!("failed to query: {}", e)))?;

        let supplies = supply_results
            .map(|supply| {
                supply.map_err(|e| {
                    Error::InfrastructureError(format!("failed to convert rows: {}", e))
                })
            })
            .collect::<Result<Vec<Supply>>>()?;

        Ok(supplies)
    }

    fn add(&self, supply: crate::core::domain::entities::stock::Supply) -> Result<()> {
        let mut conn = Connection::open(&self.db_path)
            .map_err(|e| Error::InfrastructureError(format!("failed to open connection: {}", e)))?;

        let tran = conn.transaction().map_err(|e| {
            Error::InfrastructureError(format!("failed to start transaction: {}", e))
        })?;

        let result = tran
            .execute(
                r"
                INSERT INTO supplies (
                    id,
                    name,
                    unit_name,
                    supplier_id
                ) VALUES (
                    :id,
                    :name,
                    :unit_name,
                    :supplier_id
                );
                ",
                named_params! {
                    ":id": supply.id().as_str(),
                    ":name": supply.name().as_str(),
                    ":unit_name": supply.unit_name().as_str(),
                    ":supplier_id": supply.supplier_id().as_str(),
                },
            )
            .map_err(|e| Error::InfrastructureError(format!("failed to insert new supply: {}", e)));

        if let Err(e) = result {
            tran.rollback()
                .map_err(|e| Error::InfrastructureError(format!("failed to rollback: {}", e)))?;

            return Err(e);
        }

        tran.commit()
            .map_err(|e| Error::InfrastructureError(format!("failed to commit: {}", e)))?;

        Ok(())
    }

    fn save(&self, supply: Supply) -> Result<()> {
        let mut conn = Connection::open(&self.db_path)
            .map_err(|e| Error::InfrastructureError(format!("failed to open connection: {}", e)))?;

        let tran = conn.transaction().map_err(|e| {
            Error::InfrastructureError(format!("failed to start transaction: {}", e))
        })?;

        let result = tran
            .execute(
                r"
                UPDATE supplies
                SET
                    name = :name,
                    unit_name = :unit_name,
                    supplier_id = :supplier_id
                WHERE
                    id = :id
                ",
                named_params! {
                    ":id": supply.id().as_str(),
                    ":name": supply.name().as_str(),
                    ":unit_name": supply.unit_name().as_str(),
                    ":supplier_id": supply.supplier_id().as_str(),
                },
            )
            .map_err(|e| Error::InfrastructureError(format!("failed to update supply: {}", e)));

        if let Err(e) = result {
            tran.rollback()
                .map_err(|e| Error::InfrastructureError(format!("failed to rollback: {}", e)))?;

            return Err(e);
        }

        tran.commit()
            .map_err(|e| Error::InfrastructureError(format!("failed to commit: {}", e)))?;

        Ok(())
    }
}
