import { children, type ParentComponent } from 'solid-js';

export interface ButtonProps {
	class?: string;
    color?: 'primary' | 'info' | 'ghost';
	onClick?: () => unknown;
}

const Button: ParentComponent<ButtonProps> = (props) => {
	const resolved = children(() => props.children);

	return (
		<button
			type="button"
			class={`btn ${props.color ? `btn-${props.color}` : 'btn-primary'}`}
			onclick={props.onClick}
		>
			{resolved()}
		</button>
	);
};

export default Button;
