import type { Component } from 'solid-js';
import { useFormat } from '@/shared/lib/format';

const NumberInput: Component<{
	value?: number;
	suffix?: string;
	onInput?: (value: number) => unknown;
	onChange?: (value: number) => unknown;
}> = (props) => {
	const formatter = useFormat('ja-JP');

	const onInput = (e: { currentTarget: HTMLInputElement }) => {

		// Remove non-numeric and non-dot characters
		let value = e.currentTarget.value.replace(/[^\d.]/g, '');

		// Handle leading dot
		if (value.startsWith('.')) {
			value = `0${value}`;
		}

		// Allow only one dot
		if (value.indexOf('.') !== value.lastIndexOf('.')) {

			const parts = value.split('.');

			// Keep only the first dot
			value = `${parts[0]}.${parts[1]}`;
		}

		const num = Number(value);

		// If not a number, reset to 0
		if (Number.isNaN(num)) {
			props.onInput?.(0);
			e.currentTarget.value = '0';
			return;
		}

		props.onInput?.(num);
		e.currentTarget.value = value;
	};

	const onChange = (e: { currentTarget: HTMLInputElement }) => {
		
		// Remove non-numeric and non-dot characters
		const value = e.currentTarget.value.replace(/[^\d.]/g, '');

		const num = Number(value);

		// If not a number, reset to 0
		if (Number.isNaN(num)) {
			props.onChange?.(0);
			e.currentTarget.value = '0';
			return;
		}

		props.onChange?.(num);
		e.currentTarget.value = num.toString();
	};
	return (
		<label class="floating-label input">
			<input
				type="text"
				class="text-end"
				value={formatter.number.format(props.value ?? 0)}
				oninput={onInput}
				onchange={onChange}
			/>
			{props.suffix ? <p class="label">{props.suffix}</p> : ''}
		</label>
	);
};

export default NumberInput;
