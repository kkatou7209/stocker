import type {
	JournalData,
	JournalEndpoints,
} from '@/shared/api/endpoints/jurnal';
import type {
	StocktakingData,
	StocktakingEndpoints,
} from '@/shared/api/endpoints/stocktaking';
import {
	AddSupplierCommand,
	type SupplierEndpoint,
	SupplierQuery,
	UpdateSupplierCommand,
} from '@/shared/api/endpoints/supplier';
import {
	AddSupplyCommand,
	type SupplyEndpoint,
	UpdateSupplyCommand,
} from '@/shared/api/endpoints/supply';
import {
	fakeJournals,
	fakeStocktakings,
	fakeSuppliers,
	nextJournalId,
	nextStocktakinglId,
	nextSupplierId,
	nextSupplyId,
} from '@/shared/mock/fakes';

export const mockSupplierEndpoints: SupplierEndpoint = {
	listAllSuppliers: async () => [...fakeSuppliers],

	getSupplierById: async (id: string) => {
		return fakeSuppliers.find((s) => s.id === id) ?? null;
	},

	createSupplier: async (command: AddSupplierCommand) => {
		const data = AddSupplierCommand.parse(command);

		fakeSuppliers.push({
			id: nextSupplierId(),
			name: data.supplierName,
			supplies: [],
		});
	},

	updateSupplier: async (command: UpdateSupplierCommand) => {
		const data = UpdateSupplierCommand.parse(command);

		const index = fakeSuppliers.findIndex((s) => s.id === data.supplierId);

		fakeSuppliers[index].name = data.supplierName;
	},

	searchSuppliers: async (query: SupplierQuery) => {
		const q = SupplierQuery.parse(query);

		let suppliers = [...fakeSuppliers];

		if (q.supplierName) {
			suppliers = suppliers.filter((supplier) =>
				supplier.name.includes(q.supplierName as string),
			);
		}

		if (q.supplyName) {
			suppliers = suppliers
				.map((supplier) => {
					const supplies = supplier.supplies.filter((supply) =>
						supply.name.includes(q.supplyName as string),
					);

					return {
						id: supplier.id,
						name: supplier.name,
						supplies,
					};
				})
				.filter((s) => s.supplies.length > 0);
		}

		return suppliers;
	},
};

export const mockSupplyEndpoints: SupplyEndpoint = {
	listAllSupplies: async () => [...fakeSuppliers].flatMap((s) => s.supplies),

	getSupplyById: async (id: string) => {
		return (
			fakeSuppliers.flatMap((s) => s.supplies).find((s) => s.id === id) ??
			null
		);
	},

	registerSupply: async (command: AddSupplyCommand) => {
		const data = AddSupplyCommand.parse(command);

		for (const supplier of fakeSuppliers) {
			if (supplier.id !== data.supplierId) continue;

			supplier.supplies.push({
				id: nextSupplyId(),
				name: data.supplyName,
				unitName: data.unitName,
				supplierId: supplier.id,
			});
		}
	},

	updateSupply: async (command: UpdateSupplyCommand) => {
		const data = UpdateSupplyCommand.parse(command);

		for (const supply of fakeSuppliers.flatMap((s) => s.supplies)) {
			if (supply.id !== data.supplyId) continue;

			supply.name = data.supplyName;
			supply.unitName = data.unitName;
		}
	},
};

export const mockJournalEndpoints: JournalEndpoints = {
	listAllJournals: async () => [...fakeJournals],

	getJournalById: async (id: string) =>
		fakeJournals.find((j) => j.id === id) ?? null,

	getJournalAt: async (date) => {
		return (
			fakeJournals.find((j) => {
				const entryDate = new Date(j.entryDate);

				return (
					entryDate.getFullYear() === date.getFullYear() &&
					entryDate.getMonth() === date.getMonth() &&
					entryDate.getDate() === date.getDate()
				);
			}) ?? null
		);
	},

	createJournal: async (command) => {
		const journal: JournalData = {
			id: nextJournalId(),
			entryDate: command.entryDate,
			records: [...command.records],
		};

		fakeJournals.push(journal);

		return journal;
	},

	updateJournal: async (command) => {
		const index = fakeJournals.findIndex((f) => f.id === command.id);

		if (index < 0) return;

		fakeJournals[index] = {
			id: command.id,
			entryDate: command.entryDate,
			records: [...command.records],
		};
	},

	searchJournals: async (query) => {
		let journals = [...fakeJournals];

		if (query.periodEnd) {
			journals = journals.filter(
				(f) => f.entryDate <= (query.periodEnd?.getTime() as number),
			);
		}

		if (query.periodStart) {
			journals = journals.filter(
				(f) => f.entryDate >= (query.periodStart?.getTime() as number),
			);
		}

		if (query.supplierName) {
			journals = journals
				.filter((j) => {
					return {
						id: j.id,
						entryDate: j.entryDate,
						records: j.records.filter((r) =>
							r.supplierName.includes(
								query.supplierName as string,
							),
						),
					};
				})
				.filter((j) => j.records.length > 0);
		}

		if (query.supplyName) {
			journals = journals
				.filter((j) => {
					return {
						id: j.id,
						entryDate: j.entryDate,
						records: j.records.filter((r) =>
							r.supplyName.includes(query.supplyName as string),
						),
					};
				})
				.filter((j) => j.records.length > 0);
		}

		return journals;
	},
};

export const mockStocktakingEndpoints: StocktakingEndpoints = {
	listAllStocktakings: async () => [...fakeStocktakings],

	getStocktakingById: async (id: string) =>
		fakeStocktakings.find((s) => s.id === id) ?? null,

	createStocktaking: async (command) => {
		const id = nextStocktakinglId();

		const stocktaking: StocktakingData = {
			id,
			stocktakingDate: command.stocktakingDate,
			records: [...command.records],
		};

		fakeStocktakings.push(stocktaking);

		return stocktaking;
	},

	updateStocktaking: async (command) => {
		const index = fakeStocktakings.findIndex((s) => s.id === command.id);

		if (index > 0) return;

		fakeStocktakings[index] = {
			id: command.id,
			stocktakingDate: command.stocktakingDate,
			records: [...command.records],
		};
	},

	searchStocktakings: async (query) => {
		let stocktakings = [...fakeStocktakings];

		if (query.periodStart) {
			const periodStart = query.periodStart as Date;

			stocktakings = stocktakings.filter(
				(s) => periodStart.getTime() <= s.stocktakingDate,
			);
		}

		if (query.periodEnd) {
			const periodEnd = query.periodEnd as Date;

			stocktakings = stocktakings.filter(
				(s) => s.stocktakingDate <= periodEnd.getTime(),
			);
		}

		return stocktakings;
	},
};
