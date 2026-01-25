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
import type { JournalRecord } from '@/entities/stock/models/journal';
import { useFormat } from '@/shared/lib/format';
import NumberInput from '@/shared/ui/NumberInput';

interface RecordsPerSupplier {
	id: string;
	name: string;
	records: JournalRecord[];
}

/**
 * Journal sheet component
 */
const JournalSheet: Component<{
	records?: JournalRecord[];
	onRecordsChange?: (value: JournalRecord[]) => unknown;
	totalPrice?: number;
	onTotalPriceChange?: (value: number) => unknown;
}> = (props) => {
	// Journal records
	const [records, setRecords] = createSignal<JournalRecord[]>([]);

	// Records grouped by supplier
	const [suppliers, setSuppliers] = createSignal<RecordsPerSupplier[]>([]);

	// Total price
	const [totalPrice, setTotalPrice] = createSignal(0);

	const onRecalculate = () => {

		const total = records().reduce(
			(price, record) => price + record.unitPrice * record.quantity,
			0,
		);

		setTotalPrice(Math.round(total));

		props.onTotalPriceChange?.(totalPrice());
	}

	// Handle change of a record
	const onChange = (value: JournalRecord) => {
		const index = records().findIndex(
			(r) =>
				r.supplierId === value.supplierId &&
				r.supplyId === value.supplyId,
		);

		if (index >= 0) {
			records()[index] = value;
			props.onRecordsChange?.(records());
		}

		if (totalPrice() <= 0) {

			const total = records().reduce(
				(price, record) => price + record.unitPrice * record.quantity,
				0,
			);

			setTotalPrice(Math.round(total));

			props.onTotalPriceChange?.(totalPrice());
		}

	};

	createEffect(() => {
		setRecords(props.records ?? []);

		const supps: RecordsPerSupplier[] = [];

		for (const record of records()) {
			if (!supps.some((s) => s.id === record.supplierId)) {
				supps.push({
					id: record.supplierId,
					name: record.supplierName,
					records: [],
				});
			}

			supps.find((s) => s.id === record.supplierId)?.records.push(record);
		}

		setSuppliers(supps);
	});

	return (
		<table class="table text-nowrap table-pin-rows table-fixed pr-3">
			<thead>
				<tr>
					<th class="w-[5%]"></th>
					<th class="w-[30%]">仕入品</th>
					<th class="w-[5%]">単位</th>
					<th class="text-end">単価</th>
					<th class="text-end">数量</th>
					<th class="text-end">金額</th>
				</tr>
			</thead>
			<tbody>
				<For
					each={suppliers()}
					fallback={
						<tr>
							<td class="text-center" colspan={6}>
								登録されている仕入品がありません。
							</td>
						</tr>
					}
				>
					{(supp) => (
						<>
							<tr class="bg-base-200">
								<td colspan={6}>{supp.name}</td>
							</tr>
							<For each={supp.records} fallback={''}>
								{(record) => (
									<JournalRecordInput
										value={record}
										onChange={onChange}
									/>
								)}
							</For>
						</>
					)}
				</For>
			</tbody>
			<Show when={records().length > 0} fallback={''}>
				<tfoot>
					<tr class="rounded-none bg-base-300">
						<td colSpan={4}>合計</td>
						<td class='text-end' colSpan={2}>
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

const JournalRecordInput: Component<{
	value: JournalRecord;
	onChange: (value: JournalRecord) => unknown;
}> = (props) => {
	// The unit price of the supply
	const [unitPrice, setUnitPrice] = createSignal(0);

	// The quantity of the supply
	const [quantity, setQuantity] = createSignal(0);

	// The total price of the supply
	const [totalPrice, setTotalPrice] = createSignal(0);

	createEffect(
		on([unitPrice, quantity], () => {
			if (totalPrice() <= 0) {
				setTotalPrice(Math.round(unitPrice() * quantity()));
			}

			const record: JournalRecord = {
				...props.value,
				unitPrice: unitPrice(),
				quantity: quantity(),
				totalPrice: totalPrice(),
			};

			props.onChange(record);
		}),
	);

	const onRecalculate = () => {

		setTotalPrice(unitPrice() * quantity());

		const record: JournalRecord = {
			...props.value,
			unitPrice: unitPrice(),
			quantity: quantity(),
			totalPrice: totalPrice(),
		};

		props.onChange(record);
	}

	onMount(() => {
		const record = props.value;

		setUnitPrice(record.unitPrice);
		setQuantity(record.quantity);
		setTotalPrice(record.totalPrice);
	});

	return (
		<tr>
			<td></td>
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

export default JournalSheet;
