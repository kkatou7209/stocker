import { StoreIcon, XIcon } from 'lucide-solid';
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
import { Confirm } from '@/shared/ui/modals/Confirm';
import TextInput from '@/shared/ui/TextInput';

/**
 * Page component of supplier management
 */
const SupplierListPage: Component = () => {
	const app = useApp();

	const supplierRepository = useSupplierRespository();

	app.setPageTitle('仕入先');

	const [searchSupplierName, setSearchSupplierName] = createSignal<string>('');

	const [records, setRecords] = createSignal<SupplierTableRecord[]>([]);

	const [supplier, setSupplier] = createSignal<Supplier | null>(null);

	const [supplierInput, setSupplierInput] = createSignal<SupplierInputValue | null>(null);

	const [addDialogOpen, setAddDialogOpen] = createSignal(false);

	const [editDialogOpen, setEditDialogOpen] = createSignal(false);

	const [confirmOpen, setConfirmOpen] = createSignal(false);

	/**
	 * Reload suppliers
	 */
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

	/**
	 * Select supplier for editing
	 */
	const select = async (record: SupplierTableRecord) => {
		const supp = await supplierRepository.get(record.id);

		if (!supp) {
			app.handleError('システムエラーが発生しました。', new Error('ID is missing'));
			return;
		}

		setSupplier(supp);

		setSupplierInput({
			supplierName: supp.name,
		});

		setEditDialogOpen(true);
	};

	/**
	 * Register new supplier
	 */
	const add = async (input: SupplierInputValue) => {

		try {
			await supplierRepository.add({
				name: input.supplierName,
			});
		} catch (err) {
			app.handleError('仕入先の登録に失敗しました。', err);
			return;
		}

		app.toastInfo('仕入先を登録しました。');

		reload();
	};

	/**
	 * Update supplier
	 */
	const edit = async (input: SupplierInputValue) => {
		const id = supplier()?.id;

		if (!id) {
			app.handleError('システムエラーが発生しました。', new Error('ID is missing'));
			return;
		}

		try {
			await supplierRepository.edit({
				id,
				name: input.supplierName,
			});
		} catch (err) {
			app.handleError('仕入先の更新に失敗しました。', err);
			return;
		}

		app.toastInfo('仕入先を更新しました。');

		reload();
	};

	/**
	 * Handle delete button click
	 */
	const onDeleteClick = async (record: SupplierTableRecord) => {
		const supp = await supplierRepository.get(record.id);

		if (!supp) return;

		setSupplier(supp);

		setConfirmOpen(true);
	};

	/**
	 * Handle delete confirmation
	 */
	const onDeleteConfirm = async () => {
		setConfirmOpen(false);

		const id = supplier()?.id;

		if (!id) {
			app.handleError('システムエラーが発生しました。', new Error('ID is missing'));
			return;
		}

		try {
			await supplierRepository.delete(id);
		} catch (err) {
			app.handleError('仕入先の削除に失敗しました。', err);
			return;
		}

		app.toastInfo('仕入先を削除しました。');

		reload();
	};

	onMount(() => {
		reload();
	});

	return (
		<article class="size-full flex flex-col gap-10 p-10">
			{/* Search and add section */}
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

			{/* Supplier table */}
			<section>
				<Show
					when={records().length > 0}
					fallback={
						<div class="flex justify-center">
							<p class="text-gray-400">データがありません</p>
						</div>
					}
				>
					<SupplierTable
						value={records()}
						onSelect={select}
						onDelete={onDeleteClick}
					/>
				</Show>
			</section>

			{/* Modal for register supplier */}
			<dialog class="modal" open={addDialogOpen()}>
				<div class="modal-box w-100">
					<button
						type="button"
						class="hover:cursor-pointer hover:opacity-50 absolute right-3 top-3"
						onclick={() => setAddDialogOpen(false)}
					>
						<XIcon />
					</button>
					<SupplierInput onAction={add} actionLabel="追加" />
				</div>
				<button
					type="button"
					class="modal-backdrop"
					onclick={() => setAddDialogOpen(false)}
				></button>
			</dialog>

			{/* Modal for edit supplier */}
			<dialog class="modal" open={editDialogOpen()}>
				<div class="modal-box w-100">
					<button
						type="button"
						class="hover:cursor-pointer hover:opacity-50 absolute right-3 top-3"
						onclick={() => setEditDialogOpen(false)}
					>
						<XIcon />
					</button>
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

			<Confirm
				open={confirmOpen()}
				onCancel={() => setConfirmOpen(false)}
				onConfirm={onDeleteConfirm}
			>
				<div class="flex flex-col items-center gap-3">
					<p>一度削除すると元に戻せません。</p>
					<p>本当にこの仕入先を削除しますか？</p>
					<p class='mt-3 text-lg text-base-content'>{supplier()?.name}</p>
				</div>
			</Confirm>
		</article>
	);
};

export default SupplierListPage;
