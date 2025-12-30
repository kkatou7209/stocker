import type { JournalEndpoints } from '@/shared/api/endpoints/jurnal';
import type { StocktakingEndpoints } from '@/shared/api/endpoints/stocktaking';
import type { SupplierEndpoint } from '@/shared/api/endpoints/supplier';
import type { SupplyEndpoint } from '@/shared/api/endpoints/supply';
import { tauriJournalEndpoint } from '@/shared/api/tauri/journal';
import { tauriStocktakingEndpoint } from '@/shared/api/tauri/stocktaking';
import { tauriSupplierEndpoint } from '@/shared/api/tauri/supplier';
import { tauriSupplyEndpoint } from '@/shared/api/tauri/supply';
import {
	mockJournalEndpoints,
	mockStocktakingEndpoints,
	mockSupplierEndpoints,
	mockSupplyEndpoints,
} from '@/shared/mock/endpoints';

export interface Api {
	readonly supplier: SupplierEndpoint;
	readonly supply: SupplyEndpoint;
	readonly journal: JournalEndpoints;
	readonly stocktaking: StocktakingEndpoints;
}

const mockApi: Api = Object.freeze({
	supplier: mockSupplierEndpoints,
	supply: mockSupplyEndpoints,
	journal: mockJournalEndpoints,
	stocktaking: mockStocktakingEndpoints,
});

const tauriApi: Api = Object.freeze({
	supplier: tauriSupplierEndpoint,
	supply: tauriSupplyEndpoint,
	journal: tauriJournalEndpoint,
	stocktaking: tauriStocktakingEndpoint,
});

export const useApi = (): Api => {
	return import.meta.env.VITE_MOCK === 'true' ? mockApi : tauriApi;
};
