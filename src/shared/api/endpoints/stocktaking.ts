import z from 'zod';

export const StocktakingRecordData = z.object({
	supplyName: z.string().nonempty(),
	unitName: z.string().nonempty(),
	unitPrice: z.number().nonnegative(),
	quantity: z.number().nonnegative(),
	totalPrice: z.number().nonnegative(),
	supplyId: z.string().nonempty(),
});

export const StocktakingData = z.object({
	id: z.string().nonempty(),
	stocktakingDate: z.date().nonoptional(),
	records: z.array(StocktakingRecordData),
});

export type StocktakingRecordData = z.infer<typeof StocktakingRecordData>;

export type StocktakingData = z.infer<typeof StocktakingData>;

export type AddStocktakingCommand = Omit<StocktakingData, 'id'>;

export type UpdateStocktakingCommand = StocktakingData;

export interface StocktakingQuery {
	periodStart?: Date;
	periodEnd?: Date;
}

export interface StocktakingEndpoints {
	listAllStocktakings: () => Promise<StocktakingData[]>;
	getStocktakingById: (id: string) => Promise<StocktakingData | null>;
	updateStocktaking: (command: UpdateStocktakingCommand) => Promise<void>;
	addStocktaking: (
		command: AddStocktakingCommand,
	) => Promise<StocktakingData>;
	searchStocktakings: (query: StocktakingQuery) => Promise<StocktakingData[]>;
}
