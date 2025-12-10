import { SearchIcon } from 'lucide-solid';
import { type Component, createSignal, onMount } from 'solid-js';
import { useApp } from '@/app/contexts/AppContext';
import type { Supply } from '@/entities/stock/models/supply';
import { SupplierId } from '@/entities/stock/values/supplier-id';
import { SupplyId } from '@/entities/stock/values/supply-id';
import { SupplyName } from '@/entities/stock/values/supply-name';
import { UnitName } from '@/entities/stock/values/unit-name';
import { listSuppliesAccordionValues } from '@/features/stock/api/list-supplies-accordion-values';
import type {
	SuppliesAccordionValue,
	SupplyAccordionValue,
} from '@/features/stock/models/supplies-accordion-value';
import SuppliesAccordion from '@/features/stock/ui/SuppliesAccordion';
import SupplyInput, {
	type SupplyInputSupplierModel,
	type SupplyInputValue,
} from '@/features/stock/ui/SupplyInput';
import { useApi } from '@/shared/api';
import type {
	AddSupplyCommand,
	UpdateSupplyCommand,
} from '@/shared/api/usecases';
import Button from '@/shared/ui/Button';
import TextInput from '@/shared/ui/TextInput';

const SupplyListPage: Component = () => {
	const app = useApp();
	const api = useApi();

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

	const select = async (value: SupplyAccordionValue) => {
		setSelectedSupply(null);

		const id = SupplyId.of(value.supplyId);

		const supply = await api.getSupply(id);

		if (!supply) return;

		setSelectedSupply(supply);

		setSupplyInput({
			supplyName: supply.name().value(),
			unitName: supply.unitName().value(),
			supplierId: supply.supplierId().value(),
		});

		setEditDialogOpen(true);
	};

	const add = async (supply: SupplyInputValue) => {
		const command: AddSupplyCommand = {
			name: SupplyName.of(supply.supplyName),
			supplierId: SupplierId.of(supply.supplierId),
			unitName: UnitName.of(supply.unitName),
		};

		await api.addSupply(command);

		const values = await listSuppliesAccordionValues();

		setSuppliesAccordionValues(values);

		setAddDialogOpen(false);
	};

	const edit = async (supply: SupplyInputValue) => {
		const id = selectedSupply()?.id();

		if (!id) return;

		const command: UpdateSupplyCommand = {
			supplyId: id,
			supplyName: SupplyName.of(supply.supplyName),
			supplierId: SupplierId.of(supply.supplierId),
			unitName: UnitName.of(supply.unitName),
		};

		await api.updateSupply(command);

		const values = await listSuppliesAccordionValues();

		setSuppliesAccordionValues(values);

		setEditDialogOpen(false);
	};

	onMount(async () => {
		const values = await listSuppliesAccordionValues();

		setSuppliesAccordionValues(values);

		const supps = await api.listSuppliers();

		setSuppliers(
			supps.map((s) => ({
				supplierId: s.id().value(),
				supplierName: s.name().value(),
			})),
		);
	});

	return (
		<article class="size-full flex flex-col gap-10 p-10">
			<section class="flex justify-between items-center">
				<div class="flex items-center gap-5">
					<TextInput
						label="仕入品名"
						onInput={(value) => setSearchSupplyName(value)}
					/>
					<TextInput
						label="仕入先名"
						onInput={(value) => setSearchSupplierName(value)}
					/>
					<Button color='info'>
						<SearchIcon />
					</Button>
				</div>
				<Button onClick={() => setAddDialogOpen(true)}>追加</Button>
			</section>

			<section class="grow h-[70vh] overflow-auto">
				<div class="flex flex-col gap-10 pb-100">
					{suppliesAccordionValues().map((value) => (
						<SuppliesAccordion value={value} onSelect={select} />
					))}
				</div>
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
