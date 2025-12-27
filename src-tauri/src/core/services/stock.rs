use std::sync::Arc;

use crate::core::domain::entities::stock::{Journal, Stocktaking, Supplier, Supply};
use crate::core::domain::values::stock::*;
use crate::core::{provided_ports, Result};
use crate::core::{provided_ports::*, required_ports};
use crate::core::{required_ports::*, Error};

/// supply usecase
pub struct SupplyService {
    supply_repository: Arc<dyn ForSupplyPersistence>,
    supplier_repository: Arc<dyn ForSupplierPersistence>,
}

impl SupplyService {
    pub fn new(
        supply_repository: Arc<dyn ForSupplyPersistence>,
        supplier_repository: Arc<dyn ForSupplierPersistence>,
    ) -> Self {
        Self {
            supply_repository,
            supplier_repository,
        }
    }
}

impl SupplyUsecase for SupplyService {
    fn get(&self, query: provided_ports::GetSupplyQuery) -> Result<Option<SupplyDTO>> {
        let supply_id = SupplyId::new(query.supply_id.as_str())?;

        let supply = self
            .supply_repository
            .get(required_ports::GetSupplyQuery { supply_id })?;

        if supply.is_none() {
            return Ok(None);
        }

        let supply = supply.unwrap();

        Ok(Some(SupplyDTO {
            id: supply.id().to_string(),
            name: supply.name().to_string(),
            unit_name: supply.unit_name().to_string(),
            supplier_id: supply.supplier_id().to_string(),
        }))
    }

    fn list(&self) -> Result<Vec<SupplyDTO>> {
        let supplies = self.supply_repository.list()?;

        let supplies: Vec<SupplyDTO> = supplies
            .iter()
            .map(|supply| SupplyDTO {
                id: supply.id().to_string(),
                name: supply.name().to_string(),
                unit_name: supply.unit_name().to_string(),
                supplier_id: supply.supplier_id().to_string(),
            })
            .collect();

        Ok(supplies)
    }

    fn search(&self, query: SearchSuppliesQuery) -> Result<Vec<SupplyDTO>> {
        let query = FindSuppliesQuery {
            supply_name: query
                .supply_name
                .map(|name| SupplyName::new(name))
                .transpose()?,
            supplier_name: query
                .supplier_name
                .map(|name| SupplierName::new(name))
                .transpose()?,
        };

        let supplies = self.supply_repository.find(query)?;

        let supplies: Vec<SupplyDTO> = supplies
            .iter()
            .map(|supply| SupplyDTO {
                id: supply.id().to_string(),
                name: supply.name().to_string(),
                unit_name: supply.unit_name().to_string(),
                supplier_id: supply.supplier_id().to_string(),
            })
            .collect();

        Ok(supplies)
    }

    fn register(&self, command: CreateSupplyCommand) -> Result<SupplyDTO> {
        let id = self.supply_repository.next_id()?;

        let supplier_id = SupplierId::new(command.supplier_id)?;

        let supplier = self
            .supplier_repository
            .get(required_ports::GetSupplierQuery { supplier_id })?
            .ok_or(Error::DomainError(format!("supplier does not exist.")))?;

        let supply = Supply::new(
            id,
            SupplyName::new(command.supply_name)?,
            UnitName::new(command.unit_name)?,
            supplier.id().clone(),
        );

        self.supply_repository.add(supply.clone())?;

        Ok(SupplyDTO {
            id: supply.id().to_string(),
            name: supply.name().to_string(),
            unit_name: supply.unit_name().to_string(),
            supplier_id: supply.supplier_id().to_string(),
        })
    }

    fn update(&self, commad: UpdateSupplyCommand) -> Result<()> {
        let supply_id = SupplyId::new(commad.supply_id)?;

        let mut supply = self
            .supply_repository
            .get(required_ports::GetSupplyQuery { supply_id })?
            .ok_or(Error::DomainError(format!("supply does not exist.")))?;

        let supplier_id = SupplierId::new(commad.supplier_id)?;

        let supplier = self
            .supplier_repository
            .get(required_ports::GetSupplierQuery { supplier_id })?
            .ok_or(Error::DomainError(format!("supplier does not exist.")))?;

        supply.rename(SupplyName::new(commad.supply_name)?);
        supply.rename_unit(UnitName::new(commad.unit_name)?);
        supply.change_supplier(supplier.id().clone());

        self.supply_repository.save(supply)?;

        Ok(())
    }
}

