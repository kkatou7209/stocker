use std::ops::{Add, Mul};

use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
pub struct SupplierId {
    value: String,
}

impl SupplierId {
    pub fn new(value: impl AsRef<String>) -> Result<Self> {
        let value = value.as_ref().trim().to_string();

        if value.is_empty() {
            return Err(anyhow!("empty string cannot be set"));
        }

        return Ok(Self { value });
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

#[derive(Debug, Clone)]
pub struct SupplierName {
    value: String,
}

impl SupplierName {
    pub fn new(value: impl AsRef<String>) -> Result<Self> {
        let value = value.as_ref().trim().to_string();

        if value.is_empty() {
            return Err(anyhow!("empty string cannot be set"));
        }

        return Ok(Self { value });
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

#[derive(Debug, Clone)]
pub struct SupplyId {
    value: String,
}

impl SupplyId {
    pub fn new(value: impl AsRef<String>) -> Result<Self> {
        let value = value.as_ref().trim().to_string();

        if value.is_empty() {
            return Err(anyhow!("empty string cannot be set"));
        }

        return Ok(SupplyId { value });
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

#[derive(Debug, Clone)]
pub struct SupplyName {
    value: String,
}

impl SupplyName {
    pub fn new(value: impl AsRef<String>) -> Result<Self> {
        let value = value.as_ref().trim().to_string();

        if value.is_empty() {
            return Err(anyhow!("empty string cannot be set"));
        }

        return Ok(Self { value });
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

#[derive(Debug, Clone)]
pub struct JournalId {
    value: String,
}

impl JournalId {
    pub fn new(value: impl AsRef<String>) -> Result<Self> {
        let value = value.as_ref().trim().to_string();

        if value.is_empty() {
            return Err(anyhow!("empty string cannot be set"));
        }

        return Ok(Self { value });
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct UnitName {
    value: String,
}

impl UnitName {
    pub fn new(value: impl AsRef<String>) -> Result<Self> {
        let value = value.as_ref().trim().to_string();

        if value.is_empty() {
            return Err(anyhow!("empty string cannot be set"));
        }

        return Ok(Self { value });
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

#[derive(Debug, Clone)]
pub struct PurchaseUnitPrice {
    value: u32,
}

impl PurchaseUnitPrice {
    pub fn new(value: impl Into<u32>) -> Result<Self> {
        let value = value.into();

        return Ok(Self { value });
    }

    pub fn as_u32(&self) -> u32 {
        self.value
    }
}

#[derive(Debug, Clone)]
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

impl Mul<PurchaseUnitPrice> for PurchaseQuantity {
    type Output = PurchasePrice;

    fn mul(self, rhs: PurchaseUnitPrice) -> Self::Output {
        PurchasePrice::new(self.value * rhs.as_u32()).unwrap()
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

impl Add for PurchasePrice {
    type Output = PurchasePrice;

    fn add(self, rhs: Self) -> Self::Output {
        PurchasePrice::new(self.value + rhs.value).unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct StocktakingId {
    value: String,
}

impl StocktakingId {
    pub fn new(value: impl AsRef<String>) -> Result<Self> {
        let value = value.as_ref().trim().to_string();

        if value.is_empty() {
            return Err(anyhow!("empty string cannot be set"));
        }

        return Ok(Self { value });
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

#[derive(Debug, Clone)]
pub struct StocktakingDateTime {
    value: i64,
}

impl StocktakingDateTime {
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

    pub fn as_f64(&self) -> u32 {
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

    pub fn as_f64(&self) -> u32 {
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

        return Ok(Self { value });
    }

    pub fn as_f64(&self) -> u32 {
        self.value
    }
}
