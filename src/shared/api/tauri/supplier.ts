import { invoke } from '@tauri-apps/api/core';
import z from 'zod';
import {
	type AddSupplierCommand,
	SupplierData,
	type SupplierEndpoint,
	type SupplierQuery,
	type UpdateSupplierCommand,
} from '@/shared/api/endpoints/supplier';

export const tauriSupplierEndpoint: SupplierEndpoint = {
	listAllSuppliers: async (): Promise<SupplierData[]> => {
		const suppliers = await invoke<SupplierData[]>('list_all_suppliers');

		return suppliers;
	},
	getSupplierById: async (id: string): Promise<SupplierData | null> => {
		const supplier = await invoke<SupplierData | null>(
			'get_supplier_by_id',
			{ id },
		);

		const validated = SupplierData.nullable().parse(supplier);

		return validated;
	},
	searchSuppliers: async (query: SupplierQuery): Promise<SupplierData[]> => {
		const suppliers = await invoke<SupplierData[]>('search_suppliers', {
			query,
		});

		const validated = z.array(SupplierData).parse(suppliers);

		return validated;
	},
	registerSupplier: async (command: AddSupplierCommand): Promise<void> => {
		await invoke('register_supplier', { command });
	},
	updateSupplier: async (command: UpdateSupplierCommand): Promise<void> => {
		await invoke('update_supplier', { command });
	},
};
