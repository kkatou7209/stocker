use crate::core::domain::entities::stock::*;
use crate::core::domain::values::stock::*;
use crate::core::provided_ports::*;
use crate::core::required_ports::{mock::*, *};
use crate::core::services::stock::{journal::*, stocktaking::*, supplier::*, supply::*};
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

    let supply = service.register(create_command).unwrap();

    assert_eq!(
        supply,
        SupplyDTO {
            id: "1".into(),
            name: "SupplyA".into(),
            unit_name: "g".into(),
            supplier_id: "1".into(),
        }
    );

    let supplies = service.list().unwrap();

    assert_eq!(
        supplies,
        vec![SupplyDTO {
            id: "1".into(),
            name: "SupplyA".into(),
            unit_name: "g".into(),
            supplier_id: "1".into(),
        }]
    );

    let supply = service.get("1").unwrap();

    assert_eq!(
        supply,
        Some(SupplyDTO {
            id: "1".into(),
            name: "SupplyA".into(),
            unit_name: "g".into(),
            supplier_id: "1".into(),
        })
    );

    let result = service.update(UpdateSupplyCommand {
        supply_id: "1".into(),
        supply_name: "SupplyB".into(),
        unit_name: "kg".into(),
        supplier_id: "1".into(),
    });

    assert!(result.is_ok());

    let supply = service.get("1").unwrap();

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

    service.delete("1").unwrap();

    let supplies = service.list().unwrap();

    assert!(supplies.is_empty());
}

#[test]
fn supplier_service_test() {
    let storage = Arc::new(Mutex::new(Storage::default()));

    let supplier_repository = Arc::new(MockSupplierRepository::new(Arc::clone(&storage)));

    let service = SupplierService::new(supplier_repository);

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

    let supplier = service.get("1").unwrap();

    assert_eq!(
        supplier,
        Some(SupplierDTO {
            id: "1".into(),
            name: "SupplierA".into(),
        })
    );

    service
        .update(UpdateSupplierCommand {
            supplier_id: "1".into(),
            supplier_name: "SupplierB".into(),
        })
        .unwrap();

    let supplier = service.get("1").unwrap();

    assert_eq!(
        supplier,
        Some(SupplierDTO {
            id: "1".into(),
            name: "SupplierB".into(),
        })
    );

    service.delete("1").unwrap();

    let suppliers = service.list().unwrap();

    assert!(suppliers.is_empty());
}

