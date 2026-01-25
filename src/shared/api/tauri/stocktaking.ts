import { invoke } from '@tauri-apps/api/core';
import z from 'zod';
import {
	type AddStocktakingCommand,
	StocktakingData,
	type StocktakingEndpoints,
	type StocktakingQuery,
	type UpdateStocktakingCommand,
} from '@/shared/api/endpoints/stocktaking';

export const tauriStocktakingEndpoint: StocktakingEndpoints = Object.freeze({
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
		await invoke('update_stocktaking', { command });
	},
	recordStocktaking: async (
		command: AddStocktakingCommand,
	): Promise<StocktakingData> => {
		const stocktaking = await invoke<StocktakingData>(
			'record_stocktaking',
			{ command },
		);

		const validated = StocktakingData.parse(stocktaking);

		return validated;
	},
	searchStocktakings: async (
		query: StocktakingQuery,
	): Promise<StocktakingData[]> => {
		const stocktakings = await invoke<StocktakingData[]>(
			'search_stocktakings',
			{ query },
		);

		const validated = z.array(StocktakingData).parse(stocktakings);

		return validated;
	},
	getStocktakingAt: async (date: number): Promise<StocktakingData | null> => {
		const stocktaking = await invoke<StocktakingData | null>(
			'get_stocktaking_at',
			{ date },
		);

		console.log(stocktaking);

		return stocktaking;
	},
	deleteStocktaking: async (id: string): Promise<void> => {
		await invoke<void>('delete_stocktaking', { id });
	},
	downloadStocktakingCsv: async (id: string): Promise<void> => {
		await invoke<void>('download_stocktaking_csv', { id });
	},
});
