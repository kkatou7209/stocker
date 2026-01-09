import { PencilLineIcon, Trash2Icon } from 'lucide-solid';
import { type Component, Show } from 'solid-js';
import type { SuppliesAccordionValue } from '@/features/stock/models/supplies-accordion-value';

export interface SupplierListItemProps {
	value: SuppliesAccordionValue;
	onSelect?: (
		value: SuppliesAccordionValue['supplies'][number],
	) => Promise<void> | void;
    onDelete?: (
        value: SuppliesAccordionValue['supplies'][number],
    ) => Promise<void> | void;
}

const SuppliesAccordion: Component<SupplierListItemProps> = (props) => {

	return (
		<details
			class="collapse collapse-arrow bg-base-200 join-item"
			name={`supplier-${props.value.supplierId}`}
		>
			<summary class="collapse-title">
				<section class='px-3 flex items-center justify-between'>
					{props.value.supplierName}
				</section>
			</summary>

			<Show
				when={props.value.supplies.length > 0}
				fallback={<p class="p-5 text-gray-400">仕入品はありません</p>}
			>
				<table class="table table-fixed">
					<thead>
						<tr>
							<th class="w-20"></th>
							<th>仕入品名</th>
							<th>単位</th>
							<th class="w-20"></th>
						</tr>
					</thead>
					{props.value.supplies.map((supply) => (
						<tbody>
							<tr>
								<td>
									<button
										type="button"
										class="btn btn-ghost"
										onclick={() => props.onSelect?.(supply)}
									>
										<PencilLineIcon class="size-4" />
									</button>
								</td>
								<td>{supply.supplyName}</td>
								<td>{supply.unitName}</td>
								<td>
									<button
										type="button"
										class="btn btn-error btn-ghost"
										onclick={() => props.onDelete?.(supply)}
									>
										<Trash2Icon class="size-4" />
									</button>
								</td>
							</tr>
						</tbody>
					))}
				</table>
			</Show>
		</details>
	);
};

export default SuppliesAccordion;
