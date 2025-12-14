import type { Journal, JournalRecord } from '@/entities/stock/models/journal';
import { useApi } from '@/shared/api';

class Repository {
	private api = useApi();

	public list = async (): Promise<Journal[]> => {
		return this.api.journal.listAllJournals();
	};

	public get = async (id: string): Promise<Journal | null> => {
		return this.api.journal.getJournalById(id);
	};

	public add = async (journal: {
		entryDate: Date;
		records: JournalRecord[];
	}): Promise<void> => {
		await this.api.journal.createJournal({
			entryDate: journal.entryDate,
			records: [...journal.records],
		});
	};

	public edit = async (journal: {
		id: string;
		entryDate: Date;
		records: JournalRecord[];
	}): Promise<void> => {
		await this.api.journal.updateJournal({
			id: journal.id,
			entryDate: journal.entryDate,
			records: [...journal.records],
		});
	};

	public find = async (query: {
		periodStart?: Date;
		periodEnd?: Date;
		supplierName?: string;
		supplyName?: string;
	}): Promise<Journal[]> => {
		return await this.api.journal.searchJournals(query);
	};
}

export type JournalRepository = InstanceType<typeof Repository>;

const instance = Object.freeze(new Repository());

export const useJournalepository = () => instance;
