use crate::core::Result;

use super::dto::*;

pub trait ForListingSupplies {
    /// list all supplies
    fn list(&self) -> Result<Vec<SupplyDTO>>;
}

pub trait ForGettingSupply {
    /// get a supply
    fn get(&self, query: GetSupplyQuery) -> Result<Option<SupplyDTO>>;
}

pub trait ForSearchingSupplies {
    /// search supplies
    fn search(&self, query: SearchSuppliesQuery) -> Result<Vec<SupplyDTO>>;
}

pub trait ForRegisteringSupply {
    /// create a new supply
    fn register(&self, command: CreateSupplyCommand) -> Result<SupplyDTO>;
}

pub trait ForUpdatingSupply {
    /// update a supply
    fn update(&self, commad: UpdateSupplyCommand) -> Result<()>;
}

pub trait ForListingSuppliers {
    /// list suppliers
    fn list(&self) -> Result<Vec<SupplierDTO>>;
}

pub trait ForGettingSupplier {
    fn get(&self, query: GetSupplierQuery) -> Result<Option<SupplierDTO>>;
}

pub trait ForSearchingSuppliers {
    fn search(&self, query: SearchSuppliersQuery) -> Result<Vec<SupplierDTO>>;
}

pub trait ForRegisteringSupplier {
    /// create supplier
    fn register(&self, command: RegisterSupplierCommand) -> Result<SupplierDTO>;
}

pub trait ForUpdatingSupplier {
    /// update supplier
    fn update(&self, command: UpdateSupplierCommand) -> Result<()>;
}

pub trait ForListingStocktakings {
    /// list all stocktakings
    fn list(&self) -> Result<Vec<StocktakingDTO>>;
}

pub trait ForGettingStocktaking {
    /// get a stocktaking by id
    fn get(&self, query: GetStocktakingQuery) -> Result<Option<StocktakingDTO>>;
}

pub trait ForSearchingStocktakings {
    /// search stocktakings
    fn search(&self, command: SearchStocktakingQuery) -> Result<Vec<StocktakingDTO>>;
}

pub trait ForRecordStocktaking {
    /// create a new stocktaking
    fn record(&self, command: RecordStocktakingCommand) -> Result<StocktakingDTO>;
}

pub trait ForEditingStocktaking {
    /// update stocktaking
    fn edit(&self, command: EditStocktakingCommand) -> Result<()>;
}

pub trait ForListingJournals {
    /// list all journals
    fn list(&self) -> Result<Vec<JournalDTO>>;
}

pub trait ForGettingJournal {
    /// get a journal by id
    fn get(&self, query: GetJournalQuery) -> Result<Option<JournalDTO>>;
}

pub trait ForSearchingJournals {
    /// search journals
    fn search(&self, query: SearchJournalsQuery) -> Result<Vec<JournalDTO>>;
}

pub trait ForRecordingJournal {
    /// create a new journal
    fn record(&self, command: RecordJournalCommand) -> Result<JournalDTO>;
}

pub trait ForEditingJournal {
    /// update journal
    fn edit(&self, command: EditJournalCommand) -> Result<()>;
}
