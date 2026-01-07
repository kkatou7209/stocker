import { ChevronLeftIcon, ChevronRightIcon, FileCheckIcon, PencilLineIcon } from 'lucide-solid';
import * as luxon from 'luxon';
import { type Component, createSignal, For, onMount, Show } from 'solid-js';
import type { Journal } from '@/entities/stock/models/journal';
import Button from '@/shared/ui/Button';

const months = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12] as const;

type Month = (typeof months)[number];

/**
 * Componet of journal calendar
 */
const JournalCalendar: Component<{
    year?: number;
    month?: Month;
	journals?: Journal[];
	onMonthChange?: (year: number, month: number) => unknown;
    onJournalClick?: (journal: Journal) => unknown;
	onEditClick?: (journal: Date) => unknown;
}> = (props) => {
	const journals = () => props.journals;

	const [year, setYear] = createSignal(0);

	const [month, setMonth] = createSignal<Month>(1);

	const [dates, setDates] = createSignal<Date[][]>([]);

	/**
	 * Back to previous month
	 */
	const prev = () => {
		const prev = luxon.DateTime.local(year(), month(), 1).minus({
			months: 1,
		});

		setYear(prev.year);
		setMonth(prev.month as Month);

		props.onMonthChange?.(year(), month());

		reload();
	};

	/**
	 * Forward to next month
	 */
	const next = () => {
		const next = luxon.DateTime.local(year(), month(), 1).plus({
			months: 1,
		});

		setYear(next.year);
		setMonth(next.month as Month);

		props.onMonthChange?.(year(), month());

		reload();
	};

	/**
	 * Get journal of specific date
	 */
	const journalOfDate = (date: Date) => {
		return (
			journals()?.find(
				(j) =>
					j.entryDate.getFullYear() === date.getFullYear() &&
					j.entryDate.getMonth() === date.getMonth() &&
					j.entryDate.getDate() === date.getDate(),
			) ?? null
		);
	};

	/**
	 * Reload calendar
	 */
	const reload = () => {
		const firstDate = luxon.DateTime.local(year(), month(), 1);

		const calendarFirstDate = firstDate.minus({
			days: firstDate.weekday,
		});

		let current = calendarFirstDate.plus({});

		const calendarDates: Date[][] = [];

		let dates: Date[] = [];

		for (const num of Array.from({ length: 35 }).map((_, i) => i + 1)) {
			dates.push(current.toJSDate());
			current = current.plus({ days: 1 });

			if (num % 7 === 0) {
				calendarDates.push(dates);
				dates = [];
			}
		}

		setDates(calendarDates);
	};

	onMount(() => {
		const today = luxon.DateTime.now();

		setYear(props.year ?? today.year);
		setMonth(props.month ?? today.month);

		reload();
	});

	return (
		<table class="table-fixed size-full [&_th]:border [&_td]:border [&_th]:border-neutral-300 [&_td]:border-neutral-300 [&_td]:h-1/5">
			<thead>
				<tr class="h-20">
					<th colspan={7}>
						<div class="flex items-center justify-center gap-10">
							<Button color="soft" onClick={prev}>
								<ChevronLeftIcon />
							</Button>
							<div class="w-30 grid grid-cols-2">
								<span class="text-center">{year()}年</span>
								<span class="text-center">{month()}月</span>
							</div>
							<Button color="soft" onClick={next}>
								<ChevronRightIcon />
							</Button>
						</div>
					</th>
				</tr>
				<tr class="bg-base-200">
					<th class="text-secondary">日</th>
					<th>月</th>
					<th>火</th>
					<th>水</th>
					<th>木</th>
					<th>金</th>
					<th class="text-accent">土</th>
				</tr>
			</thead>
			<tbody>
				<For each={dates()}>
					{(weekDates) => (
						<tr>
							<For each={weekDates}>
								{(date) => (
									<td
										class={`${date.getDay() === 0 ? 'text-secondary' : date.getDay() === 6 ? 'text-accent' : ''}`}
									>
										<div class="flex flex-col size-full p-1">
											<div class="flex px-3 pt-1 justify-between items-center">
												<span
													class={`text-md ${(date.getMonth() + 1) !== month() ? 'opacity-50' : ''}`}
												>
													{date.getDate()}
												</span>
												<PencilLineIcon
													size={16}
													class='text-base-300 hover:opacity-50 cursor-pointer'
													onclick={() => props.onEditClick?.(date)}
												/>
											</div>
											<div class="grow h-full flex items-center justify-center">
												<Show
													when={journalOfDate(date)}
													fallback={''}
												>
													<span class="text-success hover:opacity-50">
														<FileCheckIcon size={27} onclick={() => {
                                                            const journal = journalOfDate(date);
                                                            journal && props.onJournalClick?.(journal);
                                                        }}/>
													</span>
												</Show>
											</div>
										</div>
									</td>
								)}
							</For>
						</tr>
					)}
				</For>
			</tbody>
		</table>
	);
};

export default JournalCalendar;
