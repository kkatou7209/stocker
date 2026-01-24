import {
	type Component,
	createEffect,
	createSignal,
	For,
	on,
	Show,
} from 'solid-js';
import type { StocktakingRecord } from '@/entities/stock/models/stocktaking';
import { useFormat } from '@/shared/lib/format';
import NumberInput from '@/shared/ui/NumberInput';

const StocktakingSheet: Component<{
	value?: StocktakingRecord[];
	onChange?: (value: StocktakingRecord[]) => unknown;
}> = (props) => {

    const formatter = useFormat('ja-JP');

    const [records, setRecords] = createSignal<StocktakingRecord[]>([]);

    const [totalPrice, setTotalPrice] = createSignal(0);
	
	const onChange = (value: StocktakingRecord) => {

        const index = records().findIndex(r => r.supplyId === value.supplyId);

        if (index >= 0) {
			const rec = records();
            rec[index] = value;
			setRecords(rec);
            props.onChange?.(records());
        }

        const total = records().reduce((price, record) => price + record.quantity * record.unitPrice, 0);

        setTotalPrice(total);
    };

    createEffect(() => {
        setRecords(props.value ?? []);
    });

    return (
		<table class="table text-nowrap table-pin-rows table-fixed pr-3">
			<thead>
				<tr>
					<th>仕入品名</th>
					<th class='w-[5%]'>単位</th>
					<th class='w-[20%] text-end'>単価</th>
					<th class='w-[20%] text-end'>数量</th>
					<th class='w-[20%] text-end'>金額</th>
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
			<Show
				when={records().length > 0}
				fallback={''}
			>
				<tfoot>
					<tr class='bg-base-300 rounded-none'>
						<td colspan={4}>合計</td>
						<td class='text-end'>
							{formatter.number.format(totalPrice())} 円
						</td>
					</tr>
				</tfoot>
			</Show>
		</table>
	);
};

const StocktakingRecordInput: Component<{
	value: StocktakingRecord;
	onChange: (value: StocktakingRecord) => unknown;
}> = (props) => {
	const formatter = useFormat('ja-JP');

	const [unitPrice, setUnitPrice] = createSignal(0);

	const [quantity, setQuantity] = createSignal(0);

	const [totalPrice, setTotalPrice] = createSignal(0);

	createEffect(
		on([unitPrice, quantity], () => {

			setTotalPrice(Math.round(unitPrice() * quantity()));

			const record: StocktakingRecord = {
				supplyId: props.value.supplyId,
				supplyName: props.value.supplyName,
				unitName: props.value.unitName,
				unitPrice: unitPrice(),
				quantity: quantity(),
			};

			props.onChange?.(record);
		})
	)

	createEffect(
		on(() => [props.value.unitPrice, props.value.quantity], () => {
			setUnitPrice(props.value.unitPrice);
			setQuantity(props.value.quantity);
		}),
	)

	return (
		<tr>
			<td>{props.value.supplyName}</td>
			<td>{props.value.unitName}</td>
			<td>
				<NumberInput value={unitPrice()} onChange={setUnitPrice} suffix='円'/>
			</td>
			<td>
				<NumberInput value={quantity()} onChange={setQuantity} suffix={props.value.unitName}/>
			</td>
			<td class="text-end">{formatter.number.format(totalPrice())} 円</td>
		</tr>
	);
};

export default StocktakingSheet;
