import z from 'zod';

export const Supply = z.object({
	/**
	 * id of supply
	 */
	id: z.string().trim().nonempty().readonly(),
	/**
	 * name of supply
	 */
	name: z.string().trim().nonempty(),
	/**
	 * unit of supply
	 */
	unitName: z.string().trim().nonempty(),
	/**
	 * id of supplier the supply belongs to
	 */
	supplierId: z.string().trim().nonempty().readonly(),
});

/**
 * supply model
 */
export type Supply = z.infer<typeof Supply>;
