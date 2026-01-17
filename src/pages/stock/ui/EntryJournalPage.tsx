import { useLocation } from '@solidjs/router';
import { FilePenLineIcon } from 'lucide-solid';
import * as luxon from 'luxon';
import { type Component, createSignal, onMount } from 'solid-js';
import { useApp } from '@/app/contexts/AppContext';
import type { Journal, JournalRecord } from '@/entities/stock/models/journal';
import type { Supplier } from '@/entities/stock/models/supplier';
import { useJournalepository } from '@/entities/stock/respository/journal';
import { useSupplierRespository } from '@/entities/stock/respository/supplier';
import JournalSheet from '@/features/stock/ui/journal/JournalSheet';
import Button from '@/shared/ui/Button';
import DateInput from '@/shared/ui/DateInput';

/**
 * Page component of journal entry
 */
const EntryJournalPage: Component = () => {
	const app = useApp();

	const location = useLocation<{ date?: Date }>();

	const supplierRepository = useSupplierRespository();
	const journalRepository = useJournalepository();

	const isNewJournal = () => {
		const id = journalId();

		if (!id) return true;

		return id.trim().length === 0;
	};

	const [journalId, setJournalId] = createSignal<string | null>(null);

	const [entryDate, setEntryDate] = createSignal(
		luxon.DateTime.now().toJSDate(),
	);

	const [records, setRecords] = createSignal<JournalRecord[]>([]);

	app.setPageTitle('記帳');

	const reload = async () => {
		let journal: Journal | null;

		try {
			journal = await journalRepository.getAt(entryDate());
		} catch (err) {
			
			app.handleError(
				'記帳の読み込みに失敗しました。',
				err,
			);

			return;
		}

		if (journal) {
			setJournalId(journal.id);

			setRecords(journal.records);

			return;
		}

		setJournalId(null);

		let suppliers: Supplier[];

		try {
			suppliers = await supplierRepository.list();
		} catch (error) {
			app.handleError('仕入先の取得に失敗しました。', error);
			return;
		}

		const records: JournalRecord[] = suppliers.flatMap((supplier) => {
			return supplier.supplies.map((supply) => {
				return {
					supplierId: supplier.id,
					supplyId: supply.id,
					supplierName: supplier.name,
					supplyName: supply.name,
					unitName: supply.unitName,
					unitPrice: 0,
					quantity: 0,
					taxRate: 0,
					totalPrice: 0,
					totalPriceIncludeTax: 0,
				} as JournalRecord;
			});
		});

		setRecords(records);
	};

	const add = async () => {
		const date = entryDate();

		let journal: Journal | null;

		try {
			journal = await journalRepository.add({
				entryDate: date,
				records: [...records()],
			});
		} catch (err) {
			app.handleError(
				'記帳の登録に失敗しました。',
				err,
			);
			return;
		}

		setJournalId(journal.id);

		app.toastInfo('記帳を登録しました。');
	};

	const edit = async () => {
		const id = journalId();

		if (!id) return;

		try {
			await journalRepository.edit({
				id,
				records: [...records()],
			});
		} catch (err) {
			app.handleError(
				'記帳の更新に失敗しました。',
				err as Error,
			);
			return;
		}

		app.toastInfo('記帳を更新しました。');
	};

	const onDateChange = async (date: Date | null) => {
		if (!date) {
			setJournalId(null);
			setRecords([]);
			return;
		}

		setEntryDate(date);

		await reload();
	};

	onMount(async () => {
		const date = location.state?.date;

		if (date) {
			setEntryDate(date);
		}

		await reload();
	});

	return (
		<article class="size-full flex flex-col gap-10 p-10">
			<section class="flex justify-between">
				<DateInput
					label="記帳日"
					value={entryDate()}
					onChange={onDateChange}
				/>
				<Button onClick={() => (isNewJournal() ? add() : edit())}>
					<FilePenLineIcon class="size-4" />
					<span>登録</span>
				</Button>
			</section>
			<section class="max-h-[70vh] overflow-auto">
				<JournalSheet value={records()} onChange={setRecords} />
			</section>
		</article>
	);
};

export default EntryJournalPage;
