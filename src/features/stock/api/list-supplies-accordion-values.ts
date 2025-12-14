import type { SuppliesAccordionValue } from '@/features/stock/models/supplies-accordion-value';
import { useApi } from '@/shared/api';

export const listSuppliesAccordionValues = async () => {
	const api = useApi();

	const suppliers = await api.supplier.listAllSuppliers();

	const supplierSuppliesItemValues: SuppliesAccordionValue[] = suppliers.map(
		(supplier) => {
			const value: SuppliesAccordionValue = {
				supplierId: supplier.id,
				supplierName: supplier.name,
				supplies: supplier.supplies.map((supply) => ({
					supplyId: supply.id,
					supplyName: supply.name,
					unitName: supply.unitName,
				})),
			};

			return value;
		},
	);

	return supplierSuppliesItemValues;
};
