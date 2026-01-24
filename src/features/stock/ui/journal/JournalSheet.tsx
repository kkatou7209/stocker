import { type Component, createEffect, createSignal, For, onMount, Show } from 'solid-js';
import type { JournalRecord } from '@/entities/stock/models/journal';
import { useFormat } from '@/shared/lib/format';
import NumberInput from '@/shared/ui/NumberInput';

interface RecordsPerSupplier {
	id: string;
	name: string;
	records: JournalRecord[];
};

const JournalSheet: Component<{
	value?: JournalRecord[];
	onChange?: (value: JournalRecord[]) => unknown;
}> = (props) => {

	const formatter = useFormat('ja-JP');

	const [records, setRecords] = createSignal<JournalRecord[]>([]);

	const [suppliers, setSuppliers] = createSignal<RecordsPerSupplier[]>([]);

	const [totalPrice, setTotalPrice] = createSignal(0);

	const onChange = (value: JournalRecord) => {

		const index =  records().findIndex(r => 
			r.supplierId === value.supplierId && r.supplyId === value.supplyId);

		if (index >= 0) {
			records()[index] = value;
			props.onChange?.(records());
		}

		const total = records().reduce((price, record) => price + (record.unitPrice * record.quantity), 0);

		setTotalPrice(Math.round(total));
	}

	createEffect(() => {

		setRecords(props.value ?? []);

		const supps: RecordsPerSupplier[] = [];

		for (const record of records()) {
			if (!supps.some(s => s.id === record.supplierId)) {
				supps.push({
					id: record.supplierId,
					name: record.supplierName,
					records: [],
				});
			}

			supps.find(s => s.id === record.supplierId)?.records.push(record)
		}

		setSuppliers(supps);
	});

	return (
		<table class="table text-nowrap table-pin-rows table-fixed pr-3">
			<thead>
				<tr>
					<th class='w-[5%]'></th>
					<th class='w-[30%]'>仕入品</th>
					<th class='w-[5%]'>単位</th>
					<th class='w-[10%] text-end'>単価</th>
					<th class='w-[10%] text-end'>数量</th>
					<th class='w-[10%] text-end'>金額</th>
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
							<tr class='bg-base-200'>
								<td colspan={6}>{supp.name}</td>
							</tr>
							<For
								each={supp.records}
								fallback={''}
							>
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
			<Show
				when={records().length > 0}
				fallback={''}
			>
				<tfoot>
					<tr class='rounded-none bg-base-300'>
						<td colSpan={5}>合計</td>
						<td class='text-end'>{formatter.number.format(totalPrice())} 円</td>
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
	const formatter = useFormat('ja-JP');

	const [unitPrice, setUnitPrice] = createSignal(0);

	const [quantity, setQuantity] = createSignal(0);

	const [totalPrice, setTotalPrice] = createSignal(0);

	createEffect(() => {
		setTotalPrice(Math.round(unitPrice() * quantity()));
		props.onChange({
			supplyName: props.value.supplyName,
			supplierName: props.value.supplierName,
			unitName: props.value.unitName,
			supplierId: props.value.supplierId,
			supplyId: props.value.supplyId,
			unitPrice: unitPrice(),
			quantity: quantity(),
		});
	});

	onMount(() => {

		const record = props.value;

		setUnitPrice(record.unitPrice);
		setQuantity(record.quantity);

		setTotalPrice(Math.round(unitPrice() * quantity()));
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
					suffix='円'
				/>
			</td>
			<td>
				<NumberInput 
					value={quantity()} 
					onChange={setQuantity} 
					suffix={props.value.unitName}
				/>
			</td>
			<td class="text-end">{formatter.number.format(totalPrice())} 円</td>
		</tr>
	);
};

export default JournalSheet;