/// supplier usecase
pub struct SupplierService {
    supplier_repository: Arc<dyn ForSupplierPersistence>,
}

impl SupplierService {
    pub fn new(supplier_repository: Arc<dyn ForSupplierPersistence>) -> Self {
        Self {
            supplier_repository,
        }
    }
}

impl SupplierUsecase for SupplierService {
    fn get(&self, query: provided_ports::GetSupplierQuery) -> Result<Option<SupplierDTO>> {
        let supplier_id = SupplierId::new(query.supplier_id)?;

        let supplier = self
            .supplier_repository
            .get(required_ports::GetSupplierQuery { supplier_id })?;

        if supplier.is_none() {
            return Ok(None);
        }

        let supplier = supplier.unwrap();

        let supplier = SupplierDTO {
            id: supplier.id().to_string(),
            name: supplier.name().to_string(),
        };

        Ok(Some(supplier))
    }

    fn list(&self) -> Result<Vec<SupplierDTO>> {
        let suppliers = self.supplier_repository.list()?;

        let suppliers: Vec<SupplierDTO> = suppliers
            .iter()
            .map(|supplier| SupplierDTO {
                id: supplier.id().to_string(),
                name: supplier.name().to_string(),
            })
            .collect();

        Ok(suppliers)
    }

    fn search(&self, query: SearchSuppliersQuery) -> Result<Vec<SupplierDTO>> {
        let query = required_ports::FindSupplierQuery {
            supplier_name: query
                .supplier_name
                .map(|name| SupplierName::new(name))
                .transpose()?,
        };

        let suppliers = self.supplier_repository.find(query)?;

        let suppliers: Vec<SupplierDTO> = suppliers
            .iter()
            .map(|supplier| SupplierDTO {
                id: supplier.id().to_string(),
                name: supplier.name().to_string(),
            })
            .collect();

        Ok(suppliers)
    }

    fn register(&self, command: RegisterSupplierCommand) -> Result<SupplierDTO> {
        let id = self.supplier_repository.next_id()?;

        let supplier = Supplier::restore(id, SupplierName::new(command.supplier_name)?);

        self.supplier_repository.add(supplier.clone())?;

        let supplier = SupplierDTO {
            id: supplier.id().to_string(),
            name: supplier.name().to_string(),
        };

        Ok(supplier)
    }

    fn update(&self, command: UpdateSupplierCommand) -> Result<()> {
        let supplier_id = SupplierId::new(command.supplier_id)?;

        let mut supplier = self
            .supplier_repository
            .get(required_ports::GetSupplierQuery { supplier_id })?
            .ok_or(Error::DomainError(format!("suppler does not exist.")))?;

        supplier.rename(SupplierName::new(command.supplier_name)?);

        self.supplier_repository.save(supplier)?;

        Ok(())
    }
}

/// journal usecase
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

