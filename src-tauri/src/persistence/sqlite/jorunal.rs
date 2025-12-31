use std::path::Path;

use rusqlite::named_params;
use rusqlite::Connection;
use rusqlite::OptionalExtension;

use crate::core::domain::entities::stock::Journal;
use crate::core::domain::values::stock::*;
use crate::core::required_ports::*;
use crate::core::Error;
use crate::core::Result;

pub struct SqliteJournalRepository {
    db_path: String,
}

impl SqliteJournalRepository {
    pub fn new(db_path: impl AsRef<str>) -> Self {
        Self {
            db_path: Path::new(db_path.as_ref()).to_string_lossy().to_string(),
        }
    }
}

impl ForJournalPersistence for SqliteJournalRepository {
    fn next_id(&self) -> Result<JournalId> {
        let conn = Connection::open(&self.db_path)
            .map_err(|e| Error::InfrastructureError(format!("failed to open connection: {}", e)))?;

        let next_id = conn
            .query_row(
                r"
                UPDATE journals_id_sequence
                SET value = value + 1
                WHERE name = 'journal_id'
                RETURNING value
                ",
                [],
                |r| r.get::<_, i64>(0),
            )
            .map_err(|e| Error::InfrastructureError(format!("fail to query: {}", e)))?;

        let next_id = JournalId::new(next_id.to_string())?;

        Ok(next_id)
    }

    fn list(&self) -> Result<Vec<crate::core::domain::entities::stock::Journal>> {
        let conn = Connection::open(&self.db_path)
            .map_err(|e| Error::InfrastructureError(format!("failed to open connection: {}", e)))?;

        let mut statement = conn
            .prepare(
                r"
                SELECT
                    journals.id,
                    journals.recorded_at,
                    journal_records.supply_id,
                    journal_records.supply_name,
                    journal_records.supplier_id,
                    journal_records.supplier_name,
                    journal_records.unit_name,
                    journal_records.unit_price,
                    journal_records.quantity
                FROM journals
                INNER JOIN journal_records
                    ON journal_records.journal_id = journals.id
                ORDER BY journals.recorded_at ASC
                ",
            )
            .map_err(|e| {
                Error::InfrastructureError(format!("failed to prepare statement: {}", e))
            })?;

        let mut jorunals = statement
            .query_map([], |row| {
                let journal = Journal::restore(
                    JournalId::new(row.get::<_, i64>(0)?.to_string())?,
                    EntryDateTime::new(row.get::<_, i64>(1)?),
                    Vec::new(),
                );

                Ok(journal)
            })
            .map_err(|e| Error::InfrastructureError(format!("failed to query: {}", e)))?
            .map(|journal| {
                journal.map_err(|e| {
                    Error::InfrastructureError(format!("failed to convert rows: {}", e))
                })
            })
            .collect::<Result<Vec<Journal>>>()?;

        let jorunal_records = statement
            .query_map([], |row| {
                let journal_id = JournalId::new(row.get::<_, i64>(0)?.to_string())
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

                let journal_record = JournalRecord::new(
                    SupplyId::new(row.get::<_, i64>(2)?.to_string())?,
                    SupplyName::new(row.get::<_, String>(3)?)?,
                    SupplierId::new(row.get::<_, i64>(4)?.to_string())?,
                    SupplierName::new(row.get::<_, String>(5)?)?,
                    UnitName::new(row.get::<_, String>(6)?)?,
                    PurchaseUnitPrice::new(
                        u32::try_from(row.get::<_, i64>(7)?)
                            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
                    )?,
                    PurchaseQuantity::new(
                        u32::try_from(row.get::<_, i64>(8)?)
                            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
                    )?,
                );

                Ok((journal_id, journal_record))
            })
            .map_err(|e| Error::InfrastructureError(format!("failed to query: {}", e)))?
            .map(|journal| {
                journal.map_err(|e| {
                    Error::InfrastructureError(format!("failed to convert rows: {}", e))
                })
            })
            .collect::<Result<Vec<(JournalId, JournalRecord)>>>()?;

        for journal in jorunals.iter_mut() {
            let records: Vec<JournalRecord> = jorunal_records
                .iter()
                .filter_map(|record| {
                    let journal_id = &record.0;
                    let journal_record = &record.1;
                    if journal_id == journal.id() {
                        Some(journal_record)
                    } else {
                        None
                    }
                })
                .cloned()
                .collect();

            journal.swap_records(records);
        }

        Ok(jorunals)
    }

