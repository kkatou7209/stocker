import { Supplier } from '@/entities/stock/models/supplier';
import { Supply } from '@/entities/stock/models/supply';
import { SupplierId } from '@/entities/stock/values/supplier-id';
import { SupplierName } from '@/entities/stock/values/supplier-name';
import { SupplyId } from '@/entities/stock/values/supply-id';
import {
	type AddSupplierCommand,
	type AddSupplierUsecase,
	type AddSupplyUsecase,
	type GetSupplierUsecase,
	type GetSupplyUsecase,
	type ListSupplierSupplies,
	type ListSuppliersUsecase,
	type ListSuppliesUsecase,
	SearchSuppliesQuery,
	type SearchSuppliesUsecase,
	type UpdateSupplierCommand,
	type UpdateSupplierUsecase,
	type UpdateSupplyUsecase,
} from '@/shared/api/usecases';
import { fakeSuppliers, fakeSupplies } from '@/shared/mock/fakes';

export const mockAddSupplyUsecase: AddSupplyUsecase = async (
	supply,
): Promise<void> => {
	const id = (fakeSupplies.length + 1).toString().padStart(3, '0');

	const supp = Supply.restore(
		SupplyId.of(id),
		supply.name,
		supply.unitName,
		supply.supplierId,
	);

	fakeSupplies.push(supp);
};

export const mockListSuppliesUsecase: ListSuppliesUsecase = async () => {
	return [...fakeSupplies];
};

export const mockListSuppliersUsecase: ListSuppliersUsecase = async () => {
	return [...fakeSuppliers];
};

export const mockAddSupplierUsecase: AddSupplierUsecase = async (
	command: AddSupplierCommand,
) => {
	const id = (fakeSuppliers.length + 1).toString().padStart(3, '0');
	fakeSuppliers.push(
		Supplier.restore(SupplierId.of(id), command.supplierName),
	);
};

export const mockGetSupplierUsecase: GetSupplierUsecase = async (
	supplierId: SupplierId,
) => {
	return fakeSuppliers.find((s) => s.id().equals(supplierId)) ?? null;
};

export const mockUpdateSupplierUsecase: UpdateSupplierUsecase = async (
	command: UpdateSupplierCommand,
) => {
	const index = fakeSuppliers.findIndex((s) =>
		s.id().equals(command.supplierId),
	);

	if (index < 0) return;

	fakeSuppliers[index] = Supplier.restore(
		command.supplierId,
		command.supplierName,
	);
};

export const mockListSupplierSupplies: ListSupplierSupplies = async (
	supplierId: SupplierId,
): Promise<Supply[]> => {
	return fakeSupplies.filter((s) => s.supplierId().equals(supplierId));
};

export const mockGetSupplyUsecase: GetSupplyUsecase = async (
	supplyId: SupplyId,
) => {
	return fakeSupplies.find((s) => s.id().equals(supplyId)) ?? null;
};

export const mockUpdateSupplyUsecase: UpdateSupplyUsecase = async (supply) => {
	const index = fakeSupplies.findIndex((s) => s.id().equals(supply.supplyId));

	if (index < 0) return;

	fakeSupplies[index] = Supply.restore(
		supply.supplyId,
		supply.supplyName,
		supply.unitName,
		supply.supplierId,
	);
};

export const mockSearchSuppliesUsecase: SearchSuppliesUsecase = async (
	query,
) => {
	const q = SearchSuppliesQuery.parse(query);

	let supplies = [...fakeSupplies];
	let suppliers = [...fakeSuppliers];

	if (q.supplierIds) {
		suppliers = suppliers.filter((supplier) =>
			q.supplierIds.some((id) => id === supplier.id().value()),
		);
	}

	if (q.supplyIds) {
		supplies = supplies.filter((supply) =>
			q.supplyIds.some((id) => id === supply.id().value()),
		);
	}

	if (q.supplierName !== null) {
		suppliers = suppliers.filter((supplier) =>
			supplier
				.name()
				.value()
				.includes(q.supplierName as string),
		);
	}

	if (q.supplyName !== null) {
		supplies = supplies.filter((supply) =>
			supply
				.name()
				.value()
				.includes(q.supplyName as string),
		);
	}

	return supplies.filter((supply) =>
		suppliers.some((supplier) => supplier.id().equals(supply.supplierId())),
	);
};
