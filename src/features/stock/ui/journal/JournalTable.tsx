import { type Component, createEffect, createSignal, For } from 'solid-js';
import type { JournalRecord } from '@/entities/stock/models/journal';
import { useFormat } from '@/shared/lib/format';

const JournalTable: Component<{ value?: JournalRecord[] }> = (props) => {
	const formatter = useFormat('ja-JP');

	const [totalPrice, setTotalPrice] = createSignal(0);

	const records = () => props.value ?? [];

	createEffect(() => {
		const price = records().reduce((price, record) => price + (record.unitPrice * record.quantity), 0);

		setTotalPrice(price);
	})

	return (
		<table class="table text-nowrap table-pin-rows">
			<thead>
				<tr>
					<th>仕入品名</th>
					<th>単位</th>
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
							<td colspan={5} class="text-gray-400">
								データがありません
							</td>
						</tr>
					}
				>
					{(record) => (
						<tr>
							<td>{record.supplyName}</td>
							<td>{record.unitName}</td>
							<td class="text-end">
								{formatter.number.format(record.unitPrice)} 円
							</td>
							<td class="text-end">
								{formatter.number.format(record.quantity)}{' '}
								{record.unitName}
							</td>
							<td class="text-end">
								{formatter.number.format(record.unitPrice * record.quantity)} 円
							</td>
						</tr>
					)}
				</For>
			</tbody>
			<tfoot>
				<tr>
					<td colspan={4}>
						合計
					</td>
					<td class='text-end'>
						{formatter.number.format(totalPrice())} 円
					</td>
				</tr>
			</tfoot>
		</table>
	);
};

export default JournalTable;
