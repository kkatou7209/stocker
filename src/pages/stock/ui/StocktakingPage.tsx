import { PackageOpenIcon } from 'lucide-solid';
import * as luxon from 'luxon';
import { type Component, createSignal, onMount } from 'solid-js';
import { useApp } from '@/app/contexts/AppContext';
import type { Stocktaking, StocktakingRecord } from '@/entities/stock/models/stocktaking';
import type { Supply } from '@/entities/stock/models/supply';
import { useStocktakingRepository } from '@/entities/stock/respository/stocktaking';
import { useSupplyRepository } from '@/entities/stock/respository/supply';
import StocktakingSheet from '@/features/stock/ui/stocktaking/StocktakingSheet';
import Button from '@/shared/ui/Button';
import DateInput from '@/shared/ui/DateInput';

/**
 * Page component for stocktaking
 */
const StocktakingPage: Component = () => {
	const app = useApp();

	const supplyRepository = useSupplyRepository();
	const stocktakingRepository = useStocktakingRepository();

	app.setPageTitle('棚卸');

	const isNewStocktaking = (): boolean => {
		const id = stocktakingId();

		if (!id) return true;

		return id.trim().length === 0;
	};

	const [stocktakingId, setStocktakingId] = createSignal<string | null>(null);

	const [stocktakingDate, setStocktakingDate] = createSignal<Date | null>(
		luxon.DateTime.now().toJSDate(),
	);

	const [stocktakingRecords, setStocktakingRecords] = createSignal<
		StocktakingRecord[]
	>([]);

	/**
	 * Reload records
	 */
	const reload = async () => {
		setStocktakingId(null);

		let supplies: Supply[];

		try {
			supplies = await supplyRepository.list();
		} catch (err) {
			app.handleError('仕入品の取得に失敗しました。', err);
			return;
		}

		const records: StocktakingRecord[] = supplies.map((supply) => {
			return {
				supplyId: supply.id,
				supplyName: supply.name,
				unitName: supply.unitName,
				unitPrice: 0,
				quantity: 0,
			} as StocktakingRecord;
		});

		setStocktakingRecords(records);

		const date = stocktakingDate();

		if (!date) return;

		let stocktaking: Stocktaking | null;

		try {
			stocktaking = await stocktakingRepository.getAt(date)
		} catch (err) {
			app.handleError('棚卸の取得に失敗しました。', err);
			return;
		}

		if (!stocktaking) return;

		setStocktakingId(stocktaking.id);

		for (const [i, record] of records.entries()) {

			const registered = stocktaking.records.find(
				(r) => r.supplyId === record.supplyId,
			);

			if (!registered) {
				records.splice(i, 0, record);
				continue;
			}

			record.supplyName = registered.supplyName;
			record.unitName = registered.unitName;
			record.unitPrice = registered.unitPrice;
			record.quantity = registered.quantity;
		}

		setStocktakingRecords([...records]);
	};

	/**
	 * Add new stocktaking records
	 */
	const add = async () => {
		const date = stocktakingDate();

		if (!date) return;

		const stocktaking = await stocktakingRepository.add({
			stocktakingDate: date,
			records: [...stocktakingRecords()],
		});

		setStocktakingId(stocktaking.id);

		app.toastInfo('棚卸を登録しました。');
	};

	/**
	 * Update stocktaking records
	 */
	const edit = async () => {
		
		const date = stocktakingDate();

		const id = stocktakingId();

		if (!date || !id || id.trim().length === 0) return;

		try {
			await stocktakingRepository.edit({
				id,
				records: [...stocktakingRecords()],
			});
		} catch (error) {
			app.handleError('棚卸の更新に失敗しました。', error);
			return;
		}

		app.toastInfo('棚卸を更新しました。');
	};

	/**
	 * Handle date change
	 */
	const onDateChange = async (date: Date | null) => {
		setStocktakingDate(date);

		await reload();
	};

	onMount(async () => {
		await reload();
	});

	return (
		<article class="size-full flex flex-col gap-10 p-10">
			<section class="flex justify-between">
				<DateInput
					label="棚卸日"
					value={stocktakingDate()}
					onChange={onDateChange}
				/>
				<Button onClick={() => (isNewStocktaking() ? add() : edit())}>
					<PackageOpenIcon class="size-4" />
					<span>登録</span>
				</Button>
			</section>
			<section class="max-h-[70vh] overflow-auto">
				<StocktakingSheet
					value={stocktakingRecords()}
					onChange={setStocktakingRecords}
				/>
			</section>
		</article>
	);
};

export default StocktakingPage;
