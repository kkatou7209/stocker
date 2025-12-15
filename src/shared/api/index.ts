import type { JournalEndpoints } from '@/shared/api/endpoints/jurnal';
import type { StocktakingEndpoints } from '@/shared/api/endpoints/stocktaking';
import type { SupplierEndpoint } from '@/shared/api/endpoints/supplier';
import type { SupplyEndpoint } from '@/shared/api/endpoints/supply';
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

export const useApi = (): Api => {
	return mockApi;
};
