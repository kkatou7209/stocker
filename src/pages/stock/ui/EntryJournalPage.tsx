/**
 * @fileoverview page for writing journal
 */
import { FilePenLineIcon } from 'lucide-solid';
import * as luxon from 'luxon';
import { type Component, createSignal, onMount } from 'solid-js';
import { useApp } from '@/app/contexts/AppContext';
import type { JournalRecord } from '@/entities/stock/models/journal';
import { useJournalepository } from '@/entities/stock/respository/journal';
import { useSupplierRespository } from '@/entities/stock/respository/supplier';
import JournalSheet from '@/features/stock/ui/journal/JournalSheet';
import Button from '@/shared/ui/Button';
import DateInput from '@/shared/ui/DateInput';

/**
 * @summary page for entrying journal.
 */
const EntryJournalPage: Component = () => {

	const app = useApp();
	const supplierRepository = useSupplierRespository();
	const journalRepository = useJournalepository();

	const isNewJournal = () => {
		const id = journalId();

		if (!id) return true;

		return id.trim().length === 0;
	}

	const [journalId, setJournalId] = createSignal<string | null>(null);

	const [entryDate, setEntryDate] = createSignal(
		luxon.DateTime.now().toJSDate(),
	);

	const [records, setRecords] = createSignal<JournalRecord[]>([]);

	app.setPageTitle('記帳');

	const reload = async () => {

		const journal = await journalRepository.getAt(entryDate());

		if (journal) {

			setJournalId(journal.id);
	
			setRecords(journal.records);

			return;
		}

		setJournalId(null);

		const suppliers = await supplierRepository.list();

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
	}

	const add = async () => {

		const date = entryDate();

		const journal = await journalRepository.add({
			entryDate: date,
			records: [...records()],
		});

		setJournalId(journal.id);

		app.toastInfo('登録しました。');
	};

	const edit = async () => {

		const id = journalId();

		if (!id) return;

		const date = entryDate();

		await journalRepository.edit({
			id,
			entryDate: date,
			records: [...records()],
		});

		app.toastInfo('更新しました。');
	}

	const onDateChange = async (date: Date | null) => {

		if (!date) {
			setJournalId(null);
			setRecords([]);
			return;
		}

		setEntryDate(date);

		await reload();
	}

	onMount(async () => {

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
				<Button onClick={() => isNewJournal() ? add() : edit()}>
					<FilePenLineIcon class="size-4" />
					<span>登録</span>
				</Button>
			</section>
			<section class="max-h-[70vh] overflow-auto">
				<JournalSheet value={records()} onChange={setRecords}/>
			</section>
		</article>
	);
};

export default EntryJournalPage;
