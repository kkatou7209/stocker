import type {
	Stocktaking,
	StocktakingRecord,
} from '@/entities/stock/models/stocktaking';
import { useApi } from '@/shared/api';

class Repository {
	private api = useApi();

	public list = async (): Promise<Stocktaking[]> => {
		const stocktakings = await this.api.stocktaking.listAllStocktakings();

		return stocktakings.map((stocktaking) => ({
			...stocktaking,
			stocktakingDate: new Date(stocktaking.stocktakingDate),
		}));
	};

	public get = async (id: string): Promise<Stocktaking | null> => {
		const stocktaking = await this.api.stocktaking.getStocktakingById(id);

		if (!stocktaking) return null;

		return {
			...stocktaking,
			stocktakingDate: new Date(stocktaking.stocktakingDate),
		};
	};

	public getAt = async (date: Date): Promise<Stocktaking | null> => {
		const stocktaking = await this.api.stocktaking.getStocktakingAt(
			date.getTime(),
		);

		if (!stocktaking) return null;

		return {
			...stocktaking,
			stocktakingDate: new Date(stocktaking.stocktakingDate),
		};
	};

	public add = async (stocktaking: {
		stocktakingDate: Date;
		records: StocktakingRecord[];
	}): Promise<Stocktaking> => {
		const registered = await this.api.stocktaking.recordStocktaking({
			...stocktaking,
			stocktakingDate: stocktaking.stocktakingDate.getTime(),
		});

		return {
			...registered,
			stocktakingDate: new Date(stocktaking.stocktakingDate),
		};
	};

	public edit = async (stocktaking: {
		id: string;
		records: StocktakingRecord[];
	}): Promise<void> => {
		await this.api.stocktaking.updateStocktaking({
			...stocktaking,
		});
	};

	public find = async (query: {
		periodStart?: Date;
		periodEnd?: Date;
	}): Promise<Stocktaking[]> => {
		const stocktakings = await this.api.stocktaking.searchStocktakings({
			periodStart: query.periodStart?.getTime(),
			periodEnd: query.periodEnd?.getTime(),
		});

		return stocktakings.map((stocktaking) => ({
			...stocktaking,
			stocktakingDate: new Date(stocktaking.stocktakingDate),
		}));
	};

	public delete = async (id: string): Promise<void> => {
		await this.api.stocktaking.deleteStocktaking(id);
	};
}

export type StocktakingRepository = InstanceType<typeof Repository>;

const instance = Object.freeze(new Repository());

export const useStocktakingRepository = () => instance;
