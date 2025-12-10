import { PencilLineIcon } from 'lucide-solid';
import type { Component } from 'solid-js';
import type { SuppliesAccordionValue } from '@/features/stock/models/supplies-accordion-value';

export interface SupplierListItemProps {
	value: SuppliesAccordionValue;
    onSelect?: (value: SuppliesAccordionValue['supplies'][number]) => Promise<void> | void;
}

const SuppliesAccordion: Component<SupplierListItemProps> = (props) => {
	return (
		<details
			class="collapse collapse-arrow bg-base-200 join-item"
			name={`supplier-${props.value.supplierId}`}
			open
		>
			<summary class="collapse-title">{props.value.supplierName}</summary>

            {
                props.value.supplies.length > 0
                    ? (

                        <table class="table table-fixed">
                            <thead>
                                <tr>
                                    <th class="w-20"></th>
                                    <th>仕入品名</th>
                                    <th>単位</th>
                                </tr>
                            </thead>
                            {props.value.supplies.map((supply) => (
                                <tbody>
                                    <tr>
                                        <td>
                                            <button type="button" class="btn btn-ghost" onclick={() => props.onSelect?.(supply)}>
                                                <PencilLineIcon class='size-4'/>
                                            </button>
                                        </td>
                                        <td>{supply.supplyName}</td>
                                        <td>{supply.unitName}</td>
                                    </tr>
                                </tbody>
                            ))}
                        </table>
                    )
                    : (
                        <p class='p-5 text-gray-400'>仕入品はありません</p>
                    )
            }
		</details>
	);
};

export default SuppliesAccordion;
