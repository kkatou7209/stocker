import {
	type Accessor,
	type Component,
	createSignal,
	type Setter,
} from 'solid-js';
import {
	type FormatterLocale,
	useFormat,
} from '@/shared/lib/format';

export interface CurrencyInputProps {
	value?: Accessor<number>;
	setValue?: Setter<number>;
	locale?: FormatterLocale;
    class?: string;
    style?: string;
    placeholder?: string;
}

/**
 * input field of currency
 */
const CurrencyInput: Component<CurrencyInputProps> = (props) => {
	const formmater = useFormat(props.locale ?? 'ja-JP');

	const [defaultValue, setDefaultValue] = createSignal(props.value?.() ?? 0);

    const value = props.value ?? defaultValue;

    const setValue = props.setValue ?? setDefaultValue;

	const handleInput = (
		e: InputEvent & { currentTarget: HTMLInputElement },
	) => {
		const number = e.currentTarget.value.replace(/[^0-9]+/g, '');

		const price = Number(number);

		if (!Number.isNaN(price)) {
            setValue(price);
			return;
		}
	};

	return <input
        type="text"
        value={formmater.number.format(value())}
        oninput={handleInput}
        class={props.class}
        style={props.style}
        placeholder={props.placeholder}
    />;
};

export default CurrencyInput;
