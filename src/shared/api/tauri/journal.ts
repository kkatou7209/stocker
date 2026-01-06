import { invoke } from '@tauri-apps/api/core';
import z from 'zod';
import {
	JournalData,
	type JournalEndpoints,
	type JournalQuery,
	type RecordJournalCommand,
	type UpdateJournalCommand,
} from '@/shared/api/endpoints/jurnal';

export const tauriJournalEndpoint: JournalEndpoints = Object.freeze({
	listAllJournals: async (): Promise<JournalData[]> => {
		const journals = await invoke<JournalData[]>('list_all_journals');

		return journals;
	},
	getJournalById: async (id: string): Promise<JournalData | null> => {
		const journal = await invoke<JournalData | null>('get_journal_by_id', {
			id,
		});

		const validated = JournalData.nullable().parse(journal);

		return validated;
	},
	getJournalAt: async (date: Date): Promise<JournalData | null> => {
		const journal = await invoke<JournalData | null>('get_journal_at', {
			date: date.getTime(),
		});

		const validated = JournalData.nullable().parse(journal);

		return validated;
	},
	recordJournal: async (
		command: RecordJournalCommand,
	): Promise<JournalData> => {
		const journal = await invoke<JournalData>('record_journal', {
			command,
		});

		const validated = JournalData.parse(journal);

		return validated;
	},
	updateJournal: async (command: UpdateJournalCommand): Promise<void> => {
		await invoke<JournalData>('update_journal', { command });
	},
	searchJournals: async (query: JournalQuery): Promise<JournalData[]> => {
		const journals = await invoke<JournalData[]>('search_journals', {
			query,
		});

		const validated = z.array(JournalData).parse(journals);

		return validated;
	},
	deleteJournal: async (id: string): Promise<void> => {
		await invoke<void>('delete_journal', { id });
	},
});
