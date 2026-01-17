import { type Component, createEffect, createSignal, onMount } from 'solid-js';
import '@/app/App.css';
import { Router } from '@solidjs/router';
import { error } from '@tauri-apps/plugin-log';
import { InfoIcon } from 'lucide-solid';
import { AppContext, type AppContextValue } from '@/app/contexts/AppContext';
import { routes } from '@/app/routes';
import AppBar from '@/app/ui/AppBar';
import SideNavigation from '@/app/ui/SideNavigation';
import { ErrorDialog } from './ui/ErrorDialog';
import { Updater } from './ui/Updater';

const App: Component = () => {
	const [showError, setShowError] = createSignal(false);

	const [errorMessage, setErrorMessage] = createSignal<string | null>(null);

	const [isDark, setIsDark] = createSignal(false);

	const [pageTitle, setPageTitle] = createSignal('');

	const [isDrawerOpen, setOpen] = createSignal(false);

	const [showToastInfo, setShowToastInfo] = createSignal(false);

	const [toastMessage, setToastMessage] = createSignal('');

	const toastInfo = (message: string) => {
		setToastMessage(message);

		setShowToastInfo(true);

		const id = setTimeout(() => {
			setShowToastInfo(false);
			clearTimeout(id);
		}, 1500);
	};

	const handleError = (message: string, err: unknown) => {

		if (err instanceof Error) {
			error(`${message}\n${err.message}\n${err.stack}`);
		} else {
			error(`${message}\n${String(err)}`);
		}

		setErrorMessage(message);
		setShowError(true);
	};

	const contextValue: AppContextValue = {
		pageTitle,
		isDrawerOpen,
		isDark,
		drawer: {
			open: () => setOpen(true),
			close: () => setOpen(false),
			toggle: () => setOpen(!isDrawerOpen()),
		},
		setPageTitle,
		setIsDark,
		toastInfo,
		handleError,
	};

	createEffect(() => {
		localStorage.setItem('theme', isDark() ? 'dark' : 'light');
	});

	onMount(async () => {
		setIsDark(localStorage.getItem('theme') === 'dark');
	});

	return (
		<AppContext.Provider value={contextValue}>
			<SideNavigation open={isDrawerOpen}>
				<div class="flex flex-col">
					<AppBar />
					<main class="w-full grow">
						<Router>{routes}</Router>
					</main>
				</div>
			</SideNavigation>

			<div
				class={`top-10 z-100 toast toast-top toast-center transition-opacity duration-150 ${showToastInfo() ? 'opacity-100' : 'opacity-0'}`}
			>
				<div class="alert alert-horizontal">
					<InfoIcon class="stroke-info" />
					{toastMessage()}
				</div>
			</div>

			<ErrorDialog
				open={showError()}
				message={errorMessage()}
				reset={() => setShowError(false)}
			/>

			{/* Application updater */}
			<Updater />
		</AppContext.Provider>
	);
};

export default App;
