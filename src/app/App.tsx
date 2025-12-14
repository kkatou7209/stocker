import { type Component, createEffect, createSignal, onMount } from 'solid-js';
import '@/app/App.css';
import { Router } from '@solidjs/router';
import { routes } from '@/app/routes';
import SideNavigation from '@/app/ui/SideNavigation';
import { AppContext, type AppContextValue } from './contexts/AppContext';
import AppBar from './ui/AppBar';

const App: Component = () => {

	const [isDark, setIsDark] = createSignal(false);

	const [pageTitle, setPageTitle] = createSignal('');

	const [isDrawerOpen, setOpen] = createSignal(false);

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
	};

	onMount(() => {
		setIsDark(localStorage.getItem('theme') === 'dark');
	});

	return (
		<AppContext.Provider value={contextValue}>
			<SideNavigation open={isDrawerOpen}>
				<AppBar />
				<main class='w-full'>
					<Router>{routes}</Router>
				</main>
			</SideNavigation>
		</AppContext.Provider>
	);
};

export default App;
