//! This module provides the implementation for `JournalUsecase`.
use std::collections::HashSet;
use std::sync::Arc;

use crate::core::domain::entities::stock::*;
use crate::core::domain::values::stock::*;
use crate::core::provided_ports::*;
use crate::core::required_ports::*;
use crate::core::*;

/// Journal usecase
pub struct JournalService {
    supply_respository: Arc<dyn ForSupplyPersistence>,
    supplier_repository: Arc<dyn ForSupplierPersistence>,
    journal_respository: Arc<dyn ForJournalPersistence>,
}

impl JournalService {
    pub fn new(
        supply_respository: Arc<dyn ForSupplyPersistence>,
        supplier_repository: Arc<dyn ForSupplierPersistence>,
        journal_respository: Arc<dyn ForJournalPersistence>,
    ) -> Self {
        Self {
            supply_respository,
            supplier_repository,
            journal_respository,
        }
    }
}

/// Journal usecase implementation
impl JournalUsecase for JournalService {
    fn get(&self, journal_id: impl AsRef<str>) -> Result<Option<JournalDTO>> {
        let journal_id = JournalId::new(journal_id.as_ref())?;

        let journal = self.journal_respository.get(journal_id)?;

        if journal.is_none() {
            return Ok(None);
        }

        let journal = journal.unwrap();

        let journal = JournalDTO {
            id: journal.id().to_string(),
            entry_date: journal.entry_datetime().as_i64(),
            total_price: journal.total_price().as_f64(),
            records: journal
                .records()
                .iter()
                .map(|record| JournalRecordDTO {
                    supplier_id: record.supplier_id().to_string(),
                    supplier_name: record.supplier_name().to_string(),
                    supply_id: record.supply_id().to_string(),
                    supply_name: record.supply_name().to_string(),
                    unit_name: record.unit_name().to_string(),
                    unit_price: record.unit_price().as_f64(),
                    quantity: record.quantity().as_f64(),
                    total_price: record.total_price().as_f64(),
                })
                .collect(),
        };

        Ok(Some(journal))
    }

    fn list(&self) -> Result<Vec<JournalDTO>> {
        let journals = self.journal_respository.list()?;

        let journals: Vec<JournalDTO> = journals
            .iter()
            .map(|journal| JournalDTO {
                id: journal.id().to_string(),
                entry_date: journal.entry_datetime().as_i64(),
                total_price: journal.total_price().as_f64(),
                records: journal
                    .records()
                    .iter()
                    .map(|record| JournalRecordDTO {
                        supplier_id: record.supplier_id().to_string(),
                        supplier_name: record.supplier_name().to_string(),
                        supply_id: record.supply_id().to_string(),
                        supply_name: record.supply_name().to_string(),
                        unit_name: record.unit_name().to_string(),
                        unit_price: record.unit_price().as_f64(),
                        quantity: record.quantity().as_f64(),
                        total_price: record.total_price().as_f64(),
                    })
                    .collect(),
            })
            .collect();

        Ok(journals)
    }

    fn search(&self, query: SearchJournalsQuery) -> Result<Vec<JournalDTO>> {
        let query = FindJournalsQuery {
            period_start: query.period_start.map(|start| EntryDateTime::new(start)),
            period_end: query.period_end.map(|end| EntryDateTime::new(end)),
            supplier_name: query
                .supplier_name
                .and_then(|name| {
                    if name.trim().is_empty() {
                        None
                    } else {
                        Some(SupplierName::new(name.trim()))
                    }
                })
                .transpose()?,
            supply_name: query
                .supply_name
                .and_then(|name| {
                    if name.trim().is_empty() {
                        None
                    } else {
                        Some(SupplyName::new(name.trim()))
                    }
                })
                .transpose()?,
        };

        let journals = self.journal_respository.find(query)?;

        let journals: Vec<JournalDTO> = journals
            .iter()
            .map(|journal| JournalDTO {
                id: journal.id().to_string(),
                entry_date: journal.entry_datetime().as_i64(),
                total_price: journal.total_price().as_f64(),
                records: journal
                    .records()
                    .iter()
                    .map(|record| JournalRecordDTO {
                        supplier_id: record.supplier_id().to_string(),
                        supplier_name: record.supplier_name().to_string(),
                        supply_id: record.supply_id().to_string(),
                        supply_name: record.supply_name().to_string(),
                        unit_name: record.unit_name().to_string(),
                        unit_price: record.unit_price().as_f64(),
                        quantity: record.quantity().as_f64(),
                        total_price: record.total_price().as_f64(),
                    })
                    .collect(),
            })
            .collect();

        Ok(journals)
    }

