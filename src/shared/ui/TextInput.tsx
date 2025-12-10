import type { Component } from 'solid-js';

export interface TextInputProps {
	value?: string;
	label?: string;
	onInput?: (value: string) => Promise<unknown> | unknown;
	validate?: boolean;
	hint?: string;
	required?: boolean;
	pattern?: string;
}

const TextInput: Component<TextInputProps> = (props) => {
	return (
		<label class="floating-label">
			{props.label ? <span>{props.label}</span> : null}
			<input
				type="text"
				value={props.value ?? ''}
				placeholder={props.label}
				class={`input ${props.validate ? 'validator' : ''}`}
				required={props.required}
				pattern={props.pattern}
			/>
			{props.hint ? <p class="validator-hint">{props.hint}</p> : ''}
		</label>
	);
};

export default TextInput;
