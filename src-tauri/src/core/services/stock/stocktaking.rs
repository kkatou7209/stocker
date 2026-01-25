//! This module provides the implementation of the `StocktakingUsecase`.
use std::sync::Arc;

use crate::core::domain::entities::stock::*;
use crate::core::domain::values::stock::*;
use crate::core::provided_ports::*;
use crate::core::required_ports::*;
use crate::core::*;

/// Stocktaking usecase
pub struct StocktakingService {
    supply_respository: Arc<dyn ForSupplyPersistence>,
    stocktaking_respository: Arc<dyn ForStocktakingPersistence>,
}

impl StocktakingService {
    pub fn new(
        supply_respository: Arc<dyn ForSupplyPersistence>,
        stocktaking_respository: Arc<dyn ForStocktakingPersistence>,
    ) -> Self {
        Self {
            supply_respository,
            stocktaking_respository,
        }
    }
}

/// Stocktaking usecase implementation
impl StocktakingUsecase for StocktakingService {
    fn get(&self, stocktaking_id: impl AsRef<str>) -> Result<Option<StocktakingDTO>> {
        let stocktaking_id = StocktakingId::new(stocktaking_id.as_ref())?;

        let stocktaking = self.stocktaking_respository.get(stocktaking_id)?;

        if stocktaking.is_none() {
            return Ok(None);
        }

        let stocktaking = stocktaking.unwrap();

        let stocktaking = StocktakingDTO {
            id: stocktaking.id().to_string(),
            stocktaken_date: stocktaking.stocktaken_at().as_i64(),
            total_price: stocktaking.total_price().as_f64(),
            records: stocktaking
                .records()
                .iter()
                .map(|record| StocktakingRecordDTO {
                    supply_id: record.supply_id().to_string(),
                    supply_name: record.supply_name().to_string(),
                    quantity: record.quantity().as_f64(),
                    unit_name: record.unit_name().to_string(),
                    unit_price: record.unit_price().as_f64(),
                    total_price: record.total_price().as_f64(),
                })
                .collect(),
        };

        Ok(Some(stocktaking))
    }

    fn list(&self) -> Result<Vec<StocktakingDTO>> {
        let stocktakings = self.stocktaking_respository.list()?;

        let stocktakings: Vec<StocktakingDTO> = stocktakings
            .iter()
            .map(|stocktaking| StocktakingDTO {
                id: stocktaking.id().to_string(),
                stocktaken_date: stocktaking.stocktaken_at().as_i64(),
                total_price: stocktaking.total_price().as_f64(),
                records: stocktaking
                    .records()
                    .iter()
                    .map(|record| StocktakingRecordDTO {
                        quantity: record.quantity().as_f64(),
                        supply_id: record.supply_id().to_string(),
                        supply_name: record.supply_name().to_string(),
                        unit_name: record.unit_name().to_string(),
                        unit_price: record.unit_price().as_f64(),
                        total_price: record.total_price().as_f64(),
                    })
                    .collect(),
            })
            .collect();

        Ok(stocktakings)
    }

    fn search(&self, query: SearchStocktakingQuery) -> Result<Vec<StocktakingDTO>> {
        let query = FindStocktakingsQuery {
            period_start: query
                .period_start
                .map(|start| StocktakenDateTime::new(start)),
            period_end: query.period_end.map(|end| StocktakenDateTime::new(end)),
        };

        let stocktakings = self.stocktaking_respository.find(query)?;

        let stocktakings: Vec<StocktakingDTO> = stocktakings
            .iter()
            .map(|stocktaking| StocktakingDTO {
                id: stocktaking.id().to_string(),
                stocktaken_date: stocktaking.stocktaken_at().as_i64(),
                total_price: stocktaking.total_price().as_f64(),
                records: stocktaking
                    .records()
                    .iter()
                    .map(|record| StocktakingRecordDTO {
                        quantity: record.quantity().as_f64(),
                        supply_id: record.supply_id().to_string(),
                        supply_name: record.supply_name().to_string(),
                        unit_name: record.unit_name().to_string(),
                        unit_price: record.unit_price().as_f64(),
                        total_price: record.total_price().as_f64(),
                    })
                    .collect(),
            })
            .collect();

        Ok(stocktakings)
    }

    fn record(&self, command: RecordStocktakingCommand) -> Result<StocktakingDTO> {
        let id = self.stocktaking_respository.next_id()?;

        let supply_ids = command
            .records
            .iter()
            .map(|record| SupplyId::new(&record.supply_id))
            .collect::<Result<Vec<SupplyId>>>()?;

        if !self.supply_respository.has(&supply_ids)? {
            return Err(Error::DomainError(format!("supply does not exist.")));
        }

        let mut records: Vec<StocktakingRecord> = Vec::new();

        for record in &command.records {
            records.push(StocktakingRecord::new(
                SupplyId::new(&record.supply_id)?,
                SupplyName::new(&record.supply_name)?,
                UnitName::new(&record.unit_name)?,
                StocktakingUnitPrice::new(record.unit_price)?,
                StocktakingQuantity::new(record.quantity)?,
                TotalPrice::new(record.total_price)?,
            ));
        }

        let stocktaking = Stocktaking::restore(
            id,
            StocktakenDateTime::new(command.stocktaken_date),
            TotalPrice::new(command.total_price)?,
            records,
        );

        self.stocktaking_respository.add(stocktaking.clone())?;

        let stocktaking = StocktakingDTO {
            id: stocktaking.id().to_string(),
            stocktaken_date: stocktaking.stocktaken_at().as_i64(),
            total_price: stocktaking.total_price().as_f64(),
            records: stocktaking
                .records()
                .iter()
                .map(|record| StocktakingRecordDTO {
                    quantity: record.quantity().as_f64(),
                    supply_id: record.supply_id().to_string(),
                    supply_name: record.supply_name().to_string(),
                    unit_name: record.unit_name().to_string(),
                    unit_price: record.unit_price().as_f64(),
                    total_price: record.total_price().as_f64(),
                })
                .collect(),
        };

        Ok(stocktaking)
    }

    fn edit(&self, command: EditStocktakingCommand) -> Result<()> {
        let stocktaking_id = StocktakingId::new(command.stocktaking_id)?;

        let mut stocktaking = self
            .stocktaking_respository
            .get(stocktaking_id)?
            .ok_or(Error::DomainError(format!("stocktaking does not exist.")))?;

        let total_price = TotalPrice::new(command.total_price)?;

        stocktaking.change_total_price(total_price);

        let mut records: Vec<StocktakingRecord> = Vec::new();

        for record in &command.records {
            records.push(StocktakingRecord::new(
                SupplyId::new(&record.supply_id)?,
                SupplyName::new(&record.supply_name)?,
                UnitName::new(&record.unit_name)?,
                StocktakingUnitPrice::new(record.unit_price)?,
                StocktakingQuantity::new(record.quantity)?,
                TotalPrice::new(record.total_price)?,
            ));
        }

        stocktaking.swap_records(records);

        self.stocktaking_respository.save(stocktaking)?;

        Ok(())
    }

    fn delete(&self, stocktaking_id: impl AsRef<str>) -> Result<()> {
        let stocktaking_id = StocktakingId::new(stocktaking_id.as_ref())?;

        self.stocktaking_respository.delete(stocktaking_id)?;

        Ok(())
    }
}
