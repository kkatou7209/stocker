#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupplyDTO {
    pub id: String,
    pub name: String,
    pub unit_name: String,
    pub supplier_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupplierDTO {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StocktakingDTO {
    pub id: String,
    pub stocktaken_date: i64,
    pub records: Vec<StocktakingRecordDTO>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StocktakingRecordDTO {
    pub supply_id: String,
    pub supply_name: String,
    pub unit_name: String,
    pub unit_price: u32,
    pub quantity: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JournalDTO {
    pub id: String,
    pub entry_date: i64,
    pub records: Vec<JournalRecordDTO>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JournalRecordDTO {
    pub supply_id: String,
    pub supply_name: String,
    pub supplier_id: String,
    pub supplier_name: String,
    pub unit_name: String,
    pub unit_price: u32,
    pub quantity: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetSupplyQuery {
    pub supply_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchSuppliesQuery {
    pub supply_name: Option<String>,
    pub supplier_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateSupplyCommand {
    pub supply_name: String,
    pub unit_name: String,
    pub supplier_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateSupplyCommand {
    pub supply_id: String,
    pub supply_name: String,
    pub unit_name: String,
    pub supplier_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetSupplierQuery {
    pub supplier_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchSuppliersQuery {
    pub supplier_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisterSupplierCommand {
    pub supplier_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateSupplierCommand {
    pub supplier_id: String,
    pub supplier_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetStocktakingQuery {
    pub stocktaking_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchStocktakingQuery {
    pub period_start: Option<i64>,
    pub period_end: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordStocktakingCommand {
    pub stocktaken_date: i64,
    pub records: Vec<StocktakingRecordDTO>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditStocktakingCommand {
    pub stocktaking_id: String,
    pub stocktaken_date: i64,
    pub records: Vec<StocktakingRecordDTO>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetJournalQuery {
    pub journal_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchJournalsQuery {
    pub period_start: Option<i64>,
    pub period_end: Option<i64>,
    pub supplier_name: Option<String>,
    pub supply_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordJournalCommand {
    pub entry_date: i64,
    pub records: Vec<JournalRecordDTO>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditJournalCommand {
    pub journal_id: String,
    pub entry_date: i64,
    pub records: Vec<JournalRecordDTO>,
}
