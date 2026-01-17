/**
 * @fileoverview page for viewing journal history
 */
import { useNavigate } from '@solidjs/router';
import { XIcon } from 'lucide-solid';
import * as luxon from 'luxon';
import { type Component, createSignal, onMount, Show } from 'solid-js';
import { useApp } from '@/app/contexts/AppContext';
import type { Journal } from '@/entities/stock/models/journal';
import { useJournalepository } from '@/entities/stock/respository/journal';
import JournalCalendar from '@/features/stock/ui/journal/JournalCalendar';
import JournalTable from '@/features/stock/ui/journal/JournalTable';
import { Confirm } from '@/shared/ui/modals/Confirm';

/**
 * Page component of journal history
 */
const JournalListPage: Component = () => {
	const app = useApp();

	const navigation = useNavigate();

	app.setPageTitle('記帳履歴');

	const journalRepository = useJournalepository();

	const [year, setYear] = createSignal(2025);

	const [month, setMonth] = createSignal(1);

	const [journals, setJournals] = createSignal<Journal[]>([]);

	const [selectedJournal, setSelectedJournal] = createSignal<Journal>();

	const [showJournalView, setShowJournalView] = createSignal(false);

	const [confirmOpen, setConfirmOpen] = createSignal(false);

	/**
	 * Reload journal history
	 */
	const reload = async () => {
		const firstDate = luxon.DateTime.local(year(), month(), 1);

		const lastDate = luxon.DateTime.local(year(), month()).endOf('month');

		try {
			const journals = await journalRepository.find({
				periodStart: firstDate.toJSDate(),
				periodEnd: lastDate.toJSDate(),
			});

			setJournals(journals);
		} catch (err) {
			app.handleError('記帳履歴の読み込みに失敗しました。', err);
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

	const confirm = async () => {
		const journal = selectedJournal();

		try {
			if (!journal) {
				app.handleError('システムエラーが発生しました。', new Error('Selected journal is null'));
				return;
			}

			try {
				await journalRepository.delete(journal.id);

				reload();
			} catch (_) {
				app.handleError('記帳履歴の削除に失敗しました。', new Error('Failed to delete journal history'));
				return;
			}

            app.toastInfo('記帳を削除しました。');

			try {
				await reload();
			} catch (_) {
				app.handleError('記帳履歴の更新に失敗しました。', new Error('Failed to reload journal history'));
			}
		} finally {
			setConfirmOpen(false);
		}
	};

	const onClickDelete = async () => {
		setConfirmOpen(true);
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
						<button
							type="button"
                            class='absolute right-4 top-4 hover:opacity-50 hover:cursor-pointer'
							onclick={() => setShowJournalView(false)}
						>
							<XIcon />
						</button>
						<section class="flex justify-between">
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
						<section>
							<button
								type="button"
								class="btn btn-error"
								onclick={onClickDelete}
							>
								削除
							</button>
						</section>
					</div>
					<button
						type="button"
						class="modal-backdrop"
						onclick={() => setShowJournalView(false)}
					></button>
				</dialog>
			</Show>

			<Confirm
				open={confirmOpen()}
				onCancel={() => setConfirmOpen(false)}
				onConfirm={confirm}
			>
				<div class="flex flex-col items-center gap-3">
					<p>一度削除すると元に戻せません。</p>
					<p>本当に削除しますか？</p>
				</div>
			</Confirm>
		</article>
	);
};

export default JournalListPage;
