import z from 'zod';

export const JournalRecordData = z.object({
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

export type JournalRecordData = z.infer<typeof JournalRecordData>;

export const JournalData = z.object({
	id: z.string().nonempty(),
	entryDate: z.date().nonoptional(),
	records: z.array(JournalRecordData),
});

export type JournalData = z.infer<typeof JournalData>;

export type CreateJournalCommand = Omit<JournalData, 'id'>;

export type UpdateJournalCommand = JournalData;

export interface JournalQuery {
	periodStart?: Date | null;
	periodEnd?: Date | null;
	supplierName?: string | null;
	supplyName?: string | null;
}

export interface JournalEndpoints {
	listAllJournals: () => Promise<JournalData[]>;

	getJournalById: (id: string) => Promise<JournalData | null>;

	getJournalAt: (date: Date) => Promise<JournalData | null>;

	createJournal: (command: CreateJournalCommand) => Promise<JournalData>;

	updateJournal: (command: UpdateJournalCommand) => Promise<void>;

	searchJournals: (query: JournalQuery) => Promise<JournalData[]>;
}
