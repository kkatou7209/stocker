import z from 'zod';

export const SupplyData = z.object({
	id: z.string().trim().nonempty().readonly(),
	name: z.string().trim().nonempty(),
	unitName: z.string().trim().nonempty(),
	supplierId: z.string().trim().nonempty().readonly(),
});

export type SupplyData = z.infer<typeof SupplyData>;

export const AddSupplyCommand = z.object({
	supplierId: z.string().trim().nonempty(),
	supplyName: z.string().trim().nonempty(),
	unitName: z.string().trim().nonempty(),
});

export type AddSupplyCommand = z.infer<typeof AddSupplyCommand>;

export const UpdateSupplyCommand = z.object({
	supplyId: z.string().trim().nonempty(),
	supplyName: z.string().trim().nonempty(),
	unitName: z.string().trim().nonempty(),
});

export type UpdateSupplyCommand = z.infer<typeof UpdateSupplyCommand>;

export interface SupplyEndpoint {
	/**
	 * list all supplies.
	 */
	readonly listAllSupplies: () => Promise<SupplyData[]>;
	/**
	 * get a supply from id
	 */
	readonly getSupplyById: (id: string) => Promise<SupplyData | null>;
	/**
	 * add a new supply
	 */
	readonly createSupply: (command: AddSupplyCommand) => Promise<void>;
	/**
	 * update a supply
	 */
	readonly updateSupply: (command: UpdateSupplyCommand) => Promise<void>;
}
