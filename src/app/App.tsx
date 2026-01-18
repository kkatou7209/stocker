import { type Component, createEffect, createSignal, on, onMount } from 'solid-js';
import '@/app/App.css';
import { Router } from '@solidjs/router';
import { getVersion } from '@tauri-apps/api/app';
import { error } from '@tauri-apps/plugin-log';
import { InfoIcon } from 'lucide-solid';
import { AppContext, type AppContextValue } from '@/app/contexts/AppContext';
import { routes } from '@/app/routes';
import AppBar from '@/app/ui/AppBar';
import SideNavigation from '@/app/ui/SideNavigation';
import { getCurrentTheme, setTheme } from '@/shared/api/tauri/theme';
import { ErrorDialog } from './ui/ErrorDialog';
import { Updater } from './ui/Updater';

/**
 * Application root component.
 */
const App: Component = () => {
	// Application version
	const [version, setVersion] = createSignal<string | null>(null);

	// Error dialog state
	const [showError, setShowError] = createSignal(false);

	// Error message state
	const [errorMessage, setErrorMessage] = createSignal<string | null>(null);

	// Dark mode state
	const [isDark, setIsDark] = createSignal(false);

	// Page title state
	const [pageTitle, setPageTitle] = createSignal('');

	// Side drawer state
	const [isDrawerOpen, setOpen] = createSignal(false);

	// Toast info state
	const [showToastInfo, setShowToastInfo] = createSignal(false);

	// Toast message state
	const [toastMessage, setToastMessage] = createSignal('');

	// Shows a toast info message
	const toastInfo = (message: string) => {
		setToastMessage(message);

		setShowToastInfo(true);

		const id = setTimeout(() => {
			setShowToastInfo(false);
			clearTimeout(id);
		}, 1500);
	};

	// Handles an error by logging and showing the error dialog
	const handleError = (message: string, err: unknown) => {
		if (err instanceof Error) {
			error(`${message}\n${err.message}\n${err.stack}`);
		} else {
			error(`${message}\n${String(err)}`);
		}

		setErrorMessage(message);
		setShowError(true);
	};

	// Application context value
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

	createEffect(
		on(isDark, async (dark) => {

			try {
				await setTheme(dark ? 'dark' : 'light');
			} catch (error) {
				handleError('システムエラーが発生しました。', error);
			}

		}, {
			defer: true,
		}),
	);

	onMount(async () => {
		/// Get current theme
		const theme = await getCurrentTheme();

		console.log('Current theme:', theme);

		setIsDark(theme === 'dark');

		// Get application version
		const ver = await getVersion();

		setVersion(ver);
	});

	return (
		<AppContext.Provider value={contextValue}>
			<SideNavigation open={isDrawerOpen}>
				<div class="flex flex-col">
					<AppBar />
					<main class="w-full grow">
						<Router>{routes}</Router>
					</main>
					<footer class="f-4">
						<section class="size-full flex justify-end items-center px-7">
							<span class="text-base-300">v {version()}</span>
						</section>
					</footer>
				</div>
			</SideNavigation>

			{/* Toast info message */}
			<div
				class={`top-10 z-100 toast toast-top toast-center transition-opacity duration-150 ${showToastInfo() ? 'opacity-100' : 'opacity-0'}`}
			>
				<div class="alert alert-horizontal">
					<InfoIcon class="stroke-info" />
					{toastMessage()}
				</div>
			</div>

			{/* Error dialog */}
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
