import type { Component, JSXElement } from 'solid-js';

export interface TextInputProps {
	value?: string | null;
	label?: string | null;
	onInput?: (value: string) => Promise<unknown> | unknown;
	onChange?: (value: string) => Promise<unknown> | unknown;
	validate?: boolean;
	hint?: string | null;
	required?: boolean;
	pattern?: string | null;
	prefix?: string | JSXElement;
}

const TextInput: Component<TextInputProps> = (props) => {
	return (
		<label class="floating-label input">
			{ props.prefix ? <p>{props.prefix}</p> : '' }
			{props.label ? <span>{props.label}</span> : null}
			<input
				type="text"
				value={props.value ?? ''}
				placeholder={props.label ?? undefined}
				class={`${props.validate ? 'validator' : ''}`}
				required={props.required}
				pattern={props.pattern ?? undefined}
                oninput={e => props.onInput?.(e.currentTarget.value)}
                onchange={e => props.onChange?.(e.currentTarget.value)}
			/>
			{props.hint ? <p class="validator-hint">{props.hint}</p> : ''}
		</label>
	);
};

export default TextInput;
