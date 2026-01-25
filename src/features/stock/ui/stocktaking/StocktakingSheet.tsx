import { CalculatorIcon } from 'lucide-solid';
import {
	type Component,
	createEffect,
	createSignal,
	For,
	on,
	onMount,
	Show,
} from 'solid-js';
import type { StocktakingRecord } from '@/entities/stock/models/stocktaking';
import NumberInput from '@/shared/ui/NumberInput';

/**
 * Stocktaking sheet component
 */
const StocktakingSheet: Component<{
	records?: StocktakingRecord[] | null;
	totalPrice?: number | null;
	onRecordsChange?: (stocktaking: StocktakingRecord[]) => unknown;
	onTotalPriceChange?: (totalPrice: number) => unknown;
}> = (props) => {
	// Records of stocktaking
	const [records, setRecords] = createSignal<StocktakingRecord[]>([]);

	// Total price of stocktaking
	const [totalPrice, setTotalPrice] = createSignal(0);

	// Recalculate total price
	const onRecalculate = () => {
		setTotalPrice(
			Math.round(
				records().reduce(
					(price, record) =>
						price + record.unitPrice * record.quantity,
					0,
				),
			),
		);
	};

	// Handle change of a record
	const onChange = (value: StocktakingRecord) => {
		const index = records().findIndex((r) => r.supplyId === value.supplyId);

		if (index >= 0) {
			const rec = records();
			rec[index] = value;
			setRecords(rec);
			props.onRecordsChange?.(records());
		}
	};

	createEffect(
		on([() => props.records, () => props.totalPrice], () => {
			if (props.records) {
				setRecords(props.records);
			}

			if (props.totalPrice) {
				setTotalPrice(props.totalPrice);
			}
		}),
	);

	createEffect(
		on([records, totalPrice], () => {
			props.onRecordsChange?.(records());
			props.onTotalPriceChange?.(totalPrice());
		}),
	);

	return (
		<table class="table text-nowrap table-pin-rows table-fixed pr-3">
			<thead>
				<tr>
					<th class="w-[30%]">仕入品名</th>
					<th class="w-[5%]">単位</th>
					<th class="text-end">単価</th>
					<th class="text-end">数量</th>
					<th class="text-end">金額</th>
				</tr>
			</thead>
			<tbody>
				<For
					each={records()}
					fallback={
						<tr>
							<td class="text-center" colspan={5}>
								登録されている仕入品がありません。
							</td>
						</tr>
					}
				>
					{(record) => (
						<StocktakingRecordInput
							value={record}
							onChange={onChange}
						/>
					)}
				</For>
			</tbody>
			<Show when={records().length > 0} fallback={''}>
				<tfoot>
					<tr class="bg-base-300 rounded-none">
						<td colspan={4}>合計</td>
						<td>
							<NumberInput
								value={totalPrice()}
								onChange={setTotalPrice}
								suffix="円"
								prefix={
									<button
										type="button"
										class="btn btn-ghost btn-sm"
										onClick={onRecalculate}
									>
										<CalculatorIcon class="size-4" />
									</button>
								}
							/>
						</td>
					</tr>
				</tfoot>
			</Show>
		</table>
	);
};

/**
 * Input row for stocktaking record
 */
const StocktakingRecordInput: Component<{
	value: StocktakingRecord;
	onChange: (value: StocktakingRecord) => unknown;
}> = (props) => {
	// The unit price of the supply
	const [unitPrice, setUnitPrice] = createSignal(0);

	// The quantity of the supply
	const [quantity, setQuantity] = createSignal(0);

	// The total price of the supply
	const [totalPrice, setTotalPrice] = createSignal(0);

	// Recalculate total price
	const onRecalculate = () => {
		setTotalPrice(Math.round(unitPrice() * quantity()));

		const record: StocktakingRecord = {
			...props.value,
			unitPrice: unitPrice(),
			quantity: quantity(),
			totalPrice: totalPrice(),
		};

		props.onChange(record);
	};

	createEffect(
		on([unitPrice, quantity, totalPrice], () => {

			if (totalPrice() <= 0) {
				setTotalPrice(Math.round(unitPrice() * quantity()));
			}

			const record: StocktakingRecord = {
				supplyId: props.value.supplyId,
				supplyName: props.value.supplyName,
				unitName: props.value.unitName,
				unitPrice: unitPrice(),
				quantity: quantity(),
				totalPrice: totalPrice(),
			};

			props.onChange?.(record);
		}),
	);

	onMount(() => {
		const record = props.value;

		console.log(record);

		setUnitPrice(record.unitPrice);
		setQuantity(record.quantity);
		setTotalPrice(record.totalPrice);
	});

	return (
		<tr>
			<td>{props.value.supplyName}</td>
			<td>{props.value.unitName}</td>
			<td>
				<NumberInput
					value={unitPrice()}
					onChange={setUnitPrice}
					suffix="円"
				/>
			</td>
			<td>
				<NumberInput
					value={quantity()}
					onChange={setQuantity}
					suffix={props.value.unitName}
				/>
			</td>
			<td>
				<NumberInput
					value={totalPrice()}
					onChange={setTotalPrice}
					suffix="円"
					prefix={
						<button
							type="button"
							class="btn btn-ghost btn-sm"
							onClick={onRecalculate}
						>
							<CalculatorIcon class="size-4" />
						</button>
					}
				/>
			</td>
		</tr>
	);
};

export default StocktakingSheet;
