import type { Supplier } from '@/entities/stock/models/supplier';
import { type Api, useApi } from '@/shared/api';

class Repository {
	private api: Api = useApi();

	/**
	 * list all suppliers
	 */
	public list = async (): Promise<Supplier[]> => {
		const suppliers = await this.api.supplier.listAllSuppliers();

		return suppliers;
	};

	/**
	 * get a supplier by id
	 */
	public get = async (id: string): Promise<Supplier | null> => {
		const supplier = await this.api.supplier.getSupplierById(id);

		return supplier;
	};

	/**
	 * add a new supplier
	 */
	public add = async (supplier: { name: string }): Promise<void> => {
		await this.api.supplier.registerSupplier({
			supplierName: supplier.name,
		});
	};

	/**
	 * update a supplier
	 */
	public edit = async (supplier: {
		id: string;
		name: string;
	}): Promise<void> => {
		await this.api.supplier.updateSupplier({
			supplierId: supplier.id,
			supplierName: supplier.name,
		});
	};

	/**
	 * find suppliers
	 */
	public find = async (query: {
		supplierName?: string | null;
		supplyName?: string | null;
	}): Promise<Supplier[]> => {
		return await this.api.supplier.searchSuppliers({
			supplierName: query.supplierName ?? null,
			supplyName: query.supplyName ?? null,
		});
	};
}

/**
 * repository of suppliers
 */
export type SupplierRepository = InstanceType<typeof Repository>;

const insatnce = Object.freeze(new Repository());

/**
 * get a respository of suppliers.
 */
export const useSupplierRespository = () => insatnce;
