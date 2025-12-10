import { PencilLineIcon } from 'lucide-solid';
import type { Component } from 'solid-js';
import type { SupplierTableRecord } from '@/features/stock/models/supplier-table-record';

export interface SupplierTableProps {
	value?: SupplierTableRecord[];
	onSelect?: (record: SupplierTableRecord) => Promise<void> | void;
}

const SupplierTable: Component<SupplierTableProps> = (props) => {
	const select = (record: SupplierTableRecord) => {
		props.onSelect?.(record);
	};

	return (
		<table class="table">
			<thead class="sticky">
				<tr class="table block table-fixed">
					<th class='w-20'></th>
					<th>仕入先名</th>
				</tr>
			</thead>
			<tbody class="block overflow-auto">
				{props.value?.map((record) => (
					<tr class="table table-fixed">
						<td class='w-20'>
							<button
								type="button"
								class='btn btn-ghost'
								onclick={() => select(record)}
							>
								<PencilLineIcon class='size-4'/>
							</button>
						</td>
						<td>{record.name}</td>
					</tr>
				))}
			</tbody>
		</table>
	);
};

export default SupplierTable;
