import * as luxon from 'luxon';
import { type Component, createSignal } from 'solid-js';
import type { Journal } from '@/entities/stock/models/journal';
import { useJournalepository } from '@/entities/stock/respository/journal';
import JournalCalendar from '@/features/stock/ui/journal/JournalCalendar';
import DateInput from '@/shared/ui/DateInput';

const JournalListPage: Component = () => {
	const journalRepository = useJournalepository();

	const [journals, setJournals] = createSignal<Journal[]>([]);

	const onMonthChange = async (year: number, month: number) => {
		const firstDate = luxon.DateTime.local(year, month, 1);

		const lastDate = luxon.DateTime.local(year, month).endOf('month');

		const journals = await journalRepository.find({
			periodStart: firstDate.toJSDate(),
			periodEnd: lastDate.toJSDate(),
		});

        console.log(journals);

		setJournals(journals);
	};
	return (
		<article class="size-full flex flex-col gap-10 p-10">
			<section class="flex justify-between gap-10">
				<div class="flex gap-10">
					<DateInput label="検索（開始日）" />
					<DateInput label="検索（終了日）" />
				</div>
			</section>
			<section class="flex flex-col gap-10 h-[70vh]">
				<JournalCalendar journals={journals()} onMonthChange={onMonthChange}/>
			</section>
		</article>
	);
};

export default JournalListPage;
