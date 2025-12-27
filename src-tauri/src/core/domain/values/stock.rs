use std::ops::{Add, Mul};

use crate::core::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
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
    ) -> Self {
        Self {
            supply_id,
            supply_name,
            supplier_id,
            suplier_name,
            unit_name,
            unit_price,
            quantity,
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
    pub fn new(value: impl Into<u32>) -> Result<Self> {
        let value = value.into();

        if value <= 0 {
            return Err(Error::DomainError(format!("price must be greater than 0")));
        }

        Ok(Self { value })
    }

    pub fn as_u32(&self) -> u32 {
        self.value
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PurchaseQuantity {
    value: u32,
}

impl PurchaseQuantity {
    pub fn new(value: impl Into<u32>) -> Result<Self> {
        let value = value.into();

        return Ok(Self { value });
    }

    pub fn as_u32(&self) -> u32 {
        self.value
    }
}

#[derive(Debug, Clone)]
pub struct PurchasePrice {
    value: u32,
}

impl PurchasePrice {
    pub fn new(value: impl Into<u32>) -> Result<Self> {
        let value = value.into();

        return Ok(Self { value });
    }

    pub fn as_u32(&self) -> u32 {
        self.value
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone)]
pub struct StocktakingQuantity {
    value: u32,
}

impl StocktakingQuantity {
    pub fn new(value: impl Into<u32>) -> Result<Self> {
        let value = value.into();

        return Ok(Self { value });
    }

    pub fn as_u32(&self) -> u32 {
        self.value
    }
}

#[derive(Debug, Clone)]
pub struct StocktakingUnitPrice {
    value: u32,
}

impl StocktakingUnitPrice {
    pub fn new(value: impl Into<u32>) -> Result<Self> {
        let value = value.into();

        return Ok(Self { value });
    }

    pub fn as_u32(&self) -> u32 {
        self.value
    }
}

#[derive(Debug, Clone)]
pub struct StocktakingPrice {
    value: u32,
}

impl StocktakingPrice {
    pub fn new(value: impl Into<u32>) -> Result<Self> {
        let value = value.into();

        if value <= 0 {
            return Err(Error::DomainError(format!("price must be greater than 0")));
        }

        return Ok(Self { value });
    }

    pub fn as_f64(&self) -> u32 {
        self.value
    }
}

#[derive(Debug, Clone)]
pub struct StocktakingRecord {
    supply_id: SupplyId,
    supply_name: SupplyName,
    unit_name: UnitName,
    unit_price: StocktakingUnitPrice,
    quantity: StocktakingQuantity,
}

impl StocktakingRecord {
    pub fn new(
        supply_id: SupplyId,
        supply_name: SupplyName,
        unit_name: UnitName,
        unit_price: StocktakingUnitPrice,
        quantity: StocktakingQuantity,
    ) -> Self {
        Self {
            supply_id,
            supply_name,
            unit_name,
            unit_price,
            quantity,
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
}
