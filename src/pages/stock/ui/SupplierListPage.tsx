import { SearchIcon } from 'lucide-solid';
import { type Component, createSignal, onMount } from 'solid-js';
import { useApp } from '@/app/contexts/AppContext';
import type { Supplier } from '@/entities/stock/models/supplier';
import { SupplierId } from '@/entities/stock/values/supplier-id';
import { SupplierName } from '@/entities/stock/values/supplier-name';
import type { SupplierTableRecord } from '@/features/stock/models/supplier-table-record';
import SupplierInput, {
	type SupplierInputValue,
} from '@/features/stock/ui/SupplierInput';
import SupplierTable from '@/features/stock/ui/SupplierTable';
import { useApi } from '@/shared/api';
import type {
	AddSupplierCommand,
	UpdateSupplierCommand,
} from '@/shared/api/usecases';
import Button from '@/shared/ui/Button';
import TextInput from '@/shared/ui/TextInput';

const SupplierListPage: Component = () => {
	const app = useApp();
	const api = useApi();

	app.setPageTitle('仕入先');

	const [records, setRecords] = createSignal<SupplierTableRecord[]>([]);

	const [supplier, setSupplier] = createSignal<Supplier | null>(null);

	const [supplierInput, setSupplierInput] =
		createSignal<SupplierInputValue | null>(null);

	const [addDialogOpen, setAddDialogOpen] = createSignal(false);

	const [editDialogOpen, setEditDialogOpen] = createSignal(false);

	const getRecords = async (): Promise<SupplierTableRecord[]> => {
		const suppliers = await api.listSuppliers();

		const records: SupplierTableRecord[] = [];

		for (const supplier of suppliers) {
			records.push({
				id: supplier.id().value(),
				name: supplier.name().value(),
			});
		}

		return records;
	};

	const select = async (record: SupplierTableRecord) => {
		const supp = await api.getSupplier(SupplierId.of(record.id));

		if (!supp) {
			setSupplier(null);
			setSupplierInput(null);
			return;
		}

		setSupplier(supp);

		setSupplierInput({
			supplierName: supp.name().value(),
		});

		setEditDialogOpen(true);
	};

	const add = async (input: SupplierInputValue) => {
		const command: AddSupplierCommand = {
			supplierName: SupplierName.of(input.supplierName),
		};

		await api.addSupplier(command);

		setAddDialogOpen(false);

		const records = await getRecords();

		setRecords(records);
	};

	const edit = async (input: SupplierInputValue) => {
		const id = supplier()?.id();

		if (!id) return;

		const command: UpdateSupplierCommand = {
			supplierId: id,
			supplierName: SupplierName.of(input.supplierName),
		};

		await api.updateSupplier(command);

		setEditDialogOpen(false);

		const records = await getRecords();

		setRecords(records);
	};

	onMount(async () => {
		const records = await getRecords();

		setRecords(records);
	});

	return (
		<article class="size-full">
			<div class="drawer-content pt-10 sm:px-3 lg:px-10">
				<div class="flex flex-col gap-10 transition-all duration-150">
					<section class="flex justify-between">
						<div class="flex items-center gap-5">
							<TextInput label="仕入先名" />
							<Button color="info">
								<SearchIcon />
							</Button>
						</div>
						<Button onClick={() => setAddDialogOpen(true)}>
							<span>追加</span>
						</Button>
					</section>
					<section>
						<SupplierTable value={records()} onSelect={select} />
					</section>
				</div>
			</div>

			<dialog class="modal" open={addDialogOpen()}>
				<div class="modal-box w-100">
					<SupplierInput onAction={add} />
				</div>
				<button
					type="button"
					class="modal-backdrop"
					onclick={() => setAddDialogOpen(false)}
				></button>
			</dialog>

			<dialog class="modal" open={editDialogOpen()}>
				<div class="modal-box w-100">
					<SupplierInput
                        value={supplierInput()}
                        onAction={edit}
                    />
				</div>
				<button
					type="button"
					class="modal-backdrop"
					onclick={() => setEditDialogOpen(false)}
				></button>
			</dialog>
		</article>
	);
};

export default SupplierListPage;
