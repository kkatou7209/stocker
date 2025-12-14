import { StoreIcon, WheatIcon } from 'lucide-solid';
import { type Component, createSignal, onMount, Show } from 'solid-js';
import { useApp } from '@/app/contexts/AppContext';
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
import TextInput from '@/shared/ui/TextInput';

const SupplyListPage: Component = () => {
	const app = useApp();
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
	};

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

	const add = async (supply: SupplyInputValue) => {
		await supplyRepository.add(supply);

		setAddDialogOpen(false);

		await reload();
	};

	const edit = async (supply: SupplyInputValue) => {
		const id = selectedSupply()?.id;

		if (!id) return;

		await supplyRepository.edit({
			id,
			name: supply.supplyName,
			unitName: supply.unitName,
		});

		setEditDialogOpen(false);

		await reload();
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
						prefix={<WheatIcon class='size-4'/>}
						label="仕入品名"
						onInput={(value) => {
							setSearchSupplyName(value);
							reload();
						}}
					/>
					<TextInput
						prefix={<StoreIcon class='size-4'/>}
						label="仕入先名"
						onInput={(value) => {
							setSearchSupplierName(value);
							reload();
						}}
						onChange={reload}
					/>
				</div>
				<Button onClick={() => setAddDialogOpen(true)}>追加</Button>
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
								onSelect={select}
							/>
						))}
					</div>
				</Show>
			</section>

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
		</article>
	);
};

export default SupplyListPage;
