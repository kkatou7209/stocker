use std::hash::Hash;

use crate::core::domain::values::stock::*;

#[derive(Debug, Clone, Eq)]
pub struct Supply {
    id: SupplyId,
    name: SupplyName,
    unit_name: UnitName,
    supplier_id: SupplierId,
}

impl Supply {
    pub fn new(
        id: SupplyId,
        name: SupplyName,
        unit_name: UnitName,
        supplier_id: SupplierId,
    ) -> Self {
        Self {
            id,
            name,
            unit_name,
            supplier_id,
        }
    }

    pub fn id(&self) -> &SupplyId {
        &self.id
    }

    pub fn name(&self) -> &SupplyName {
        &self.name
    }

    pub fn unit_name(&self) -> &UnitName {
        &self.unit_name
    }

    pub fn rename(&mut self, name: SupplyName) {
        self.name = name;
    }

    pub fn supplier_id(&self) -> &SupplierId {
        &self.supplier_id
    }

    pub fn rename_unit(&mut self, name: UnitName) {
        self.unit_name = name;
    }

    pub fn change_supplier(&mut self, supplier_id: SupplierId) {
        self.supplier_id = supplier_id
    }
}

impl PartialEq for Supply {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for Supply {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Debug, Clone, Eq)]
pub struct Supplier {
    id: SupplierId,
    name: SupplierName,
}

impl Supplier {
    pub fn restore(id: SupplierId, name: SupplierName) -> Self {
        Self { id, name: name }
    }

    pub fn id(&self) -> &SupplierId {
        &self.id
    }

    pub fn name(&self) -> &SupplierName {
        &self.name
    }

    pub fn rename(&mut self, name: SupplierName) {
        self.name = name;
    }
}

impl PartialEq for Supplier {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for Supplier {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

/// Journal entity
#[derive(Debug, Clone, Eq)]
pub struct Journal {
    id: JournalId,
    entry_datetime: EntryDateTime,
    total_price: TotalPrice,
    records: Vec<JournalRecord>,
}

impl Journal {
    /// Restore a Journal entity.
    pub fn restore(
        id: JournalId,
        entry_datetime: EntryDateTime,
        total_price: TotalPrice,
        records: Vec<JournalRecord>,
    ) -> Self {
        Self {
            id,
            entry_datetime,
            total_price,
            records,
        }
    }

    /// Returns the Journal ID.
    pub fn id(&self) -> &JournalId {
        &self.id
    }

    /// Returns the entry date and time of the Journal.
    pub fn entry_datetime(&self) -> &EntryDateTime {
        &self.entry_datetime
    }

    /// Returns the total price of the Journal.
    pub fn total_price(&self) -> &TotalPrice {
        &self.total_price
    }

    /// Returns the records of the Journal.
    pub fn records(&self) -> &[JournalRecord] {
        &self.records
    }

    /// Changes the total price of the Journal.
    pub fn change_total_price(&mut self, total_price: TotalPrice) {
        self.total_price = total_price;
    }

    /// Swaps the records of the Journal with the provided records.
    pub fn swap_records(&mut self, records: Vec<JournalRecord>) {
        self.records = records.into_iter().collect();
    }
}

impl PartialEq for Journal {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for Journal {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

/// Stocktaking entity
#[derive(Debug, Clone, Eq)]
pub struct Stocktaking {
    id: StocktakingId,
    stocktaken_datetime: StocktakenDateTime,
    total_price: TotalPrice,
    records: Vec<StocktakingRecord>,
}

impl Stocktaking {
    /// Restore a Stocktaking entity.
    pub fn restore(
        id: StocktakingId,
        stocktaken_datetime: StocktakenDateTime,
        total_price: TotalPrice,
        records: Vec<StocktakingRecord>,
    ) -> Self {
        Self {
            id,
            stocktaken_datetime,
            total_price,
            records,
        }
    }

    /// Returns stocktaking ID.
    pub fn id(&self) -> &StocktakingId {
        &self.id
    }

    /// Returns the date and time when the stock was taken.
    pub fn stocktaken_at(&self) -> &StocktakenDateTime {
        &self.stocktaken_datetime
    }

    /// Returns the total price of the stocktaking.
    pub fn total_price(&self) -> &TotalPrice {
        &self.total_price
    }

    /// Returns the records of the stocktaking.
    pub fn records(&self) -> &[StocktakingRecord] {
        &self.records
    }

    /// Changes the total price of the stocktaking.
    pub fn change_total_price(&mut self, total_price: TotalPrice) {
        self.total_price = total_price;
    }

    /// Swaps the records of the stocktaking with the provided records.
    pub fn swap_records(&mut self, records: impl IntoIterator<Item = StocktakingRecord>) {
        self.records = records.into_iter().collect();
    }
}

impl PartialEq for Stocktaking {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for Stocktaking {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
