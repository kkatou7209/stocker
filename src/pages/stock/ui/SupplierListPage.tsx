import { StoreIcon } from 'lucide-solid';
import { type Component, createSignal, onMount, Show } from 'solid-js';
import { useApp } from '@/app/contexts/AppContext';
import type { Supplier } from '@/entities/stock/models/supplier';
import { useSupplierRespository } from '@/entities/stock/respository/supplier';
import type { SupplierTableRecord } from '@/features/stock/models/supplier-table-record';
import SupplierInput, {
	type SupplierInputValue,
} from '@/features/stock/ui/supplier/SupplierInput';
import SupplierTable from '@/features/stock/ui/supplier/SupplierTable';
import Button from '@/shared/ui/Button';
import TextInput from '@/shared/ui/TextInput';

const SupplierListPage: Component = () => {
	
	const app = useApp();
	const supplierRepository = useSupplierRespository();

	app.setPageTitle('仕入先');

	const [searchSupplierName, setSearchSupplierName] =
		createSignal<string>('');

	const [records, setRecords] = createSignal<SupplierTableRecord[]>([]);

	const [supplier, setSupplier] = createSignal<Supplier | null>(null);

	const [supplierInput, setSupplierInput] =
		createSignal<SupplierInputValue | null>(null);

	const [addDialogOpen, setAddDialogOpen] = createSignal(false);

	const [editDialogOpen, setEditDialogOpen] = createSignal(false);

	const reload = async () => {
		const suppliers = await supplierRepository.find({
			supplierName: searchSupplierName(),
			supplyName: null,
		});

		const records: SupplierTableRecord[] = [];

		for (const supplier of suppliers) {
			records.push({
				id: supplier.id,
				name: supplier.name,
			});
		}

		setRecords(records);
	};

	const select = async (record: SupplierTableRecord) => {

		const supp = await supplierRepository.get(record.id);

		if (!supp) {
			setSupplier(null);
			setSupplierInput(null);
			return;
		}

		setSupplier(supp);

		setSupplierInput({
			supplierName: supp.name,
		});

		setEditDialogOpen(true);
	};

	const add = async (input: SupplierInputValue) => {

		await supplierRepository.add({
			name: input.supplierName,
		});

		app.toastInfo('登録しました。')

		await reload();
	};

	const edit = async (input: SupplierInputValue) => {

		const id = supplier()?.id;

		if (!id) return;

		await supplierRepository.edit({
			id,
			name: input.supplierName,
		});

		app.toastInfo('更新しました。');

		await reload();
	};

	onMount(async () => {
		await reload();
	});

	return (
		<article class="size-full flex flex-col gap-10 p-10">
			<section class="flex justify-between">
				<div class="flex items-center gap-5">
					<TextInput
						prefix={<StoreIcon class="size-4" />}
						label="仕入先名"
						value={searchSupplierName()}
						onInput={(v) => {
							setSearchSupplierName(v);
							reload();
						}}
					/>
				</div>
				<Button onClick={() => setAddDialogOpen(true)}>
					<span>追加</span>
				</Button>
			</section>
			<section>
				<Show
					when={records().length > 0}
					fallback={
						<div class="flex justify-center">
							<p class="text-gray-400">
								データがありません
							</p>
						</div>
					}
				>
					<SupplierTable
						value={records()}
						onSelect={select}
					/>
				</Show>
			</section>

			<dialog class="modal" open={addDialogOpen()}>
				<div class="modal-box w-100">
					<SupplierInput onAction={add} actionLabel="追加" />
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
						actionLabel="更新"
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
