//! Required persistence port traits used by core use-cases.
//!
//! This module defines the persistence interfaces (ports) that the core
//! business logic depends on. Concrete adapters (e.g. SQLite, in-memory,
//! or external services) implement these traits to provide storage and
//! retrieval for domain entities such as supplies, suppliers, journals,
//! and stocktakings.
use crate::core::domain::entities::stock::*;
use crate::core::domain::values::stock::*;
use crate::core::required_ports::dto::*;
use crate::core::Result;

/// persistence implementations for supplies
pub trait ForSupplyPersistence {
    /// get next supply id
    fn next_id(&self) -> Result<SupplyId>;
    /// check if supplies exists
    fn has(&self, supply_ids: &[SupplyId]) -> Result<bool>;
    /// get all supplies
    fn list(&self) -> Result<Vec<Supply>>;
    /// get a supply
    fn get(&self, id: SupplyId) -> Result<Option<Supply>>;
    /// get all supplies of supplier
    fn list_of_supplier(&self, supplier_id: SupplierId) -> Result<Vec<Supply>>;
    /// add a new supply
    fn add(&self, supply: Supply) -> Result<()>;
    /// save changes of a supply
    fn save(&self, supply: Supply) -> Result<()>;
    /// delete supply
    fn delete(&self, id: SupplyId) -> Result<()>;
}

/// persistence implementations for supplier
pub trait ForSupplierPersistence {
    /// get next supplier id
    fn next_id(&self) -> Result<SupplierId>;
    /// check if suppliers exists
    fn has(&self, supplier_ids: &[SupplierId]) -> Result<bool>;
    /// get all suppliers
    fn list(&self) -> Result<Vec<Supplier>>;
    /// get a supplier
    fn get(&self, id: SupplierId) -> Result<Option<Supplier>>;
    /// find suppliers
    fn find(&self, query: FindSupplierQuery) -> Result<Vec<Supplier>>;
    /// add a new supplier
    fn add(&self, supplier: Supplier) -> Result<()>;
    /// save changes of a supplier
    fn save(&self, supplier: Supplier) -> Result<()>;
    /// delete supplier
    fn delete(&self, id: SupplierId) -> Result<()>;
}

/// persistence implementations for journal
pub trait ForJournalPersistence {
    /// get next journal id
    fn next_id(&self) -> Result<JournalId>;
    /// get all journals
    fn list(&self) -> Result<Vec<Journal>>;
    /// get a journal
    fn get(&self, id: JournalId) -> Result<Option<Journal>>;
    /// find journals
    fn find(&self, query: FindJournalsQuery) -> Result<Vec<Journal>>;
    /// add a new journal
    fn add(&self, journal: Journal) -> Result<()>;
    /// save changes of a journal
    fn save(&self, journal: Journal) -> Result<()>;
    /// delete journal
    fn delete(&self, id: JournalId) -> Result<()>;
}

/// persistence implementations for stocktaking
pub trait ForStocktakingPersistence {
    /// get next stocktaking id
    fn next_id(&self) -> Result<StocktakingId>;
    /// get all stocktakings
    fn list(&self) -> Result<Vec<Stocktaking>>;
    /// find stocktakings
    fn find(&self, query: FindStocktakingsQuery) -> Result<Vec<Stocktaking>>;
    /// get a stocktaking
    fn get(&self, id: StocktakingId) -> Result<Option<Stocktaking>>;
    /// add a new stocktaking
    fn add(&self, stocktaking: Stocktaking) -> Result<()>;
    /// save changes of a stocktaking
    fn save(&self, stocktaking: Stocktaking) -> Result<()>;
    /// delete stocktaking
    fn delete(&self, id: StocktakingId) -> Result<()>;
}
