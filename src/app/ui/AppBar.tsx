import { PanelLeftCloseIcon, PanelLeftOpenIcon } from 'lucide-solid';
import type { Component } from 'solid-js';
import { useApp } from '@/app/contexts/AppContext';

const AppBar: Component = () => {
	const app = useApp();

	return (
		<header class="navbar shadow-md flex items-center gap-5">
			<button
				type="button"
				class="btn btn-ghost drawer-button"
				onClick={app.drawer.toggle}
			>
				{app.isDrawerOpen() ? (
					<PanelLeftCloseIcon class="size-4" />
				) : (
					<PanelLeftOpenIcon class="size-4" />
				)}
			</button>
			<h2 class="text-xl">{app.pageTitle()}</h2>
		</header>
	);
};

export default AppBar;
