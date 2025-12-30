import z from 'zod';
import { SupplyData } from '@/shared/api/endpoints/supply';

export const SupplierData = z.object({
	id: z.string().trim().nonempty().readonly(),
	name: z.string().trim().nonempty(),
	supplies: z.array(SupplyData),
});

export type SupplierData = z.infer<typeof SupplierData>;

export const SupplierQuery = z.object({
	supplierName: z.string().trim().nullable().optional(),
	supplyName: z.string().trim().nullable().optional(),
});

export type SupplierQuery = z.infer<typeof SupplierQuery>;

export const AddSupplierCommand = z.object({
	supplierName: z.string().trim().nonempty(),
});

export type AddSupplierCommand = z.infer<typeof AddSupplierCommand>;

export const UpdateSupplierCommand = z.object({
	supplierId: z.string().trim().nonempty(),
	supplierName: z.string().trim().nonempty(),
});

export type UpdateSupplierCommand = z.infer<typeof UpdateSupplierCommand>;

/**
 * API endpoints of supplier
 */
export interface SupplierEndpoint {
	/**
	 * list all suppliers.
	 */
	readonly listAllSuppliers: () => Promise<SupplierData[]>;
	/**
	 * get a supplier from id
	 */
	readonly getSupplierById: (id: string) => Promise<SupplierData | null>;
	/**
	 * search suppliers.
	 */
	readonly searchSuppliers: (query: SupplierQuery) => Promise<SupplierData[]>;
	/**
	 * add a new supplier
	 */
	readonly registerSupplier: (command: AddSupplierCommand) => Promise<void>;
	/**
	 * update a supplier
	 */
	readonly updateSupplier: (command: UpdateSupplierCommand) => Promise<void>;
}
