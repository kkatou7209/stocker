use std::path::Path;

use rusqlite::Connection;

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
                WHERE name = 'jorunal_id'
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
                    journal_records.quantity,
                    journal_records.journal_id
                FROM journals
                INNER JOIN journal_records
                    ON journal_records.journal_id = journals.id
                ",
            )
            .map_err(|e| {
                Error::InfrastructureError(format!("failed to prepare statement: {}", e))
            })?;

        let mut jorunals = statement
            .query_map([], |row| {
                let journal = Journal::restore(
                    JournalId::new(row.get::<_, i64>(0)?.to_string())
                        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
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
                    SupplyId::new(row.get::<_, i64>(2)?.to_string())
                        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
                    SupplyName::new(row.get::<_, String>(3)?)
                        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
                    SupplierId::new(row.get::<_, String>(4)?)
                        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
                    SupplierName::new(row.get::<_, String>(5)?)
                        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
                    UnitName::new(row.get::<_, String>(6)?)
                        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
                    PurchaseUnitPrice::new(
                        u32::try_from(row.get::<_, i64>(7)?)
                            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
                    )
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
                    PurchaseQuantity::new(
                        u32::try_from(row.get::<_, i64>(8)?)
                            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
                    )
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
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

    fn get(&self, id: JournalId) -> Result<Option<crate::core::domain::entities::stock::Journal>> {
        todo!()
    }

    fn find(
        &self,
        query: FindJournalsQuery,
    ) -> Result<Vec<crate::core::domain::entities::stock::Journal>> {
        todo!()
    }

    fn add(&self, journal: crate::core::domain::entities::stock::Journal) -> Result<()> {
        todo!()
    }

    fn save(&self, journal: crate::core::domain::entities::stock::Journal) -> Result<()> {
        todo!()
    }
}
