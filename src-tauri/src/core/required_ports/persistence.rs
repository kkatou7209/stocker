use crate::core::domain::entities::stock::*;
use crate::core::domain::values::stock::*;
use crate::core::required_ports::dto::*;
use crate::core::Result;

pub trait ForSupplyPersistence {
    fn next_id(&self) -> Result<SupplyId>;
    fn has(&self, supply_ids: &[SupplyId]) -> Result<bool>;
    fn list(&self) -> Result<Vec<Supply>>;
    fn get(&self, id: SupplyId) -> Result<Option<Supply>>;
    fn get_of_supplier(&self, supplier_id: SupplierId) -> Result<Vec<Supply>>;
    fn find(&self, query: FindSuppliesQuery) -> Result<Vec<Supply>>;
    fn add(&self, supply: Supply) -> Result<()>;
    fn save(&self, supply: Supply) -> Result<()>;
}

pub trait ForSupplierPersistence {
    fn next_id(&self) -> Result<SupplierId>;
    fn has(&self, supplier_ids: &[SupplierId]) -> Result<bool>;
    fn list(&self) -> Result<Vec<Supplier>>;
    fn get(&self, id: SupplierId) -> Result<Option<Supplier>>;
    fn find(&self, query: FindSupplierQuery) -> Result<Vec<Supplier>>;
    fn add(&self, supplier: Supplier) -> Result<()>;
    fn save(&self, supplier: Supplier) -> Result<()>;
}

pub trait ForJournalPersistence {
    fn next_id(&self) -> Result<JournalId>;
    fn list(&self) -> Result<Vec<Journal>>;
    fn get(&self, id: JournalId) -> Result<Option<Journal>>;
    fn find(&self, query: FindJournalsQuery) -> Result<Vec<Journal>>;
    fn add(&self, journal: Journal) -> Result<()>;
    fn save(&self, journal: Journal) -> Result<()>;
}

pub trait ForStocktakingPersistence {
    fn next_id(&self) -> Result<StocktakingId>;
    fn list(&self) -> Result<Vec<Stocktaking>>;
    fn find(&self, query: FindStocktakingsQuery) -> Result<Vec<Stocktaking>>;
    fn get(&self, id: StocktakingId) -> Result<Option<Stocktaking>>;
    fn add(&self, stocktaking: Stocktaking) -> Result<()>;
    fn save(&self, stocktaking: Stocktaking) -> Result<()>;
}
