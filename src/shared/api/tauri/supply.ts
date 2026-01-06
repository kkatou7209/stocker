import { invoke } from '@tauri-apps/api/core';
import z from 'zod';
import {
	type AddSupplyCommand,
	SupplyData,
	type SupplyEndpoint,
	type UpdateSupplyCommand,
} from '@/shared/api/endpoints/supply';

export const tauriSupplyEndpoint: SupplyEndpoint = Object.freeze({
	listAllSupplies: async (): Promise<SupplyData[]> => {
		const supplies = await invoke<SupplyData[]>('list_all_supplies');

		const validated = z.array(SupplyData).parse(supplies);

		return validated;
	},
	getSupplyById: async (id: string): Promise<SupplyData | null> => {
		const supply = await invoke<SupplyData | null>('get_supply_by_id', {
			id,
		});

		const validated = SupplyData.nullable().parse(supply);

		return validated;
	},
	registerSupply: async (command: AddSupplyCommand): Promise<void> => {
		await invoke('register_supply', { command });
	},
	updateSupply: async (command: UpdateSupplyCommand): Promise<void> => {
		await invoke('update_supply', { command });
	},
	deleteSupply: async (id: string): Promise<void> => {
		await invoke<void>('delete_supply', { id });
	},
});
