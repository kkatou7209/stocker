//! Core stock usecase traits for application
use super::dto::*;
use crate::core::Result;

/// Usecase trait for supply management
pub trait SupplyUsecase {
    /// List all supplies
    fn list(&self) -> Result<Vec<SupplyDTO>>;
    /// Get a supply
    fn get(&self, supply_id: impl AsRef<str>) -> Result<Option<SupplyDTO>>;
    /// List all supplies of a supplier
    fn get_of_supplier(&self, supplier_id: String) -> Result<Vec<SupplyDTO>>;
    /// Register a new supply
    fn register(&self, command: CreateSupplyCommand) -> Result<SupplyDTO>;
    /// Update a supply
    fn update(&self, command: UpdateSupplyCommand) -> Result<()>;
    /// Delete a supply
    fn delete(&self, supply_id: impl AsRef<str>) -> Result<()>;
}

/// Usecase trait for supplier management
pub trait SupplierUsecase {
    /// List all suppliers
    fn list(&self) -> Result<Vec<SupplierDTO>>;
    /// Get a supplier
    fn get(&self, supplier_id: impl AsRef<str>) -> Result<Option<SupplierDTO>>;
    /// Search suppliers
    fn search(&self, query: SearchSuppliersQuery) -> Result<Vec<SupplierDTO>>;
    /// Register a new supplier
    fn register(&self, command: RegisterSupplierCommand) -> Result<SupplierDTO>;
    /// Update a supplier
    fn update(&self, command: UpdateSupplierCommand) -> Result<()>;
    /// Delete a supplier
    fn delete(&self, supplier_id: impl AsRef<str>) -> Result<()>;
}

/// Usecase trait for stocktaking management
pub trait StocktakingUsecase {
    /// List all stocktakings
    fn list(&self) -> Result<Vec<StocktakingDTO>>;
    /// Get a stocktaking
    fn get(&self, stocktaking_id: impl AsRef<str>) -> Result<Option<StocktakingDTO>>;
    /// Search stocktakings
    fn search(&self, query: SearchStocktakingQuery) -> Result<Vec<StocktakingDTO>>;
    /// Record a new stocktaking
    fn record(&self, command: RecordStocktakingCommand) -> Result<StocktakingDTO>;
    /// Edit a stocktaking
    fn edit(&self, command: EditStocktakingCommand) -> Result<()>;
    /// Delete a stocktaking
    fn delete(&self, stocktaking_id: impl AsRef<str>) -> Result<()>;
}

/// Usecase trait for journal management
pub trait JournalUsecase {
    /// List all journals
    fn list(&self) -> Result<Vec<JournalDTO>>;
    /// Get a journal
    fn get(&self, journal_id: impl AsRef<str>) -> Result<Option<JournalDTO>>;
    /// Search journals
    fn search(&self, query: SearchJournalsQuery) -> Result<Vec<JournalDTO>>;
    /// Record a new journal
    fn record(&self, command: RecordJournalCommand) -> Result<JournalDTO>;
    /// Edit a journal
    fn edit(&self, command: EditJournalCommand) -> Result<()>;
    /// Delete a journal
    fn delete(&self, journal_id: impl AsRef<str>) -> Result<()>;
}