    fn get(&self, id: JournalId) -> Result<Option<Journal>> {
        let conn = Connection::open(&self.db_path)
            .map_err(|e| Error::InfrastructureError(format!("failed to open connection: {}", e)))?;

        let mut statement = conn
            .prepare(
                r"
                SELECT
                    journals.id,
                    journals.recorded_at,
                    journal_records.supply_id,
                    journal_records.supply_name,
                    journal_records.supplier_id,
                    journal_records.supplier_name,
                    journal_records.unit_name,
                    journal_records.unit_price,
                    journal_records.quantity,
                    journal_records.journal_id
                FROM journals
                INNER JOIN journal_records
                    ON journal_records.journal_id = journals.id
                WHERE
                    journals.id = :id
                ",
            )
            .map_err(|e| {
                Error::InfrastructureError(format!("failed to prepare statement: {}", e))
            })?;

        let mut journal = statement
            .query_row(
                named_params! {
                    ":id": id.as_str(),
                },
                |row| {
                    let journal = Journal::restore(
                        JournalId::new(row.get::<_, i64>(0)?.to_string())?,
                        EntryDateTime::new(row.get::<_, i64>(1)?),
                        Vec::new(),
                    );

                    Ok(journal)
                },
            )
            .optional()
            .map_err(|e| Error::InfrastructureError(format!("failed to query: {}", e)))?;

        let jorunal_records = statement
            .query_map(
                named_params! {
                    ":id": id.as_str(),
                },
                |row| {
                    let journal_record =
                        JournalRecord::new(
                            SupplyId::new(row.get::<_, i64>(2)?.to_string())?,
                            SupplyName::new(row.get::<_, String>(3)?)?,
                            SupplierId::new(row.get::<_, i64>(4)?.to_string())?,
                            SupplierName::new(row.get::<_, String>(5)?)?,
                            UnitName::new(row.get::<_, String>(6)?)?,
                            PurchaseUnitPrice::new(u32::try_from(row.get::<_, i64>(7)?).map_err(
                                |e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)),
                            )?)?,
                            PurchaseQuantity::new(u32::try_from(row.get::<_, i64>(8)?).map_err(
                                |e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)),
                            )?)?,
                        );

                    Ok(journal_record)
                },
            )
            .map_err(|e| Error::InfrastructureError(format!("failed to query: {}", e)))?
            .map(|journal| {
                journal.map_err(|e| {
                    Error::InfrastructureError(format!("failed to convert rows: {}", e))
                })
            })
            .collect::<Result<Vec<JournalRecord>>>()?;

        if let Some(journal) = journal.as_mut() {
            journal.swap_records(jorunal_records);
        }

        Ok(journal)
    }

    fn find(&self, query: FindJournalsQuery) -> Result<Vec<Journal>> {
        let conn = Connection::open(&self.db_path)
            .map_err(|e| Error::InfrastructureError(format!("failed to open connection: {}", e)))?;

        let mut statement = conn
            .prepare(
                r"
                SELECT
                    journals.id,
                    journals.recorded_at,
                    journal_records.supply_id,
                    journal_records.supply_name,
                    journal_records.supplier_id,
                    journal_records.supplier_name,
                    journal_records.unit_name,
                    journal_records.unit_price,
                    journal_records.quantity,
                    journal_records.journal_id
                FROM journals
                INNER JOIN journal_records
                    ON journal_records.journal_id = journals.id
                WHERE
                    (:start IS NULL OR :start <= journals.recorded_at)
                    AND
                    (:end IS NULL OR journals.recorded_at <= :end)
                    AND
                    (:supply_name IS NULL OR journal_records.supply_name LIKE :supply_name)
                    AND
                    (:supplier_name IS NULL OR journal_records.supplier_name LIKE :supplier_name)
                ORDER BY journals.recorded_at ASC
                ",
            )
            .map_err(|e| {
                Error::InfrastructureError(format!("failed to prepare statement: {}", e))
            })?;

        let mut jorunals = statement
            .query_map(
                named_params! {
                    ":start": query.period_start.as_ref().and_then(|start| Some(start.as_i64())),
                    ":end": query.period_end.as_ref().and_then(|end| Some(end.as_i64())),
                    ":supply_name": query.supply_name.as_ref().and_then(|name| Some(format!("%{}%", name.as_str()))),
                    ":supplier_name": query.supplier_name.as_ref().and_then(|name| Some(format!("%{}%", name.as_str()))),
                },
                |row| {
                    let journal = Journal::restore(
                        JournalId::new(row.get::<_, i64>(0)?.to_string())?,
                        EntryDateTime::new(row.get::<_, i64>(1)?),
                        Vec::new(),
                    );

                    Ok(journal)
                },
            )
            .map_err(|e| Error::InfrastructureError(format!("failed to query: {}", e)))?
            .map(|journal| {
                journal.map_err(|e| {
                    Error::InfrastructureError(format!("failed to convert rows: {}", e))
                })
            })
            .collect::<Result<Vec<Journal>>>()?;

        let jorunal_records = statement
            .query_map(named_params! {
                    ":start": query.period_start.and_then(|start| Some(start.as_i64())),
                    ":end": query.period_end.and_then(|end| Some(end.as_i64())),
                    ":supply_name": query.supply_name.and_then(|name| Some(format!("%{}%", name.as_str()))),
                    ":supplier_name": query.supplier_name.and_then(|name| Some(format!("%{}%", name.as_str()))),
                }, |row| {
                let journal_id = JournalId::new(row.get::<_, i64>(0)?.to_string())
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

                let journal_record = JournalRecord::new(
                    SupplyId::new(row.get::<_, i64>(2)?.to_string())?,
                    SupplyName::new(row.get::<_, String>(3)?)?,
                    SupplierId::new(row.get::<_, i64>(4)?.to_string())?,
                    SupplierName::new(row.get::<_, String>(5)?)?,
                    UnitName::new(row.get::<_, String>(6)?)?,
                    PurchaseUnitPrice::new(
                        u32::try_from(row.get::<_, i64>(7)?)
                            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
                    )?,
                    PurchaseQuantity::new(
                        u32::try_from(row.get::<_, i64>(8)?)
                            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
                    )?,
                );

                Ok((journal_id, journal_record))
            })
            .map_err(|e| Error::InfrastructureError(format!("failed to query: {}", e)))?
            .map(|journal| {
                journal.map_err(|e| {
                    Error::InfrastructureError(format!("failed to convert rows: {}", e))
                })
            })
            .collect::<Result<Vec<(JournalId, JournalRecord)>>>()?;

        for journal in jorunals.iter_mut() {
            let records: Vec<JournalRecord> = jorunal_records
                .iter()
                .filter_map(|record| {
                    let journal_id = &record.0;
                    let journal_record = &record.1;
                    if journal_id == journal.id() {
                        Some(journal_record)
                    } else {
                        None
                    }
                })
                .cloned()
                .collect();

            journal.swap_records(records);
        }

        Ok(jorunals)
    }

    fn add(&self, journal: Journal) -> Result<()> {
        let mut conn = Connection::open(&self.db_path)
            .map_err(|e| Error::InfrastructureError(format!("failed to open connection: {}", e)))?;

        let tran = conn.transaction().map_err(|e| {
            Error::InfrastructureError(format!("failed to start transaction: {}", e))
        })?;

        let result: Result<()> = (|| {
            let mut statement = tran
                .prepare(
                    r"
                    INSERT INTO journals (
                        id,
                        recorded_at
                    ) VALUES (
                        :id,
                        :recorded_at
                    );
                    ",
                )
                .map_err(|e| {
                    Error::InfrastructureError(format!("failed to prepare statement: {}", e))
                })?;

            statement
                .execute(named_params! {
                    ":id": journal.id().as_str(),
                    ":recorded_at": journal.entry_datetime().as_i64(),
                })
                .map_err(|e| {
                    Error::InfrastructureError(format!("failed to execute statement: {}", e))
                })?;

            let mut statement = tran
                .prepare(
                    r"
                    INSERT INTO journal_records (
                        supply_id,
                        supply_name,
                        supplier_id,
                        supplier_name,
                        unit_name,
                        unit_price,
                        quantity,
                        journal_id
                    ) VALUES (
                        :supply_id,
                        :supply_name,
                        :supplier_id,
                        :supplier_name,
                        :unit_name,
                        :unit_price,
                        :quantity,
                        :journal_id
                    )
                    ",
                )
                .map_err(|e| {
                    Error::InfrastructureError(format!("failed to prepare statement: {}", e))
                })?;

            for record in journal.records() {
                statement
                    .execute(named_params! {
                        ":supply_id": record.supply_id().as_str(),
                        ":supply_name": record.supply_name().as_str(),
                        ":supplier_id": record.supplier_id().as_str(),
                        ":supplier_name": record.supplier_name().as_str(),
                        ":unit_name": record.unit_name().as_str(),
                        ":unit_price": record.unit_price().as_u32() as i64,
                        ":quantity": record.quantity().as_u32() as i64,
                        ":journal_id": journal.id().as_str(),
                    })
                    .map_err(|e| {
                        Error::InfrastructureError(format!("failed to execute statement: {}", e))
                    })?;
            }

            Ok(())
        })();

        if let Err(e) = result {
            tran.rollback()
                .map_err(|e| Error::InfrastructureError(format!("failed to rollback: {}", e)))?;

            return Err(e);
        }

        tran.commit()
            .map_err(|e| Error::InfrastructureError(format!("failed to commit: {}", e)))?;

        Ok(())
    }

    fn save(&self, journal: Journal) -> Result<()> {
        let mut conn = Connection::open(&self.db_path)
            .map_err(|e| Error::InfrastructureError(format!("failed to open connection: {}", e)))?;

        let tran = conn.transaction().map_err(|e| {
            Error::InfrastructureError(format!("failed to start transaction: {}", e))
        })?;

        let result: Result<()> = (|| {
            let mut statement = tran
                .prepare(
                    r"
                    UPDATE journals
                    SET
                        recorded_at = :recorded_at
                    WHERE
                        id = :id
                    ",
                )
                .map_err(|e| {
                    Error::InfrastructureError(format!("failed to prepare statement: {}", e))
                })?;

            statement
                .execute(named_params! {
                    ":id": journal.id().as_str(),
                    ":recorded_at": journal.entry_datetime().as_i64(),
                })
                .map_err(|e| {
                    Error::InfrastructureError(format!("failed to execute statement: {}", e))
                })?;

            let mut statement = tran
                .prepare(
                    r"
                    DELETE FROM journal_records
                    WHERE
                        journal_id = :journal_id
                    ",
                )
                .map_err(|e| {
                    Error::InfrastructureError(format!("failed to execute statement: {}", e))
                })?;

            statement
                .execute(named_params! {
                    ":journal_id": journal.id().as_str(),
                })
                .map_err(|e| {
                    Error::InfrastructureError(format!("failed to execute statement: {}", e))
                })?;

            let mut statement = tran
                .prepare(
                    r"
                    INSERT INTO journal_records (
                        supply_id,
                        supply_name,
                        supplier_id,
                        supplier_name,
                        unit_name,
                        unit_price,
                        quantity,
                        journal_id
                    ) VALUES (
                        :supply_id,
                        :supply_name,
                        :supplier_id,
                        :supplier_name,
                        :unit_name,
                        :unit_price,
                        :quantity,
                        :journal_id
                    )
                    ",
                )
                .map_err(|e| {
                    Error::InfrastructureError(format!("failed to prepare statement: {}", e))
                })?;

            for record in journal.records() {
                statement
                    .execute(named_params! {
                        ":supply_id": record.supply_id().as_str(),
                        ":supply_name": record.supply_name().as_str(),
                        ":supplier_id": record.supplier_id().as_str(),
                        ":supplier_name": record.supplier_name().as_str(),
                        ":unit_name": record.unit_name().as_str(),
                        ":unit_price": record.unit_price().as_u32() as i64,
                        ":quantity": record.quantity().as_u32() as i64,
                        ":journal_id": journal.id().as_str(),
                    })
                    .map_err(|e| {
                        Error::InfrastructureError(format!("failed to execute statement: {}", e))
                    })?;
            }

            Ok(())
        })();

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
