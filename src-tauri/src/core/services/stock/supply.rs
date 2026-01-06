//! This module provides the implementation for `SupplyUsecase`.
use std::sync::Arc;

use crate::core::domain::entities::stock::*;
use crate::core::domain::values::stock::*;
use crate::core::provided_ports::*;
use crate::core::required_ports::*;
use crate::core::*;

/// Supply usecase
pub struct SupplyService {
    supply_repository: Arc<dyn ForSupplyPersistence>,
    supplier_repository: Arc<dyn ForSupplierPersistence>,
}

impl SupplyService {
    pub fn new(
        supply_repository: Arc<dyn ForSupplyPersistence>,
        supplier_repository: Arc<dyn ForSupplierPersistence>,
    ) -> Self {
        Self {
            supply_repository,
            supplier_repository,
        }
    }
}

/// Supply usecase implementation
impl SupplyUsecase for SupplyService {
    fn get(&self, supply_id: impl AsRef<str>) -> Result<Option<SupplyDTO>> {
        let supply_id = SupplyId::new(supply_id.as_ref())?;

        let supply = self.supply_repository.get(supply_id)?;

        if supply.is_none() {
            return Ok(None);
        }

        let supply = supply.unwrap();

        Ok(Some(SupplyDTO {
            id: supply.id().to_string(),
            name: supply.name().to_string(),
            unit_name: supply.unit_name().to_string(),
            supplier_id: supply.supplier_id().to_string(),
        }))
    }

    fn get_of_supplier(&self, supplier_id: String) -> Result<Vec<SupplyDTO>> {
        let supplier_id = SupplierId::new(supplier_id)?;

        let supplies = self.supply_repository.list_of_supplier(supplier_id)?;

        let supplies: Vec<SupplyDTO> = supplies
            .iter()
            .map(|supply| SupplyDTO {
                id: supply.id().to_string(),
                name: supply.name().to_string(),
                unit_name: supply.unit_name().to_string(),
                supplier_id: supply.supplier_id().to_string(),
            })
            .collect();

        Ok(supplies)
    }

    fn list(&self) -> Result<Vec<SupplyDTO>> {
        let supplies = self.supply_repository.list()?;

        let supplies: Vec<SupplyDTO> = supplies
            .iter()
            .map(|supply| SupplyDTO {
                id: supply.id().to_string(),
                name: supply.name().to_string(),
                unit_name: supply.unit_name().to_string(),
                supplier_id: supply.supplier_id().to_string(),
            })
            .collect();

        Ok(supplies)
    }

    fn register(&self, command: CreateSupplyCommand) -> Result<SupplyDTO> {
        let id = self.supply_repository.next_id()?;

        let supplier_id = SupplierId::new(command.supplier_id)?;

        let supplier = self
            .supplier_repository
            .get(supplier_id)?
            .ok_or(Error::DomainError(format!("supplier does not exist.")))?;

        let supply = Supply::new(
            id,
            SupplyName::new(command.supply_name)?,
            UnitName::new(command.unit_name)?,
            supplier.id().clone(),
        );

        self.supply_repository.add(supply.clone())?;

        Ok(SupplyDTO {
            id: supply.id().to_string(),
            name: supply.name().to_string(),
            unit_name: supply.unit_name().to_string(),
            supplier_id: supply.supplier_id().to_string(),
        })
    }

    fn update(&self, commad: UpdateSupplyCommand) -> Result<()> {
        let supply_id = SupplyId::new(commad.supply_id)?;

        let mut supply = self
            .supply_repository
            .get(supply_id)?
            .ok_or(Error::DomainError(format!("supply does not exist.")))?;

        let supplier_id = SupplierId::new(commad.supplier_id)?;

        let supplier = self
            .supplier_repository
            .get(supplier_id)?
            .ok_or(Error::DomainError(format!("supplier does not exist.")))?;

        supply.rename(SupplyName::new(commad.supply_name)?);
        supply.rename_unit(UnitName::new(commad.unit_name)?);
        supply.change_supplier(supplier.id().clone());

        self.supply_repository.save(supply)?;

        Ok(())
    }

    fn delete(&self, supply_id: impl AsRef<str>) -> Result<()> {
        let supply_id = SupplyId::new(supply_id.as_ref())?;

        self.supply_repository.delete(supply_id)?;

        Ok(())
    }
}
