use chrono::{Local, TimeZone};
use serde::{Deserialize, Serialize};

use crate::core::provided_ports::{
    self, GetJournalQuery, JournalRecordDTO, JournalUsecase, SearchJournalsQuery,
};
use crate::core::stocker::Stocker;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JournalData {
    id: String,
    entry_date: i64,
    records: Vec<JournalRecordData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JournalRecordData {
    supply_id: String,
    supply_name: String,
    supplier_id: String,
    supplier_name: String,
    unit_name: String,
    unit_price: u32,
    quantity: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordJournalCommand {
    entry_date: i64,
    records: Vec<JournalRecordData>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateJournalCommand {
    id: String,
    entry_date: i64,
    records: Vec<JournalRecordData>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JournalQuery {
    period_start: Option<i64>,
    period_end: Option<i64>,
    supplier_name: Option<String>,
    supply_name: Option<String>,
}

#[tauri::command]
pub fn list_all_journals(app: tauri::State<Stocker>) -> Result<Vec<JournalData>, String> {
    let jorunals = app.journal_usecase().list().map_err(|e| e.to_string())?;

    let journals = jorunals
        .into_iter()
        .map(|journal| JournalData {
            id: journal.id,
            entry_date: journal.entry_date,
            records: journal
                .records
                .into_iter()
                .map(|record| JournalRecordData {
                    supply_id: record.supply_id,
                    supply_name: record.supply_name,
                    supplier_id: record.supplier_id,
                    supplier_name: record.supplier_name,
                    unit_name: record.unit_name,
                    unit_price: record.unit_price,
                    quantity: record.quantity,
                })
                .collect::<Vec<JournalRecordData>>(),
        })
        .collect::<Vec<JournalData>>();

    Ok(journals)
}

#[tauri::command]
pub fn get_journal_by_id(
    app: tauri::State<Stocker>,
    id: String,
) -> Result<Option<JournalData>, String> {
    let journal = app
        .journal_usecase()
        .get(GetJournalQuery { journal_id: id })
        .map_err(|e| e.to_string())?;

    let journal = journal.and_then(|journal| {
        Some(JournalData {
            id: journal.id,
            entry_date: journal.entry_date,
            records: journal
                .records
                .into_iter()
                .map(|record| JournalRecordData {
                    supply_id: record.supply_id,
                    supply_name: record.supply_name,
                    supplier_id: record.supplier_id,
                    supplier_name: record.supplier_name,
                    unit_name: record.unit_name,
                    unit_price: record.unit_price,
                    quantity: record.quantity,
                })
                .collect::<Vec<JournalRecordData>>(),
        })
    });

    Ok(journal)
}

#[tauri::command]
pub fn record_journal(
    app: tauri::State<Stocker>,
    command: RecordJournalCommand,
) -> Result<JournalData, String> {
    let journal = app
        .journal_usecase()
        .record(provided_ports::RecordJournalCommand {
            entry_date: command.entry_date,
            records: command
                .records
                .into_iter()
                .map(|record| JournalRecordDTO {
                    supply_id: record.supply_id,
                    supply_name: record.supply_name,
                    supplier_id: record.supplier_id,
                    supplier_name: record.supplier_name,
                    unit_name: record.unit_name,
                    unit_price: record.unit_price,
                    quantity: record.quantity,
                })
                .collect::<Vec<JournalRecordDTO>>(),
        })
        .map_err(|e| e.to_string())?;

    let journal = JournalData {
        id: journal.id,
        entry_date: journal.entry_date,
        records: journal
            .records
            .into_iter()
            .map(|record| JournalRecordData {
                supply_id: record.supply_id,
                supply_name: record.supply_name,
                supplier_id: record.supplier_id,
                supplier_name: record.supplier_name,
                unit_name: record.unit_name,
                unit_price: record.unit_price,
                quantity: record.quantity,
            })
            .collect::<Vec<JournalRecordData>>(),
    };

    Ok(journal)
}

#[tauri::command]
pub fn get_journal_at(
    app: tauri::State<Stocker>,
    date: i64,
) -> Result<Option<JournalData>, String> {
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

    let journals = app
        .journal_usecase()
        .search(SearchJournalsQuery {
            period_start: Some(start),
            period_end: Some(end),
            supplier_name: None,
            supply_name: None,
        })
        .map_err(|e| e.to_string())?;

    let journal = journals.first().cloned().and_then(|journal| {
        Some(JournalData {
            id: journal.id,
            entry_date: journal.entry_date,
            records: journal
                .records
                .into_iter()
                .map(|record| JournalRecordData {
                    supply_id: record.supply_id,
                    supply_name: record.supply_name,
                    supplier_id: record.supplier_id,
                    supplier_name: record.supplier_name,
                    unit_name: record.unit_name,
                    unit_price: record.unit_price,
                    quantity: record.quantity,
                })
                .collect::<Vec<JournalRecordData>>(),
        })
    });

    Ok(journal)
}

#[tauri::command]
pub fn update_journal(
    app: tauri::State<Stocker>,
    command: UpdateJournalCommand,
) -> Result<(), String> {
    app.journal_usecase()
        .edit(provided_ports::EditJournalCommand {
            journal_id: command.id,
            entry_date: command.entry_date,
            records: command
                .records
                .into_iter()
                .map(|record| JournalRecordDTO {
                    supply_id: record.supply_id,
                    supply_name: record.supply_name,
                    supplier_id: record.supplier_id,
                    supplier_name: record.supplier_name,
                    unit_name: record.unit_name,
                    unit_price: record.unit_price,
                    quantity: record.quantity,
                })
                .collect::<Vec<JournalRecordDTO>>(),
        })
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn search_journals(
    app: tauri::State<Stocker>,
    query: JournalQuery,
) -> Result<Vec<JournalData>, String> {
    let journals = app
        .journal_usecase()
        .search(SearchJournalsQuery {
            period_start: query.period_start,
            period_end: query.period_end,
            supply_name: query.supply_name,
            supplier_name: query.supplier_name,
        })
        .map_err(|e| e.to_string())?;

    let journals = journals
        .into_iter()
        .map(|journal| JournalData {
            id: journal.id,
            entry_date: journal.entry_date,
            records: journal
                .records
                .into_iter()
                .map(|record| JournalRecordData {
                    supply_id: record.supply_id,
                    supply_name: record.supply_name,
                    supplier_id: record.supplier_id,
                    supplier_name: record.supplier_name,
                    unit_name: record.unit_name,
                    unit_price: record.unit_price,
                    quantity: record.quantity,
                })
                .collect::<Vec<JournalRecordData>>(),
        })
        .collect::<Vec<JournalData>>();

    Ok(journals)
}
