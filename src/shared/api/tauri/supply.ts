import { invoke } from '@tauri-apps/api/core';
import type {
	AddSupplyCommand,
	SupplyData,
	SupplyEndpoint,
	UpdateSupplyCommand,
} from '@/shared/api/endpoints/supply';

export const tauriSupplyEndpoint: SupplyEndpoint = {
	listAllSupplies: async (): Promise<SupplyData[]> => {
		const supplies = await invoke<SupplyData[]>('list_all_supplies');

		return supplies;
	},
	getSupplyById: async (id: string): Promise<SupplyData | null> => {
		const supply = await invoke<SupplyData | null>('get_supply_by_id', {
			supplyId: id,
		});

		return supply;
	},
	registerSupply: async (command: AddSupplyCommand): Promise<void> => {
		await invoke('register_supply', command);
	},
	updateSupply: async (command: UpdateSupplyCommand): Promise<void> => {
		await invoke('update_supply', command);
	},
};
