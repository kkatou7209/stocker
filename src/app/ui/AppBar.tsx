import {
	FilePenIcon,
	MoonIcon,
	PanelLeftCloseIcon,
	PanelLeftOpenIcon,
	SunIcon,
} from 'lucide-solid';
import { type Component, createEffect, createSignal } from 'solid-js';
import { useApp } from '@/app/contexts/AppContext';

const AppBar: Component = () => {
	const app = useApp();

	const [isDark, setIsDark] = createSignal(false);

	createEffect(() => {
		app.setIsDark(isDark());
	});

	return (
		<header class="navbar shadow-md flex items-center gap-5">
			<button
				type="button"
				class="btn btn-ghost drawer-button flex-none"
				onClick={app.drawer.toggle}
			>
				{app.isDrawerOpen() ? (
					<PanelLeftCloseIcon class="size-4" />
				) : (
					<PanelLeftOpenIcon class="size-4" />
				)}
			</button>
			<h2 class="text-xl flex-1">{app.pageTitle()}</h2>
			<nav class="menu">
				<ul class="flex items-center gap-5">
					<li>
						<a
							href="/journal"
							class="text-nowrap flex items-center gap-3 h-9"
						>
							<FilePenIcon class="size-4" />
						</a>
					</li>
					<li>
						<label class="swap swap-rotate">
							<input
								type="checkbox"
								class="theme-controller"
								value="sunset"
								checked={isDark()}
								onchange={(e) =>
									setIsDark(e.currentTarget.checked)
								}
							/>

							<i class="swap-off">
								<SunIcon class="size-4" />
							</i>
							<i class="swap-on">
								<MoonIcon class="size-4" />
							</i>
						</label>
					</li>
				</ul>
			</nav>
		</header>
	);
};

export default AppBar;
