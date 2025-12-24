import type { Journal, JournalRecord } from '@/entities/stock/models/journal';
import { useApi } from '@/shared/api';

class Repository {
	private api = useApi();

	public list = async (): Promise<Journal[]> => {
		const journals = await this.api.journal.listAllJournals();

		return journals.map((journal) => ({
			...journal,
			entryDate: new Date(journal.entryDate),
		}));
	};

	public get = async (id: string): Promise<Journal | null> => {
		const journal = await this.api.journal.getJournalById(id);

		if (!journal) return null;

		return { ...journal, entryDate: new Date(journal.entryDate) };
	};

	public getAt = async (date: Date): Promise<Journal | null> => {
		const journal = await this.api.journal.getJournalAt(date);

		if (!journal) return null;

		return { ...journal, entryDate: new Date(journal.entryDate) };
	};

	public add = async (journal: {
		entryDate: Date;
		records: JournalRecord[];
	}): Promise<Journal> => {
		const registered = await this.api.journal.createJournal({
			entryDate: journal.entryDate.getTime(),
			records: [...journal.records],
		});

		return { ...registered, entryDate: new Date(registered.entryDate) };
	};

	public edit = async (journal: {
		id: string;
		entryDate: Date;
		records: JournalRecord[];
	}): Promise<void> => {
		await this.api.journal.updateJournal({
			id: journal.id,
			entryDate: journal.entryDate.getTime(),
			records: [...journal.records],
		});
	};

	public find = async (query: {
		periodStart?: Date;
		periodEnd?: Date;
		supplierName?: string;
		supplyName?: string;
	}): Promise<Journal[]> => {
		const journals = await this.api.journal.searchJournals(query);

		return journals.map((journal) => ({
			...journal,
			entryDate: new Date(journal.entryDate),
		}));
	};
}

export type JournalRepository = InstanceType<typeof Repository>;

const instance = Object.freeze(new Repository());

export const useJournalepository = () => instance;
