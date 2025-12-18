#[derive(Debug, Clone)]
pub struct SupplyDTO {
    pub id: String,
    pub name: String,
    pub unit_name: String,
    pub supplier_id: String,
}

#[derive(Debug, Clone)]
pub struct SupplierDTO {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct StocktakingDTO {
    pub id: String,
    pub stocktaking_date: String,
    pub records: Vec<StocktakingRecordDTO>,
}

#[derive(Debug, Clone)]
pub struct StocktakingRecordDTO {
    pub supply_id: String,
    pub supply_name: String,
    pub supplier_id: String,
    pub supplier_name: String,
    pub unit_name: String,
    pub unit_price: u32,
    pub quantity: f64,
    pub total_price: u32,
}

#[derive(Debug, Clone)]
pub struct JournalDTO {
    pub id: String,
    pub entry_date: String,
    pub records: Vec<JournalRecordDTO>,
}

#[derive(Debug, Clone)]
pub struct JournalRecordDTO {
    pub supply_id: String,
    pub supply_name: String,
    pub supplier_id: String,
    pub supplier_name: String,
    pub unit_name: String,
    pub unit_price: u32,
    pub quantity: f64,
    pub total_price: u32,
}

pub trait ForListingSupplies {
    /// list all supplies
    fn list() -> Vec<SupplyDTO>;
}

#[derive(Debug, Clone)]
pub struct GetSupplyQuery {
    pub supply_id: String,
}

pub trait ForGettingSupply {
    /// get a supply
    fn get(query: GetSupplyQuery) -> Option<SupplyDTO>;
}

#[derive(Debug, Clone)]
pub struct SearchSuppliesQuery {
    pub supply_name: String,
    pub supplier_name: String,
}

pub trait ForSearchingSupplies {
    /// search supplies
    fn search(query: SearchSuppliesQuery) -> Vec<SupplyDTO>;
}

#[derive(Debug, Clone)]
pub struct CreateSupplyCommand {
    pub supply_name: String,
    pub unit_name: String,
}

pub trait ForCreatingSupply {
    /// create a new supply
    fn create(command: CreateSupplyCommand) -> SupplyDTO;
}

#[derive(Debug, Clone)]
pub struct UpdateSupplyCommand {
    pub supply_id: String,
    pub supply_name: String,
    pub unit_name: String,
}

pub trait ForUpdatingSupply {
    /// update a supply
    fn update(commad: UpdateSupplyCommand) -> ();
}

pub trait ForListingSuppliers {
    /// list suppliers
    fn list() -> Vec<SupplierDTO>;
}

#[derive(Debug, Clone)]
pub struct GetSupplierQuery {
    pub supplier_id: String,
}

pub trait ForGettingSupplier {
    fn get(query: GetSupplierQuery) -> Option<SupplierDTO>;
}

#[derive(Debug, Clone)]
pub struct SearchSuppliersQuery {
    pub supplier_name: String,
}

pub trait ForSearchingSuppliers {
    fn search(query: SearchSuppliersQuery) -> Vec<SupplierDTO>;
}

#[derive(Debug, Clone)]
pub struct RegisterSupplierCommand {
    pub supplier_name: String,
}

pub trait ForRegisteringSupplier {
    /// create supplier
    fn register(command: RegisterSupplierCommand) -> SupplierDTO;
}

#[derive(Debug, Clone)]
pub struct UpdateSupplierCommand {
    pub supplier_id: String,
    pub supplier_name: String,
}

pub trait ForUpdatingSupplier {
    /// update supplier
    fn update(command: UpdateSupplierCommand) -> ();
}

pub trait ForListingStocktakings {
    /// list all stocktakings
    fn list() -> Vec<StocktakingDTO>;
}

#[derive(Debug, Clone)]
pub struct GetStocktakingQuery {
    pub stocktaking_id: String,
}

pub trait ForGettingStocktaking {
    /// get a stocktaking by id
    fn get(query: GetStocktakingQuery) -> Option<StocktakingDTO>;
}

#[derive(Debug, Clone)]
pub struct SearchStocktakingQuery {
    pub period_start: String,
    pub period_end: String,
}

pub trait ForSearchingStocktakings {
    /// search stocktakings
    fn search(command: SearchStocktakingQuery) -> Vec<StocktakingDTO>;
}

#[derive(Debug, Clone)]
pub struct RecordStocktakingCommand {
    pub supply_id: String,
    pub supply_name: String,
    pub supplier_id: String,
    pub supplier_name: String,
    pub unit_name: String,
    pub unit_price: u32,
    pub quantity: f64,
    pub total_price: u32,
}

pub trait ForRecordStocktaking {
    /// create a new stocktaking
    fn record(command: RecordStocktakingCommand) -> StocktakingDTO;
}

#[derive(Debug, Clone)]
pub struct EditStocktakingCommand {
    pub stocktaking_id: String,
    pub supply_id: String,
    pub supply_name: String,
    pub supplier_id: String,
    pub supplier_name: String,
    pub unit_name: String,
    pub unit_price: u32,
    pub quantity: f64,
    pub total_price: u32,
}

pub trait ForEditingStocktaking {
    /// update stocktaking
    fn edit(command: EditStocktakingCommand) -> ();
}

pub trait ForListingJournals {
    /// list all journals
    fn list() -> Vec<JournalDTO>;
}

#[derive(Debug, Clone)]
pub struct GetJournalQuery {
    pub journal_id: String,
}

pub trait ForGettingJournal {
    /// get a journal by id
    fn get(query: GetJournalQuery) -> Option<JournalDTO>;
}

#[derive(Debug, Clone)]
pub struct SearchJournalsQuery {
    pub period_start: String,
    pub period_end: String,
    pub supplier_name: String,
    pub supply_name: String,
}

pub trait ForSearchingJournals {
    /// search journals
    fn search(query: SearchJournalsQuery) -> Vec<JournalDTO>;
}

#[derive(Debug, Clone)]
pub struct RecordJournalCommand {
    pub stocktaking_id: String,
    pub supply_id: String,
    pub supply_name: String,
    pub supplier_id: String,
    pub supplier_name: String,
    pub unit_name: String,
    pub unit_price: u32,
    pub quantity: f64,
    pub total_price: u32,
}

pub trait ForRecordingJournal {
    /// create a new journal
    fn record(command: RecordJournalCommand) -> JournalDTO;
}

#[derive(Debug, Clone)]
pub struct EditJournalCommand {
    pub journal_id: String,
    pub stocktaking_id: String,
    pub supply_id: String,
    pub supply_name: String,
    pub supplier_id: String,
    pub supplier_name: String,
    pub unit_name: String,
    pub unit_price: u32,
    pub quantity: f64,
    pub total_price: u32,
}

pub trait ForEditingJournal {
    /// update journal
    fn edit(command: EditJournalCommand) -> ();
}
