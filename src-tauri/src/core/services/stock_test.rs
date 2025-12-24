use crate::core::domain::entities::stock::*;
use crate::core::domain::values::stock::*;
use crate::core::provided_ports;
use crate::core::provided_ports::*;
use crate::core::required_ports::{mock::*, *};
use crate::core::services::stock::{SupplierService, SupplyService};
use std::sync::{Arc, Mutex};

#[test]
fn supply_service_test() {
    let storage = Arc::new(Mutex::new(Storage::default()));

    let supplier_repository = MockSupplierRepository::new(Arc::clone(&storage));

    supplier_repository
        .add(Supplier::restore(
            SupplierId::new("1").unwrap(),
            SupplierName::new("SupplierA").unwrap(),
        ))
        .unwrap();

    let service = SupplyService::new(
        Arc::new(MockSupplyRepository::new(Arc::clone(&storage))),
        Arc::new(supplier_repository),
    );

    let create_command = CreateSupplyCommand {
        supplier_id: "1".into(),
        supply_name: "SupplyA".into(),
        unit_name: "g".into(),
    };

    let result = service.register(create_command);

    assert!(result.is_ok());

    let supply = result.unwrap();

    assert_eq!(
        supply,
        SupplyDTO {
            id: "1".into(),
            name: "SupplyA".into(),
            unit_name: "g".into(),
            supplier_id: "1".into(),
        }
    );

    let result = service.list();

    assert!(result.is_ok());

    let supplies = result.unwrap();

    assert_eq!(
        supplies,
        vec![SupplyDTO {
            id: "1".into(),
            name: "SupplyA".into(),
            unit_name: "g".into(),
            supplier_id: "1".into(),
        }]
    );

    let result = service.get(provided_ports::GetSupplyQuery {
        supply_id: "1".into(),
    });

    assert!(result.is_ok());

    let supply = result.unwrap();

    assert_eq!(
        supply,
        Some(SupplyDTO {
            id: "1".into(),
            name: "SupplyA".into(),
            unit_name: "g".into(),
            supplier_id: "1".into(),
        })
    );

    let result = service.search(SearchSuppliesQuery {
        supply_name: Some("A".into()),
        supplier_name: Some("A".into()),
    });

    assert!(result.is_ok());

    let supplies = result.unwrap();

    assert_eq!(supplies.len(), 1);

    assert_eq!(
        supplies,
        vec![SupplyDTO {
            id: "1".into(),
            name: "SupplyA".into(),
            unit_name: "g".into(),
            supplier_id: "1".into(),
        }]
    );

    let result = service.update(UpdateSupplyCommand {
        supply_id: "1".into(),
        supply_name: "SupplyB".into(),
        unit_name: "kg".into(),
        supplier_id: "1".into(),
    });

    assert!(result.is_ok());

    let supply = service
        .get(provided_ports::GetSupplyQuery {
            supply_id: "1".into(),
        })
        .unwrap();

    assert_eq!(
        supply,
        Some(SupplyDTO {
            id: "1".into(),
            name: "SupplyB".into(),
            unit_name: "kg".into(),
            supplier_id: "1".into(),
        })
    );

    let result = service.update(UpdateSupplyCommand {
        supply_id: "2".into(),
        supply_name: "SupplyB".into(),
        unit_name: "kg".into(),
        supplier_id: "1".into(),
    });

    assert!(result.is_err());

    let result = service.update(UpdateSupplyCommand {
        supply_id: "1".into(),
        supply_name: "SupplyB".into(),
        unit_name: "kg".into(),
        supplier_id: "2".into(),
    });

    assert!(result.is_err());
}

#[test]
fn supplier_service() {
    let storage = Arc::new(Mutex::new(Storage::default()));

    let service = SupplierService::new(Arc::new(MockSupplierRepository::new(Arc::clone(&storage))));

    service
        .register(RegisterSupplierCommand {
            supplier_name: "SupplierA".into(),
        })
        .unwrap();

    let suppliers = service.list().unwrap();

    assert_eq!(
        suppliers,
        vec![SupplierDTO {
            id: "1".into(),
            name: "SupplierA".into(),
        }]
    );
}
