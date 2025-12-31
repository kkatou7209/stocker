use crate::core::domain::values::stock::*;

#[derive(Debug, Clone)]
pub struct FindSupplierQuery {
    pub supplier_name: Option<SupplierName>,
    pub supply_name: Option<SupplyName>,
}

#[derive(Debug, Clone)]
pub struct FindJournalsQuery {
    pub period_start: Option<EntryDateTime>,
    pub period_end: Option<EntryDateTime>,
    pub supplier_name: Option<SupplierName>,
    pub supply_name: Option<SupplyName>,
}

pub struct FindStocktakingsQuery {
    pub period_start: Option<StocktakenDateTime>,
    pub period_end: Option<StocktakenDateTime>,
}
