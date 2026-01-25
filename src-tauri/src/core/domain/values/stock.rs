use crate::core::{Error, Result};

/// Number of digits for guaranteed price precision.
pub const GUARANTEED_DECIMAL_PRECISION: u32 = 2;

/// ID of supplier.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SupplierId {
    value: String,
}

impl SupplierId {
    pub fn new(value: impl AsRef<str>) -> Result<Self> {
        let value = value.as_ref().trim().to_string();

        if value.is_empty() {
            return Err(Error::DomainError(format!("empty string cannot be set.")));
        }

        return Ok(Self { value });
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl ToString for SupplierId {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SupplierName {
    value: String,
}

impl SupplierName {
    pub fn new(value: impl AsRef<str>) -> Result<Self> {
        let value = value.as_ref().trim().to_string();

        if value.is_empty() {
            return Err(Error::DomainError(format!("empty string cannot be set")));
        }

        return Ok(Self { value });
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl ToString for SupplierName {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SupplyId {
    value: String,
}

impl SupplyId {
    pub fn new(value: impl AsRef<str>) -> Result<Self> {
        let value = value.as_ref().trim().to_string();

        if value.is_empty() {
            return Err(Error::DomainError(format!("empty string cannot be set")));
        }

        return Ok(SupplyId { value });
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl ToString for SupplyId {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SupplyName {
    value: String,
}

impl SupplyName {
    pub fn new(value: impl AsRef<str>) -> Result<Self> {
        let value = value.as_ref().trim().to_string();

        if value.is_empty() {
            return Err(Error::DomainError(format!("empty string cannot be set")));
        }

        return Ok(Self { value });
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl ToString for SupplyName {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JournalId {
    value: String,
}

impl JournalId {
    pub fn new(value: impl AsRef<str>) -> Result<Self> {
        let value = value.as_ref().trim().to_string();

        if value.is_empty() {
            return Err(Error::DomainError(format!("empty string cannot be set")));
        }

        return Ok(Self { value });
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl ToString for JournalId {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JournalRecord {
    supply_id: SupplyId,
    supply_name: SupplyName,
    supplier_id: SupplierId,
    suplier_name: SupplierName,
    unit_name: UnitName,
    unit_price: PurchaseUnitPrice,
    quantity: PurchaseQuantity,
    total_price: TotalPrice,
}

impl JournalRecord {
    pub fn new(
        supply_id: SupplyId,
        supply_name: SupplyName,
        supplier_id: SupplierId,
        suplier_name: SupplierName,
        unit_name: UnitName,
        unit_price: PurchaseUnitPrice,
        quantity: PurchaseQuantity,
        total_price: TotalPrice,
    ) -> Self {
        Self {
            supply_id,
            supply_name,
            supplier_id,
            suplier_name,
            unit_name,
            unit_price,
            quantity,
            total_price,
        }
    }

    pub fn supply_id(&self) -> &SupplyId {
        &self.supply_id
    }

    pub fn supply_name(&self) -> &SupplyName {
        &self.supply_name
    }

    pub fn supplier_id(&self) -> &SupplierId {
        &self.supplier_id
    }

    pub fn supplier_name(&self) -> &SupplierName {
        &self.suplier_name
    }

    pub fn unit_name(&self) -> &UnitName {
        &self.unit_name
    }

    pub fn unit_price(&self) -> &PurchaseUnitPrice {
        &self.unit_price
    }

    pub fn quantity(&self) -> &PurchaseQuantity {
        &self.quantity
    }

    pub fn total_price(&self) -> &TotalPrice {
        &self.total_price
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EntryDateTime {
    value: i64,
}

impl EntryDateTime {
    pub fn new(value: impl Into<i64>) -> Self {
        Self {
            value: value.into(),
        }
    }

    pub fn as_i64(&self) -> i64 {
        self.value
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitName {
    value: String,
}

impl UnitName {
    pub fn new(value: impl AsRef<str>) -> Result<Self> {
        let value = value.as_ref().trim().to_string();

        if value.is_empty() {
            return Err(Error::DomainError(format!("empty string cannot be set")));
        }

        return Ok(Self { value });
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl ToString for UnitName {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PurchaseUnitPrice {
    value: u32,
}

impl PurchaseUnitPrice {
    pub fn new(value: impl Into<f64>) -> Result<Self> {
        let value = (value.into() * (GUARANTEED_DECIMAL_PRECISION * 10) as f64) as u32;

        Ok(Self { value })
    }

    pub fn as_f64(&self) -> f64 {
        self.value as f64 / (GUARANTEED_DECIMAL_PRECISION * 10) as f64
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PurchaseQuantity {
    value: u32,
}

impl PurchaseQuantity {
    pub fn new(value: impl Into<f64>) -> Result<Self> {
        let value = (value.into() * (GUARANTEED_DECIMAL_PRECISION * 10) as f64) as u32;

        return Ok(Self { value });
    }

    pub fn as_f64(&self) -> f64 {
        self.value as f64 / (GUARANTEED_DECIMAL_PRECISION * 10) as f64
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StocktakingId {
    value: String,
}

impl StocktakingId {
    pub fn new(value: impl AsRef<str>) -> Result<Self> {
        let value = value.as_ref().trim().to_string();

        if value.is_empty() {
            return Err(Error::DomainError(format!("empty string cannot be set")));
        }

        return Ok(Self { value });
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl ToString for StocktakingId {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StocktakenDateTime {
    value: i64,
}

impl StocktakenDateTime {
    pub fn new(value: impl Into<i64>) -> Self {
        Self {
            value: value.into(),
        }
    }

    pub fn as_i64(&self) -> i64 {
        self.value
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StocktakingQuantity {
    value: u32,
}

impl StocktakingQuantity {
    pub fn new(value: impl Into<f64>) -> Result<Self> {
        let value = (value.into() * (GUARANTEED_DECIMAL_PRECISION * 10) as f64) as u32;

        return Ok(Self { value });
    }

    pub fn as_f64(&self) -> f64 {
        self.value as f64 / (GUARANTEED_DECIMAL_PRECISION * 10) as f64
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StocktakingUnitPrice {
    value: u32,
}

impl StocktakingUnitPrice {
    pub fn new(value: impl Into<f64>) -> Result<Self> {
        let value = (value.into() * (GUARANTEED_DECIMAL_PRECISION * 10) as f64) as u32;

        return Ok(Self { value });
    }

    pub fn as_f64(&self) -> f64 {
        self.value as f64 / (GUARANTEED_DECIMAL_PRECISION * 10) as f64
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StocktakingRecord {
    supply_id: SupplyId,
    supply_name: SupplyName,
    unit_name: UnitName,
    unit_price: StocktakingUnitPrice,
    quantity: StocktakingQuantity,
    total_price: TotalPrice,
}

impl StocktakingRecord {
    pub fn new(
        supply_id: SupplyId,
        supply_name: SupplyName,
        unit_name: UnitName,
        unit_price: StocktakingUnitPrice,
        quantity: StocktakingQuantity,
        total_price: TotalPrice,
    ) -> Self {
        Self {
            supply_id,
            supply_name,
            unit_name,
            unit_price,
            quantity,
            total_price,
        }
    }

    pub fn supply_id(&self) -> &SupplyId {
        &self.supply_id
    }

    pub fn supply_name(&self) -> &SupplyName {
        &self.supply_name
    }

    pub fn unit_name(&self) -> &UnitName {
        &self.unit_name
    }

    pub fn unit_price(&self) -> &StocktakingUnitPrice {
        &self.unit_price
    }

    pub fn quantity(&self) -> &StocktakingQuantity {
        &self.quantity
    }

    pub fn total_price(&self) -> &TotalPrice {
        &self.total_price
    }
}

/// The total price value object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TotalPrice {
    value: u32,
}

impl TotalPrice {
    /// Create `TotalPrice`
    pub fn new(price: impl Into<f64>) -> Result<Self> {
        let value = price.into();

        if value < 0.0 {
            panic!("The total price must be at least more than 0.");
        }

        let value = (value * (GUARANTEED_DECIMAL_PRECISION * 10) as f64) as u32;

        Ok(Self { value })
    }

    /// Returns total price value as `f64`.
    pub fn as_f64(&self) -> f64 {
        self.value as f64 / (GUARANTEED_DECIMAL_PRECISION * 10) as f64
    }
}
