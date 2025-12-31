use std::sync::Arc;

use crate::core::domain::entities::stock::*;
use crate::core::domain::values::stock::*;
use crate::core::provided_ports;
use crate::core::provided_ports::*;
use crate::core::required_ports::*;
use crate::core::*;

/// stocktaking usecase
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

impl StocktakingUsecase for StocktakingService {
    fn get(&self, query: provided_ports::GetStocktakingQuery) -> Result<Option<StocktakingDTO>> {
        let stocktaking_id = StocktakingId::new(query.stocktaking_id)?;

        let stocktaking = self.stocktaking_respository.get(stocktaking_id)?;

        if stocktaking.is_none() {
            return Ok(None);
        }

        let stocktaking = stocktaking.unwrap();

        let stocktaking = StocktakingDTO {
            id: stocktaking.id().to_string(),
            stocktaken_date: stocktaking.stocktaken_at().as_i64(),
            records: stocktaking
                .records()
                .iter()
                .map(|record| StocktakingRecordDTO {
                    supply_id: record.supply_id().to_string(),
                    supply_name: record.supply_name().to_string(),
                    quantity: record.quantity().as_u32(),
                    unit_name: record.unit_name().to_string(),
                    unit_price: record.unit_price().as_u32(),
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
                records: stocktaking
                    .records()
                    .iter()
                    .map(|record| StocktakingRecordDTO {
                        quantity: record.quantity().as_u32(),
                        supply_id: record.supply_id().to_string(),
                        supply_name: record.supply_name().to_string(),
                        unit_name: record.unit_name().to_string(),
                        unit_price: record.unit_price().as_u32(),
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
                records: stocktaking
                    .records()
                    .iter()
                    .map(|record| StocktakingRecordDTO {
                        quantity: record.quantity().as_u32(),
                        supply_id: record.supply_id().to_string(),
                        supply_name: record.supply_name().to_string(),
                        unit_name: record.unit_name().to_string(),
                        unit_price: record.unit_price().as_u32(),
                    })
                    .collect(),
            })
            .collect();

        Ok(stocktakings)
    }

    fn record(&self, command: RecordStocktakingCommand) -> Result<StocktakingDTO> {
        let id = self.stocktaking_respository.next_id()?;

        let supply_ids: Vec<Result<SupplyId>> = command
            .records
            .iter()
            .map(|record| SupplyId::new(&record.supply_id))
            .collect();

        let supply_ids: Result<Vec<SupplyId>> = supply_ids.into_iter().collect();

        let supply_ids = supply_ids?;

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
            ));
        }

        let stocktaking = Stocktaking::restore(
            id,
            StocktakenDateTime::new(command.stocktaken_date),
            records,
        );

        self.stocktaking_respository.add(stocktaking.clone())?;

        let stocktaking = StocktakingDTO {
            id: stocktaking.id().to_string(),
            stocktaken_date: stocktaking.stocktaken_at().as_i64(),
            records: stocktaking
                .records()
                .iter()
                .map(|record| StocktakingRecordDTO {
                    quantity: record.quantity().as_u32(),
                    supply_id: record.supply_id().to_string(),
                    supply_name: record.supply_name().to_string(),
                    unit_name: record.unit_name().to_string(),
                    unit_price: record.unit_price().as_u32(),
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

        let mut records: Vec<StocktakingRecord> = Vec::new();

        for record in &command.records {
            records.push(StocktakingRecord::new(
                SupplyId::new(&record.supply_id)?,
                SupplyName::new(&record.supply_name)?,
                UnitName::new(&record.unit_name)?,
                StocktakingUnitPrice::new(record.unit_price)?,
                StocktakingQuantity::new(record.quantity)?,
            ));
        }

        stocktaking.swap_records(records);

        self.stocktaking_respository.save(stocktaking)?;

        Ok(())
    }
}
