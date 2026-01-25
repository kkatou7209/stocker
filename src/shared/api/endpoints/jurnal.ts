import z from 'zod';

export const JournalRecordData = z.object({
	supplierName: z.string().nonempty(),
	supplyName: z.string().nonempty(),
	unitName: z.string().nonempty(),
	unitPrice: z.number().nonnegative(),
	quantity: z.number().nonnegative(),
	supplyId: z.string().nonempty(),
	supplierId: z.string(),
	totalPrice: z.number().nonnegative(),
});

export type JournalRecordData = z.infer<typeof JournalRecordData>;

export const JournalData = z.object({
	id: z.string().nonempty(),
	entryDate: z.number().nonoptional(),
	totalPrice: z.number().nonnegative(),
	records: z.array(JournalRecordData),
});

export type JournalData = z.infer<typeof JournalData>;

export type RecordJournalCommand = Omit<JournalData, 'id'>;

export type UpdateJournalCommand = Omit<JournalData, 'entryDate'>;

export type JournalQuery = {
	periodStart?: number | null;
	periodEnd?: number | null;
	supplierName?: string | null;
	supplyName?: string | null;
};

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
	recordJournal: (command: RecordJournalCommand) => Promise<JournalData>;
	/**
	 * update a jorunal
	 */
	updateJournal: (command: UpdateJournalCommand) => Promise<void>;
	/**
	 * search jorunals
	 */
	searchJournals: (query: JournalQuery) => Promise<JournalData[]>;
	/**
	 * delete a journal
	 */
	deleteJournal: (id: string) => Promise<void>;
}
