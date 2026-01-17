//! This module provides the SQLite-based implementation of the `ForStocktakingPersistence` trait.
use std::collections::HashSet;
use std::path::Path;

use chrono::Utc;
use rusqlite::named_params;
use rusqlite::Connection;
use rusqlite::OptionalExtension;

use crate::core::domain::entities::stock::Stocktaking;
use crate::core::domain::values::stock::*;
use crate::core::required_ports::*;
use crate::core::Error;
use crate::core::Result;

/// SQLite implementation of `ForStocktakingPersistence`
pub struct SqliteStocktakingRepository {
    db_path: String,
}

impl SqliteStocktakingRepository {
    pub fn new(db_path: impl AsRef<str>) -> Self {
        Self {
            db_path: Path::new(db_path.as_ref()).to_string_lossy().to_string(),
        }
    }
}

/// Implementation of `ForStocktakingPersistence` for `SqliteStocktakingRepository`
impl ForStocktakingPersistence for SqliteStocktakingRepository {
    fn next_id(&self) -> Result<StocktakingId> {
        let conn = Connection::open(&self.db_path)
            .map_err(|e| Error::InfrastructureError(format!("failed to open connection: {}", e)))?;

        let next_id = conn
            .query_row(
                r"
                UPDATE stocktakings_id_sequence
                SET value = value + 1
                WHERE name = 'stocktaking_id'
                RETURNING value
                ",
                [],
                |r| r.get::<_, i64>(0),
            )
            .map_err(|e| Error::InfrastructureError(format!("fail to query: {}", e)))?;

        let next_id = StocktakingId::new(next_id.to_string())?;

        Ok(next_id)
    }

    fn list(&self) -> Result<Vec<Stocktaking>> {
        let conn = Connection::open(&self.db_path)
            .map_err(|e| Error::InfrastructureError(format!("failed to open connection: {}", e)))?;

        let mut statement = conn
            .prepare(
                r"
                SELECT
                    stocktakings.id,
                    stocktakings.recorded_at,
                    stocktaking_records.supply_id,
                    stocktaking_records.supply_name,
                    stocktaking_records.unit_name,
                    stocktaking_records.unit_price,
                    stocktaking_records.quantity
                FROM stocktakings
                INNER JOIN stocktaking_records
                    ON stocktaking_records.stocktaking_id = stocktakings.id
                WHERE
                    stocktakings.deleted_at IS NULL
                ORDER BY stocktakings.recorded_at DESC
                ",
            )
            .map_err(|e| {
                Error::InfrastructureError(format!("failed to prepare statement: {}", e))
            })?;

        let mut stocktakings = statement
            .query_map([], |row| {
                let stocktaking = Stocktaking::restore(
                    StocktakingId::new(row.get::<_, i64>(0)?.to_string())?,
                    StocktakenDateTime::new(row.get::<_, i64>(1)?),
                    vec![],
                );

                Ok(stocktaking)
            })
            .map_err(|e| Error::InfrastructureError(format!("failed to query: {}", e)))?
            .map(|stocktaking| {
                stocktaking.map_err(|e| {
                    Error::InfrastructureError(format!("failed to convert rows: {}", e))
                })
            })
            .collect::<Result<HashSet<Stocktaking>>>()?
            .into_iter()
            .collect::<Vec<Stocktaking>>();

        let stocktaking_records = statement
            .query_map([], |row| {
                let id = StocktakingId::new(row.get::<_, i64>(0)?.to_string())?;

                let stocktaking_record = StocktakingRecord::new(
                    SupplyId::new(row.get::<_, i64>(2)?.to_string())?,
                    SupplyName::new(row.get::<_, String>(3)?)?,
                    UnitName::new(row.get::<_, String>(4)?)?,
                    StocktakingUnitPrice::new(
                        u32::try_from(row.get::<_, i64>(5)?)
                            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
                    )?,
                    StocktakingQuantity::new(
                        f64::try_from(row.get::<_, i64>(6)? as f64 / 100.0)
                            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
                    )?,
                );

                Ok((id, stocktaking_record))
            })
            .map_err(|e| Error::InfrastructureError(format!("failed to query: {}", e)))?
            .map(|record| {
                record.map_err(|e| {
                    Error::InfrastructureError(format!("failed to convert rows: {}", e))
                })
            })
            .collect::<Result<Vec<(StocktakingId, StocktakingRecord)>>>()?;

        for stocktaking in stocktakings.iter_mut() {
            let records: Vec<StocktakingRecord> = stocktaking_records
                .iter()
                .filter_map(|record| {
                    let stocktaking_id = &record.0;
                    let stocktaking_record = &record.1;

                    if stocktaking_id == stocktaking.id() {
                        Some(stocktaking_record)
                    } else {
                        None
                    }
                })
                .cloned()
                .collect();

            stocktaking.swap_records(records);
        }

        Ok(stocktakings)
    }

