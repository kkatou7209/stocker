import type { Supply } from '@/entities/stock/models/supply';
import { useApi } from '@/shared/api';

class Repository {
	private api = useApi();

	public list = async (): Promise<Supply[]> => {
		return await this.api.supply.listAllSupplies();
	};

	public get = async (id: string): Promise<Supply | null> => {
		return this.api.supply.getSupplyById(id);
	};

	public add = async (supply: {
		supplyName: string;
		unitName: string;
		supplierId: string;
	}): Promise<void> => {
		await this.api.supply.registerSupply(supply);
	};

	public edit = async (supply: {
		id: string;
		name: string;
		unitName: string;
	}): Promise<void> => {
		await this.api.supply.updateSupply({
			supplyId: supply.id,
			supplyName: supply.name,
			unitName: supply.unitName,
		});
	};

	public delete = async (id: string): Promise<void> => {
		await this.api.supply.deleteSupply(id);
	};
}

/**
 * repository of supply
 */
export type SupplyRepository = InstanceType<typeof Repository>;

const instance = Object.freeze(new Repository());

/**
 * get supply repository
 */
export const useSupplyRepository = () => instance;
