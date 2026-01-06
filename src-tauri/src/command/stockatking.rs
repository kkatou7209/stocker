use chrono::{Local, TimeZone};
use serde::{Deserialize, Serialize};

use crate::core::provided_ports::{
    self, SearchStocktakingQuery, StocktakingRecordDTO, StocktakingUsecase,
};
use crate::core::stocker::Stocker;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StocktakingData {
    id: String,
    stocktaking_date: i64,
    records: Vec<StocktakingRecordData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StocktakingRecordData {
    supply_id: String,
    supply_name: String,
    unit_name: String,
    unit_price: u32,
    quantity: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordStocktakingCommand {
    stocktaking_date: i64,
    records: Vec<StocktakingRecordData>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStocktakingCommand {
    id: String,
    records: Vec<StocktakingRecordData>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StocktakingQuery {
    period_start: Option<i64>,
    period_end: Option<i64>,
}

#[tauri::command]
pub fn list_all_stocktakings(app: tauri::State<Stocker>) -> Result<Vec<StocktakingData>, String> {
    let stocktakings = app
        .stocktaking_usecase()
        .list()
        .map_err(|e| e.to_string())?;

    let stocktakings = stocktakings
        .into_iter()
        .map(|stockatking| StocktakingData {
            id: stockatking.id,
            stocktaking_date: stockatking.stocktaken_date,
            records: stockatking
                .records
                .into_iter()
                .map(|record| StocktakingRecordData {
                    supply_id: record.supply_id,
                    supply_name: record.supply_name,
                    unit_name: record.unit_name,
                    unit_price: record.unit_price,
                    quantity: record.quantity,
                })
                .collect::<Vec<StocktakingRecordData>>(),
        })
        .collect::<Vec<StocktakingData>>();

    Ok(stocktakings)
}

#[tauri::command]
pub fn get_stocktaking_by_id(
    app: tauri::State<Stocker>,
    id: String,
) -> Result<Option<StocktakingData>, String> {
    let stocktaking = app
        .stocktaking_usecase()
        .get(&id)
        .map_err(|e| e.to_string())?;

    let stocktaking = stocktaking.and_then(|stocktaking| {
        Some(StocktakingData {
            id: stocktaking.id,
            stocktaking_date: stocktaking.stocktaken_date,
            records: stocktaking
                .records
                .into_iter()
                .map(|record| StocktakingRecordData {
                    supply_id: record.supply_id,
                    supply_name: record.supply_name,
                    unit_name: record.unit_name,
                    unit_price: record.unit_price,
                    quantity: record.quantity,
                })
                .collect::<Vec<StocktakingRecordData>>(),
        })
    });

    Ok(stocktaking)
}

#[tauri::command]
pub fn record_stocktaking(
    app: tauri::State<Stocker>,
    command: RecordStocktakingCommand,
) -> Result<StocktakingData, String> {
    let stocktaking = app
        .stocktaking_usecase()
        .record(provided_ports::RecordStocktakingCommand {
            stocktaken_date: command.stocktaking_date,
            records: command
                .records
                .into_iter()
                .map(|record| StocktakingRecordDTO {
                    supply_id: record.supply_id,
                    supply_name: record.supply_name,
                    unit_name: record.unit_name,
                    unit_price: record.unit_price,
                    quantity: record.quantity,
                })
                .collect::<Vec<StocktakingRecordDTO>>(),
        })
        .map_err(|e| e.to_string())?;

    let stocktaking = StocktakingData {
        id: stocktaking.id,
        stocktaking_date: stocktaking.stocktaken_date,
        records: stocktaking
            .records
            .into_iter()
            .map(|record| StocktakingRecordData {
                supply_id: record.supply_id,
                supply_name: record.supply_name,
                unit_name: record.unit_name,
                unit_price: record.unit_price,
                quantity: record.quantity,
            })
            .collect::<Vec<StocktakingRecordData>>(),
    };

    Ok(stocktaking)
}

#[tauri::command]
pub fn update_stocktaking(
    app: tauri::State<Stocker>,
    command: UpdateStocktakingCommand,
) -> Result<(), String> {
    app.stocktaking_usecase()
        .edit(provided_ports::EditStocktakingCommand {
            stocktaking_id: command.id,
            records: command
                .records
                .into_iter()
                .map(|record| StocktakingRecordDTO {
                    supply_id: record.supply_id,
                    supply_name: record.supply_name,
                    unit_name: record.unit_name,
                    unit_price: record.unit_price,
                    quantity: record.quantity,
                })
                .collect::<Vec<StocktakingRecordDTO>>(),
        })
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn search_stocktakings(
    app: tauri::State<Stocker>,
    query: StocktakingQuery,
) -> Result<Vec<StocktakingData>, String> {
    let stocktakings = app
        .stocktaking_usecase()
        .search(SearchStocktakingQuery {
            period_start: query.period_start,
            period_end: query.period_end,
        })
        .map_err(|e| e.to_string())?;

    let stocktakings = stocktakings
        .into_iter()
        .map(|stockatking| StocktakingData {
            id: stockatking.id,
            stocktaking_date: stockatking.stocktaken_date,
            records: stockatking
                .records
                .into_iter()
                .map(|record| StocktakingRecordData {
                    supply_id: record.supply_id,
                    supply_name: record.supply_name,
                    unit_name: record.unit_name,
                    unit_price: record.unit_price,
                    quantity: record.quantity,
                })
                .collect::<Vec<StocktakingRecordData>>(),
        })
        .collect::<Vec<StocktakingData>>();

    Ok(stocktakings)
}

#[tauri::command]
pub fn get_stocktaking_at(
    app: tauri::State<Stocker>,
    date: i64,
) -> Result<Option<StocktakingData>, String> {
    println!("timestamp: {}", date);
    let date = Local.timestamp_millis_opt(date).unwrap();

    let start = date.date_naive().and_hms_milli_opt(0, 0, 0, 0).unwrap();

    let start = Local
        .from_local_datetime(&start)
        .unwrap()
        .timestamp_millis();

    let end = date
        .date_naive()
        .and_hms_milli_opt(23, 59, 59, 999)
        .unwrap();

    let end = Local.from_local_datetime(&end).unwrap().timestamp_millis();

    println!("start: {}, end: {}", start, end);

    let stocktakings = app
        .stocktaking_usecase()
        .search(SearchStocktakingQuery {
            period_start: Some(start),
            period_end: Some(end),
        })
        .map_err(|e| e.to_string())?;

    let stocktaking = stocktakings.first().cloned().and_then(|stocktaking| {
        Some(StocktakingData {
            id: stocktaking.id,
            stocktaking_date: stocktaking.stocktaken_date,
            records: stocktaking
                .records
                .into_iter()
                .map(|record| StocktakingRecordData {
                    supply_id: record.supply_id,
                    supply_name: record.supply_name,
                    unit_name: record.unit_name,
                    unit_price: record.unit_price,
                    quantity: record.quantity,
                })
                .collect::<Vec<StocktakingRecordData>>(),
        })
    });

    Ok(stocktaking)
}

#[tauri::command]
pub fn delete_stocktaking(app: tauri::State<Stocker>, id: String) -> Result<(), String> {
    app.stocktaking_usecase()
        .delete(id)
        .map_err(|e| e.to_string())?;

    Ok(())
}
