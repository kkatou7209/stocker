use std::fs;
use std::path::Path;

use rusqlite::Connection;
use scopeguard::defer;

use crate::core::domain::entities::stock::Supply;
use crate::core::domain::values::stock::{SupplierId, SupplyId, SupplyName, UnitName};
use crate::core::required_ports::ForSupplyPersistence;
use crate::persistence::sqlite::{migrate, SqliteSupplyRepository};

#[test]
fn supply_repository_test() {
    let tmp_path = Path::new("tmp/supply_repository_test.db");

    defer! {
        fs::remove_file(tmp_path).unwrap();
    }

    if let Some(parent) = tmp_path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    migrate(tmp_path.to_string_lossy()).unwrap();

    let conn = Connection::open(tmp_path).unwrap();

    conn.execute(
        "INSERT INTO suppliers (id, name) VALUES (1, 'SupplierA'), (2, 'SupplierB')",
        [],
    )
    .unwrap();

    let repository = SqliteSupplyRepository::new(tmp_path.to_string_lossy());

    let next_id = repository.next_id().unwrap();

    assert_eq!(next_id, SupplyId::new("1").unwrap());

    repository
        .add(Supply::new(
            SupplyId::new("1").unwrap(),
            SupplyName::new("SupplyA").unwrap(),
            UnitName::new("g").unwrap(),
            SupplierId::new("1").unwrap(),
        ))
        .unwrap();

    let supplies = repository.list().unwrap();

    assert!(supplies.first().is_some_and(|supply| {
        assert_eq!(supply.id(), &SupplyId::new("1").unwrap());
        assert_eq!(supply.name(), &SupplyName::new("SupplyA").unwrap());
        assert_eq!(supply.unit_name(), &UnitName::new("g").unwrap());
        assert_eq!(supply.supplier_id(), &SupplierId::new("1").unwrap());
        true
    }));

    repository
        .save(Supply::new(
            SupplyId::new("1").unwrap(),
            SupplyName::new("SupplyB").unwrap(),
            UnitName::new("kg").unwrap(),
            SupplierId::new("2").unwrap(),
        ))
        .unwrap();

    let supply = repository.get(SupplyId::new("1").unwrap()).unwrap();

    assert!(supply.as_ref().is_some_and(|supply| {
        assert_eq!(supply.id(), &SupplyId::new("1").unwrap());
        assert_eq!(supply.name(), &SupplyName::new("SupplyB").unwrap());
        assert_eq!(supply.unit_name(), &UnitName::new("kg").unwrap());
        assert_eq!(supply.supplier_id(), &SupplierId::new("2").unwrap());
        true
    }));

    repository.delete(supply.unwrap().id().clone()).unwrap();

    let supplies = repository.list().unwrap();

    assert_eq!(supplies, vec![]);
}
