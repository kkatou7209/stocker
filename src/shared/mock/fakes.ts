import * as luxon from 'luxon';
import type { JournalData } from '@/shared/api/endpoints/jurnal';
import type { StocktakingData } from '@/shared/api/endpoints/stocktaking';
import type { SupplierData } from '@/shared/api/endpoints/supplier';

let _nextSupplierId = 4;
let _nextSupplyId = 12;
let _nextJournalId = 3;
let _nextStocktakinglId = 2;

export const nextSupplierId = (): string =>
	(_nextSupplierId++).toString().padStart(3, '0');

export const nextSupplyId = (): string =>
	(_nextSupplyId++).toString().padStart(3, '0');

export const nextJournalId = (): string =>
	(_nextJournalId++).toString().padStart(3, '0');

export const nextStocktakinglId = (): string =>
	(_nextStocktakinglId++).toString().padStart(3, '0');

export const fakeSuppliers: SupplierData[] = [
	{
		id: '001',
		name: '仕入先A',
		supplies: [
			{
				id: '001',
				name: '仕入品A',
				unitName: 'g',
				supplierId: '001',
			},
			{
				id: '002',
				name: '仕入品B',
				unitName: 'g',
				supplierId: '001',
			},
			{
				id: '006',
				name: '仕入品F',
				unitName: 'g',
				supplierId: '001',
			},
		],
	},
	{
		id: '002',
		name: '仕入先B',
		supplies: [
			{
				id: '003',
				name: '仕入品C',
				unitName: 'g',
				supplierId: '002',
			},
			{
				id: '007',
				name: '仕入品G',
				unitName: 'g',
				supplierId: '002',
			},
			{
				id: '010',
				name: '仕入品J',
				unitName: 'kg',
				supplierId: '002',
			},
			{
				id: '011',
				name: '仕入品K',
				unitName: 'kg',
				supplierId: '002',
			},
		],
	},
	{
		id: '003',
		name: '仕入先C',
		supplies: [
			{
				id: '004',
				name: '仕入品D',
				unitName: 'kg',
				supplierId: '003',
			},
			{
				id: '005',
				name: '仕入品E',
				unitName: 'g',
				supplierId: '003',
			},
			{
				id: '008',
				name: '仕入品H',
				unitName: 'g',
				supplierId: '003',
			},
			{
				id: '009',
				name: '仕入品I',
				unitName: 'g',
				supplierId: '003',
			},
		],
	},
];

export const fakeJournals: JournalData[] = [
	{
		id: '001',
		entryDate: luxon.DateTime.utc(2025, 12, 1).toJSDate().getTime(),
		records: [
			{
				supplierId: '001',
				supplierName: '仕入先A',
				supplyId: '001',
				supplyName: '仕入品A',
				unitName: 'g',
				unitPrice: 100,
				quantity: 10,
			},
			{
				supplierId: '001',
				supplierName: '仕入先A',
				supplyId: '002',
				supplyName: '仕入品B',
				unitName: 'g',
				unitPrice: 100,
				quantity: 10,
			},
		],
	},
	{
		id: '002',
		entryDate: luxon.DateTime.utc(2025, 12, 2).toJSDate().getTime(),
		records: [
			{
				supplierId: '001',
				supplierName: '仕入先A',
				supplyId: '001',
				supplyName: '仕入品A',
				unitName: 'g',
				unitPrice: 100,
				quantity: 10,
			},
			{
				supplierId: '001',
				supplierName: '仕入先A',
				supplyId: '002',
				supplyName: '仕入品B',
				unitName: 'g',
				unitPrice: 100,
				quantity: 10,
			},
		],
	},
];

export const fakeStocktakings: StocktakingData[] = [
	{
		id: '001',
		stocktakingDate: luxon.DateTime.utc(2025, 1, 23).millisecond,
		records: [
			{
				supplyId: '001',
				supplyName: '仕入品A',
				unitName: 'g',
				unitPrice: 100,
				quantity: 20,
			},
			{
				supplyId: '002',
				supplyName: '仕入品B',
				unitName: 'g',
				unitPrice: 120,
				quantity: 10,
			},
			{
				supplyId: '003',
				supplyName: '仕入品C',
				unitName: 'g',
				unitPrice: 150,
				quantity: 15,
			},
		],
	},
];
