import z from 'zod';

export const JournalRecord = z.object({
	supplierName: z.string().nonempty(),
	supplyName: z.string().nonempty(),
	unitName: z.string().nonempty(),
	unitPrice: z.number().nonnegative(),
	quantity: z.number().nonnegative(),
	taxRate: z.number().nonnegative(),
	totalPrice: z.number().nonnegative(),
	totalPriceIncludeTax: z.number().nonnegative(),
	supplyId: z.string().nonempty(),
	supplierId: z.string(),
});

export type JournalRecord = z.infer<typeof JournalRecord>;

export const Journal = z.object({
	id: z.string().nonempty(),
	entryDate: z.date().nonoptional(),
	records: z.array(JournalRecord),
});

export type Journal = z.infer<typeof Journal>;
