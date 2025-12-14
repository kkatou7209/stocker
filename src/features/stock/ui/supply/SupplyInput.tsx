import { type Component, createEffect, createSignal } from 'solid-js';
import Button from '@/shared/ui/Button';

export interface SupplyInputValue {
	supplyName: string;
	unitName: string;
	supplierId: string;
}

export interface SupplyInputSupplierModel {
	supplierId: string;
	supplierName: string;
}

export interface SupplyInputProps {
	actionLabel?: string;
	value?: SupplyInputValue | null;
	suppliers?: SupplyInputSupplierModel[] | null;
	onAction?: (value: SupplyInputValue) => Promise<void> | void;
}

const SupplyInput: Component<SupplyInputProps> = (props) => {
	const [supplyName, setSupplyName] = createSignal('');

	const [unitName, setUnitName] = createSignal('');

	const [supplierId, setSupplierId] = createSignal('');

	const action = (e: SubmitEvent) => {
		e.preventDefault();
		if (!props.onAction) return;

		props.onAction({
			supplierId: supplierId(),
			supplyName: supplyName(),
			unitName: unitName(),
		});
	};

	createEffect(() => {
		setSupplyName(props.value?.supplyName ?? '');
		setUnitName(props.value?.unitName ?? '');
		setSupplierId(props.value?.supplierId ?? '');
	});

	return (
		<form class="fieldset contents" onsubmit={action}>
			<div class="card">
				<div class="card-body flex flex-col gap-10">
					<div class="flex flex-col gap-10">
						<label class="floating-label">
							<span>仕入品名</span>
							<input
								type="text"
								class="input validator"
								placeholder="仕入品名"
								value={supplyName()}
								required
								pattern="[^\s]+"
								oninput={(e) =>
									setSupplyName(e.currentTarget.value)
								}
							/>
							<p class="validator-hint">仕入品名は必須です</p>
						</label>
						<label class="floating-label">
							<span>単位</span>
							<input
								type="text"
								class="input validator"
								placeholder="単位"
								required
								pattern="[^\s]+"
								value={unitName()}
								oninput={(e) =>
									setUnitName(e.currentTarget.value)
								}
							/>
							<p class="validator-hint">単位は必須です</p>
						</label>
						<label class="floating-label">
							<span>仕入先</span>
							<select
								class="select validator"
								required
								value={supplierId()}
								onchange={(e) =>
									setSupplierId(e.currentTarget.value)
								}
							>
								<option disabled value="">
									仕入先を選択
								</option>
								{props.suppliers?.map((s) => (
									<option value={s.supplierId}>
										{s.supplierName}
									</option>
								))}
							</select>
							<p class="validator-hint">仕入先は必須です</p>
						</label>
					</div>
					<div class="card-actions">
						<Button type="submit" class="w-full">
							<span>{props.actionLabel ?? '決定'}</span>
						</Button>
					</div>
				</div>
			</div>
		</form>
	);
};

export default SupplyInput;
