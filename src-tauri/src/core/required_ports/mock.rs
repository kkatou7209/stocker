use std::sync::Arc;
use std::sync::LazyLock;
use std::sync::Mutex;

use crate::core::domain::entities::stock;
use crate::core::domain::entities::stock::*;
use crate::core::domain::values::stock::*;
use crate::core::required_ports::*;
use crate::core::*;

#[derive(Debug, Default)]
pub struct Storage {
    supplies: Vec<Supply>,
    suppliers: Vec<Supplier>,
    journals: Vec<Journal>,
    stocktakings: Vec<Stocktaking>,
}

#[derive(Debug, Clone, Default)]
pub struct MockSupplyRepository {
    storage: Arc<Mutex<Storage>>,
}

impl MockSupplyRepository {
    pub fn new(storage: Arc<Mutex<Storage>>) -> Self {
        Self { storage }
    }
}

impl ForSupplyPersistence for MockSupplyRepository {
    fn next_id(&self) -> Result<SupplyId> {
        let id = self.storage.lock().unwrap().supplies.len() + 1;

        let id = SupplyId::new(id.to_string())?;

        Ok(id)
    }

    fn has(&self, supply_ids: &[SupplyId]) -> Result<bool> {
        let supplies = &self.storage.lock().unwrap().supplies;

        for id in supply_ids {
            if !supplies.iter().any(|s| s.id().eq(id)) {
                return Ok(false);
            }
        }

        Ok(true)
    }

    fn list(&self) -> Result<Vec<Supply>> {
        let supplies = &self.storage.lock().unwrap().supplies;

        Ok(supplies.iter().cloned().collect())
    }

    fn get(&self, query: GetSupplyQuery) -> Result<Option<Supply>> {
        let supplies = &self.storage.lock().unwrap().supplies;

        let id = query.supply_id;

        for supply in supplies {
            if supply.id().eq(&id) {
                return Ok(Some(supply.clone()));
            }
        }

        Ok(None)
    }

    fn find(&self, query: FindSuppliesQuery) -> Result<Vec<Supply>> {
        let storage = self.storage.lock().unwrap();
        let mut supplies: Vec<&Supply> = storage.supplies.iter().collect();

        let supply_name = query.supply_name;

        if let Some(name) = supply_name {
            supplies.retain(|supply| supply.name().as_str().contains(name.as_str()));
        }

        let supplier_name = query.supplier_name;

        if let Some(name) = supplier_name {
            let mut suppliers: Vec<&Supplier> = storage.suppliers.iter().collect();

            suppliers.retain(|supplier| supplier.name().as_str().contains(name.as_str()));

            supplies.retain(|supply| {
                suppliers
                    .iter()
                    .any(|supplier| supplier.id().eq(supply.supplier_id()))
            });
        }

        let supplies: Vec<Supply> = supplies.into_iter().cloned().collect();

        Ok(supplies)
    }

    fn add(&self, supply: Supply) -> Result<()> {
        let mut storage = self.storage.lock().unwrap();

        storage.supplies.push(supply.clone());

        Ok(())
    }

