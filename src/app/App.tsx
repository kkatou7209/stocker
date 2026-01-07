import { type Component, createSignal, ErrorBoundary, onMount } from 'solid-js';
import '@/app/App.css';
import { Router } from '@solidjs/router';
import { InfoIcon } from 'lucide-solid';
import { routes } from '@/app/routes';
import SideNavigation from '@/app/ui/SideNavigation';
import { ErrorPage } from '@/pages/Error';
import { AppContext, type AppContextValue } from './contexts/AppContext';
import AppBar from './ui/AppBar';

const App: Component = () => {
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
	};

	onMount(() => {
		setIsDark(localStorage.getItem('theme') === 'dark');
	});

	return (
		<AppContext.Provider value={contextValue}>
			<SideNavigation open={isDrawerOpen}>
				<AppBar />
				<main class="size-full">
					<Router>{routes}</Router>
				</main>
			</SideNavigation>

			<div
				class={`top-10 z-100 toast toast-top toast-center transition-opacity duration-150 ${showToastInfo() ? 'opacity-100' : 'opacity-0'}`}
			>
				<div class="alert alert-horizontal">
					<InfoIcon class="stroke-info" />
					{toastMessage()}
				</div>
			</div>
		</AppContext.Provider>
	);
};

export default App;
