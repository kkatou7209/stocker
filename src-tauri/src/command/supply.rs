use serde::{Deserialize, Serialize};

use crate::core::provided_ports::{self, *};
use crate::core::stocker::Stocker;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SupplyData {
    pub id: String,
    pub name: String,
    pub unit_name: String,
    pub supplier_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddSupplyCommand {
    pub supplier_id: String,
    pub supply_name: String,
    pub unit_name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSupplyCommand {
    pub supply_id: String,
    pub supply_name: String,
    pub unit_name: String,
}

#[tauri::command]
pub fn list_all_supplies(app: tauri::State<Stocker>) -> Result<Vec<SupplyData>, String> {
    let supplies = app.supply_usecase().list().map_err(|e| e.to_string())?;

    let supplies: Vec<SupplyData> = supplies
        .into_iter()
        .map(|supply| SupplyData {
            id: supply.id,
            name: supply.name,
            unit_name: supply.unit_name,
            supplier_id: supply.supplier_id,
        })
        .collect();

    Ok(supplies)
}

#[tauri::command]
pub fn get_supply_by_id(
    app: tauri::State<Stocker>,
    supply_id: String,
) -> Result<Option<SupplyData>, String> {
    let supply = app
        .supply_usecase()
        .get(GetSupplyQuery { supply_id })
        .map_err(|e| e.to_string())?;

    let supply = supply.and_then(|supply| {
        Some(SupplyData {
            id: supply.id,
            name: supply.name,
            unit_name: supply.unit_name,
            supplier_id: supply.supplier_id,
        })
    });

    Ok(supply)
}

#[tauri::command]
pub fn register_supply(
    app: tauri::State<Stocker>,
    command: AddSupplyCommand,
) -> Result<(), String> {
    app.supply_usecase()
        .register(CreateSupplyCommand {
            supply_name: command.supply_name,
            unit_name: command.unit_name,
            supplier_id: command.supplier_id,
        })
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn update_supply(
    app: tauri::State<Stocker>,
    command: UpdateSupplyCommand,
) -> Result<(), String> {
    let supply = app
        .supply_usecase()
        .get(GetSupplyQuery {
            supply_id: command.supply_id.to_string(),
        })
        .map_err(|e| e.to_string())?;

    if supply.is_none() {
        return Err(format!("supply does not exist."));
    }

    let supply = supply.unwrap();

    app.supply_usecase()
        .update(provided_ports::UpdateSupplyCommand {
            supply_id: command.supply_id,
            supply_name: command.supply_name,
            unit_name: command.unit_name,
            supplier_id: supply.supplier_id,
        })
        .map_err(|e| e.to_string())?;

    Ok(())
}
