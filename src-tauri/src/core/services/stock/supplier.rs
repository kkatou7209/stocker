use std::sync::Arc;

use crate::core::domain::entities::stock::*;
use crate::core::domain::values::stock::*;
use crate::core::provided_ports::*;
use crate::core::required_ports::*;
use crate::core::*;

/// supplier usecase
pub struct SupplierService {
    supplier_repository: Arc<dyn ForSupplierPersistence>,
}

impl SupplierService {
    pub fn new(supplier_repository: Arc<dyn ForSupplierPersistence>) -> Self {
        Self {
            supplier_repository,
        }
    }
}

impl SupplierUsecase for SupplierService {
    fn get(&self, query: provided_ports::GetSupplierQuery) -> Result<Option<SupplierDTO>> {
        let supplier_id = SupplierId::new(query.supplier_id)?;

        let supplier = self.supplier_repository.get(supplier_id)?;

        if supplier.is_none() {
            return Ok(None);
        }

        let supplier = supplier.unwrap();

        let supplier = SupplierDTO {
            id: supplier.id().to_string(),
            name: supplier.name().to_string(),
        };

        Ok(Some(supplier))
    }

    fn list(&self) -> Result<Vec<SupplierDTO>> {
        let suppliers = self.supplier_repository.list()?;

        let suppliers: Vec<SupplierDTO> = suppliers
            .iter()
            .map(|supplier| SupplierDTO {
                id: supplier.id().to_string(),
                name: supplier.name().to_string(),
            })
            .collect();

        Ok(suppliers)
    }

    fn search(&self, query: SearchSuppliersQuery) -> Result<Vec<SupplierDTO>> {
        let query = required_ports::FindSupplierQuery {
            supplier_name: query
                .supplier_name
                .and_then(|name| {
                    if name.trim().is_empty() {
                        None
                    } else {
                        Some(SupplierName::new(name))
                    }
                })
                .transpose()?,
            supply_name: query
                .supply_name
                .and_then(|name| {
                    if name.trim().is_empty() {
                        None
                    } else {
                        Some(SupplyName::new(name))
                    }
                })
                .transpose()?,
        };

        let suppliers = self.supplier_repository.find(query)?;

        let suppliers: Vec<SupplierDTO> = suppliers
            .iter()
            .map(|supplier| SupplierDTO {
                id: supplier.id().to_string(),
                name: supplier.name().to_string(),
            })
            .collect();

        Ok(suppliers)
    }

    fn register(&self, command: RegisterSupplierCommand) -> Result<SupplierDTO> {
        let id = self.supplier_repository.next_id()?;

        let supplier = Supplier::restore(id, SupplierName::new(command.supplier_name)?);

        self.supplier_repository.add(supplier.clone())?;

        let supplier = SupplierDTO {
            id: supplier.id().to_string(),
            name: supplier.name().to_string(),
        };

        Ok(supplier)
    }

    fn update(&self, command: UpdateSupplierCommand) -> Result<()> {
        let supplier_id = SupplierId::new(command.supplier_id)?;

        let mut supplier = self
            .supplier_repository
            .get(supplier_id)?
            .ok_or(Error::DomainError(format!("suppler does not exist.")))?;

        supplier.rename(SupplierName::new(command.supplier_name)?);

        self.supplier_repository.save(supplier)?;

        Ok(())
    }
}