#[test]
fn journal_service_test() {
    let storage = Arc::new(Mutex::new(Storage::default()));

    let supply_respository = MockSupplyRepository::new(Arc::clone(&storage));
    let supplier_respository = MockSupplierRepository::new(Arc::clone(&storage));
    let journal_repository = MockJournalRepository::new(Arc::clone(&storage));

    supplier_respository
        .add(Supplier::restore(
            SupplierId::new("1").unwrap(),
            SupplierName::new("SupplierA").unwrap(),
        ))
        .unwrap();

    supply_respository
        .add(Supply::new(
            SupplyId::new("1").unwrap(),
            SupplyName::new("SupplyA").unwrap(),
            UnitName::new("g").unwrap(),
            SupplierId::new("1").unwrap(),
        ))
        .unwrap();

    supply_respository
        .add(Supply::new(
            SupplyId::new("2").unwrap(),
            SupplyName::new("SupplyB").unwrap(),
            UnitName::new("g").unwrap(),
            SupplierId::new("1").unwrap(),
        ))
        .unwrap();

    let service = JournalService::new(
        Arc::new(supply_respository),
        Arc::new(supplier_respository),
        Arc::new(journal_repository),
    );

    service
        .record(RecordJournalCommand {
            entry_date: 100000,
            records: vec![
                JournalRecordDTO {
                    supply_id: "1".into(),
                    supplier_id: "1".into(),
                    supply_name: "SupplyA".into(),
                    supplier_name: "SupplierA".into(),
                    unit_name: "g".into(),
                    unit_price: 100,
                    quantity: 10.0,
                },
                JournalRecordDTO {
                    supply_id: "2".into(),
                    supplier_id: "1".into(),
                    supply_name: "SupplyB".into(),
                    supplier_name: "SupplierA".into(),
                    unit_name: "g".into(),
                    unit_price: 120,
                    quantity: 5.0,
                },
            ],
        })
        .unwrap();

    let journals = service.list().unwrap();

    assert_eq!(
        journals,
        vec![JournalDTO {
            id: "1".into(),
            entry_date: 100000,
            records: vec![
                JournalRecordDTO {
                    supply_id: "1".into(),
                    supplier_id: "1".into(),
                    supply_name: "SupplyA".into(),
                    supplier_name: "SupplierA".into(),
                    unit_name: "g".into(),
                    unit_price: 100,
                    quantity: 10.0,
                },
                JournalRecordDTO {
                    supply_id: "2".into(),
                    supplier_id: "1".into(),
                    supply_name: "SupplyB".into(),
                    supplier_name: "SupplierA".into(),
                    unit_name: "g".into(),
                    unit_price: 120,
                    quantity: 5.0,
                },
            ],
        }]
    );

    let journal = service.get("1").unwrap();

    assert_eq!(
        journal,
        Some(JournalDTO {
            id: "1".into(),
            entry_date: 100000,
            records: vec![
                JournalRecordDTO {
                    supply_id: "1".into(),
                    supplier_id: "1".into(),
                    supply_name: "SupplyA".into(),
                    supplier_name: "SupplierA".into(),
                    unit_name: "g".into(),
                    unit_price: 100,
                    quantity: 10.0,
                },
                JournalRecordDTO {
                    supply_id: "2".into(),
                    supplier_id: "1".into(),
                    supply_name: "SupplyB".into(),
                    supplier_name: "SupplierA".into(),
                    unit_name: "g".into(),
                    unit_price: 120,
                    quantity: 5.0,
                },
            ],
        })
    );

    service
        .edit(EditJournalCommand {
            journal_id: "1".into(),
            records: vec![JournalRecordDTO {
                supply_id: "2".into(),
                supplier_id: "1".into(),
                supply_name: "SupplyB".into(),
                supplier_name: "SupplierC".into(),
                unit_name: "g".into(),
                unit_price: 200,
                quantity: 10.0,
            }],
        })
        .unwrap();

    let journal = service.get("1").unwrap();

    assert_eq!(
        journal,
        Some(JournalDTO {
            id: "1".into(),
            entry_date: 100000,
            records: vec![JournalRecordDTO {
                supply_id: "2".into(),
                supplier_id: "1".into(),
                supply_name: "SupplyB".into(),
                supplier_name: "SupplierC".into(),
                unit_name: "g".into(),
                unit_price: 200,
                quantity: 10.0,
            },],
        })
    );

    let search_results = service
        .search(SearchJournalsQuery {
            period_start: Some(100000),
            period_end: Some(200000),
            supply_name: Some("B".into()),
            supplier_name: Some("C".into()),
        })
        .unwrap();

    assert_eq!(
        search_results,
        vec![JournalDTO {
            id: "1".into(),
            entry_date: 100000,
            records: vec![JournalRecordDTO {
                supply_id: "2".into(),
                supplier_id: "1".into(),
                supply_name: "SupplyB".into(),
                supplier_name: "SupplierC".into(),
                unit_name: "g".into(),
                unit_price: 200,
                quantity: 10.0,
            },],
        }]
    );

    service.delete("1").unwrap();

    let journals = service.list().unwrap();

    assert!(journals.is_empty());
}