    fn find(&self, query: FindStocktakingsQuery) -> Result<Vec<Stocktaking>> {
        let conn = Connection::open(&self.db_path)
            .map_err(|e| Error::InfrastructureError(format!("failed to open connection: {}", e)))?;

        let mut statement = conn
            .prepare(
                r"
                SELECT
                    stocktakings.id,
                    stocktakings.recorded_at,
                    stocktaking_records.supply_id,
                    stocktaking_records.supply_name,
                    stocktaking_records.unit_name,
                    stocktaking_records.unit_price,
                    stocktaking_records.quantity
                FROM stocktakings
                INNER JOIN stocktaking_records
                    ON stocktaking_records.stocktaking_id = stocktakings.id
                WHERE
                    (:start IS NULL OR :start <= stocktakings.recorded_at)
                    AND
                    (:end IS NULL OR stocktakings.recorded_at <= :end)
                    AND
                    stocktakings.deleted_at IS NULL
                ",
            )
            .map_err(|e| {
                Error::InfrastructureError(format!("failed to prepare statement: {}", e))
            })?;

        let mut stocktakings = statement
            .query_map(
                named_params! {
                    ":start": query.period_start.as_ref().and_then(|start| Some(start.as_i64())),
                    ":end": query.period_end.as_ref().and_then(|end| Some(end.as_i64())),
                },
                |row| {
                    let stocktaking = Stocktaking::restore(
                        StocktakingId::new(row.get::<_, i64>(0)?.to_string())?,
                        StocktakenDateTime::new(row.get::<_, i64>(1)?),
                        vec![],
                    );

                    Ok(stocktaking)
                },
            )
            .map_err(|e| Error::InfrastructureError(format!("failed to query: {}", e)))?
            .map(|stocktaking| {
                stocktaking.map_err(|e| {
                    Error::InfrastructureError(format!("failed to convert rows: {}", e))
                })
            })
            .collect::<Result<HashSet<Stocktaking>>>()?
            .into_iter()
            .collect::<Vec<Stocktaking>>();

        let stocktaking_records = statement
            .query_map(
                named_params! {
                    ":start": query.period_start.as_ref().and_then(|start| Some(start.as_i64())),
                    ":end": query.period_end.as_ref().and_then(|end| Some(end.as_i64())),
                },
                |row| {
                    let id = StocktakingId::new(row.get::<_, i64>(0)?.to_string())?;

                    let stocktaking_record = StocktakingRecord::new(
                        SupplyId::new(row.get::<_, i64>(2)?.to_string())?,
                        SupplyName::new(row.get::<_, String>(3)?)?,
                        UnitName::new(row.get::<_, String>(4)?)?,
                        StocktakingUnitPrice::new(
                            u32::try_from(row.get::<_, i64>(5)?).map_err(|e| {
                                rusqlite::Error::ToSqlConversionFailure(Box::new(e))
                            })?,
                        )?,
                        StocktakingQuantity::new(
                            f64::try_from(row.get::<_, i64>(6)? as f64 / 100.0).map_err(|e| {
                                rusqlite::Error::ToSqlConversionFailure(Box::new(e))
                            })?,
                        )?,
                    );

                    Ok((id, stocktaking_record))
                },
            )
            .map_err(|e| Error::InfrastructureError(format!("failed to query: {}", e)))?
            .map(|record| {
                record.map_err(|e| {
                    Error::InfrastructureError(format!("failed to convert rows: {}", e))
                })
            })
            .collect::<Result<Vec<(StocktakingId, StocktakingRecord)>>>()?;

        for stocktaking in stocktakings.iter_mut() {
            let records: Vec<StocktakingRecord> = stocktaking_records
                .iter()
                .filter_map(|record| {
                    let stocktaking_id = &record.0;
                    let stocktaking_record = &record.1;

                    if stocktaking_id == stocktaking.id() {
                        Some(stocktaking_record)
                    } else {
                        None
                    }
                })
                .cloned()
                .collect();

            stocktaking.swap_records(records);
        }

        Ok(stocktakings)
    }

