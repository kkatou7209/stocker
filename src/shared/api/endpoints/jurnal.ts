import z from 'zod';

export const JournalRecordData = z.object({
	supplierName: z.string().nonempty(),
	supplyName: z.string().nonempty(),
	unitName: z.string().nonempty(),
	unitPrice: z.number().nonnegative(),
	quantity: z.number().nonnegative(),
	supplyId: z.string().nonempty(),
	supplierId: z.string(),
});

export type JournalRecordData = z.infer<typeof JournalRecordData>;

export const JournalData = z.object({
	id: z.string().nonempty(),
	entryDate: z.number().nonoptional(),
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
	/**
	 * list all journals
	 */
	listAllJournals: () => Promise<JournalData[]>;
	/**
	 * get a journal by id
	 */
	getJournalById: (id: string) => Promise<JournalData | null>;
	/**
	 * get a jorunal at specific date
	 */
	getJournalAt: (date: Date) => Promise<JournalData | null>;
	/**
	 * create a new journal
	 */
	createJournal: (command: CreateJournalCommand) => Promise<JournalData>;
	/**
	 * update a jorunal
	 */
	updateJournal: (command: UpdateJournalCommand) => Promise<void>;
	/**
	 * search jorunals
	 */
	searchJournals: (query: JournalQuery) => Promise<JournalData[]>;
}
