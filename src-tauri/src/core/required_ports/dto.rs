use crate::core::domain::values::stock::*;

#[derive(Debug, Clone)]
pub struct GetSupplyQuery {
    pub supply_id: SupplyId,
}

#[derive(Debug, Clone)]
pub struct FindSuppliesQuery {
    pub supply_name: Option<SupplyName>,
    pub supplier_name: Option<SupplierName>,
}

#[derive(Debug, Clone)]
pub struct GetSupplierQuery {
    pub supplier_id: SupplierId,
}

#[derive(Debug, Clone)]
pub struct FindSupplierQuery {
    pub supplier_name: Option<SupplierName>,
    pub supply_name: Option<SupplyName>,
}

#[derive(Debug, Clone)]
pub struct GetJournalQuery {
    pub journal_id: JournalId,
}

#[derive(Debug, Clone)]
pub struct FindJournalsQuery {
    pub period_start: Option<EntryDateTime>,
    pub period_end: Option<EntryDateTime>,
    pub supplier_name: Option<SupplierName>,
    pub supply_name: Option<SupplyName>,
}

pub struct GetStocktakingQuery {
    pub stocktaking_id: StocktakingId,
}

pub struct FindStocktakingsQuery {
    pub period_start: Option<StocktakenDateTime>,
    pub period_end: Option<StocktakenDateTime>,
}
