import { PackageOpenIcon } from 'lucide-solid';
import * as luxon from 'luxon';
import { type Component, createSignal, onMount } from 'solid-js';
import { useApp } from '@/app/contexts/AppContext';
import type { StocktakingRecord } from '@/entities/stock/models/stocktaking';
import type { Supply } from '@/entities/stock/models/supply';
import { useStocktakingRepository } from '@/entities/stock/respository/stocktaking';
import { useSupplyRepository } from '@/entities/stock/respository/supply';
import StocktakingSheet from '@/features/stock/ui/stocktaking/StocktakingSheet';
import Button from '@/shared/ui/Button';
import DateInput from '@/shared/ui/DateInput';

const StocktakingPage: Component = () => {
	
	const app = useApp();
	const supplyRepository = useSupplyRepository();
	const stocktakingRepository = useStocktakingRepository();

	app.setPageTitle('棚卸');

	const [stocktakingDate, setStocktakingDate] = createSignal(
		luxon.DateTime.now().toJSDate(),
	);

	const [stocktakingRecords, setStocktakingRecords] = createSignal<
		StocktakingRecord[]
	>([]);

	const add = async () => {

		await stocktakingRepository.add({
			stocktakingDate: stocktakingDate(),
			records: [...stocktakingRecords()],
		});

		app.toastInfo('登録しました。');
	}

	onMount(async () => {

		const supplies: Supply[] = await supplyRepository.list();

		const records: StocktakingRecord[] = supplies.map((supply) => {
			return {
				supplyId: supply.id,
				supplyName: supply.name,
				unitName: supply.unitName,
				unitPrice: 0,
				quantity: 0,
				totalPrice: 0,
			} as StocktakingRecord;
		});

		setStocktakingRecords(records);
	});

	return (
		<article class="size-full flex flex-col gap-10 p-10">
			<section class="flex justify-between">
				<DateInput
					label="棚卸日"
					value={stocktakingDate()}
					onChange={setStocktakingDate}
				/>
				<Button onClick={add}>
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
