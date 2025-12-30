use std::fs::{self, File};
use std::path::Path;

use scopeguard::defer;

use crate::core::domain::entities::stock::Supplier;
use crate::core::domain::values::stock::{SupplierId, SupplierName};
use crate::core::required_ports::{self, FindSupplierQuery, ForSupplierPersistence};
use crate::persistence::sqlite::{migrate, SqliteSupplierRepository};

#[test]
fn supplier_repository_test() {
    let tmp_path = Path::new("tmp/supplier_repository_test.db");

    defer! {
        fs::remove_file(tmp_path).unwrap();
    }

    if let Some(parent) = tmp_path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    migrate(tmp_path.to_string_lossy()).unwrap();

    let repository = SqliteSupplierRepository::new(tmp_path.to_string_lossy());

    let next_id = repository.next_id().unwrap();

    assert_eq!(next_id, SupplierId::new("1").unwrap());

    let supplier = Supplier::restore(next_id.clone(), SupplierName::new("SupplierA").unwrap());

    repository.add(supplier).unwrap();

    let has_supplies = repository.has(&[next_id]).unwrap();

    assert!(has_supplies);

    let supplier = repository
        .get(required_ports::GetSupplierQuery {
            supplier_id: SupplierId::new("1").unwrap(),
        })
        .unwrap();

    assert!(supplier.is_some_and(|supplier| {
        assert_eq!(supplier.id(), &SupplierId::new("1").unwrap());
        assert_eq!(supplier.name(), &SupplierName::new("SupplierA").unwrap());
        true
    }));

    let suppliers = repository.list().unwrap();

    assert!(suppliers.first().is_some_and(|supplier| {
        assert_eq!(supplier.id(), &SupplierId::new("1").unwrap());
        assert_eq!(supplier.name(), &SupplierName::new("SupplierA").unwrap());
        true
    }));

    let suppliers = repository
        .find(FindSupplierQuery {
            supplier_name: Some(SupplierName::new("A").unwrap()),
            supply_name: None,
        })
        .unwrap();

    assert!(suppliers.first().is_some_and(|supplier| {
        assert_eq!(supplier.id(), &SupplierId::new("1").unwrap());
        assert_eq!(supplier.name(), &SupplierName::new("SupplierA").unwrap());
        true
    }));

    repository
        .save(Supplier::restore(
            SupplierId::new("1").unwrap(),
            SupplierName::new("SupplierB").unwrap(),
        ))
        .unwrap();

    let supplier = repository
        .get(required_ports::GetSupplierQuery {
            supplier_id: SupplierId::new("1").unwrap(),
        })
        .unwrap();

    assert!(supplier.is_some_and(|supplier| {
        assert_eq!(supplier.id(), &SupplierId::new("1").unwrap());
        assert_eq!(supplier.name(), &SupplierName::new("SupplierB").unwrap());
        true
    }));
}
