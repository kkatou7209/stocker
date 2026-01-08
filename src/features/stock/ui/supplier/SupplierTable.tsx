import { PencilLineIcon, Trash2Icon } from 'lucide-solid';
import type { Component } from 'solid-js';
import type { SupplierTableRecord } from '@/features/stock/models/supplier-table-record';

export interface SupplierTableProps {
	value?: SupplierTableRecord[];
	onSelect?: (record: SupplierTableRecord) => Promise<void> | void;
	onDelete?: (record: SupplierTableRecord) => Promise<void> | void;
}

const SupplierTable: Component<SupplierTableProps> = (props) => {

	return (
		<table class="table">
			<thead class="sticky top-0 left-0">
				<tr class="table block table-fixed">
					<th class='w-20'></th>
					<th>仕入先名</th>
					<th class='w-20'></th>
				</tr>
			</thead>
			<tbody class="block overflow-auto">
				{props.value?.map((record) => (
					<tr class="table table-fixed">
						<td class='w-20'>
							<button
								type="button"
								class='btn btn-ghost'
								onclick={() => props.onSelect?.(record)}
							>
								<PencilLineIcon class='size-4'/>
							</button>
						</td>
						<td>{record.name}</td>
						<td class='w-20'>
							<button
								type="button"
								class='btn btn-error btn-ghost'
								onclick={() => props.onDelete?.(record)}
							>
								<Trash2Icon class='size-4'/>
							</button>
						</td>
					</tr>
				))}
			</tbody>
		</table>
	);
};

export default SupplierTable;