    fn save(&self, supply: Supply) -> Result<()> {
        let mut storage = self.storage.lock().unwrap();

        if let Some(index) = storage
            .supplies
            .iter()
            .position(|supply| supply.id().eq(supply.id()))
        {
            storage.supplies[index] = supply;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct MockSupplierRepository {
    storage: Arc<Mutex<Storage>>,
}

impl MockSupplierRepository {
    pub fn new(storage: Arc<Mutex<Storage>>) -> Self {
        Self { storage }
    }
}

impl ForSupplierPersistence for MockSupplierRepository {
    fn next_id(&self) -> Result<SupplierId> {
        let id = self.storage.lock().unwrap().suppliers.len() + 1;

        let id = SupplierId::new(id.to_string())?;

        Ok(id)
    }

    fn has(&self, supplier_ids: &[SupplierId]) -> Result<bool> {
        let suppliers = &self.storage.lock().unwrap().suppliers;

        for id in supplier_ids {
            if !suppliers.iter().any(|s| s.id().eq(id)) {
                return Ok(false);
            }
        }

        Ok(true)
    }

    fn list(&self) -> Result<Vec<Supplier>> {
        let suppliers = self.storage.lock().unwrap().suppliers.clone();

        Ok(suppliers)
    }

    fn get(&self, query: GetSupplierQuery) -> Result<Option<Supplier>> {
        let id = query.supplier_id;

        let supplier = self
            .storage
            .lock()
            .unwrap()
            .suppliers
            .iter()
            .find(|s| s.id().eq(&id))
            .cloned();

        Ok(supplier)
    }

    fn find(&self, query: FindSupplierQuery) -> Result<Vec<Supplier>> {
        let storage = self.storage.lock().unwrap();

        let supplier_name = query.supplier_name;

        let mut suppliers: Vec<&Supplier> = storage.suppliers.iter().collect();

        if let Some(name) = supplier_name {
            suppliers.retain(|s| s.name().as_str().contains(name.as_str()));
        }

        let suppliers: Vec<Supplier> = suppliers.into_iter().cloned().collect();

        Ok(suppliers)
    }

    fn add(&self, supplier: Supplier) -> Result<()> {
        let mut storage = self.storage.lock().unwrap();

        if storage.suppliers.iter().any(|s| s.id().eq(supplier.id())) {
            return Err(Error::InfrastructureError("supplier already exists."));
        }

        storage.suppliers.push(supplier.clone());

        Ok(())
    }

    fn save(&self, supplier: Supplier) -> Result<()> {
        let mut storage = self.storage.lock().unwrap();

        if let Some(index) = storage
            .suppliers
            .iter()
            .position(|s| s.id().eq(supplier.id()))
        {
            storage.suppliers[index] = supplier.clone();
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct MockJournalRepository {
    storage: Arc<Mutex<Storage>>,
}

impl MockJournalRepository {
    pub fn new(storage: Arc<Mutex<Storage>>) -> Self {
        Self { storage }
    }
}

impl ForJournalPersistence for MockJournalRepository {
    fn next_id(&self) -> Result<JournalId> {
        let id = self.storage.lock().unwrap().journals.len() + 1;

        let id = JournalId::new(id.to_string())?;

        Ok(id)
    }

    fn list(&self) -> Result<Vec<Journal>> {
        let journals: Vec<Journal> = self
            .storage
            .lock()
            .unwrap()
            .journals
            .iter()
            .cloned()
            .collect();

        Ok(journals)
    }

    fn get(&self, id: JournalId) -> Result<Option<Journal>> {
        let journal = self
            .storage
            .lock()
            .unwrap()
            .journals
            .iter()
            .find(|j| j.id().eq(&id))
            .cloned();

        Ok(journal)
    }

    fn find(&self, query: FindJournalsQuery) -> Result<Vec<Journal>> {
        let storage = self.storage.lock().unwrap();

        let mut journals: Vec<&Journal> = storage.journals.iter().collect();

        if let Some(date) = query.period_start {
            journals.retain(|j| j.entry_datetime().ge(&date));
        }

        if let Some(date) = query.period_end {
            journals.retain(|j| j.entry_datetime().le(&date));
        }

        if let Some(name) = query.supply_name {
            let supply_ids: Vec<&SupplyId> = storage
                .supplies
                .iter()
                .filter(|s| s.name().as_str().contains(name.as_str()))
                .map(|s| s.id())
                .collect();

            journals.retain(|j| {
                j.records()
                    .iter()
                    .any(|r| supply_ids.iter().any(|id| r.supply_id().eq(id)))
            });
        }

        if let Some(name) = query.supplier_name {
            let supplier_ids: Vec<&SupplierId> = storage
                .suppliers
                .iter()
                .filter(|s| s.name().as_str().contains(name.as_str()))
                .map(|s| s.id())
                .collect();

            journals.retain(|j| {
                j.records()
                    .iter()
                    .any(|r| supplier_ids.iter().any(|id| r.supplier_id().eq(id)))
            });
        }

        let journals: Vec<Journal> = journals.into_iter().cloned().collect();

        Ok(journals)
    }

    fn add(&self, journal: Journal) -> Result<()> {
        let mut storage = self.storage.lock().unwrap();

        if storage.journals.iter().any(|j| j.id().eq(journal.id())) {
            return Err(Error::InfrastructureError("journal already exists."));
        }

        storage.journals.push(journal.clone());

        Ok(())
    }

    fn save(&self, journal: Journal) -> Result<()> {
        let mut storage = self.storage.lock().unwrap();

        if let Some(index) = storage
            .journals
            .iter()
            .position(|j| j.id().eq(journal.id()))
        {
            storage.journals[index] = journal.clone();
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct MockStocktakingRepository {
    storage: Arc<Mutex<Storage>>,
}

impl MockStocktakingRepository {
    pub fn new(storage: Arc<Mutex<Storage>>) -> Self {
        Self { storage }
    }
}

impl ForStocktakingPersistence for MockStocktakingRepository {
    fn next_id(&self) -> Result<StocktakingId> {
        let id = self.storage.lock().unwrap().stocktakings.len() + 1;

        let id = StocktakingId::new(id.to_string())?;

        Ok(id)
    }

    fn list(&self) -> Result<Vec<Stocktaking>> {
        let stocktakings: Vec<Stocktaking> = self
            .storage
            .lock()
            .unwrap()
            .stocktakings
            .iter()
            .cloned()
            .collect();

        Ok(stocktakings)
    }

    fn find(&self, query: FindStocktakingsQuery) -> Result<Vec<Stocktaking>> {
        let storage = self.storage.lock().unwrap();

        let mut stocktakings: Vec<&Stocktaking> = storage.stocktakings.iter().collect();

        if let Some(date) = query.period_start {
            stocktakings.retain(|s| s.stocktaken_at().ge(&date));
        }

        if let Some(date) = query.period_end {
            stocktakings.retain(|s| s.stocktaken_at().le(&date));
        }

        let stocktakings: Vec<Stocktaking> = stocktakings.into_iter().cloned().collect();

        Ok(stocktakings)
    }

    fn get(&self, query: GetStocktakingQuery) -> Result<Option<Stocktaking>> {
        let id = query.stocktaking_id;

        let stocktaking = self
            .storage
            .lock()
            .unwrap()
            .stocktakings
            .iter()
            .find(|s| s.id().eq(&id))
            .cloned();

        Ok(stocktaking)
    }

    fn add(&self, stockatking: Stocktaking) -> Result<()> {
        let mut storage = self.storage.lock().unwrap();

        if storage
            .stocktakings
            .iter()
            .any(|s| s.id().eq(stockatking.id()))
        {
            return Err(Error::InfrastructureError("stocktaking already exists."));
        }

        storage.stocktakings.push(stockatking.clone());

        Ok(())
    }

    fn save(&self, stocktaking: Stocktaking) -> Result<()> {
        let mut storage = self.storage.lock().unwrap();

        if let Some(index) = storage
            .stocktakings
            .iter()
            .position(|s| s.id().eq(stocktaking.id()))
        {
            storage.stocktakings[index] = stocktaking.clone();
        }

        Ok(())
    }
}
