use std::fs;
use std::path::Path;

use scopeguard::defer;

use crate::core::domain::entities::stock::Stocktaking;
use crate::core::domain::values::stock::{
    StocktakenDateTime, StocktakingId, StocktakingQuantity, StocktakingRecord,
    StocktakingUnitPrice, SupplyId, SupplyName, UnitName,
};
use crate::core::required_ports::{self, FindStocktakingsQuery, ForStocktakingPersistence};
use crate::persistence::sqlite::{migrate, SqliteStocktakingRepository};

#[test]
fn stocktaking_repository_test() {
    let tmp_path = Path::new("tmp/stocktaking_repository_test.db");

    defer! {
        fs::remove_file(tmp_path).unwrap();
    }

    if let Some(parent) = tmp_path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    migrate(tmp_path.to_string_lossy()).unwrap();

    let repository = SqliteStocktakingRepository::new(tmp_path.to_string_lossy());

    let next_id = repository.next_id().unwrap();

    assert_eq!(next_id, StocktakingId::new("1").unwrap());

    repository
        .add(Stocktaking::restore(
            StocktakingId::new("1").unwrap(),
            StocktakenDateTime::new(200000),
            vec![
                StocktakingRecord::new(
                    SupplyId::new("1").unwrap(),
                    SupplyName::new("SupplyA").unwrap(),
                    UnitName::new("g").unwrap(),
                    StocktakingUnitPrice::new(100_u32).unwrap(),
                    StocktakingQuantity::new(10_u32).unwrap(),
                ),
                StocktakingRecord::new(
                    SupplyId::new("2").unwrap(),
                    SupplyName::new("SupplyB").unwrap(),
                    UnitName::new("g").unwrap(),
                    StocktakingUnitPrice::new(130_u32).unwrap(),
                    StocktakingQuantity::new(5_u32).unwrap(),
                ),
            ],
        ))
        .unwrap();

    let stocktakings = repository.list().unwrap();

    assert!(stocktakings.first().is_some_and(|stocktaking| {
        assert_eq!(stocktaking.id(), &StocktakingId::new("1").unwrap());
        assert_eq!(
            stocktaking.stocktaken_at(),
            &StocktakenDateTime::new(200000)
        );
        assert_eq!(
            stocktaking.records(),
            &[
                StocktakingRecord::new(
                    SupplyId::new("1").unwrap(),
                    SupplyName::new("SupplyA").unwrap(),
                    UnitName::new("g").unwrap(),
                    StocktakingUnitPrice::new(100_u32).unwrap(),
                    StocktakingQuantity::new(10_u32).unwrap(),
                ),
                StocktakingRecord::new(
                    SupplyId::new("2").unwrap(),
                    SupplyName::new("SupplyB").unwrap(),
                    UnitName::new("g").unwrap(),
                    StocktakingUnitPrice::new(130_u32).unwrap(),
                    StocktakingQuantity::new(5_u32).unwrap(),
                ),
            ]
        );
        true
    }));

    repository
        .save(Stocktaking::restore(
            StocktakingId::new("1").unwrap(),
            StocktakenDateTime::new(240000),
            vec![StocktakingRecord::new(
                SupplyId::new("1").unwrap(),
                SupplyName::new("SupplyC").unwrap(),
                UnitName::new("kg").unwrap(),
                StocktakingUnitPrice::new(140_u32).unwrap(),
                StocktakingQuantity::new(15_u32).unwrap(),
            )],
        ))
        .unwrap();

    let stocktaking = repository.get(StocktakingId::new("1").unwrap()).unwrap();

    assert!(stocktaking.is_some_and(|stocktaking| {
        assert_eq!(stocktaking.id(), &StocktakingId::new("1").unwrap());
        assert_eq!(
            stocktaking.stocktaken_at(),
            &StocktakenDateTime::new(240000)
        );
        assert_eq!(
            stocktaking.records(),
            &[StocktakingRecord::new(
                SupplyId::new("1").unwrap(),
                SupplyName::new("SupplyC").unwrap(),
                UnitName::new("kg").unwrap(),
                StocktakingUnitPrice::new(140_u32).unwrap(),
                StocktakingQuantity::new(15_u32).unwrap(),
            ),]
        );
        true
    }));

    let stocktakings = repository
        .find(FindStocktakingsQuery {
            period_start: Some(StocktakenDateTime::new(100000)),
            period_end: Some(StocktakenDateTime::new(300000)),
        })
        .unwrap();

    assert!(stocktakings.first().is_some_and(|stocktaking| {
        assert_eq!(stocktaking.id(), &StocktakingId::new("1").unwrap());
        assert_eq!(
            stocktaking.stocktaken_at(),
            &StocktakenDateTime::new(240000)
        );
        assert_eq!(
            stocktaking.records(),
            &[StocktakingRecord::new(
                SupplyId::new("1").unwrap(),
                SupplyName::new("SupplyC").unwrap(),
                UnitName::new("kg").unwrap(),
                StocktakingUnitPrice::new(140_u32).unwrap(),
                StocktakingQuantity::new(15_u32).unwrap(),
            ),]
        );
        true
    }));
}
