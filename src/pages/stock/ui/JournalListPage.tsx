/**
 * @fileoverview page for viewing journal history
 */
import * as luxon from 'luxon';
import { type Component, createSignal, onMount } from 'solid-js';
import type { Journal } from '@/entities/stock/models/journal';
import { useJournalepository } from '@/entities/stock/respository/journal';
import JournalCalendar from '@/features/stock/ui/journal/JournalCalendar';
import JournalTable from '@/features/stock/ui/journal/JournalTable';

const JournalListPage: Component = () => {
	
    const journalRepository = useJournalepository();

    const [year, setYear] = createSignal(2025);

    const [month, setMonth] = createSignal(1);

	const [journals, setJournals] = createSignal<Journal[]>([]);

    const [selectedJournal, setSelectedJournal] = createSignal<Journal>();

    const [showJournalView, setShowJournalView] = createSignal(false);

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

    const onJournalClick = (journal: Journal) => {

        setSelectedJournal(journal);

        setShowJournalView(true);
    }

    onMount(() => {

        const today = luxon.DateTime.now();

        setYear(today.year);
        setMonth(today.month);

        reload();
    });

	return (
		<article class="size-full flex flex-col gap-10 p-10">
			<section class="flex flex-col gap-10 h-[80vh]">
				<JournalCalendar journals={journals()} onMonthChange={onMonthChange} onJournalClick={onJournalClick}/>
			</section>

            <dialog class='modal' open={showJournalView()}>
                <div class='modal-box max-h-[70vh] flex flex-col gap-6 p-10 min-w-[50vw]'>
                    <section>
                        {(() => {
                            const date = selectedJournal()?.entryDate;
                            return <h3>{date ? luxon.DateTime.fromJSDate(date).toFormat('yyyy年M月dd日') : ''}</h3>;
                        })()}
                    </section>
                    <section class='overflow-auto'>
                        <JournalTable value={selectedJournal()?.records}/>
                    </section>
                </div>
                <button type='button' class='modal-backdrop' onclick={() => setShowJournalView(false)}></button>
            </dialog>
		</article>
	);
};

export default JournalListPage;