    fn record(&self, command: RecordJournalCommand) -> Result<JournalDTO> {
        let id = self.journal_respository.next_id()?;

        let supply_ids: Vec<SupplyId> = command
            .records
            .iter()
            .map(|record| SupplyId::new(&record.supply_id))
            .collect::<Result<HashSet<SupplyId>>>()?
            .into_iter()
            .collect::<Vec<SupplyId>>();

        let supply_exists = self.supply_respository.has(&supply_ids)?;

        if !supply_exists {
            return Err(Error::DomainError(format!("supply does not exist.")));
        }

        let supplier_ids = command
            .records
            .iter()
            .map(|record| SupplierId::new(&record.supplier_id))
            .collect::<Result<HashSet<SupplierId>>>()?
            .into_iter()
            .collect::<Vec<SupplierId>>();

        let supplier_exists = self.supplier_repository.has(&supplier_ids)?;

        if !supplier_exists {
            return Err(Error::DomainError(format!("supplier does not exist.")));
        }

        let mut records: Vec<JournalRecord> = Vec::new();

        for record in &command.records {
            records.push(JournalRecord::new(
                SupplyId::new(&record.supply_id)?,
                SupplyName::new(&record.supply_name)?,
                SupplierId::new(&record.supplier_id)?,
                SupplierName::new(&record.supplier_name)?,
                UnitName::new(&record.unit_name)?,
                PurchaseUnitPrice::new(record.unit_price)?,
                PurchaseQuantity::new(record.quantity)?,
                TotalPrice::new(record.total_price)?,
            ));
        }

        let journal = Journal::restore(
            id,
            EntryDateTime::new(command.entry_date),
            TotalPrice::new(command.total_price)?,
            records,
        );

        self.journal_respository.add(journal.clone())?;

        let journal = JournalDTO {
            id: journal.id().to_string(),
            entry_date: journal.entry_datetime().as_i64(),
            total_price: journal.total_price().as_f64(),
            records: journal
                .records()
                .iter()
                .map(|record| JournalRecordDTO {
                    supplier_id: record.supplier_id().to_string(),
                    supplier_name: record.supplier_name().to_string(),
                    supply_id: record.supply_id().to_string(),
                    supply_name: record.supply_name().to_string(),
                    unit_name: record.unit_name().to_string(),
                    unit_price: record.unit_price().as_f64(),
                    quantity: record.quantity().as_f64(),
                    total_price: record.total_price().as_f64(),
                })
                .collect(),
        };

        Ok(journal)
    }

    fn edit(&self, command: EditJournalCommand) -> Result<()> {
        let journal_id = JournalId::new(command.journal_id)?;

        let mut journal = self
            .journal_respository
            .get(journal_id)?
            .ok_or(Error::DomainError(format!("journal does not exist.")))?;

        let total_price = TotalPrice::new(command.total_price)?;

        journal.change_total_price(total_price);

        let mut records: Vec<JournalRecord> = Vec::new();

        for record in &command.records {
            records.push(JournalRecord::new(
                SupplyId::new(&record.supply_id)?,
                SupplyName::new(&record.supply_name)?,
                SupplierId::new(&record.supplier_id)?,
                SupplierName::new(&record.supplier_name)?,
                UnitName::new(&record.unit_name)?,
                PurchaseUnitPrice::new(record.unit_price)?,
                PurchaseQuantity::new(record.quantity)?,
                TotalPrice::new(record.total_price)?,
            ));
        }

        journal.swap_records(records);

        self.journal_respository.save(journal)?;

        Ok(())
    }

    fn delete(&self, journal_id: impl AsRef<str>) -> Result<()> {
        let journal_id = JournalId::new(journal_id.as_ref())?;

        self.journal_respository.delete(journal_id)?;

        Ok(())
    }
}
