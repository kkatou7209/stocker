/**
 * @fileoverview page for viewing journal history
 */

import { useNavigate } from '@solidjs/router';
import * as luxon from 'luxon';
import { type Component, createSignal, onMount, Show } from 'solid-js';
import { useApp } from '@/app/contexts/AppContext';
import { useError } from '@/app/stores/error';
import type { Journal } from '@/entities/stock/models/journal';
import { useJournalepository } from '@/entities/stock/respository/journal';
import JournalCalendar from '@/features/stock/ui/journal/JournalCalendar';
import JournalTable from '@/features/stock/ui/journal/JournalTable';

const JournalListPage: Component = () => {
	const app = useApp();

	const error = useError();

	const navigation = useNavigate();

	app.setPageTitle('記帳履歴');

	const journalRepository = useJournalepository();

	const [year, setYear] = createSignal(2025);

	const [month, setMonth] = createSignal(1);

	const [journals, setJournals] = createSignal<Journal[]>([]);

	const [selectedJournal, setSelectedJournal] = createSignal<Journal>();

	const [showJournalView, setShowJournalView] = createSignal(false);

	const reload = async () => {
		const firstDate = luxon.DateTime.local(year(), month(), 1);

		const lastDate = luxon.DateTime.local(year(), month()).endOf('month');

        try {
            
            const journals = await journalRepository.find({
                periodStart: firstDate.toJSDate(),
                periodEnd: lastDate.toJSDate(),
            });
    
            setJournals(journals);

        } catch (_) {
            error.handle(new Error(`記帳履歴の読み込みに失敗しました。`), () => navigation('/journal/list'));
        }
	};

	const onMonthChange = async (year: number, month: number) => {
		setYear(year);
		setMonth(month);

		reload();
	};

	const onJournalClick = (journal: Journal) => {
		setSelectedJournal(journal);

		setShowJournalView(true);
	};

	const onEditClick = (date: Date) => {
		navigation('/journal', {
			state: {
				date,
			},
		});
	};

	onMount(() => {

        error.handle(new Error(`記帳履歴の読み込みに失敗しました。`), () => navigation('/error'));
        
		const today = luxon.DateTime.now();

		setYear(today.year);
		setMonth(today.month);

		reload();
	});

	return (
		<article class="size-full flex flex-col gap-10 p-10">
			<section class="flex flex-col gap-10 h-[80vh]">
				<JournalCalendar
					journals={journals()}
					onMonthChange={onMonthChange}
					onJournalClick={onJournalClick}
					onEditClick={onEditClick}
				/>
			</section>

			<Show when={selectedJournal()}>
				<dialog class="modal" open={showJournalView()}>
					<div class="modal-box max-h-[70vh] flex flex-col gap-6 p-10 min-w-[50vw]">
						<section>
							{(() => {
								const date = selectedJournal()?.entryDate;

								if (!date)
									throw new Error(
										'システムエラーが発生しました。',
									);

								return (
									<h3>
										{luxon.DateTime.fromJSDate(
											date,
										).toFormat('yyyy年M月dd日')}
									</h3>
								);
							})()}
						</section>
						<section class="overflow-auto">
							<JournalTable value={selectedJournal()?.records} />
						</section>
					</div>
					<button
						type="button"
						class="modal-backdrop"
						onclick={() => setShowJournalView(false)}
					></button>
				</dialog>
			</Show>
		</article>
	);
};

export default JournalListPage;
