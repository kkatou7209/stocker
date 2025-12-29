use std::fs;
use std::path::Path;

use scopeguard::defer;

use crate::core::domain::entities::stock::Journal;
use crate::core::domain::values::stock::{
    EntryDateTime, JournalId, JournalRecord, PurchaseQuantity, PurchaseUnitPrice, SupplierId,
    SupplierName, SupplyId, SupplyName, UnitName,
};
use crate::core::required_ports::{FindJournalsQuery, ForJournalPersistence};
use crate::persistence::sqlite::{migrate, SqliteJournalRepository};

#[test]
fn journal_repository_test() {
    let tmp_path = Path::new("tmp/journal_repository_test.db");

    defer! {
        fs::remove_file(tmp_path).unwrap();
    }

    if let Some(parent) = tmp_path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    migrate(tmp_path.to_string_lossy()).unwrap();

    let repository = SqliteJournalRepository::new(tmp_path.to_string_lossy());

    let next_id = repository.next_id().unwrap();

    assert_eq!(next_id, JournalId::new("1").unwrap());

    repository
        .add(Journal::restore(
            JournalId::new("1").unwrap(),
            EntryDateTime::new(200000),
            vec![
                JournalRecord::new(
                    SupplyId::new("1").unwrap(),
                    SupplyName::new("SupplyA").unwrap(),
                    SupplierId::new("1").unwrap(),
                    SupplierName::new("SupplierA").unwrap(),
                    UnitName::new("g").unwrap(),
                    PurchaseUnitPrice::new(100_u32).unwrap(),
                    PurchaseQuantity::new(5_u32).unwrap(),
                ),
                JournalRecord::new(
                    SupplyId::new("2").unwrap(),
                    SupplyName::new("SupplyB").unwrap(),
                    SupplierId::new("1").unwrap(),
                    SupplierName::new("SupplierA").unwrap(),
                    UnitName::new("g").unwrap(),
                    PurchaseUnitPrice::new(120_u32).unwrap(),
                    PurchaseQuantity::new(10_u32).unwrap(),
                ),
            ],
        ))
        .unwrap();

    let journals = repository.list().unwrap();

    assert!(journals.first().is_some_and(|journal| {
        assert_eq!(journal.id(), &JournalId::new("1").unwrap());
        assert_eq!(journal.entry_datetime(), &EntryDateTime::new(200000),);
        assert_eq!(
            journal.records(),
            &[
                JournalRecord::new(
                    SupplyId::new("1").unwrap(),
                    SupplyName::new("SupplyA").unwrap(),
                    SupplierId::new("1").unwrap(),
                    SupplierName::new("SupplierA").unwrap(),
                    UnitName::new("g").unwrap(),
                    PurchaseUnitPrice::new(100_u32).unwrap(),
                    PurchaseQuantity::new(5_u32).unwrap(),
                ),
                JournalRecord::new(
                    SupplyId::new("2").unwrap(),
                    SupplyName::new("SupplyB").unwrap(),
                    SupplierId::new("1").unwrap(),
                    SupplierName::new("SupplierA").unwrap(),
                    UnitName::new("g").unwrap(),
                    PurchaseUnitPrice::new(120_u32).unwrap(),
                    PurchaseQuantity::new(10_u32).unwrap(),
                ),
            ]
        );
        true
    }));

    repository
        .save(Journal::restore(
            JournalId::new("1").unwrap(),
            EntryDateTime::new(200000),
            vec![JournalRecord::new(
                SupplyId::new("1").unwrap(),
                SupplyName::new("SupplyC").unwrap(),
                SupplierId::new("1").unwrap(),
                SupplierName::new("SupplierA").unwrap(),
                UnitName::new("kg").unwrap(),
                PurchaseUnitPrice::new(130_u32).unwrap(),
                PurchaseQuantity::new(15_u32).unwrap(),
            )],
        ))
        .unwrap();

    let journal = repository.get(JournalId::new("1").unwrap()).unwrap();

    assert!(journal.is_some_and(|journal| {
        assert_eq!(journal.id(), &JournalId::new("1").unwrap());
        assert_eq!(journal.entry_datetime(), &EntryDateTime::new(200000),);
        assert_eq!(
            journal.records(),
            &[JournalRecord::new(
                SupplyId::new("1").unwrap(),
                SupplyName::new("SupplyC").unwrap(),
                SupplierId::new("1").unwrap(),
                SupplierName::new("SupplierA").unwrap(),
                UnitName::new("kg").unwrap(),
                PurchaseUnitPrice::new(130_u32).unwrap(),
                PurchaseQuantity::new(15_u32).unwrap(),
            ),]
        );
        true
    }));

    let journals = repository
        .find(FindJournalsQuery {
            period_start: Some(EntryDateTime::new(100000)),
            period_end: Some(EntryDateTime::new(300000)),
            supply_name: Some(SupplyName::new("C").unwrap()),
            supplier_name: Some(SupplierName::new("A").unwrap()),
        })
        .unwrap();

    assert!(journals.first().is_some_and(|journal| {
        assert_eq!(journal.id(), &JournalId::new("1").unwrap());
        assert_eq!(journal.entry_datetime(), &EntryDateTime::new(200000),);
        assert_eq!(
            journal.records(),
            &[JournalRecord::new(
                SupplyId::new("1").unwrap(),
                SupplyName::new("SupplyC").unwrap(),
                SupplierId::new("1").unwrap(),
                SupplierName::new("SupplierA").unwrap(),
                UnitName::new("kg").unwrap(),
                PurchaseUnitPrice::new(130_u32).unwrap(),
                PurchaseQuantity::new(15_u32).unwrap(),
            ),]
        );
        true
    }));
}
