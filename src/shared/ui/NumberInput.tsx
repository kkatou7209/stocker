import { create } from 'node:domain';
import { _ } from 'node_modules/tailwindcss/dist/colors-b_6i0Oi7';
import {
	type Component,
	createEffect,
	createSignal,
	type JSXElement,
} from 'solid-js';
import { useFormat } from '@/shared/lib/format';

const NumberInput: Component<{
	value?: number;
	suffix?: JSXElement;
	prefix?: JSXElement;
	onChange?: (value: number) => unknown;
}> = (props) => {
	const formatter = useFormat('ja-JP');

	const [value, setValue] = createSignal(props.value ?? 0);

	const onInput = (e: { currentTarget: HTMLInputElement }) => {
		// Remove non-numeric and non-dot characters
		let _value = e.currentTarget.value.replace(/[^\d.]/g, '');

		// Handle leading dot
		if (_value.startsWith('.')) {
			_value = `0${_value}`;
		}

		// Allow only one dot
		if (_value.indexOf('.') !== _value.lastIndexOf('.')) {
			const parts = _value.split('.');

			// Keep only the first dot
			_value = `${parts[0]}.${parts[1]}`;
		}

		let num = Number(_value);

		// If not a number, reset to 0
		if (Number.isNaN(num)) {
			num = 0;
		}

		e.currentTarget.value = _value;
	};

	const onChange = (e: { currentTarget: HTMLInputElement }) => {
		// Remove non-numeric and non-dot characters
		let _value = e.currentTarget.value.replace(/[^\d.]/g, '');

		let num = Number(_value);

		// If not a number, reset to 0
		if (Number.isNaN(num)) {
			num = 0;
			_value = '0';
		}

		setValue(num);

		props.onChange?.(value());
		e.currentTarget.value = formatter.number.format(value());
	};

	createEffect(() => {
		setValue(props.value ?? 0);
	});

	return (
		<label class="floating-label input">
			{props.prefix ? <p class="label">{props.prefix}</p> : ''}
			<input
				type="text"
				class="text-end"
				value={formatter.number.format(value())}
				oninput={onInput}
				onchange={onChange}
			/>
			{props.suffix ? <p class="label">{props.suffix}</p> : ''}
		</label>
	);
};

export default NumberInput;
