import z from 'zod';

export const StocktakingRecord = z.object({
	/**
	 * supply name of the supply stocktaken
	 */
	supplyName: z.string().nonempty(),
	/**
	 * unit of supply
	 */
	unitName: z.string().nonempty(),
	/**
	 * unit price of supply
	 */
	unitPrice: z.number().nonnegative(),
	/**
	 * quantity that stocktaken
	 */
	quantity: z.number().nonnegative(),
	/**
	 * total price of this record
	 */
	totalPrice: z.number().nonnegative(),
	/**
	 * id of supply
	 */
	supplyId: z.string().nonempty(),
});

export const Stocktaking = z.object({
	/**
	 * id of stocktaking
	 */
	id: z.string().nonempty(),
	/**
	 * date of stocktaking
	 */
	stocktakingDate: z.date().nonoptional(),
	/**
	 * total price of this stocktaking
	 */
	totalPrice: z.number().nonnegative(),
	/**
	 * records of stocktaking
	 */
	records: z.array(StocktakingRecord),
});

export type StocktakingRecord = z.infer<typeof StocktakingRecord>;

export type Stocktaking = z.infer<typeof Stocktaking>;