    fn get(&self, id: StocktakingId) -> Result<Option<Stocktaking>> {
        let conn = Connection::open(&self.db_path)
            .map_err(|e| Error::InfrastructureError(format!("failed to open connection: {}", e)))?;

        let mut statement = conn
            .prepare(
                r"
                SELECT
                    stocktakings.id,
                    stocktakings.recorded_at,
                    stocktaking_records.supply_id,
                    stocktaking_records.supply_name,
                    stocktaking_records.unit_name,
                    stocktaking_records.unit_price,
                    stocktaking_records.quantity
                FROM stocktakings
                INNER JOIN stocktaking_records
                    ON stocktaking_records.stocktaking_id = stocktakings.id
                WHERE
                    stocktakings.id = :id
                    AND
                    stocktakings.deleted_at IS NULL
                ",
            )
            .map_err(|e| {
                Error::InfrastructureError(format!("failed to prepare statement: {}", e))
            })?;

        let mut stocktaking = statement
            .query_row(
                named_params! {
                    ":id": id.as_str(),
                },
                |row| {
                    let stocktaking = Stocktaking::restore(
                        StocktakingId::new(row.get::<_, i64>(0)?.to_string())?,
                        StocktakenDateTime::new(row.get::<_, i64>(1)?),
                        vec![],
                    );

                    Ok(stocktaking)
                },
            )
            .optional()
            .map_err(|e| Error::InfrastructureError(format!("failed to query: {}", e)))?;

        let stocktaking_records = statement
            .query_map(
                named_params! {
                    ":id": id.as_str(),
                },
                |row| {
                    let stocktaking_record = StocktakingRecord::new(
                        SupplyId::new(row.get::<_, i64>(2)?.to_string())?,
                        SupplyName::new(row.get::<_, String>(3)?)?,
                        UnitName::new(row.get::<_, String>(4)?)?,
                        StocktakingUnitPrice::new(
                            u32::try_from(row.get::<_, i64>(5)?).map_err(|e| {
                                rusqlite::Error::ToSqlConversionFailure(Box::new(e))
                            })?,
                        )?,
                        StocktakingQuantity::new(
                            f64::try_from(row.get::<_, i64>(6)? as f64 / 100.0).map_err(|e| {
                                rusqlite::Error::ToSqlConversionFailure(Box::new(e))
                            })?,
                        )?,
                    );

                    Ok(stocktaking_record)
                },
            )
            .map_err(|e| Error::InfrastructureError(format!("failed to query: {}", e)))?
            .map(|record| {
                record.map_err(|e| {
                    Error::InfrastructureError(format!("failed to convert rows: {}", e))
                })
            })
            .collect::<Result<Vec<StocktakingRecord>>>()?;

        if let Some(stocktaking) = stocktaking.as_mut() {
            stocktaking.swap_records(stocktaking_records);
        }

        Ok(stocktaking)
    }

    fn add(&self, stocktaking: Stocktaking) -> Result<()> {
        let mut conn = Connection::open(&self.db_path)
            .map_err(|e| Error::InfrastructureError(format!("failed to open connection: {}", e)))?;

        let tran = conn.transaction().map_err(|e| {
            Error::InfrastructureError(format!("failed to start transaction: {}", e))
        })?;

        let result: Result<()> = (|| {
            tran.execute(
                r"
                INSERT INTO stocktakings (
                    id,
                    recorded_at
                ) VALUES (
                    :id,
                    :recorded_at
                )
                ",
                named_params! {
                    ":id": stocktaking.id().as_str(),
                    ":recorded_at": stocktaking.stocktaken_at().as_i64(),
                },
            )
            .map_err(|e| Error::InfrastructureError(format!("failed to execute: {}", e)))?;

            let mut statement = tran
                .prepare(
                    r"
                INSERT INTO stocktaking_records (
                    supply_id,
                    supply_name,
                    unit_name,
                    unit_price,
                    quantity,
                    stocktaking_id
                ) VALUES (
                    :supply_id,
                    :supply_name,
                    :unit_name,
                    :unit_price,
                    :quantity,
                    :stocktaking_id
                )
                ",
                )
                .map_err(|e| {
                    Error::InfrastructureError(format!("failed to prepare statement: {}", e))
                })?;

            for record in stocktaking.records() {
                statement
                    .execute(named_params! {
                        ":supply_id": record.supply_id().as_str(),
                        ":supply_name": record.supply_name().as_str(),
                        ":unit_name": record.unit_name().as_str(),
                        ":unit_price": record.unit_price().as_u32() as i64,
                        ":quantity": (record.quantity().as_f64() * 100.0) as i64,
                        ":stocktaking_id": stocktaking.id().as_str(),
                    })
                    .map_err(|e| {
                        Error::InfrastructureError(format!("failed to execute statement: {}", e))
                    })?;
            }

            Ok(())
        })();

        if result.is_err() {
            tran.rollback()
                .map_err(|e| Error::InfrastructureError(format!("failed to rollback: {}", e)))?;

            return result;
        }

        tran.commit()
            .map_err(|e| Error::InfrastructureError(format!("failed to commit: {}", e)))?;

        Ok(())
    }