impl JournalUsecase for JournalService {
    fn get(&self, query: provided_ports::GetJournalQuery) -> Result<Option<JournalDTO>> {
        let journal_id = JournalId::new(query.journal_id)?;

        let journal = self.journal_respository.get(journal_id)?;

        if journal.is_none() {
            return Ok(None);
        }

        let journal = journal.unwrap();

        let journal = JournalDTO {
            id: journal.id().to_string(),
            entry_date: journal.entry_datetime().as_i64(),
            records: journal
                .records()
                .iter()
                .map(|record| JournalRecordDTO {
                    supplier_id: record.supplier_id().to_string(),
                    supplier_name: record.supplier_name().to_string(),
                    supply_id: record.supply_id().to_string(),
                    supply_name: record.supply_name().to_string(),
                    unit_name: record.unit_name().to_string(),
                    unit_price: record.unit_price().as_u32(),
                    quantity: record.quantity().as_u32(),
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
                records: journal
                    .records()
                    .iter()
                    .map(|record| JournalRecordDTO {
                        supplier_id: record.supplier_id().to_string(),
                        supplier_name: record.supplier_name().to_string(),
                        supply_id: record.supply_id().to_string(),
                        supply_name: record.supply_name().to_string(),
                        unit_name: record.unit_name().to_string(),
                        unit_price: record.unit_price().as_u32(),
                        quantity: record.quantity().as_u32(),
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
                .map(|name| SupplierName::new(name))
                .transpose()?,
            supply_name: query
                .supply_name
                .map(|name| SupplyName::new(name))
                .transpose()?,
        };

        let journals = self.journal_respository.find(query)?;

        let journals: Vec<JournalDTO> = journals
            .iter()
            .map(|journal| JournalDTO {
                id: journal.id().to_string(),
                entry_date: journal.entry_datetime().as_i64(),
                records: journal
                    .records()
                    .iter()
                    .map(|record| JournalRecordDTO {
                        supplier_id: record.supplier_id().to_string(),
                        supplier_name: record.supplier_name().to_string(),
                        supply_id: record.supply_id().to_string(),
                        supply_name: record.supply_name().to_string(),
                        unit_name: record.unit_name().to_string(),
                        unit_price: record.unit_price().as_u32(),
                        quantity: record.quantity().as_u32(),
                    })
                    .collect(),
            })
            .collect();

        Ok(journals)
    }

    fn record(&self, command: RecordJournalCommand) -> Result<JournalDTO> {
        let id = self.journal_respository.next_id()?;

        let supply_ids: Vec<Result<SupplyId>> = command
            .records
            .iter()
            .map(|record| SupplyId::new(&record.supply_id))
            .collect();

        let supply_ids: Result<Vec<SupplyId>> = supply_ids.into_iter().collect();

        let supply_ids = supply_ids?;

        if !self.supply_respository.has(&supply_ids)? {
            return Err(Error::DomainError(format!("supply does not exist.")));
        }

        let supplier_ids: Vec<Result<SupplierId>> = command
            .records
            .iter()
            .map(|record| SupplierId::new(&record.supplier_id))
            .collect();

        let supplier_ids: Result<Vec<SupplierId>> = supplier_ids.into_iter().collect();

        let supplier_ids = supplier_ids?;

        if !self.supplier_repository.has(&supplier_ids)? {
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
            ));
        }

        let journal = Journal::restore(id, EntryDateTime::new(command.entry_date), records);

        self.journal_respository.add(journal.clone())?;

        let journal = JournalDTO {
            id: journal.id().to_string(),
            entry_date: journal.entry_datetime().as_i64(),
            records: journal
                .records()
                .iter()
                .map(|record| JournalRecordDTO {
                    supplier_id: record.supplier_id().to_string(),
                    supplier_name: record.supplier_name().to_string(),
                    supply_id: record.supply_id().to_string(),
                    supply_name: record.supply_name().to_string(),
                    unit_name: record.unit_name().to_string(),
                    unit_price: record.unit_price().as_u32(),
                    quantity: record.quantity().as_u32(),
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
            ));
        }

        journal.swap_records(records);

        self.journal_respository.save(journal)?;

        Ok(())
    }
}

/// stocktaking usecase
pub struct StocktakingService {
    supply_respository: Arc<dyn ForSupplyPersistence>,
    supplier_repository: Arc<dyn ForSupplierPersistence>,
    stocktaking_respository: Arc<dyn ForStocktakingPersistence>,
}

impl StocktakingService {
    pub fn new(
        supply_respository: Arc<dyn ForSupplyPersistence>,
        supplier_repository: Arc<dyn ForSupplierPersistence>,
        stocktaking_respository: Arc<dyn ForStocktakingPersistence>,
    ) -> Self {
        Self {
            supply_respository,
            supplier_repository,
            stocktaking_respository,
        }
    }
}

impl StocktakingUsecase for StocktakingService {
    fn get(&self, query: provided_ports::GetStocktakingQuery) -> Result<Option<StocktakingDTO>> {
        let stocktaking_id = StocktakingId::new(query.stocktaking_id)?;

        let stocktaking = self
            .stocktaking_respository
            .get(required_ports::GetStocktakingQuery { stocktaking_id })?;

        if stocktaking.is_none() {
            return Ok(None);
        }

        let stocktaking = stocktaking.unwrap();

        let stocktaking = StocktakingDTO {
            id: stocktaking.id().to_string(),
            stocktaken_date: stocktaking.stocktaken_at().as_i64(),
            records: stocktaking
                .records()
                .iter()
                .map(|record| StocktakingRecordDTO {
                    supply_id: record.supply_id().to_string(),
                    supply_name: record.supply_name().to_string(),
                    quantity: record.quantity().as_u32(),
                    unit_name: record.unit_name().to_string(),
                    unit_price: record.unit_price().as_u32(),
                })
                .collect(),
        };

        Ok(Some(stocktaking))
    }

    fn list(&self) -> Result<Vec<StocktakingDTO>> {
        let stocktakings = self.stocktaking_respository.list()?;

        let stocktakings: Vec<StocktakingDTO> = stocktakings
            .iter()
            .map(|stocktaking| StocktakingDTO {
                id: stocktaking.id().to_string(),
                stocktaken_date: stocktaking.stocktaken_at().as_i64(),
                records: stocktaking
                    .records()
                    .iter()
                    .map(|record| StocktakingRecordDTO {
                        quantity: record.quantity().as_u32(),
                        supply_id: record.supply_id().to_string(),
                        supply_name: record.supply_name().to_string(),
                        unit_name: record.unit_name().to_string(),
                        unit_price: record.unit_price().as_u32(),
                    })
                    .collect(),
            })
            .collect();

        Ok(stocktakings)
    }

    fn search(&self, query: SearchStocktakingQuery) -> Result<Vec<StocktakingDTO>> {
        let query = FindStocktakingsQuery {
            period_start: query
                .period_start
                .map(|start| StocktakenDateTime::new(start)),
            period_end: query.period_end.map(|end| StocktakenDateTime::new(end)),
        };

        let stocktakings = self.stocktaking_respository.find(query)?;

        let stocktakings: Vec<StocktakingDTO> = stocktakings
            .iter()
            .map(|stocktaking| StocktakingDTO {
                id: stocktaking.id().to_string(),
                stocktaken_date: stocktaking.stocktaken_at().as_i64(),
                records: stocktaking
                    .records()
                    .iter()
                    .map(|record| StocktakingRecordDTO {
                        quantity: record.quantity().as_u32(),
                        supply_id: record.supply_id().to_string(),
                        supply_name: record.supply_name().to_string(),
                        unit_name: record.unit_name().to_string(),
                        unit_price: record.unit_price().as_u32(),
                    })
                    .collect(),
            })
            .collect();

        Ok(stocktakings)
    }

    fn record(&self, command: RecordStocktakingCommand) -> Result<StocktakingDTO> {
        let id = self.stocktaking_respository.next_id()?;

        let supply_ids: Vec<Result<SupplyId>> = command
            .records
            .iter()
            .map(|record| SupplyId::new(&record.supply_id))
            .collect();

        let supply_ids: Result<Vec<SupplyId>> = supply_ids.into_iter().collect();

        let supply_ids = supply_ids?;

        if !self.supply_respository.has(&supply_ids)? {
            return Err(Error::DomainError(format!("supply does not exist.")));
        }

        let mut records: Vec<StocktakingRecord> = Vec::new();

        for record in &command.records {
            records.push(StocktakingRecord::new(
                SupplyId::new(&record.supply_id)?,
                SupplyName::new(&record.supply_name)?,
                UnitName::new(&record.unit_name)?,
                StocktakingUnitPrice::new(record.unit_price)?,
                StocktakingQuantity::new(record.quantity)?,
            ));
        }

        let stocktaking = Stocktaking::restore(
            id,
            StocktakenDateTime::new(command.stocktaken_date),
            records,
        );

        self.stocktaking_respository.add(stocktaking.clone())?;

        let stocktaking = StocktakingDTO {
            id: stocktaking.id().to_string(),
            stocktaken_date: stocktaking.stocktaken_at().as_i64(),
            records: stocktaking
                .records()
                .iter()
                .map(|record| StocktakingRecordDTO {
                    quantity: record.quantity().as_u32(),
                    supply_id: record.supply_id().to_string(),
                    supply_name: record.supply_name().to_string(),
                    unit_name: record.unit_name().to_string(),
                    unit_price: record.unit_price().as_u32(),
                })
                .collect(),
        };

        Ok(stocktaking)
    }

    fn edit(&self, command: EditStocktakingCommand) -> Result<()> {
        let stocktaking_id = StocktakingId::new(command.stocktaking_id)?;

        let mut stocktaking = self
            .stocktaking_respository
            .get(required_ports::GetStocktakingQuery { stocktaking_id })?
            .ok_or(Error::DomainError(format!("stocktaking does not exist.")))?;

        let mut records: Vec<StocktakingRecord> = Vec::new();

        for record in &command.records {
            records.push(StocktakingRecord::new(
                SupplyId::new(&record.supply_id)?,
                SupplyName::new(&record.supply_name)?,
                UnitName::new(&record.unit_name)?,
                StocktakingUnitPrice::new(record.unit_price)?,
                StocktakingQuantity::new(record.quantity)?,
            ));
        }

        stocktaking.swap_records(records);

        self.stocktaking_respository.save(stocktaking)?;

        Ok(())
    }
}
