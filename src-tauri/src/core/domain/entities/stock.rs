use crate::core::domain::values::stock::{
    SupplierId, SupplierName, SupplyId, SupplyName, UnitName,
};

#[derive(Debug, Clone)]
pub struct Supply {
    id: SupplyId,
    name: SupplyName,
    unit_name: UnitName,
}

impl Supply {
    pub fn restore(id: SupplyId, name: SupplyName, unit_name: UnitName) -> Self {
        Self {
            id,
            name,
            unit_name,
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

    pub fn rename_unit(&mut self, name: UnitName) {
        self.unit_name = name;
    }
}

#[derive(Debug, Clone)]
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
