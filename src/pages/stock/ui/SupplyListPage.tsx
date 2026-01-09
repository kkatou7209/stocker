import { StoreIcon, WheatIcon } from 'lucide-solid';
import { type Component, createSignal, onMount, Show } from 'solid-js';
import { useApp } from '@/app/contexts/AppContext';
import { useError } from '@/app/stores/error';
import type { Supply } from '@/entities/stock/models/supply';
import { useSupplierRespository } from '@/entities/stock/respository/supplier';
import { useSupplyRepository } from '@/entities/stock/respository/supply';
import { listSuppliesAccordionValues } from '@/features/stock/api/list-supplies-accordion-values';
import type {
	SuppliesAccordionValue,
	SupplyAccordionValue,
} from '@/features/stock/models/supplies-accordion-value';
import SuppliesAccordion from '@/features/stock/ui/supply/SuppliesAccordion';
import SupplyInput, {
	type SupplyInputSupplierModel,
	type SupplyInputValue,
} from '@/features/stock/ui/supply/SupplyInput';
import Button from '@/shared/ui/Button';
import { Confirm } from '@/shared/ui/modals/Confirm';
import TextInput from '@/shared/ui/TextInput';

/**
 * Page component of supply management
 */
const SupplyListPage: Component = () => {
	const app = useApp();
	const error = useError();
	const supplyRepository = useSupplyRepository();
	const supplierRepository = useSupplierRespository();

	app.setPageTitle('仕入品');

	const [searchSupplyName, setSearchSupplyName] = createSignal('');

	const [searchSupplierName, setSearchSupplierName] = createSignal('');

	const [addDialogOpen, setAddDialogOpen] = createSignal(false);

	const [editDialogOpen, setEditDialogOpen] = createSignal(false);

	const [suppliesAccordionValues, setSuppliesAccordionValues] = createSignal<
		SuppliesAccordionValue[]
	>([]);

	const [suppliers, setSuppliers] = createSignal<SupplyInputSupplierModel[]>(
		[],
	);

	const [selectedSupply, setSelectedSupply] = createSignal<Supply | null>(
		null,
	);

	const [supplyInput, setSupplyInput] = createSignal<SupplyInputValue | null>(
		null,
	);

	const [confirmOpen, setConfirmOpen] = createSignal(false);

	const [supplierOpeningStates, setSupplierOpeningStates] = createSignal<Map<string, boolean>>(new Map());

	/**
	 * Reload supplies
	 */
	const reload = async () => {

		const suppliers = await supplierRepository.find({
			supplierName: searchSupplierName(),
			supplyName: searchSupplyName(),
		});

		const values: SuppliesAccordionValue[] = suppliers.map((supplier) => {
			const value: SuppliesAccordionValue = {
				supplierId: supplier.id,
				supplierName: supplier.name,
				supplies: supplier.supplies.map((supply) => ({
					supplyId: supply.id,
					supplyName: supply.name,
					unitName: supply.unitName,
				})),
			};

			return value;
		});

		setSuppliesAccordionValues(values);

		for (const supply of suppliesAccordionValues()) {

			if (!supplierOpeningStates().has(supply.supplierId)) {
				supplierOpeningStates().set(supply.supplierId, false);
			}
		}

		setSupplierOpeningStates(new Map(supplierOpeningStates()));

		console.log(supplierOpeningStates());
	};

	/**
	 * Handle accordion toggle
	 */
	const onToggle = (supplierId: string, open: boolean) => {
		supplierOpeningStates().set(supplierId, open);
		setSupplierOpeningStates(new Map(supplierOpeningStates()));
	}

	/**
	 * Select supply for editing
	 */
	const select = async (value: SupplyAccordionValue) => {
		setSelectedSupply(null);

		const supply = await supplyRepository.get(value.supplyId);

		if (!supply) return;

		setSelectedSupply(supply);

		setSupplyInput({
			supplyName: supply.name,
			unitName: supply.unitName,
			supplierId: supply.supplierId,
		});

		setEditDialogOpen(true);
	};

	/**
	 * Register new supply
	 */
	const add = async (supply: SupplyInputValue) => {
		await supplyRepository.add(supply);

		app.toastInfo('仕入品を登録しました。');

		reload();
	};

	/**
	 * Update supply
	 */
	const edit = async (supply: SupplyInputValue) => {
		const id = selectedSupply()?.id;

		if (!id) return;

		await supplyRepository.edit({
			id,
			name: supply.supplyName,
			unitName: supply.unitName,
		});

		app.toastInfo('仕入品を更新しました。');

		await reload();
	};

	/**
	 * Handle delete click
	 */
	const onDeleteClick = async (value: SupplyAccordionValue) => {

		setSelectedSupply(null);

		const supply = await supplyRepository.get(value.supplyId);

		if (!supply) return;

		setSelectedSupply(supply);

		setConfirmOpen(true);
	};

	/**
	 * Handle delete confirmation
	 */
	const onDeleteConfirm = async () => {
		setConfirmOpen(false);

		const id = selectedSupply()?.id;

		if (!id) {
			error.handle(new Error('システムエラーが発生しました。'));
			return;
		}

		try {
			await supplyRepository.delete(id);
		} catch (_) {
			error.handle(new Error('仕入品の削除に失敗しました。'));
		}

		app.toastInfo('仕入品を削除しました。');

		for (const value of suppliesAccordionValues()) {

			if (value.supplies.find((s) => s.supplyId === id)) {
				value.supplies = value.supplies.filter((s) => s.supplyId !== id);
				setSuppliesAccordionValues([...suppliesAccordionValues()]);
				break;
			}
		}
	};

	onMount(async () => {
		const values = await listSuppliesAccordionValues();

		setSuppliesAccordionValues(values);

		const supps = await supplierRepository.list();

		setSuppliers(
			supps.map((s) => ({
				supplierId: s.id,
				supplierName: s.name,
			})),
		);
	});

	return (
		<article class="size-full flex flex-col gap-10 p-10">
			<section class="flex justify-between items-center">
				<div class="flex items-center gap-5">
					<TextInput
						prefix={<WheatIcon class="size-4" />}
						label="仕入品名"
						onInput={(value) => {
							setSearchSupplyName(value);
							reload();
						}}
					/>
					<TextInput
						prefix={<StoreIcon class="size-4" />}
						label="仕入先名"
						onInput={(value) => {
							setSearchSupplierName(value);
							reload();
						}}
						onChange={reload}
					/>
				</div>
				<Button onClick={() => setAddDialogOpen(true)}>
					<WheatIcon class="size-4" />
					<span>追加</span>
				</Button>
			</section>

			<section class="grow h-[70vh] overflow-auto">
				<Show
					when={suppliesAccordionValues().length > 0}
					fallback={
						<div class="flex justify-center">
							<p class="text-gray-400">データがありません</p>
						</div>
					}
				>
					<div class="flex flex-col gap-10 pb-100">
						{suppliesAccordionValues().map((value) => (
							<SuppliesAccordion
								value={value}
								open={() => supplierOpeningStates().get(value.supplierId) ?? false}
								onSelect={select}
								onDelete={onDeleteClick}
								onToggle={(state) => onToggle(value.supplierId, state)}
							/>
						))}
					</div>
				</Show>
			</section>

			{/* Modal for register supply */}
			<dialog class="modal" open={addDialogOpen()}>
				<div class="modal-box w-100">
					<SupplyInput
						actionLabel="追加"
						suppliers={suppliers()}
						onAction={add}
					/>
				</div>
				<button
					type="button"
					class="modal-backdrop"
					onclick={() => setAddDialogOpen(false)}
				></button>
			</dialog>

			{/* Modal for edit supply */}
			<dialog class="modal" open={editDialogOpen()}>
				<div class="modal-box w-100">
					<SupplyInput
						actionLabel="更新"
						suppliers={suppliers()}
						onAction={edit}
						value={supplyInput()}
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
					<p>本当にこの仕入品を削除しますか？</p>
					<p class='mt-3 text-lg text-base-content'>{selectedSupply()?.name}</p>
				</div>
			</Confirm>
		</article>
	);
};

export default SupplyListPage;
