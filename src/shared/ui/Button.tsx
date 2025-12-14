import { children, type ParentComponent } from 'solid-js';
import type { JSX } from 'solid-js/h/jsx-runtime';

export interface ButtonProps {
	class?: string;
	type?: HTMLButtonElement['type'];
	color?: 'primary' | 'info' | 'ghost' | 'link' | 'outline' | 'soft';
	children?: JSX.Element;
	onClick?: () => unknown;
}

const Button: ParentComponent<ButtonProps> = (props) => {
	const resolved = children(() => props.children);

	return (
		<button
			type={props.type ?? 'button'}
			class={`btn ${props.color ? `btn-${props.color}` : 'btn-primary'} ${props.class}`}
			onclick={props.onClick}
		>
			{resolved()}
		</button>
	);
};

export default Button;
