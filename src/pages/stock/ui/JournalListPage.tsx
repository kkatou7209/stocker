import * as luxon from 'luxon';
import { type Component, createSignal, onMount } from 'solid-js';
import type { Journal } from '@/entities/stock/models/journal';
import { useJournalepository } from '@/entities/stock/respository/journal';
import JournalCalendar from '@/features/stock/ui/journal/JournalCalendar';

const JournalListPage: Component = () => {
	
    const journalRepository = useJournalepository();

    const [year, setYear] = createSignal(2025);

    const [month, setMonth] = createSignal(1);

	const [journals, setJournals] = createSignal<Journal[]>([]);

    const reload = async () => {

        const firstDate = luxon.DateTime.local(year(), month(), 1);

		const lastDate = luxon.DateTime.local(year(), month()).endOf('month');

		const journals = await journalRepository.find({
			periodStart: firstDate.toJSDate(),
			periodEnd: lastDate.toJSDate(),
		});

		setJournals(journals);
    }

	const onMonthChange = async (year: number, month: number) => {
		
        setYear(year);
        setMonth(month);

        reload();
	};

    onMount(() => {

        const today = luxon.DateTime.now();

        setYear(today.year);
        setMonth(today.month);

        reload();
    });

	return (
		<article class="size-full flex flex-col gap-10 p-10">
			<section class="flex flex-col gap-10 h-[80vh]">
				<JournalCalendar journals={journals()} onMonthChange={onMonthChange}/>
			</section>
		</article>
	);
};

export default JournalListPage;
