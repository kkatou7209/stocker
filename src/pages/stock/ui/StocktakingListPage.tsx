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

/**
 * Page component of stocktaking history
 */
const StocktakingListPage: Component = () => {
	const app = useApp();

	app.setPageTitle('棚卸履歴');

	const stocktakingRepository = useStocktakingRepository();

	const [periodStart, setPeriodStart] = createSignal<Date | null>(null);

	const [periodEnd, setPeriodEnd] = createSignal<Date | null>(null);

	const [stocktakings, setStocktakings] = createSignal<Stocktaking[]>([]);

	const reload = async () => {
		const stocks = await stocktakingRepository.find({
			periodStart: periodStart() ?? undefined,
			periodEnd: periodEnd() ?? undefined,
		});

		stocks.sort(
			(a, b) => b.stocktakingDate.getTime() - a.stocktakingDate.getTime(),
		);

		setStocktakings(stocks);
	};

	onMount(async () => {
		const today = luxon.DateTime.now();

		const threeYearsAgo = today.minus({
			years: 3,
		});

		setPeriodStart(threeYearsAgo.toJSDate());

		reload();
	});

	createEffect(async () => {
		const start = periodStart();
		const end = periodEnd();

		if (start && end && start.getTime() > end.getTime()) {
			setPeriodStart(end);
		}

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
								<li class="">
									<details
										class="collapse collapse-arrow bg-base-200"
										name={`stocktaking-${stocktaking.id}`}
									>
										<summary class="collapse-title">
											{luxon.DateTime.fromJSDate(
												stocktaking.stocktakingDate,
											).toFormat('yyyy年M月d日')}
										</summary>
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
		</article>
	);
};

export default StocktakingListPage;
