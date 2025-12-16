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
	 * sum of prices of supply
	 */
	totalPrice: z.number().nonnegative(),
	/**
	 * id of supply
	 */
	supplyId: z.string().nonempty(),
});

export const Stocktaking = z.object({
	id: z.string().nonempty(),
	stocktakingDate: z.date().nonoptional(),
	records: z.array(StocktakingRecord),
});

export type StocktakingRecord = z.infer<typeof StocktakingRecord>;

export type Stocktaking = z.infer<typeof Stocktaking>;
