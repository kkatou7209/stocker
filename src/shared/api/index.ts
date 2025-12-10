import type {
	AddSupplierUsecase,
	AddSupplyUsecase,
	GetSupplierUsecase,
	GetSupplyUsecase,
	ListSupplierSupplies,
	ListSuppliersUsecase,
	ListSuppliesUsecase,
	UpdateSupplierUsecase,
	UpdateSupplyUsecase,
} from '@/shared/api/usecases';
import {
	mockAddSupplierUsecase,
	mockAddSupplyUsecase,
	mockGetSupplierUsecase,
	mockGetSupplyUsecase,
	mockListSupplierSupplies,
	mockListSuppliersUsecase,
	mockListSuppliesUsecase,
	mockUpdateSupplierUsecase,
	mockUpdateSupplyUsecase,
} from '@/shared/mock/api';

export interface StockUsecases {
	readonly addSupplier: AddSupplierUsecase;
	readonly listSuppliers: ListSuppliersUsecase;
	readonly getSupplier: GetSupplierUsecase;
	readonly updateSupplier: UpdateSupplierUsecase;
	readonly listSupplierSupplies: ListSupplierSupplies;
	readonly addSupply: AddSupplyUsecase;
	readonly listSupplies: ListSuppliesUsecase;
	readonly getSupply: GetSupplyUsecase;
	readonly updateSupply: UpdateSupplyUsecase;
}

export const useApi = (): StockUsecases => {
	return {
		addSupplier: mockAddSupplierUsecase,
		listSuppliers: mockListSuppliersUsecase,
		getSupplier: mockGetSupplierUsecase,
		updateSupplier: mockUpdateSupplierUsecase,
		addSupply: mockAddSupplyUsecase,
		listSupplies: mockListSuppliesUsecase,
		listSupplierSupplies: mockListSupplierSupplies,
		getSupply: mockGetSupplyUsecase,
		updateSupply: mockUpdateSupplyUsecase,
	};
};
