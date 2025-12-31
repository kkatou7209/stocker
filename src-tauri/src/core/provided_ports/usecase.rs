use super::dto::*;
use crate::core::Result;

pub trait SupplyUsecase {
    fn list(&self) -> Result<Vec<SupplyDTO>>;
    fn get(&self, query: GetSupplyQuery) -> Result<Option<SupplyDTO>>;
    fn get_of_supplier(&self, supplier_id: String) -> Result<Vec<SupplyDTO>>;
    fn register(&self, command: CreateSupplyCommand) -> Result<SupplyDTO>;
    fn update(&self, commad: UpdateSupplyCommand) -> Result<()>;
}

pub trait SupplierUsecase {
    fn list(&self) -> Result<Vec<SupplierDTO>>;
    fn get(&self, query: GetSupplierQuery) -> Result<Option<SupplierDTO>>;
    fn search(&self, query: SearchSuppliersQuery) -> Result<Vec<SupplierDTO>>;
    fn register(&self, command: RegisterSupplierCommand) -> Result<SupplierDTO>;
    fn update(&self, command: UpdateSupplierCommand) -> Result<()>;
}

pub trait StocktakingUsecase {
    fn list(&self) -> Result<Vec<StocktakingDTO>>;
    fn get(&self, query: GetStocktakingQuery) -> Result<Option<StocktakingDTO>>;
    fn search(&self, query: SearchStocktakingQuery) -> Result<Vec<StocktakingDTO>>;
    fn record(&self, command: RecordStocktakingCommand) -> Result<StocktakingDTO>;
    fn edit(&self, command: EditStocktakingCommand) -> Result<()>;
}

pub trait JournalUsecase {
    fn list(&self) -> Result<Vec<JournalDTO>>;
    fn get(&self, query: GetJournalQuery) -> Result<Option<JournalDTO>>;
    fn search(&self, query: SearchJournalsQuery) -> Result<Vec<JournalDTO>>;
    fn record(&self, command: RecordJournalCommand) -> Result<JournalDTO>;
    fn edit(&self, command: EditJournalCommand) -> Result<()>;
}