#[test]
fn stocktaking_service_test() {
    let storage = Arc::new(Mutex::new(Storage::default()));

    let supply_respository = MockSupplyRepository::new(Arc::clone(&storage));
    let supplier_respository = MockSupplierRepository::new(Arc::clone(&storage));
    let stocktaking_repository = MockStocktakingRepository::new(Arc::clone(&storage));

    supplier_respository
        .add(Supplier::restore(
            SupplierId::new("1").unwrap(),
            SupplierName::new("SupplierA").unwrap(),
        ))
        .unwrap();

    supply_respository
        .add(Supply::new(
            SupplyId::new("1").unwrap(),
            SupplyName::new("SupplyA").unwrap(),
            UnitName::new("g").unwrap(),
            SupplierId::new("1").unwrap(),
        ))
        .unwrap();

    supply_respository
        .add(Supply::new(
            SupplyId::new("2").unwrap(),
            SupplyName::new("SupplyB").unwrap(),
            UnitName::new("g").unwrap(),
            SupplierId::new("1").unwrap(),
        ))
        .unwrap();

    let service = StocktakingService::new(
        Arc::new(supply_respository),
        Arc::new(stocktaking_repository),
    );

    service
        .record(RecordStocktakingCommand {
            stocktaken_date: 100000,
            records: vec![
                StocktakingRecordDTO {
                    supply_id: "1".into(),
                    supply_name: "SupplyA".into(),
                    unit_name: "g".into(),
                    unit_price: 100,
                    quantity: 10.0,
                },
                StocktakingRecordDTO {
                    supply_id: "2".into(),
                    supply_name: "SupplyB".into(),
                    unit_name: "g".into(),
                    unit_price: 150,
                    quantity: 15.0,
                },
            ],
        })
        .unwrap();

    let stocktakings = service.list().unwrap();

    assert_eq!(
        stocktakings,
        vec![StocktakingDTO {
            id: "1".into(),
            stocktaken_date: 100000,
            records: vec![
                StocktakingRecordDTO {
                    supply_id: "1".into(),
                    supply_name: "SupplyA".into(),
                    unit_name: "g".into(),
                    unit_price: 100,
                    quantity: 10.0,
                },
                StocktakingRecordDTO {
                    supply_id: "2".into(),
                    supply_name: "SupplyB".into(),
                    unit_name: "g".into(),
                    unit_price: 150,
                    quantity: 15.0,
                },
            ],
        }],
    );

    let stocktaking = service.get("1").unwrap();

    assert_eq!(
        stocktaking,
        Some(StocktakingDTO {
            id: "1".into(),
            stocktaken_date: 100000,
            records: vec![
                StocktakingRecordDTO {
                    supply_id: "1".into(),
                    supply_name: "SupplyA".into(),
                    unit_name: "g".into(),
                    unit_price: 100,
                    quantity: 10.0,
                },
                StocktakingRecordDTO {
                    supply_id: "2".into(),
                    supply_name: "SupplyB".into(),
                    unit_name: "g".into(),
                    unit_price: 150,
                    quantity: 15.0,
                },
            ],
        })
    );

    service
        .edit(EditStocktakingCommand {
            stocktaking_id: "1".into(),
            records: vec![StocktakingRecordDTO {
                supply_id: "1".into(),
                supply_name: "SupplyA".into(),
                unit_name: "kg".into(),
                unit_price: 150,
                quantity: 5.0,
            }],
        })
        .unwrap();

    let stocktaking = service.get("1").unwrap();

    assert_eq!(
        stocktaking,
        Some(StocktakingDTO {
            id: "1".into(),
            stocktaken_date: 100000,
            records: vec![StocktakingRecordDTO {
                supply_id: "1".into(),
                supply_name: "SupplyA".into(),
                unit_name: "kg".into(),
                unit_price: 150,
                quantity: 5.0,
            },],
        })
    );

    let search_results = service
        .search(SearchStocktakingQuery {
            period_start: Some(100000),
            period_end: Some(250000),
        })
        .unwrap();

    assert_eq!(
        search_results,
        vec![StocktakingDTO {
            id: "1".into(),
            stocktaken_date: 100000,
            records: vec![StocktakingRecordDTO {
                supply_id: "1".into(),
                supply_name: "SupplyA".into(),
                unit_name: "kg".into(),
                unit_price: 150,
                quantity: 5.0,
            },],
        }]
    );

    service.delete("1").unwrap();

    let stocktakings = service.list().unwrap();

    assert!(stocktakings.is_empty());
}
