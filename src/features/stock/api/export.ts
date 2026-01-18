import { invoke } from '@tauri-apps/api/core';

/**
 * Downloads the stocktaking data as a CSV file.
 */
const downloadStocktakingCsv = async (id: string): Promise<void> => {
	await invoke<void>('download_stocktaking_csv', { id });
};

export const useExport = () => {
	return {
		downloadStocktakingCsv,
	};
};
