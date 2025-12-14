import z from 'zod';

export const StocktakingRecord = z.object({
	supplyName: z.string().nonempty(),
	unitName: z.string().nonempty(),
	unitPrice: z.number().nonnegative(),
	quantity: z.number().nonnegative(),
	totalPrice: z.number().nonnegative(),
	supplyId: z.string().nonempty(),
});

export const Stocktaking = z.object({
	id: z.string().nonempty(),
	stocktakingDate: z.date().nonoptional(),
	records: z.array(StocktakingRecord),
});

export type StocktakingRecord = z.infer<typeof StocktakingRecord>;

export type Stocktaking = z.infer<typeof Stocktaking>;
