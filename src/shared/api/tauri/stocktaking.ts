import { invoke } from '@tauri-apps/api/core';
import z from 'zod';
import {
	type AddStocktakingCommand,
	StocktakingData,
	type StocktakingEndpoints,
	type StocktakingQuery,
	type UpdateStocktakingCommand,
} from '@/shared/api/endpoints/stocktaking';

export const tauriStocktakingEndpoint: StocktakingEndpoints = {
	listAllStocktakings: async (): Promise<StocktakingData[]> => {
		const stocktakings = await invoke<StocktakingData[]>(
			'list_all_stocktakings',
		);

		const validated = z.array(StocktakingData).parse(stocktakings);

		return validated;
	},
	getStocktakingById: async (id: string): Promise<StocktakingData | null> => {
		const stocktaking = await invoke<StocktakingData | null>(
			'get_stocktaking_by_id',
			{ id },
		);

		const validated = StocktakingData.parse(stocktaking);

		return validated;
	},
	updateStocktaking: async (
		command: UpdateStocktakingCommand,
	): Promise<void> => {
		await invoke('update_stocktaking', command);
	},
	recordStocktaking: async (
		command: AddStocktakingCommand,
	): Promise<StocktakingData> => {
		const stocktaking = await invoke<StocktakingData>(
			'record_stocktaking',
			command,
		);

		const validated = StocktakingData.parse(stocktaking);

		return validated;
	},
	searchStocktakings: async (
		query: StocktakingQuery,
	): Promise<StocktakingData[]> => {
		const stocktakings = await invoke<StocktakingData[]>(
			'search_stocktakings',
			query,
		);

		const validated = z.array(StocktakingData).parse(stocktakings);

		return validated;
	},
};
