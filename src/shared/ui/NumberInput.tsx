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
		let value = e.currentTarget.value
			.replace(/[^-\d.]+/g, '')
			.replace(/^0+/, '');

		value = value === '' ? '0' : value;

		const num = Number(value);

		if (Number.isNaN(num)) {
			props.onInput?.(0);
			e.currentTarget.value = value;
			return;
		}

		props.onInput?.(num);
		e.currentTarget.value = value;
	};

	const onChange = (e: { currentTarget: HTMLInputElement }) => {
		let value = e.currentTarget.value
            .replace(/[^-\d.]+/g, '')
            .replace(/^0+/, '')
            .replace(/^-+(?!\d+)/, '');

		value = value === '' ? '0' : value;

		const num = Number(value);

		if (Number.isNaN(num)) {
			props.onChange?.(0);
			e.currentTarget.value = value;
			return;
		}

		props.onChange?.(num);
		e.currentTarget.value = value;
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
