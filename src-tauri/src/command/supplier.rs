use serde::{Deserialize, Serialize};

use crate::command::*;
use crate::core::provided_ports::{self, *};
use crate::core::stocker::Stocker;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SupplierData {
    id: String,
    name: String,
    supplies: Vec<SupplyData>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddSupplierCommand {
    supplier_name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSupplierCommand {
    supplier_id: String,
    supplier_name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupplierQuery {
    supplier_name: Option<String>,
    supply_name: Option<String>,
}

#[tauri::command]
pub fn list_all_suppliers(app: tauri::State<Stocker>) -> Result<Vec<SupplierData>, String> {
    let suppliers = app.supplier_usecase().list().map_err(|e| e.to_string())?;

    let supplies = app.supply_usecase().list().map_err(|e| e.to_string())?;

    let suppliers: Vec<SupplierData> = suppliers
        .into_iter()
        .map(|supplier| {
            let supplies: Vec<SupplyData> = supplies
                .iter()
                .filter_map(|supply| {
                    if supply.supplier_id.eq(&supplier.id) {
                        Some(SupplyData {
                            id: supply.id.clone(),
                            name: supply.name.clone(),
                            unit_name: supply.unit_name.clone(),
                            supplier_id: supply.supplier_id.clone(),
                        })
                    } else {
                        None
                    }
                })
                .collect();

            SupplierData {
                id: supplier.id,
                name: supplier.name,
                supplies,
            }
        })
        .collect();

    Ok(suppliers)
}

#[tauri::command]
pub fn get_supplier_by_id(
    app: tauri::State<Stocker>,
    id: String,
) -> Result<Option<SupplierData>, String> {
    let supplier = app
        .supplier_usecase()
        .get(GetSupplierQuery {
            supplier_id: id.to_string(),
        })
        .map_err(|e| e.to_string())?;

    let supplier = if let Some(supplier) = supplier {
        let supplies = app
            .supply_usecase()
            .get_of_supplier(supplier.id.to_string())
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|supply| SupplyData {
                id: supply.id,
                name: supply.name,
                unit_name: supply.unit_name,
                supplier_id: supply.supplier_id,
            })
            .collect::<Vec<SupplyData>>();

        Some(SupplierData {
            id: supplier.id,
            name: supplier.name,
            supplies,
        })
    } else {
        None
    };

    Ok(supplier)
}

#[tauri::command]
pub fn search_suppliers(
    app: tauri::State<Stocker>,
    query: SupplierQuery,
) -> Result<Vec<SupplierData>, String> {
    let suppliers = app
        .supplier_usecase()
        .search(SearchSuppliersQuery {
            supplier_name: query.supplier_name.clone(),
            supply_name: query.supply_name.clone(),
        })
        .map_err(|e| e.to_string())?;

    let suppliers = suppliers
        .into_iter()
        .map(|supplier| {
            let supplies = app
                .supply_usecase()
                .get_of_supplier(supplier.id.to_string())
                .map_err(|e| e.to_string())?
                .into_iter()
                .map(|supply| SupplyData {
                    id: supply.id,
                    name: supply.name,
                    unit_name: supply.unit_name,
                    supplier_id: supply.supplier_id,
                })
                .collect::<Vec<SupplyData>>();

            Ok(SupplierData {
                id: supplier.id,
                name: supplier.name,
                supplies,
            })
        })
        .collect::<Result<Vec<SupplierData>, String>>()?;

    Ok(suppliers)
}

#[tauri::command]
pub fn register_supplier(
    app: tauri::State<Stocker>,
    command: AddSupplierCommand,
) -> Result<(), String> {
    app.supplier_usecase()
        .register(RegisterSupplierCommand {
            supplier_name: command.supplier_name,
        })
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn update_supplier(
    app: tauri::State<Stocker>,
    command: UpdateSupplierCommand,
) -> Result<(), String> {
    app.supplier_usecase()
        .update(provided_ports::UpdateSupplierCommand {
            supplier_id: command.supplier_id,
            supplier_name: command.supplier_name,
        })
        .map_err(|e| e.to_string())?;

    Ok(())
}
