import { type Component, createSignal, For, onMount } from 'solid-js';
import type {
	StocktakingRecord,
} from '@/entities/stock/models/stocktaking';
import { useFormat } from '@/shared/lib/format';

const StocktakingTable: Component<{
	value: StocktakingRecord[];
}> = (props) => {
	const formatter = useFormat('ja-JP');

    const [totalPrice, setTotalPrice] = createSignal(0);

    onMount(() => {

        const total = props.value.reduce((price, record) => price + (record.unitPrice * record.quantity), 0);

        setTotalPrice(Math.round(total));
    });

	return (
		<table class="table">
			<thead>
				<tr>
					<th>仕入品名</th>
					<th>単位</th>
					<th class='text-end'>単価</th>
					<th class='text-end'>数量</th>
					<th class='text-end'>金額</th>
				</tr>
			</thead>
			<tbody>
				<For each={props.value}>
					{(record) => (
						<tr>
							<td>{record.supplyName}</td>
							<td>{record.unitName}</td>
							<td class='text-end'>
								{formatter.number.format(record.unitPrice)} 円
							</td>
							<td class='text-end'>
								{formatter.number.format(record.quantity)}{' '}
								{record.unitName}
							</td>
							<td class='text-end'>
								{formatter.number.format(record.unitPrice * record.quantity)} 円
							</td>
						</tr>
					)}
				</For>
                <tr>
                    <td colspan={4}>合計</td>
                    <td class='text-end'>{formatter.number.format(totalPrice())} 円</td>
                </tr>
			</tbody>
		</table>
	);
};

export default StocktakingTable;