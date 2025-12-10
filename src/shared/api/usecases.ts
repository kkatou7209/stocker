import z from 'zod';
import type { Supplier } from '@/entities/stock/models/supplier';
import type { Supply } from '@/entities/stock/models/supply';
import { SupplierId } from '@/entities/stock/values/supplier-id';
import { SupplierName } from '@/entities/stock/values/supplier-name';
import { SupplyId } from '@/entities/stock/values/supply-id';
import { SupplyName } from '@/entities/stock/values/supply-name';
import { UnitName } from '@/entities/stock/values/unit-name';

export interface AddSupplierCommand {
	supplierName: SupplierName;
}

export type AddSupplierUsecase = (command: AddSupplierCommand) => Promise<void>;

export type ListSuppliersUsecase = () => Promise<Supplier[]>;

export type ListSupplierSupplies = (
	supplierId: SupplierId,
) => Promise<Supply[]>;

export type GetSupplierUsecase = (
	supplierId: SupplierId,
) => Promise<Supplier | null>;

export const UpdateSupplierCommand = z.object({
	supplierId: z.custom<SupplierId>((v) => v instanceof SupplierId),
	supplierName: z.custom<SupplierName>((v) => v instanceof SupplierName),
});

export type UpdateSupplierCommand = z.infer<typeof UpdateSupplierCommand>;

export type UpdateSupplierUsecase = (
	command: UpdateSupplierCommand,
) => Promise<void>;

export interface AddSupplyCommand {
	name: SupplyName;
	unitName: UnitName;
	supplierId: SupplierId;
}

export type AddSupplyUsecase = (supply: AddSupplyCommand) => Promise<void>;

export type ListSuppliesUsecase = () => Promise<Supply[]>;

export type GetSupplyUsecase = (supplyId: SupplyId) => Promise<Supply | null>;

export const UpdateSupplyCommand = z.object({
	supplyId: z.custom<SupplyId>((value) => value instanceof SupplyId),
	supplyName: z.custom<SupplyName>((value) => value instanceof SupplyName),
	unitName: z.custom<UnitName>((value) => value instanceof UnitName),
	supplierId: z.custom<SupplierId>((value) => value instanceof SupplierId),
});

export type UpdateSupplyCommand = z.infer<typeof UpdateSupplyCommand>;

export type UpdateSupplyUsecase = (
	supply: UpdateSupplyCommand,
) => Promise<void>;

export const SearchSuppliesQuery = z.object({
	supplyIds: z.array(z.string().nonempty().nonoptional()),
	supplierIds: z.array(z.string().nonempty().nonoptional()),
	supplyName: z.preprocess(
		(v) => (String(v).trim() === '' ? null : String(v).trim()),
		z.string().nullable(),
	),
	supplierName: z.preprocess(
		(v) => (String(v).trim() === '' ? null : String(v).trim()),
		z.string().nullable(),
	),
});

export type SearchSuppliesQuery = z.infer<typeof SearchSuppliesQuery>;

export type SearchSuppliesUsecase = (
	query: SearchSuppliesQuery,
) => Promise<Supply[]>;
