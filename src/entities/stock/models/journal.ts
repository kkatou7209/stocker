import z from 'zod';

/**
 * record of journal
 */
export const JournalRecord = z.object({
	/**
	 * purchased supplier name
	 */
	supplierName: z.string().nonempty(),
	/**
	 * supply name purchased the supply from
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
	 * quantity of purchasing
	 */
	quantity: z.number().nonnegative(),
	/**
	 * tax rate of purchasing
	 */
	taxRate: z.number().nonnegative(),
	/**
	 * sum of purchased prices
	 */
	totalPrice: z.number().nonnegative(),
	/**
	 * sum of purchased prices include tax
	 */
	totalPriceIncludeTax: z.number().nonnegative(),
	/**
	 * id of purchased supply
	 */
	supplyId: z.string().nonempty(),
	/**
	 * id of supplier purchased from
	 */
	supplierId: z.string(),
});

export type JournalRecord = z.infer<typeof JournalRecord>;

/**
 * journal of purchasing
 */
export const Journal = z.object({
	/**
	 * id of journal
	 */
	id: z.string().nonempty(),
	/**
	 * date that journal entered
	 */
	entryDate: z.date().nonoptional(),
	/**
	 * records of purchasing
	 */
	records: z.array(JournalRecord),
});

export type Journal = z.infer<typeof Journal>;
