import { useNavigate } from '@solidjs/router';
import { CalendarArrowDownIcon, CalendarArrowUpIcon, PencilLineIcon, Trash2Icon } from 'lucide-solid';
import * as luxon from 'luxon';
import {
	type Component,
	createEffect,
	createSignal,
	For,
	onMount,
	Show,
} from 'solid-js';
import { useApp } from '@/app/contexts/AppContext';
import type { Stocktaking } from '@/entities/stock/models/stocktaking';
import { useStocktakingRepository } from '@/entities/stock/respository/stocktaking';
import StocktakingTable from '@/features/stock/ui/stocktaking/StocktakingTable';
import DateInput from '@/shared/ui/DateInput';
import { Confirm } from '@/shared/ui/modals/Confirm';

/**
 * Page component of stocktaking history
 */
const StocktakingListPage: Component = () => {
	
	const app = useApp();

	const navigation = useNavigate();

	app.setPageTitle('棚卸履歴');

	const stocktakingRepository = useStocktakingRepository();

	const [periodStart, setPeriodStart] = createSignal<Date | null>(null);

	const [periodEnd, setPeriodEnd] = createSignal<Date | null>(null);

	/**
	 * Stocktaking records
	 */
	const [stocktakings, setStocktakings] = createSignal<Stocktaking[]>([]);

	/**
	 * Confirmation modal open state
	 */
	const [confirmOpen, setConfirmOpen] = createSignal(false);

	/** 
	 * Selected sort order
	 */
	const [sort, setSort] = createSignal<'asc' | 'desc'>('desc');

	/**
	 * Selected stocktaking for deletion
	 */
	const [selectedStocktaking, setSelectedStocktaking] =
		createSignal<Stocktaking | null>(null);

	/**
	 * Accordion open/close states
	 */
	const [accordionStates, setAccordionStates] = createSignal<Map<string, boolean>>(
		new Map(),
	);

	/**
	 * Reload stocktaking records
	 */
	const reload = async () => {
		let stocktakings: Stocktaking[];
		
		try {
			stocktakings = await stocktakingRepository.find({
				periodStart: periodStart() ?? undefined,
				periodEnd: periodEnd() ?? undefined,
			});
		} catch (error) {
			app.handleError('棚卸履歴の読み込みに失敗しました。', error);
			return;
		}

		stocktakings.sort(
			(a, b) => b.stocktakingDate.getTime() - a.stocktakingDate.getTime(),
		);

		setStocktakings(stocktakings);


		for (const stock of stocktakings) {
			if (!accordionStates().has(stock.id)) {
				accordionStates().set(stock.id, false);
			}
		}

		setAccordionStates(new Map(accordionStates()));
	};

	/**
	 * Toggle sort order
	 */
	const toggleSort = () => {

		const current = sort();
		
		const newSort = current === 'asc' ? 'desc' : 'asc';
		
		setSort(newSort);

		const stocks = [ ...stocktakings() ];

		stocks.sort((a, b) => a.stocktakingDate.getTime() - b.stocktakingDate.getTime());

		if (newSort === 'desc') {
			stocks.reverse();
		}

		setStocktakings(stocks);
	};

	/**
	 * Handle accordion toggle
	 */
	const toggleAccordion = (stocktakingId: string, open: boolean) => {
		
		accordionStates().set(stocktakingId, open);
		setAccordionStates(new Map(accordionStates()));
	}

	/**
	 * Handle edit button click
	 */
	const onEditClick = (stocktaking: Stocktaking) => {
		navigation('/stocktaking', {
			state: {
				stocktakingId: stocktaking.stocktakingDate,
			},
		});
	};

	/**
	 * Handle delete button click
	 */
	const onDeleteClick = (stocktaking: Stocktaking) => {

		setSelectedStocktaking(stocktaking);
		
		// Ask user for deletion
		setConfirmOpen(true);
	};

	/**
	 * Handle delete confirmation
	 */
	const onDeleteConfirm = async () => {

		setConfirmOpen(false);

		const id = selectedStocktaking()?.id;

		if (!id) {
			app.handleError('システムエラーが発生しました。', new Error('ID is not set'));
			return;
		}

		try {
			await stocktakingRepository.delete(id);
		} catch (err) {
			app.handleError('棚卸履歴の削除に失敗しました。', err);
			return;
		}

		reload();
	};

	createEffect(async () => {
		// Ensure periodStart is not after periodEnd
		const start = periodStart();
		const end = periodEnd();

		if (start && end && start.getTime() > end.getTime()) {
			setPeriodStart(end);
		}

		reload();
	});

	onMount(async () => {
		reload();
	});

	return (
		<article class="size-full flex flex-col gap-10 p-10">
			<section class="flex items-center justify-between">
				<div class="flex items-center gap-10">
					<DateInput
						label="検索（開始日）"
						value={periodStart()}
						onChange={setPeriodStart}
					/>
					<DateInput
						label="検索（終了日）"
						value={periodEnd()}
						onChange={setPeriodEnd}
					/>
				</div>
			</section>
			<section class="max-h-[70vh] overflow-auto pb-100">
					<div class='mb-4 pr-4 flex justify-end'>
						<button
						type='button'
						class='hover:opacity-50 hover:cursor-pointer'
						onclick={toggleSort}
					>
						<Show when={sort() === 'asc'}>
							<CalendarArrowDownIcon class='size-5'/>
						</Show>
						<Show when={sort() === 'desc'}>
							<CalendarArrowUpIcon class='size-5'/>
						</Show>
					</button>
				</div>
				<Show
					when={stocktakings().length > 0}
					fallback={
						<p class="text-center text-gray-400">
							履歴がありません
						</p>
					}
				>
					<ul class="flex flex-col gap-5">
						<For each={stocktakings()}>
							{(stocktaking) => (
								<li>
									<details
										class="collapse collapse-arrow bg-base-200"
										name={`stocktaking-${stocktaking.id}`}
										open={accordionStates().get(stocktaking.id) ?? false}
										ontoggle={(e) => toggleAccordion(stocktaking.id, e.newState === 'open')}
									>
										<summary class="collapse-title">
											<section class="flex justify-between items-center">
												<div class="flex items-center gap-3">
													{/* Edit button */}
													<button
														type="button"
														class="btn btn-ghost"
														onclick={() =>
															onEditClick(
																stocktaking,
															)
														}
													>
														<PencilLineIcon class="size-4" />
													</button>

													{/* Stocktaking date */}
													<div>
														{luxon.DateTime.fromJSDate(
															stocktaking.stocktakingDate,
														).toFormat(
															'yyyy年M月d日',
														)}
													</div>
												</div>

												{/* Delete button */}
												<button
													type="button"
													class="btn btn-error btn-ghost"
													onclick={() =>
														onDeleteClick(
															stocktaking,
														)
													}
												>
													<Trash2Icon class="size-4" />
												</button>
											</section>
										</summary>

										{/* Stocktaking records */}
										<div class="collapse-content">
											<StocktakingTable
												value={stocktaking.records}
											/>
										</div>
									</details>
								</li>
							)}
						</For>
					</ul>
				</Show>
			</section>

			{/* Modal for confirmation */}
			<Confirm
				open={confirmOpen()}
				onCancel={() => setConfirmOpen(false)}
				onConfirm={onDeleteConfirm}
			>
				<div class='flex flex-col items-center gap-3'>
					<p>一度削除すると元に戻せません。</p>
					<p>本当に削除しますか？</p>
				</div>
			</Confirm>
		</article>
	);
};

export default StocktakingListPage;
