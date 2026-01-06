import z from 'zod';

export const StocktakingRecordData = z.object({
	supplyName: z.string().nonempty(),
	unitName: z.string().nonempty(),
	unitPrice: z.number().nonnegative(),
	quantity: z.number().nonnegative(),
	supplyId: z.string().nonempty(),
});

export const StocktakingData = z.object({
	id: z.string().nonempty(),
	stocktakingDate: z.number().nonoptional(),
	records: z.array(StocktakingRecordData),
});

export type StocktakingRecordData = z.infer<typeof StocktakingRecordData>;

export type StocktakingData = z.infer<typeof StocktakingData>;

export type AddStocktakingCommand = Omit<StocktakingData, 'id'>;

export type UpdateStocktakingCommand = Omit<StocktakingData, 'stocktakingDate'>;

export type StocktakingQuery = {
	periodStart?: number;
	periodEnd?: number;
};

export interface StocktakingEndpoints {
	/**
	 * list all stocktaking data
	 */
	listAllStocktakings: () => Promise<StocktakingData[]>;
	/**
	 * get a stocktaking data by id
	 */
	getStocktakingById: (id: string) => Promise<StocktakingData | null>;
	/**
	 * get a stocktaking date of specified date
	 */
	getStocktakingAt: (date: number) => Promise<StocktakingData | null>;
	/**
	 * update a stocktaking data
	 */
	updateStocktaking: (command: UpdateStocktakingCommand) => Promise<void>;
	/**
	 * create a new stocktaking data
	 */
	recordStocktaking: (
		command: AddStocktakingCommand,
	) => Promise<StocktakingData>;
	/**
	 * search stocktakign data
	 */
	searchStocktakings: (query: StocktakingQuery) => Promise<StocktakingData[]>;
	/**
	 * delete a stocktaking data
	 */
	deleteStocktaking: (id: string) => Promise<void>;
}
