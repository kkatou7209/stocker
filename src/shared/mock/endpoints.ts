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

	registerSupplier: async (command: AddSupplierCommand) => {
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
	deleteSupplier: async (id: string) => {
		const index = fakeSuppliers.findIndex((s) => s.id === id);

		if (index < 0) return;

		fakeSuppliers.splice(index, 1);
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
	deleteSupply: async (id: string) => {
		for (const supplier of fakeSuppliers) {
			const index = supplier.supplies.findIndex((s) => s.id === id);

			if (index < 0) continue;

			supplier.supplies.splice(index, 1);
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

	recordJournal: async (command) => {
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
			entryDate: fakeJournals[index].entryDate,
			records: [...command.records],
		};
	},

	searchJournals: async (query) => {
		let journals = [...fakeJournals];

		if (query.periodEnd) {
			journals = journals.filter(
				(f) => f.entryDate <= (query.periodEnd as number),
			);
		}

		if (query.periodStart) {
			journals = journals.filter(
				(f) => f.entryDate >= (query.periodStart as number),
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
	deleteJournal: async (id: string) => {
		const index = fakeJournals.findIndex((j) => j.id === id);

		if (index < 0) return;

		fakeJournals.splice(index, 1);
	},
};

export const mockStocktakingEndpoints: StocktakingEndpoints = {
	listAllStocktakings: async () => [...fakeStocktakings],

	getStocktakingById: async (id: string) =>
		fakeStocktakings.find((s) => s.id === id) ?? null,

	recordStocktaking: async (command) => {
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
			stocktakingDate: fakeStocktakings[index].stocktakingDate,
			records: [...command.records],
		};
	},

	searchStocktakings: async (query) => {
		let stocktakings = [...fakeStocktakings];

		if (query.periodStart) {
			const periodStart = query.periodStart;

			stocktakings = stocktakings.filter(
				(s) => periodStart <= s.stocktakingDate,
			);
		}

		if (query.periodEnd) {
			const periodEnd = query.periodEnd;

			stocktakings = stocktakings.filter(
				(s) => s.stocktakingDate <= periodEnd,
			);
		}

		return stocktakings;
	},
	getStocktakingAt: async (date: number): Promise<StocktakingData | null> => {
		const d = new Date(date);

		const start = Date.UTC(
			d.getFullYear(),
			d.getMonth(),
			d.getDate(),
			0,
			0,
			0,
			0,
		);

		const end = Date.UTC(
			d.getFullYear(),
			d.getMonth(),
			d.getDate(),
			23,
			59,
			59,
			999,
		);

		return (
			fakeStocktakings.find((stocktaking) => {
				return (
					start <= stocktaking.stocktakingDate &&
					stocktaking.stocktakingDate <= end
				);
			}) ?? null
		);
	},

	deleteStocktaking: async (id: string) => {
		const index = fakeStocktakings.findIndex((s) => s.id === id);

		if (index < 0) return;

		fakeStocktakings.splice(index, 1);
	},
	downloadStocktakingCsv: async (id: string): Promise<void> => {
		// no-op in mock
	},
};