    fn save(&self, stocktaking: Stocktaking) -> Result<()> {
        let mut conn = Connection::open(&self.db_path)
            .map_err(|e| Error::InfrastructureError(format!("failed to open connection: {}", e)))?;

        let tran = conn.transaction().map_err(|e| {
            Error::InfrastructureError(format!("failed to start transaction: {}", e))
        })?;

        let result: Result<()> = (|| {
            tran.execute(
                r"
                UPDATE stocktakings
                SET recorded_at = :recorded_at
                WHERE id = :id
                ",
                named_params! {
                    ":id": stocktaking.id().as_str(),
                    ":recorded_at": stocktaking.stocktaken_at().as_i64(),
                },
            )
            .map_err(|e| Error::InfrastructureError(format!("failed to execute: {}", e)))?;

            tran.execute(
                r"
                DELETE FROM stocktaking_records
                WHERE stocktaking_id = :id
                ",
                named_params! {
                    ":id": stocktaking.id().as_str(),
                },
            )
            .map_err(|e| Error::InfrastructureError(format!("failed to execute: {}", e)))?;

            let mut statement = tran
                .prepare(
                    r"
                INSERT INTO stocktaking_records (
                    supply_id,
                    supply_name,
                    unit_name,
                    unit_price,
                    quantity,
                    stocktaking_id
                ) VALUES (
                    :supply_id,
                    :supply_name,
                    :unit_name,
                    :unit_price,
                    :quantity,
                    :stocktaking_id
                )
                ",
                )
                .map_err(|e| {
                    Error::InfrastructureError(format!("failed to prepare statement: {}", e))
                })?;

            for record in stocktaking.records() {
                statement
                    .execute(named_params! {
                        ":supply_id": record.supply_id().as_str(),
                        ":supply_name": record.supply_name().as_str(),
                        ":unit_name": record.unit_name().as_str(),
                        ":unit_price": record.unit_price().as_u32() as i64,
                        ":quantity": (record.quantity().as_f64() * 100.0) as i64,
                        ":stocktaking_id": stocktaking.id().as_str(),
                    })
                    .map_err(|e| {
                        Error::InfrastructureError(format!("failed to execute statement: {}", e))
                    })?;
            }

            Ok(())
        })();

        if result.is_err() {
            tran.rollback()
                .map_err(|e| Error::InfrastructureError(format!("failed to rollback: {}", e)))?;

            return result;
        }

        tran.commit()
            .map_err(|e| Error::InfrastructureError(format!("failed to commit: {}", e)))?;

        Ok(())
    }

    fn delete(&self, id: StocktakingId) -> Result<()> {
        let mut conn = Connection::open(&self.db_path)
            .map_err(|e| Error::InfrastructureError(format!("failed to open connection: {}", e)))?;

        let tran = conn.transaction().map_err(|e| {
            Error::InfrastructureError(format!("failed to start transaction: {}", e))
        })?;

        let result: Result<()> = (|| {
            tran.execute(
                r"
                UPDATE stocktakings
                SET deleted_at = :deleted_at
                WHERE id = :id
                ",
                named_params! {
                    ":id": id.as_str(),
                    ":deleted_at": Utc::now().timestamp_millis(),
                },
            )
            .map_err(|e| Error::InfrastructureError(format!("failed to execute: {}", e)))?;

            Ok(())
        })();

        if result.is_err() {
            tran.rollback()
                .map_err(|e| Error::InfrastructureError(format!("failed to rollback: {}", e)))?;

            return result;
        }

        tran.commit()
            .map_err(|e| Error::InfrastructureError(format!("failed to commit: {}", e)))?;

        Ok(())
    }
}
