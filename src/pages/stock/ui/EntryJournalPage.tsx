import { FilePenIcon } from 'lucide-solid';
import * as luxon from 'luxon';
import { type Component, createSignal, onMount } from 'solid-js';
import { useApp } from '@/app/contexts/AppContext';
import type { JournalRecord } from '@/entities/stock/models/journal';
import { useJournalepository } from '@/entities/stock/respository/journal';
import { useSupplierRespository } from '@/entities/stock/respository/supplier';
import JournalSheet from '@/features/stock/ui/journal/JournalSheet';
import Button from '@/shared/ui/Button';
import DateInput from '@/shared/ui/DateInput';

const EntryJournalPage: Component = () => {
	const app = useApp();
	const supplierRepository = useSupplierRespository();
	const journalRepository = useJournalepository();

	const [entryDate, setEntryDate] = createSignal(
		luxon.DateTime.now().toJSDate(),
	);

	const [records, setRecords] = createSignal<JournalRecord[]>([]);

	app.setPageTitle('記帳');

	const add = async () => {
		const date = entryDate();

		if (!date) return;

		await journalRepository.add({
			entryDate: date,
			records: [...records()],
		});
	};

	onMount(async () => {
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
	});

	onMount(() => {});

	return (
		<article class="size-full flex flex-col gap-10 p-10">
			<section class="flex justify-between">
				<DateInput
					label="記帳日"
					value={entryDate()}
					onChange={setEntryDate}
				/>
				<Button onClick={add}>
					<FilePenIcon class="size-4" />
					<span>登録</span>
				</Button>
			</section>
			<section class="max-h-[70vh] overflow-auto">
				<JournalSheet value={records()} />
			</section>
		</article>
	);
};

export default EntryJournalPage;
