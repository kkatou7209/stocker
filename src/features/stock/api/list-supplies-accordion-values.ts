import type { Supplier } from '@/entities/stock/models/supplier';
import type { Supply } from '@/entities/stock/models/supply';
import type { SuppliesAccordionValue } from '@/features/stock/models/supplies-accordion-value';
import { useApi } from '@/shared/api';

export const listSuppliesAccordionValues = async () => {
	const api = useApi();

	const suppliers = await api.listSuppliers();

	const suppliesPromises: Promise<[Supplier, Supply[]]>[] = [];

	for (const supplier of suppliers) {
		suppliesPromises.push(
			api.listSupplierSupplies(supplier.id()).then((supplies) => {
				return [supplier, supplies];
			}),
		);
	}

	const suppliesList = await Promise.all(suppliesPromises);

	const supplierSuppliesItemValues: SuppliesAccordionValue[] =
		suppliesList.map((suppliesPerSupplier) => {
			const [supplier, supplies] = suppliesPerSupplier;

			return {
				supplierId: supplier.id().value(),
				supplierName: supplier.name().value(),
				supplies: supplies.map((supply) => ({
					supplyId: supply.id().value(),
					supplyName: supply.name().value(),
					unitName: supply.unitName().value(),
				})),
			};
		});

	return supplierSuppliesItemValues;
};
