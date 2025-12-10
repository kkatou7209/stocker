import { Supplier } from '@/entities/stock/models/supplier';
import { Supply } from '@/entities/stock/models/supply';
import { SupplierId } from '@/entities/stock/values/supplier-id';
import { SupplierName } from '@/entities/stock/values/supplier-name';
import { SupplyId } from '@/entities/stock/values/supply-id';
import { SupplyName } from '@/entities/stock/values/supply-name';
import { UnitName } from '@/entities/stock/values/unit-name';

export const fakeSupplies: Supply[] = [
	Supply.restore(
		SupplyId.of('001'),
		SupplyName.of('小麦A'),
		UnitName.of('g'),
		SupplierId.of('001'),
	),
	Supply.restore(
		SupplyId.of('002'),
		SupplyName.of('小麦B'),
		UnitName.of('g'),
		SupplierId.of('001'),
	),
	Supply.restore(
		SupplyId.of('003'),
		SupplyName.of('小麦C'),
		UnitName.of('g'),
		SupplierId.of('001'),
	),
	Supply.restore(
		SupplyId.of('004'),
		SupplyName.of('小麦D'),
		UnitName.of('g'),
		SupplierId.of('002'),
	),
	Supply.restore(
		SupplyId.of('005'),
		SupplyName.of('小麦E'),
		UnitName.of('g'),
		SupplierId.of('002'),
	),
	Supply.restore(
		SupplyId.of('006'),
		SupplyName.of('小麦F'),
		UnitName.of('g'),
		SupplierId.of('002'),
	),
];

export const fakeSuppliers: Supplier[] = [
	Supplier.restore(SupplierId.of('001'), SupplierName.of('仕入先A')),
	Supplier.restore(SupplierId.of('002'), SupplierName.of('仕入先B')),
];
