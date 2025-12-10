import {
	type Accessor,
	type Component,
	createEffect,
	createSignal,
} from 'solid-js';
import type { Supplier } from '@/entities/stock/models/supplier';
import { useApi } from '@/shared/api';

export interface SupplierInputValue {
    supplierName: string;
}

export interface SupplierInputProps {
    value?: SupplierInputValue | null;
    actionLabel?: string;
    onAction?: (value: SupplierInputValue) => Promise<void> | void;
}

const SupplierInput: Component<SupplierInputProps> = (props) => {

    const [supplierName, setSupplierName] = createSignal('');

    const submit = (e: SubmitEvent) => {
        e.preventDefault();

        const value: SupplierInputValue = {
            supplierName: supplierName(),
        }

        props.onAction?.(value);
    }

	createEffect(async () => {

        setSupplierName(props.value?.supplierName ?? '');
    });

	return (
        <form class='fieldset contents' onsubmit={submit}>
            <div class="crad">
                <div class='card-body flex flex-col gap-10'>
                    <div class='flex flex-col gap-10'>
                        <label class='floating-label'>
                            <span>仕入先名</span>
                            <input
                                type="text"
                                value={supplierName()}
                                class='input validator'
                                placeholder='仕入先名'
                                required
                                pattern='[^\s]+'
                                oninput={e => setSupplierName(e.currentTarget.value)}
                            />
                            <p class='validator-hint'>仕入先名は必須です</p>
                        </label>
                    </div>
                    <div class='card-actions'>
                        <button type='submit' class='btn w-full btn-info'>
                            { props.actionLabel ?? '決定' }
                        </button>
                    </div>
                </div>
            </div>
        </form>
	);
};

export default SupplierInput;