import type {
	Stocktaking,
	StocktakingRecord,
} from '@/entities/stock/models/stocktaking';
import { useApi } from '@/shared/api';

class Repository {
	private api = useApi();

	public list = async (): Promise<Stocktaking[]> => {
		return await this.api.stocktaking.listAllStocktakings();
	};

	public get = async (id: string): Promise<Stocktaking | null> => {
		return await this.api.stocktaking.getStocktakingById(id);
	};

	public add = async (stocktaking: {
		stocktakingDate: Date;
		records: StocktakingRecord[];
	}): Promise<Stocktaking> => {
		const registered =
			await this.api.stocktaking.createStocktaking(stocktaking);

		return registered;
	};

	public edit = async (stocktaking: {
		id: string;
		stocktakingDate: Date;
		records: StocktakingRecord[];
	}): Promise<void> => {
		await this.api.stocktaking.updateStocktaking(stocktaking);
	};

	public find = async (query: {
		periodStart?: Date;
		periodEnd?: Date;
	}): Promise<Stocktaking[]> => {
		return await this.api.stocktaking.searchStocktakings(query);
	};
}

export type StocktakingRepository = InstanceType<typeof Repository>;

const instance = Object.freeze(new Repository());

export const useStocktakingRepository = () => instance;
